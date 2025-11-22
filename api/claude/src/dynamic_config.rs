//! Dynamic Configuration with Hot-Reloading
//!
//! Runtime configuration changes with file system monitoring for zero-downtime updates.
//!
//! # Features
//!
//! - File-based configuration watching
//! - Runtime reload without restart
//! - Thread-safe config updates via `Arc< RwLock<> >`
//! - Configuration validation before applying
//! - JSON configuration format
//!
//! # Benefits
//!
//! - Zero-downtime configuration updates
//! - A/B testing different configurations
//! - Dynamic scaling of rate limits
//! - Environment-specific tuning without restart

#[ cfg( feature = "dynamic-config" ) ]
mod private
{
  use std::path::{ Path, PathBuf };
  use std::sync::Arc;
  use std::time::Duration;
  use parking_lot::RwLock;
  use serde::{ Serialize, Deserialize };
  use notify::{ Watcher, RecommendedWatcher, RecursiveMode, Event };

  /// Runtime configuration that can be hot-reloaded
  #[ derive( Debug, Clone, Serialize, Deserialize, PartialEq ) ]
  pub struct RuntimeConfig
  {
    /// Base URL for API requests (default: <https://api.anthropic.com>)
    #[ serde( default = "default_base_url" ) ]
    pub base_url : String,
    /// API version (default : "2023-06-01")
    #[ serde( default = "default_api_version" ) ]
    pub api_version : String,
    /// Request timeout in milliseconds (default : 300000 = 5 minutes)
    #[ serde( default = "default_timeout_ms" ) ]
    pub timeout_ms : u64,
    /// Enable retry logic (default : true)
    #[ serde( default = "default_true" ) ]
    pub enable_retry : bool,
    /// Maximum retry attempts (default : 3)
    #[ serde( default = "default_max_retries" ) ]
    pub max_retries : u32,
    /// Enable circuit breaker (default : true)
    #[ serde( default = "default_true" ) ]
    pub enable_circuit_breaker : bool,
    /// Circuit breaker failure threshold (default : 5)
    #[ serde( default = "default_failure_threshold" ) ]
    pub circuit_breaker_threshold : u32,
    /// Enable rate limiting (default : false)
    #[ serde( default ) ]
    pub enable_rate_limiting : bool,
    /// Rate limit : requests per second (default : 10)
    #[ serde( default = "default_rate_limit" ) ]
    pub rate_limit_rps : u32,
  }

  #[ inline ]
  fn default_base_url() -> String
  {
    "https://api.anthropic.com".to_string()
  }

  #[ inline ]
  fn default_api_version() -> String
  {
    "2023-06-01".to_string()
  }

  #[ inline ]
  fn default_timeout_ms() -> u64
  {
    300_000
  }

  #[ inline ]
  fn default_true() -> bool
  {
    true
  }

  #[ inline ]
  fn default_max_retries() -> u32
  {
    3
  }

  #[ inline ]
  fn default_failure_threshold() -> u32
  {
    5
  }

  #[ inline ]
  fn default_rate_limit() -> u32
  {
    10
  }

  impl RuntimeConfig
  {
    /// Create new runtime config with defaults
    #[ inline ]
    #[ must_use ]
    pub fn new() -> Self
    {
      Self
      {
        base_url : default_base_url(),
        api_version : default_api_version(),
        timeout_ms : default_timeout_ms(),
        enable_retry : default_true(),
        max_retries : default_max_retries(),
        enable_circuit_breaker : default_true(),
        circuit_breaker_threshold : default_failure_threshold(),
        enable_rate_limiting : false,
        rate_limit_rps : default_rate_limit(),
      }
    }

    /// Load config from JSON file
    ///
    /// # Errors
    ///
    /// Returns an error if file cannot be read or parsed
    #[ inline ]
    pub fn from_json_file( path : &Path ) -> Result< Self, Box< dyn std::error::Error > >
    {
      let contents = std::fs::read_to_string( path )?;
      let config : Self = serde_json::from_str( &contents )?;
      config.validate()?;
      Ok( config )
    }

    // TOML support can be added by enabling toml dependency if needed
    // For now, JSON-only configuration is supported

