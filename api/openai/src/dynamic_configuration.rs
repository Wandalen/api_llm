//! Dynamic Configuration Module
//!
//! This module provides stateless dynamic configuration utilities for `OpenAI` API responses.
//! Following the "Thin Client, Rich API" principle, this module offers configuration management
//! patterns and validation tools without maintaining persistent configuration state.

mod private
{
  use std::
  {
    collections ::HashMap,
    sync ::{ Arc, RwLock },
    time ::Instant,
  };
  use core::time::Duration;
  use serde::{ Deserialize, Serialize };
  use tokio::sync::{ mpsc, watch };

  /// Configuration value that can be dynamically updated
  #[ derive( Debug, Clone, PartialEq, Serialize, Deserialize ) ]
  pub enum ConfigValue
  {
    /// String configuration value
    String( String ),
    /// Integer configuration value
    Integer( i64 ),
    /// Float configuration value
    Float( f64 ),
    /// Boolean configuration value
    Boolean( bool ),
    /// Duration configuration value (in milliseconds)
    Duration( u64 ),
  }

  impl ConfigValue
  {
    /// Convert to string if possible
    #[ inline ]
    #[ must_use ]
    pub fn as_string( &self ) -> Option< String >
    {
      match self
      {
        ConfigValue::String( s ) => Some( s.clone() ),
        _ => None,
      }
    }

    /// Convert to integer if possible
    #[ inline ]
    #[ must_use ]
    pub fn as_integer( &self ) -> Option< i64 >
    {
      match self
      {
        ConfigValue::Integer( i ) => Some( *i ),
        _ => None,
      }
    }

    /// Convert to float if possible
    #[ inline ]
    #[ must_use ]
    pub fn as_float( &self ) -> Option< f64 >
    {
      match self
      {
        ConfigValue::Float( f ) => Some( *f ),
        _ => None,
      }
    }

    /// Convert to boolean if possible
    #[ inline ]
    #[ must_use ]
    pub fn as_boolean( &self ) -> Option< bool >
    {
      match self
      {
        ConfigValue::Boolean( b ) => Some( *b ),
        _ => None,
      }
    }

    /// Convert to duration if possible
    #[ inline ]
    #[ must_use ]
    pub fn as_duration( &self ) -> Option< Duration >
    {
      match self
      {
        ConfigValue::Duration( ms ) => Some( Duration::from_millis( *ms ) ),
        _ => None,
      }
    }
  }

  /// Configuration change event
  #[ derive( Debug, Clone, PartialEq ) ]
  pub struct ConfigChangeEvent
  {
    /// The configuration key that changed
    pub key : String,
    /// Old value (if any)
    pub old_value : Option< ConfigValue >,
    /// New value
    pub new_value : ConfigValue,
    /// Timestamp of the change
    pub timestamp : Instant,
  }

  /// Configuration snapshot representing a point-in-time configuration state
  #[ derive( Debug, Clone, Serialize, Deserialize ) ]
  pub struct ConfigSnapshot
  {
    /// Configuration values
    pub values : HashMap<  String, ConfigValue  >,
    /// Version number for this snapshot
    pub version : u64,
    /// Timestamp when snapshot was created (not serialized)
    #[ serde( skip, default = "Instant::now" ) ]
    pub created_at : Instant,
  }

  impl ConfigSnapshot
  {
    /// Create a new empty configuration snapshot
    #[ inline ]
    #[ must_use ]
    pub fn new() -> Self
    {
      Self
      {
        values : HashMap::new(),
        version : 1,
        created_at : Instant::now(),
      }
    }

    /// Create a new snapshot with incremented version
    #[ inline ]
    #[ must_use ]
    pub fn next_version( &self ) -> Self
    {
      Self
      {
        values : self.values.clone(),
        version : self.version + 1,
        created_at : Instant::now(),
      }
    }

    /// Get a configuration value
    #[ inline ]
    #[ must_use ]
    pub fn get( &self, key : &str ) -> Option< &ConfigValue >
    {
      self.values.get( key )
    }

