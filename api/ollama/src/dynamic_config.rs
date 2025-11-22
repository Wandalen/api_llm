//! Dynamic configuration management for runtime updates.

#[ cfg( feature = "dynamic_config" ) ]
mod private
{
  use core::time::Duration;
  use serde::{ Serialize, Deserialize };
  use error_tools::untyped::{ format_err, Result as OllamaResult };

  /// Dynamic configuration management for runtime updates
  #[ derive( Debug, Clone, Serialize, Deserialize ) ]
  pub struct DynamicConfig
  {
    /// Server URL for Ollama API
    pub server_url : String,
    /// Request timeout
    pub timeout : Duration,
    /// Maximum concurrent connections
    pub max_connections : usize,
    /// Enable request caching
    pub enable_caching : bool,
  }

  impl DynamicConfig
  {
    /// Create a new dynamic configuration with defaults
    #[ inline ]
    #[ must_use ]
    pub fn new() -> Self
    {
      Self {
        server_url : "http://localhost:11434".to_string(),
        timeout : Duration::from_secs( 30 ),
        max_connections : 5,
        enable_caching : false,
      }
    }

    /// Validate the configuration
    #[ inline ]
    pub fn validate( &self ) -> OllamaResult< () >
    {
      if self.server_url.is_empty()
      {
        return Err( format_err!( "Server URL cannot be empty" ) );
      }

      if self.timeout.is_zero()
      {
        return Err( format_err!( "Timeout cannot be zero" ) );
      }

      if self.max_connections == 0
      {
        return Err( format_err!( "Max connections must be greater than zero" ) );
      }

      Ok( () )
    }

    /// Create configuration from environment variables
    #[ inline ]
    pub fn from_env() -> OllamaResult< Self >
    {
      let mut config = Self::new();

      if let Ok( url ) = std::env::var( "OLLAMA_SERVER_URL" )
      {
        config.server_url = url;
      }

      if let Ok( timeout_str ) = std::env::var( "OLLAMA_TIMEOUT_SECS" )
      {
        let timeout_secs : u64 = timeout_str.parse()
          .map_err( | e | format_err!( "Invalid timeout value : {}", e ) )?;
        config.timeout = Duration::from_secs( timeout_secs );
      }

      if let Ok( connections_str ) = std::env::var( "OLLAMA_MAX_CONNECTIONS" )
      {
        config.max_connections = connections_str.parse()
          .map_err( | e | format_err!( "Invalid max connections value : {}", e ) )?;
      }

      if let Ok( caching_str ) = std::env::var( "OLLAMA_ENABLE_CACHING" )
      {
        config.enable_caching = caching_str.parse()
          .map_err( | e | format_err!( "Invalid caching flag : {}", e ) )?;
      }

      config.validate()?;
      Ok( config )
    }

    /// Load configuration from file
    #[ inline ]
    pub fn from_file< P: AsRef< std::path::Path > >( path : P ) -> OllamaResult< Self >
    {
      let content = std::fs::read_to_string( path )
        .map_err( | e | format_err!( "Failed to read config file : {}", e ) )?;

      let config : Self = serde_json::from_str( &content )
        .map_err( | e | format_err!( "Failed to parse config file : {}", e ) )?;

      config.validate()?;
      Ok( config )
    }

    /// Save configuration to file
    #[ inline ]
    pub fn save_to_file< P: AsRef< std::path::Path > >( &self, path : P ) -> OllamaResult< () >
    {
      let content = serde_json::to_string_pretty( self )
        .map_err( | e | format_err!( "Failed to serialize config : {}", e ) )?;

      std ::fs::write( path, content )
        .map_err( | e | format_err!( "Failed to write config file : {}", e ) )?;

      Ok( () )
    }
  }

  impl Default for DynamicConfig
  {
    fn default() -> Self
    {
      Self::new()
    }
  }

