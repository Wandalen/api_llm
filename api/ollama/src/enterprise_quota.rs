//! Enterprise Quota Management and Usage Tracking
//!
//! Track request counts, tokens, and implement quota enforcement.

#[ cfg( feature = "enabled" ) ]
mod private
{
  use std::sync::{ Arc, Mutex };
  use std::collections::HashMap;
  use serde::{ Serialize, Deserialize };

  /// Usage metrics for quota tracking
  #[ derive( Debug, Clone, Serialize, Deserialize, PartialEq ) ]
  pub struct UsageMetrics
  {
    /// Total requests made
    pub request_count : u64,
    /// Total input tokens used
    pub input_tokens : u64,
    /// Total output tokens used
    pub output_tokens : u64,
    /// Timestamp of first request
    pub period_start : i64,
    /// Timestamp of last request
    pub period_end : i64,
  }

  impl Default for UsageMetrics
  {
    fn default() -> Self
    {
      let now = std::time::SystemTime::now()
        .duration_since( std::time::UNIX_EPOCH )
        .unwrap()
        .as_secs() as i64;

      Self
      {
        request_count : 0,
        input_tokens : 0,
        output_tokens : 0,
        period_start : now,
        period_end : now,
      }
    }
  }

  impl UsageMetrics
  {
    /// Record a request
    pub fn record_request( &mut self, input_tokens : u64, output_tokens : u64 )
    {
      self.request_count += 1;
      self.input_tokens += input_tokens;
      self.output_tokens += output_tokens;
      self.period_end = std::time::SystemTime::now()
        .duration_since( std::time::UNIX_EPOCH )
        .unwrap()
        .as_secs() as i64;
    }

    /// Reset metrics
    pub fn reset( &mut self )
    {
      let now = std::time::SystemTime::now()
        .duration_since( std::time::UNIX_EPOCH )
        .unwrap()
        .as_secs() as i64;

      self.request_count = 0;
      self.input_tokens = 0;
      self.output_tokens = 0;
      self.period_start = now;
      self.period_end = now;
    }

    /// Get total tokens
    #[ must_use ]
    pub fn total_tokens( &self ) -> u64
    {
      self.input_tokens + self.output_tokens
    }
  }

  /// Quota configuration
  #[ derive( Debug, Clone, PartialEq ) ]
  #[ derive( Default ) ]
  pub struct QuotaConfig
  {
    /// Maximum requests allowed
    pub max_requests : Option< u64 >,
    /// Maximum total tokens allowed
    pub max_tokens : Option< u64 >,
  }

  impl QuotaConfig
  {
    /// Create new quota configuration
    #[ must_use ]
    pub fn new() -> Self
    {
      Self::default()
    }

    /// Set maximum requests
    #[ must_use ]
    pub fn with_max_requests( mut self, max : u64 ) -> Self
    {
      self.max_requests = Some( max );
      self
    }

    /// Set maximum tokens
    #[ must_use ]
    pub fn with_max_tokens( mut self, max : u64 ) -> Self
    {
      self.max_tokens = Some( max );
      self
    }
  }

  /// Quota exceeded error
  #[ derive( Debug, Clone ) ]
  pub struct QuotaExceededError
  {
    /// Error message
    pub message : String,
  }

  impl std::fmt::Display for QuotaExceededError
  {
    fn fmt( &self, f : &mut std::fmt::Formatter< '_ > ) -> std::fmt::Result
    {
      write!( f, "Quota exceeded: {}", self.message )
    }
  }

  impl std::error::Error for QuotaExceededError {}

  /// Quota manager for tracking and enforcing usage limits
  #[ derive( Debug, Clone ) ]
  pub struct QuotaManager
  {
    config : QuotaConfig,
    daily_metrics : Arc< Mutex< UsageMetrics > >,
    monthly_metrics : Arc< Mutex< UsageMetrics > >,
    per_model_metrics : Arc< Mutex< HashMap< String, UsageMetrics > > >,
  }

  impl QuotaManager
  {
    /// Create new quota manager
    #[ must_use ]
    pub fn new( config : QuotaConfig ) -> Self
    {
      Self
      {
        config,
        daily_metrics : Arc::new( Mutex::new( UsageMetrics::default() ) ),
        monthly_metrics : Arc::new( Mutex::new( UsageMetrics::default() ) ),
        per_model_metrics : Arc::new( Mutex::new( HashMap::new() ) ),
      }
    }

    /// Record usage for a request
    ///
    /// # Errors
    ///
    /// Returns error if quota is exceeded
    pub fn record_usage
    (
      &self,
      model : &str,
      input_tokens : u64,
      output_tokens : u64,
    ) -> Result< (), QuotaExceededError >
    {
      // Check daily quotas
      {
        let daily = self.daily_metrics.lock().unwrap();
        if let Some( max ) = self.config.max_requests
        {
          if daily.request_count >= max
          {
            return Err( QuotaExceededError
            {
              message : format!( "Daily request limit of {} exceeded", max ),
            } );
          }
        }
        if let Some( max ) = self.config.max_tokens
        {
          let total_tokens = daily.total_tokens() + input_tokens + output_tokens;
          if total_tokens > max
          {
            return Err( QuotaExceededError
            {
              message : format!( "Daily token limit of {} exceeded", max ),
            } );
          }
        }
      }

      // Record usage
      {
        let mut daily = self.daily_metrics.lock().unwrap();
        daily.record_request( input_tokens, output_tokens );
      }
      {
        let mut monthly = self.monthly_metrics.lock().unwrap();
        monthly.record_request( input_tokens, output_tokens );
      }
      {
        let mut per_model = self.per_model_metrics.lock().unwrap();
        per_model
          .entry( model.to_string() )
          .or_default()
          .record_request( input_tokens, output_tokens );
      }

      Ok( () )
    }