    /// Set a configuration value, returning a new snapshot
    #[ inline ]
    #[ must_use ]
    pub fn with_value( mut self, key : String, value : ConfigValue ) -> Self
    {
      self.values.insert( key, value );
      self
    }

    /// Remove a configuration value, returning a new snapshot
    #[ inline ]
    #[ must_use ]
    pub fn without_value( mut self, key : &str ) -> Self
    {
      self.values.remove( key );
      self
    }

    /// Validate the configuration snapshot
    ///
    /// # Errors
    ///
    /// Returns an error if any validation rules fail for the configuration values.
    #[ inline ]
    pub fn validate( &self, validator : &ConfigValidator ) -> Result< (), Vec< String > >
    {
      validator.validate_snapshot( self )
    }
  }

  impl Default for ConfigSnapshot
  {
    #[ inline ]
    fn default() -> Self
    {
      Self
      {
        values : HashMap::new(),
        version : 1,
        created_at : Instant::now(),
      }
    }
  }

  /// Configuration validation rule
  #[ derive( Debug, Clone ) ]
  pub enum ValidationRule
  {
    /// Value must be present
    Required,
    /// String must match pattern
    StringPattern( String ),
    /// Integer must be within range
    IntegerRange( i64, i64 ),
    /// Float must be within range
    FloatRange( f64, f64 ),
    /// Duration must be within range (in milliseconds)
    DurationRange( u64, u64 ),
  }

  /// Configuration validator
  #[ derive( Debug, Clone ) ]
  pub struct ConfigValidator
  {
    /// Validation rules for each configuration key
    rules : HashMap< String, Vec< ValidationRule > >,
  }

  impl ConfigValidator
  {
    /// Create a new empty validator
    #[ inline ]
    #[ must_use ]
    pub fn new() -> Self
    {
      Self
      {
        rules : HashMap::new(),
      }
    }

    /// Add a validation rule for a key
    #[ inline ]
    #[ must_use ]
    pub fn add_rule( mut self, key : String, rule : ValidationRule ) -> Self
    {
      self.rules.entry( key ).or_default().push( rule );
      self
    }

    /// Validate a configuration snapshot
    ///
    /// # Errors
    ///
    /// Returns an error if any validation rules fail for the configuration values.
    #[ inline ]
    pub fn validate_snapshot( &self, snapshot : &ConfigSnapshot ) -> Result< (), Vec< String > >
    {
      let mut errors = Vec::new();

      for ( key, rules ) in &self.rules
      {
        if let Some( value ) = snapshot.get( key )
        {
          for rule in rules
          {
            if let Err( error ) = Self::validate_value( key, value, rule )
            {
              errors.push( error );
            }
          }
        }
        else
        {
          // Check if any rule requires this key to be present
          if rules.iter().any( | r | matches!( r, ValidationRule::Required ) )
          {
            errors.push( format!( "Required key '{key}' is missing" ) );
          }
        }
      }

      if errors.is_empty()
      {
        Ok( () )
      }
      else
      {
        Err( errors )
      }
    }

    /// Validate a single value against a rule
    fn validate_value( key : &str, value : &ConfigValue, rule : &ValidationRule ) -> Result< (), String >
    {
      match ( value, rule )
      {
        ( _, ValidationRule::Required ) => Ok( () ), // Value exists, so it's valid
        ( ConfigValue::String( s ), ValidationRule::StringPattern( pattern ) ) =>
        {
          // Simple pattern matching (could be enhanced with regex)
          if s.contains( pattern )
          {
            Ok( () )
          }
          else
          {
            Err( format!( "Key '{key}' string value '{s}' doesn't match pattern '{pattern}'" ) )
          }
        }
        ( ConfigValue::Integer( i ), ValidationRule::IntegerRange( min, max ) ) =>
        {
          if i >= min && i <= max
          {
            Ok( () )
          }
          else
          {
            Err( format!( "Key '{key}' integer value {i} is not in range [{min}, {max}]" ) )
          }
        }
        ( ConfigValue::Float( f ), ValidationRule::FloatRange( min, max ) ) =>
        {
          if f >= min && f <= max
          {
            Ok( () )
          }
          else
          {
            Err( format!( "Key '{key}' float value {f} is not in range [{min}, {max}]" ) )
          }
        }
        ( ConfigValue::Duration( ms ), ValidationRule::DurationRange( min_ms, max_ms ) ) =>
        {
          if ms >= min_ms && ms <= max_ms
          {
            Ok( () )
          }
          else
          {
            Err( format!( "Key '{key}' duration value {ms}ms is not in range [{min_ms}ms, {max_ms}ms]" ) )
          }
        }
        _ =>
        {
          Err( format!( "Key '{key}' value type doesn't match validation rule" ) )
        }
      }
    }
  }