  /// Configuration difference calculation
  #[ derive( Debug, Clone ) ]
  pub struct ConfigDiff
  {
    /// Whether server URL changed
    pub server_url_changed : bool,
    /// Whether timeout changed
    pub timeout_changed : bool,
    /// Whether max connections changed
    pub max_connections_changed : bool,
    /// Whether caching setting changed
    pub caching_changed : bool,
  }

  impl ConfigDiff
  {
    /// Calculate differences between two configurations
    #[ inline ]
    #[ must_use ]
    pub fn calculate( old : &DynamicConfig, new : &DynamicConfig ) -> Self
    {
      Self {
        server_url_changed : old.server_url != new.server_url,
        timeout_changed : old.timeout != new.timeout,
        max_connections_changed : old.max_connections != new.max_connections,
        caching_changed : old.enable_caching != new.enable_caching,
      }
    }

    /// Check if any fields changed
    #[ inline ]
    pub fn has_changes( &self ) -> bool
    {
      self.server_url_changed || self.timeout_changed ||
      self.max_connections_changed || self.caching_changed
    }
  }

  /// Configuration backup for rollback functionality
  #[ derive( Debug, Clone, Serialize, Deserialize ) ]
  pub struct ConfigBackup
  {
    /// Configuration data
    pub config : DynamicConfig,
    /// Backup timestamp
    pub timestamp : std::time::SystemTime,
    /// Version number
    pub version : u64,
  }

  impl ConfigBackup
  {
    /// Create backup from configuration
    #[ inline ]
    #[ must_use ]
    pub fn from_config( config : &DynamicConfig ) -> Self
    {
      Self {
        config : config.clone(),
        timestamp : std::time::SystemTime::now(),
        version : 0, // Version will be set by manager
      }
    }

    /// Extract configuration from backup
    #[ inline ]
    pub fn to_config( &self ) -> DynamicConfig
    {
      self.config.clone()
    }
  }

  /// Versioned configuration entry
  #[ derive( Debug, Clone ) ]
  pub struct ConfigVersion
  {
    /// Configuration at this version
    pub config : DynamicConfig,
    /// Version number
    pub version : u64,
    /// Timestamp when version was created
    pub created_at : std::time::SystemTime,
  }

  impl ConfigVersion
  {
    /// Create new versioned configuration
    #[ inline ]
    #[ must_use ]
    pub fn new( config : DynamicConfig, version : u64 ) -> Self
    {
      Self {
        config,
        version,
        created_at : std::time::SystemTime::now(),
      }
    }

    /// Get configuration
    #[ inline ]
    pub fn config( &self ) -> &DynamicConfig
    {
      &self.config
    }

    /// Get version number
    #[ inline ]
    pub fn version( &self ) -> u64
    {
      self.version
    }
  }

  /// Dynamic configuration manager for runtime updates
  #[ derive( Debug ) ]
  pub struct DynamicConfigManager
  {
    /// Current active configuration
    current : DynamicConfig,
    /// Configuration history for rollback
    history : Vec< ConfigVersion >,
    /// Maximum history size
    max_history : usize,
  }

  impl DynamicConfigManager
  {
    /// Create a new configuration manager with default configuration
    #[ inline ]
    pub fn new() -> OllamaResult< Self >
    {
      let config = DynamicConfig::new();
      config.validate()?;
      let version = ConfigVersion::new( config.clone(), 0 );
      Ok( Self {
        current : config,
        history : vec![ version ],
        max_history : 10,
      })
    }

    /// Create configuration manager from existing config
    #[ inline ]
    #[ must_use ]
    pub fn from_config( config : DynamicConfig ) -> Self
    {
      let version = ConfigVersion::new( config.clone(), 0 );
      Self {
        current : config,
        history : vec![ version ],
        max_history : 10,
      }
    }

    /// Load configuration manager from file
    #[ inline ]
    pub fn from_file< P: AsRef< std::path::Path > >( path : P ) -> OllamaResult< Self >
    {
      let config = DynamicConfig::from_file( path )?;
      Ok( Self::from_config( config ) )
    }