    /// Validate configuration values
    ///
    /// # Errors
    ///
    /// Returns an error if any configuration value is invalid
    #[ inline ]
    pub fn validate( &self ) -> Result< (), String >
    {
      if self.base_url.is_empty()
      {
        return Err( "base_url cannot be empty".to_string() );
      }
      if self.api_version.is_empty()
      {
        return Err( "api_version cannot be empty".to_string() );
      }
      if self.timeout_ms == 0
      {
        return Err( "timeout_ms must be greater than 0".to_string() );
      }
      if self.max_retries > 10
      {
        return Err( "max_retries cannot exceed 10".to_string() );
      }
      if self.circuit_breaker_threshold == 0
      {
        return Err( "circuit_breaker_threshold must be greater than 0".to_string() );
      }
      if self.rate_limit_rps == 0
      {
        return Err( "rate_limit_rps must be greater than 0".to_string() );
      }
      Ok( () )
    }

    /// Get timeout as Duration
    #[ inline ]
    #[ must_use ]
    pub fn timeout( &self ) -> Duration
    {
      Duration::from_millis( self.timeout_ms )
    }
  }

  impl Default for RuntimeConfig
  {
    #[ inline ]
    fn default() -> Self
    {
      Self::new()
    }
  }

  /// Configuration watcher for hot-reloading
  #[ derive( Debug ) ]
  pub struct ConfigWatcher
  {
    config : Arc< RwLock< RuntimeConfig > >,
    config_path : PathBuf,
    _watcher : RecommendedWatcher,
  }

  impl ConfigWatcher
  {
    /// Create new config watcher
    ///
    /// # Arguments
    ///
    /// * `config_path` - Path to configuration file
    /// * `initial_config` - Initial configuration to use before file is loaded
    ///
    /// # Errors
    ///
    /// Returns an error if the watcher cannot be created or the config file cannot be read
    #[ inline ]
    pub fn new
    (
      config_path : PathBuf,
      initial_config : RuntimeConfig,
    ) -> Result< Self, Box< dyn std::error::Error > >
    {
      let config = Arc::new( RwLock::new( initial_config ) );
      let config_clone = config.clone();
      let path_clone = config_path.clone();

      let mut watcher = notify::recommended_watcher( move | res : Result< Event, notify::Error > |
      {
        match res
        {
          Ok( event ) =>
          {
            // Reload on write events
            if event.kind.is_modify()
            {
              // Check if the event is for our config file
              if event.paths.iter().any( | p | p == &path_clone )
              {
                if let Ok( new_config ) = RuntimeConfig::from_json_file( &path_clone )
                {
                  *config_clone.write() = new_config;
                }
              }
            }
          }
          Err( e ) =>
          {
            eprintln!( "Watch error : {e:?}" );
          }
        }
      } )?;

      // Watch the parent directory (file may not exist yet)
      let watch_path = if config_path.exists()
      {
        config_path.clone()
      }
      else
      {
        config_path.parent().ok_or( "Config path has no parent" )?.to_path_buf()
      };

      watcher.watch( &watch_path, RecursiveMode::NonRecursive )?;

      // Load initial config from file if it exists
      if config_path.exists()
      {
        if let Ok( loaded_config ) = RuntimeConfig::from_json_file( &config_path )
        {
          *config.write() = loaded_config;
        }
      }

      Ok( Self
      {
        config,
        config_path,
        _watcher : watcher,
      } )
    }

    /// Get current configuration (read-only)
    #[ inline ]
    #[ must_use ]
    pub fn config( &self ) -> RuntimeConfig
    {
      self.config.read().clone()
    }

    /// Get shared config reference for concurrent access
    #[ inline ]
    #[ must_use ]
    pub fn config_ref( &self ) -> Arc< RwLock< RuntimeConfig > >
    {
      self.config.clone()
    }

    /// Manually reload configuration from file
    ///
    /// # Errors
    ///
    /// Returns an error if the file cannot be read or parsed
    #[ inline ]
    pub fn reload( &self ) -> Result< (), Box< dyn std::error::Error > >
    {
      let new_config = RuntimeConfig::from_json_file( &self.config_path )?;
      *self.config.write() = new_config;
      Ok( () )
    }

    /// Update configuration programmatically
    ///
    /// # Errors
    ///
    /// Returns an error if the new configuration is invalid
    #[ inline ]
    pub fn update( &self, new_config : RuntimeConfig ) -> Result< (), String >
    {
      new_config.validate()?;
      *self.config.write() = new_config;
      Ok( () )
    }
  }
}

#[ cfg( feature = "dynamic-config" ) ]
crate::mod_interface!
{
  exposed use
  {
    RuntimeConfig,
    ConfigWatcher,
  };
}