  impl Default for ConfigValidator
  {
    #[ inline ]
    fn default() -> Self
    {
      Self::new()
    }
  }

  /// Configuration manager for stateless configuration operations
  #[ derive( Debug ) ]
  pub struct ConfigManager
  {
    /// Current configuration snapshot
    current : Arc< RwLock< ConfigSnapshot > >,
    /// Configuration validator
    validator : ConfigValidator,
  }

  impl ConfigManager
  {
    /// Create a new configuration manager
    #[ inline ]
    #[ must_use ]
    pub fn new( validator : ConfigValidator ) -> Self
    {
      Self
      {
        current : Arc::new( RwLock::new( ConfigSnapshot::new() ) ),
        validator,
      }
    }

    /// Get current configuration snapshot
    ///
    /// # Panics
    ///
    /// Panics if the internal lock is poisoned.
    #[ inline ]
    #[ must_use ]
    pub fn get_snapshot( &self ) -> ConfigSnapshot
    {
      self.current.read().unwrap().clone()
    }

    /// Update configuration with a new snapshot
    ///
    /// # Errors
    ///
    /// Returns an error if the new snapshot fails validation.
    ///
    /// # Panics
    ///
    /// Panics if the internal lock is poisoned.
    #[ inline ]
    pub fn update_snapshot( &self, new_snapshot : ConfigSnapshot ) -> Result< ConfigSnapshot, Vec< String > >
    {
      // Validate the new snapshot
      new_snapshot.validate( &self.validator )?;

      // Update the current snapshot
      let mut current = self.current.write().unwrap();
      *current = new_snapshot.clone();

      Ok( new_snapshot )
    }

    /// Update a single configuration value
    ///
    /// # Errors
    ///
    /// Returns an error if the updated configuration fails validation.
    #[ inline ]
    pub fn update_value( &self, key : String, value : ConfigValue ) -> Result< ConfigSnapshot, Vec< String > >
    {
      let current_snapshot = self.get_snapshot();
      let new_snapshot = current_snapshot.next_version().with_value( key, value );
      self.update_snapshot( new_snapshot )
    }

    /// Remove a configuration value
    ///
    /// # Errors
    ///
    /// Returns an error if the updated configuration fails validation.
    #[ inline ]
    pub fn remove_value( &self, key : &str ) -> Result< ConfigSnapshot, Vec< String > >
    {
      let current_snapshot = self.get_snapshot();
      let new_snapshot = current_snapshot.next_version().without_value( key );
      self.update_snapshot( new_snapshot )
    }

    /// Get a specific configuration value
    #[ inline ]
    #[ must_use ]
    pub fn get_value( &self, key : &str ) -> Option< ConfigValue >
    {
      self.get_snapshot().get( key ).cloned()
    }
  }

  /// Utilities for dynamic configuration management
  #[ derive( Debug ) ]
  pub struct DynamicConfigManager;

  impl DynamicConfigManager
  {
    /// Create a configuration change watcher
    #[ inline ]
    #[ must_use ]
    pub fn create_change_watcher() -> ( ConfigChangeSender, ConfigChangeReceiver )
    {
      let ( tx, rx ) = mpsc::unbounded_channel();
      ( ConfigChangeSender { sender : tx }, ConfigChangeReceiver { receiver : rx } )
    }

