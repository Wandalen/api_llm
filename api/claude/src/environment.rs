//! Environment configuration and secret validation for Anthropic API

mod private
{
  use crate::secret::Secret;
  use error_tools::untyped::Result;
  use std::path::PathBuf;

  /// Environment configuration for Anthropic API
  #[ derive( Debug, Clone ) ]
  pub struct Environment
  {
    api_key : Secret,
    base_url : String,
  }

  impl Environment
  {
    /// Create new environment
    pub fn new( api_key : Secret ) -> Self
    {
      Self
      {
        api_key,
        base_url : "https://api.anthropic.com".to_string(),
      }
    }

    /// Get API key
    pub fn api_key( &self ) -> &Secret
    {
      &self.api_key
    }

    /// Get base URL
    pub fn base_url( &self ) -> &str
    {
      &self.base_url
    }

    /// Set custom base URL
    #[ must_use ]
    pub fn with_base_url( mut self, base_url : String ) -> Self
    {
      self.base_url = base_url;
      self
    }
  }

  /// Validate that required secrets are available for API operations
  ///
  /// This function checks multiple sources in priority order:
  /// 1. Environment variable `ANTHROPIC_API_KEY`
  /// 2. Workspace secrets file `secret/-secrets.sh`
  ///
  /// # Errors
  ///
  /// Returns detailed error if the secret is not found in any source
  pub fn validate_anthropic_secret() -> Result< String >
  {
    // Try environment variable first
    if let Ok( key ) = std::env::var( "ANTHROPIC_API_KEY" )
    {
      if !key.is_empty()
      {
        return Ok( "environment variable".to_string() );
      }
    }

    // Try workspace secrets
    match Secret::from_workspace()
    {
      Ok( _secret ) => Ok( "workspace secrets".to_string() ),
      Err( e ) =>
      {
        // Get diagnostic information
        let current_dir = std::env::current_dir()
          .map_or_else( | _ | "< unknown >".to_string(), | p | p.display().to_string() );

        let workspace_secret_path = workspace_tools::workspace()
          .ok()
          .and_then( | ws | {
            // Canonicalize root first to get absolute path
            let root = ws.root().canonicalize().ok()?;
            let path = root.join( "secret" ).join( "-secrets.sh" );
            path.canonicalize().ok().or( Some( path ) )
          })
          .map_or_else( || "< not found >".to_string(), | p | p.display().to_string() );

        Err( error_tools::Error::msg(
          format!(
            "❌ ANTHROPIC_API_KEY not found in any source.\n\
             \n\
             Checked locations:\n\
             1. Environment variable : ANTHROPIC_API_KEY (not set)\n\
             2. Workspace secrets : {workspace_secret_path}\n\
             \n\
             Current directory : {current_dir}\n\
             \n\
             To fix this issue:\n\
             \n\
             Option 1: Set environment variable (for examples using from_env()):\n\
             $ export ANTHROPIC_API_KEY=\"sk-ant-api03-...\"\n\
             \n\
             Option 2: Use workspace secrets (for tests using from_workspace()):\n\
             1. Ensure you're in a workspace with secret/-secrets.sh (NO dot prefix)\n\
             2. Add this line to the secrets file:\n\
                export ANTHROPIC_API_KEY=\"sk-ant-api03-...\"\n\
             3. The variable name MUST be exactly 'ANTHROPIC_API_KEY'\n\
             \n\
             Option 3: Source the secrets file manually:\n\
             $ source /path/to/workspace/secret/-secrets.sh\n\
             \n\
             Original error : {e}"
          )
        ))
      }
    }
  }