    /// Get daily usage metrics
    #[ must_use ]
    pub fn get_daily_usage( &self ) -> UsageMetrics
    {
      self.daily_metrics.lock().unwrap().clone()
    }

    /// Get monthly usage metrics
    #[ must_use ]
    pub fn get_monthly_usage( &self ) -> UsageMetrics
    {
      self.monthly_metrics.lock().unwrap().clone()
    }

    /// Get per-model usage metrics
    #[ must_use ]
    pub fn get_model_usage( &self, model : &str ) -> Option< UsageMetrics >
    {
      self.per_model_metrics.lock().unwrap().get( model ).cloned()
    }

    /// Reset daily metrics
    pub fn reset_daily( &self )
    {
      self.daily_metrics.lock().unwrap().reset();
    }

    /// Reset monthly metrics
    pub fn reset_monthly( &self )
    {
      self.monthly_metrics.lock().unwrap().reset();
      self.per_model_metrics.lock().unwrap().clear();
    }
  }

  #[ cfg( test ) ]
  mod tests
  {
    use super::*;

    #[ test ]
    fn test_usage_metrics_default()
    {
      let metrics = UsageMetrics::default();
      assert_eq!( metrics.request_count, 0 );
      assert_eq!( metrics.input_tokens, 0 );
      assert_eq!( metrics.output_tokens, 0 );
    }

    #[ test ]
    fn test_usage_metrics_record()
    {
      let mut metrics = UsageMetrics::default();
      metrics.record_request( 100, 200 );

      assert_eq!( metrics.request_count, 1 );
      assert_eq!( metrics.input_tokens, 100 );
      assert_eq!( metrics.output_tokens, 200 );
      assert_eq!( metrics.total_tokens(), 300 );
    }

    #[ test ]
    fn test_quota_config_builder()
    {
      let config = QuotaConfig::new()
        .with_max_requests( 1000 )
        .with_max_tokens( 100000 );

      assert_eq!( config.max_requests, Some( 1000 ) );
      assert_eq!( config.max_tokens, Some( 100000 ) );
    }

    #[ test ]
    fn test_quota_manager_basic()
    {
      let config = QuotaConfig::new();
      let manager = QuotaManager::new( config );

      manager.record_usage( "llama3.2", 100, 200 ).unwrap();

      let daily = manager.get_daily_usage();
      assert_eq!( daily.request_count, 1 );
      assert_eq!( daily.total_tokens(), 300 );
    }

    #[ test ]
    fn test_quota_manager_request_limit()
    {
      let config = QuotaConfig::new().with_max_requests( 2 );
      let manager = QuotaManager::new( config );

      manager.record_usage( "llama3.2", 100, 200 ).unwrap();
      manager.record_usage( "llama3.2", 100, 200 ).unwrap();

      let result = manager.record_usage( "llama3.2", 100, 200 );
      assert!( result.is_err() );
    }

    #[ test ]
    fn test_quota_manager_token_limit()
    {
      let config = QuotaConfig::new().with_max_tokens( 500 );
      let manager = QuotaManager::new( config );

      manager.record_usage( "llama3.2", 100, 200 ).unwrap();

      let result = manager.record_usage( "llama3.2", 100, 200 );
      assert!( result.is_err() );
    }

    #[ test ]
    fn test_quota_manager_per_model_tracking()
    {
      let config = QuotaConfig::new();
      let manager = QuotaManager::new( config );

      manager.record_usage( "llama3.2", 100, 200 ).unwrap();
      manager.record_usage( "codellama", 50, 100 ).unwrap();

      let llama_usage = manager.get_model_usage( "llama3.2" ).unwrap();
      assert_eq!( llama_usage.request_count, 1 );
      assert_eq!( llama_usage.total_tokens(), 300 );

      let codellama_usage = manager.get_model_usage( "codellama" ).unwrap();
      assert_eq!( codellama_usage.request_count, 1 );
      assert_eq!( codellama_usage.total_tokens(), 150 );
    }

    #[ test ]
    fn test_quota_manager_reset()
    {
      let config = QuotaConfig::new();
      let manager = QuotaManager::new( config );

      manager.record_usage( "llama3.2", 100, 200 ).unwrap();
      manager.reset_daily();

      let daily = manager.get_daily_usage();
      assert_eq!( daily.request_count, 0 );
    }
  }
}

#[ cfg( feature = "enabled" ) ]
crate::mod_interface!
{
  exposed use
  {
    UsageMetrics,
    QuotaConfig,
    QuotaExceededError,
    QuotaManager,
  };
}