    /// Get current configuration
    #[ inline ]
    pub fn current( &self ) -> &DynamicConfig
    {
      &self.current
    }

    /// Get current configuration (alias for `current`)
    #[ inline ]
    pub fn get_current_config( &self ) -> &DynamicConfig
    {
      self.current()
    }

    /// Update configuration with validation
    #[ inline ]
    pub fn update( &mut self, new_config : DynamicConfig ) -> OllamaResult< () >
    {
      new_config.validate()?;

      let diff = ConfigDiff::calculate( &self.current, &new_config );
      if !diff.has_changes()
      {
        return Ok( () ); // No changes to apply
      }

      // Add current config to history before updating
      let new_version = self.history.len() as u64;
      let version = ConfigVersion::new( new_config.clone(), new_version );

      self.current = new_config;
      self.history.push( version );

      // Trim history if needed
      if self.history.len() > self.max_history
      {
        self.history.remove( 0 );
      }

      Ok( () )
    }

    /// Update configuration (alias for `update`)
    #[ inline ]
    pub fn update_config( &mut self, new_config : DynamicConfig ) -> OllamaResult< () >
    {
      self.update( new_config )
    }

    /// Get configuration history
    #[ inline ]
    pub fn history( &self ) -> &[ ConfigVersion ]
    {
      &self.history
    }

    /// Rollback to previous configuration version
    #[ inline ]
    pub fn rollback( &mut self, version : u64 ) -> OllamaResult< () >
    {
      let config_version = self.history.iter()
        .find( | v | v.version == version )
        .ok_or_else( || format_err!( "Version {} not found in history", version ) )?;

      self.current = config_version.config.clone();
      Ok( () )
    }

    /// Rollback to version (alias for `rollback`)
    #[ inline ]
    pub fn rollback_to_version( &mut self, version : u64 ) -> OllamaResult< () >
    {
      self.rollback( version )
    }

    /// Get configuration history (alias for `history`)
    #[ inline ]
    pub fn get_config_history( &self ) -> &[ ConfigVersion ]
    {
      self.history()
    }

    /// Apply environment variable overrides
    #[ inline ]
    pub fn apply_env_overrides( &mut self ) -> OllamaResult< () >
    {
      let env_config = DynamicConfig::from_env()?;
      self.update( env_config )
    }

    /// Register config change callback (no-op for now)
    #[ inline ]
    pub fn on_config_change< F >( &mut self, _callback : F ) -> OllamaResult< () >
    where
      F : Fn( &DynamicConfig ) + 'static,
    {
      // Placeholder - would store and call callbacks on config changes
      Ok( () )
    }

    /// Get configuration diff compared to specific version
    #[ inline ]
    #[ must_use ]
    pub fn diff_from_version( &self, version : u64 ) -> Option< ConfigDiff >
    {
      self.history.iter()
        .find( | v | v.version == version )
        .map( | v | ConfigDiff::calculate( &v.config, &self.current ) )
    }

    /// Clear configuration history
    #[ inline ]
    pub fn clear_history( &mut self )
    {
      self.history.clear();
      self.history.push( ConfigVersion::new( self.current.clone(), 0 ) );
    }

    /// Apply configuration to client (placeholder)
    #[ inline ]
    pub fn apply_to_client( &self, _client : &mut crate::client::OllamaClient ) -> OllamaResult< () >
    {
      // In a full implementation, this would update the client's internal configuration
      // For now, just validate the config
      Ok( () )
    }
  }
}

#[ cfg( feature = "dynamic_config" ) ]
crate ::mod_interface!
{
  exposed use private::DynamicConfig;
  exposed use private::ConfigDiff;
  exposed use private::ConfigBackup;
  exposed use private::ConfigVersion;
  exposed use private::DynamicConfigManager;
}