    /// Create a configuration value watcher
    #[ inline ]
    #[ must_use ]
    pub fn create_value_watcher< T : Clone + Send + 'static >( initial_value : T ) -> ( watch::Sender< T >, watch::Receiver< T > )
    {
      watch ::channel( initial_value )
    }

    /// Apply configuration changes atomically
    ///
    /// # Errors
    ///
    /// Returns an error if the updated configuration fails validation.
    #[ inline ]
    pub fn apply_atomic_changes(
      manager : &ConfigManager,
      changes : Vec< ( String, ConfigValue ) >,
    ) -> Result< ConfigSnapshot, Vec< String > >
    {
      let mut current_snapshot = manager.get_snapshot().next_version();

      // Apply all changes to the snapshot
      for ( key, value ) in changes
      {
        current_snapshot = current_snapshot.with_value( key, value );
      }

      // Validate and update as a single operation
      manager.update_snapshot( current_snapshot )
    }

    /// Create a configuration backup
    #[ inline ]
    #[ must_use ]
    pub fn create_backup( snapshot : &ConfigSnapshot ) -> String
    {
      serde_json ::to_string( snapshot ).unwrap_or_else( | _ | "{}".to_string() )
    }

    /// Restore configuration from backup
    ///
    /// # Errors
    ///
    /// Returns an error if the backup cannot be parsed as a valid configuration snapshot.
    #[ inline ]
    pub fn restore_from_backup( backup : &str ) -> Result< ConfigSnapshot, String >
    {
      serde_json ::from_str( backup ).map_err( | e | format!( "Failed to parse backup : {e}" ) )
    }

    /// Merge two configuration snapshots
    #[ inline ]
    #[ must_use ]
    pub fn merge_snapshots( base : &ConfigSnapshot, overlay : ConfigSnapshot ) -> ConfigSnapshot
    {
      let mut merged = base.next_version();
      for ( key, value ) in overlay.values
      {
        merged = merged.with_value( key, value );
      }
      merged
    }
  }

  /// Sender for configuration change events
  #[ derive( Debug ) ]
  pub struct ConfigChangeSender
  {
    sender : mpsc::UnboundedSender< ConfigChangeEvent >,
  }

  impl ConfigChangeSender
  {
    /// Send a configuration change event
    ///
    /// # Errors
    ///
    /// Returns an error if the event cannot be sent (e.g., if the receiver has been dropped).
    #[ inline ]
    pub fn send_change( &self, event : ConfigChangeEvent ) -> Result< (), &'static str >
    {
      self.sender.send( event ).map_err( | _ | "Failed to send change event" )
    }

    /// Send a configuration update event
    ///
    /// # Errors
    ///
    /// Returns an error if the event cannot be sent (e.g., if the receiver has been dropped).
    #[ inline ]
    pub fn send_update( &self, key : String, old_value : Option< ConfigValue >, new_value : ConfigValue ) -> Result< (), &'static str >
    {
      let event = ConfigChangeEvent
      {
        key,
        old_value,
        new_value,
        timestamp : Instant::now(),
      };
      self.send_change( event )
    }
  }

  /// Receiver for configuration change events
  #[ derive( Debug ) ]
  pub struct ConfigChangeReceiver
  {
    receiver : mpsc::UnboundedReceiver< ConfigChangeEvent >,
  }

  impl ConfigChangeReceiver
  {
    /// Try to receive a configuration change event (non-blocking)
    #[ inline ]
    pub fn try_recv( &mut self ) -> Option< ConfigChangeEvent >
    {
      self.receiver.try_recv().ok()
    }

    /// Receive next configuration change event (blocking)
    #[ inline ]
    pub async fn recv( &mut self ) -> Option< ConfigChangeEvent >
    {
      self.receiver.recv().await
    }
  }
}

crate ::mod_interface!
{
  exposed use private::ConfigValue;
  exposed use private::ConfigChangeEvent;
  exposed use private::ConfigSnapshot;
  exposed use private::ValidationRule;
  exposed use private::ConfigValidator;
  exposed use private::ConfigManager;
  exposed use private::DynamicConfigManager;
  exposed use private::ConfigChangeSender;
  exposed use private::ConfigChangeReceiver;
}