  /// Get diagnostic information about secret loading configuration
  ///
  /// Returns a report showing all checked locations and their status
  #[ must_use ]
  pub fn secret_diagnostic_info() -> String
  {
    use std::fmt::Write;

    let mut info = String::from( "🔍 Secret Loading Diagnostic Information\n\n" );

    // Check environment variable
    match std::env::var( "ANTHROPIC_API_KEY" )
    {
      Ok( key ) if !key.is_empty() =>
      {
        info.push_str( "✅ ANTHROPIC_API_KEY environment variable : SET\n" );
        let _ = writeln!( &mut info, "   Value : {}...", &key[ ..key.len().min( 20 ) ] );
      }
      _ =>
      {
        info.push_str( "❌ ANTHROPIC_API_KEY environment variable : NOT SET\n" );
      }
    }

    // Check workspace secrets
    match workspace_tools::workspace()
    {
      Ok( ws ) =>
      {
        // Use secret/ directory (NO dot prefix) as per codestyle rulebook
        // Canonicalize workspace root to get absolute path
        match ws.root().canonicalize()
        {
          Ok( workspace_root ) =>
          {
            let secret_dir = workspace_root.join( "secret" );
            let secret_file = secret_dir.join( "-secrets.sh" );

            info.push_str( "\n📁 Workspace Information:\n" );
            let _ = writeln!( &mut info, "   Root : {}", workspace_root.display() );
            let _ = writeln!( &mut info, "   Secret directory : {}", secret_dir.display() );
            let _ = writeln!( &mut info, "   Secret file : {}", secret_file.display() );

        if secret_file.exists()
        {
          info.push_str( "   ✅ Secrets file exists\n" );

          // Try to check if ANTHROPIC_API_KEY is in the file
          if let Ok( content ) = std::fs::read_to_string( &secret_file )
          {
            if content.contains( "ANTHROPIC_API_KEY" )
            {
              info.push_str( "   ✅ ANTHROPIC_API_KEY found in secrets file\n" );
            }
            else
            {
              info.push_str( "   ❌ ANTHROPIC_API_KEY NOT found in secrets file\n" );
              info.push_str( "      Hint : Add this line to the secrets file:\n" );
              info.push_str( "      export ANTHROPIC_API_KEY=\"sk-ant-api03-...\"\n" );
            }
          }
        }
        else
        {
          info.push_str( "   ❌ Secrets file does not exist\n" );
        }
          }
          Err( e ) =>
          {
            let _ = writeln!( &mut info, "\n❌ Failed to resolve workspace root : {e}" );
          }
        }
      }
      Err( e ) =>
      {
        let _ = writeln!( &mut info, "\n❌ Workspace Error : {e}" );
      }
    }

    // Current directory
    if let Ok( cwd ) = std::env::current_dir()
    {
      let _ = writeln!( &mut info, "\n📂 Current Directory : {}", cwd.display() );
    }

    info
  }

  /// Check if workspace secrets directory exists and is properly configured
  ///
  /// # Errors
  ///
  /// Returns error if workspace or secrets directory is not properly configured
  pub fn validate_workspace_structure() -> Result< PathBuf >
  {
    let ws = workspace_tools::workspace()
      .map_err( | e | error_tools::Error::msg( format!( "Workspace error : {e}" ) ) )?;

    // Use secret/ directory (NO dot prefix) as per codestyle rulebook
    // Canonicalize workspace root to get absolute path
    let workspace_root = ws.root().canonicalize()
      .map_err( | e | error_tools::Error::msg(
        format!( "Failed to resolve workspace root to absolute path : {e}" )
      ) )?;

    let secret_dir = workspace_root.join( "secret" );

    if !secret_dir.exists()
    {
      return Err( error_tools::Error::msg(
        format!(
          "Secrets directory does not exist : {}\n\
           Expected location : workspace_root/secret/ (NO dot prefix)",
          secret_dir.display()
        )
      ));
    }

    let secret_file = secret_dir.join( "-secrets.sh" );
    if !secret_file.exists()
    {
      return Err( error_tools::Error::msg(
        format!(
          "Secrets file does not exist : {}\n\
           Create this file and add your API keys.",
          secret_file.display()
        )
      ));
    }

    Ok( secret_file )
  }
}

crate::mod_interface!
{
  exposed use Environment;
  exposed use validate_anthropic_secret;
  exposed use secret_diagnostic_info;
  exposed use validate_workspace_structure;
}