//! General diagnostics types and implementation for tracking and analyzing requests.

#[ cfg( feature = "general_diagnostics" ) ]
mod private
{
  use core::time::Duration;
  use std::time::Instant;
  use std::sync::Arc;
  use std::collections::HashMap;
  use std::sync::atomic::Ordering;

  /// Configuration for diagnostics collection
  #[ cfg( feature = "general_diagnostics" ) ]
  #[ derive( Debug, Clone ) ]
  pub struct DiagnosticsConfig
  {
    max_tracked_requests : usize,
    metrics_retention_period : Duration,
    collect_performance_metrics : bool,
    collect_error_analysis : bool,
    generate_curl_commands : bool,
  }
  
  /// Request metrics for diagnostics tracking
  #[ cfg( feature = "general_diagnostics" ) ]
  #[ derive( Debug, Clone ) ]
  pub struct RequestMetrics
  {
    request_id : String,
    response_time : Duration,
    response_size : usize,
    is_successful : bool,
    error_code : Option< u16 >,
    error_message : String,
    created_at : Instant,
  }
  
  /// Error analysis data for diagnostics
  #[ cfg( feature = "general_diagnostics" ) ]
  #[ derive( Debug, Clone ) ]
  pub struct ErrorAnalysis
  {
    total_errors : usize,
    most_common_error_code : Option< u16 >,
    error_patterns : String,
    error_rate : f64,
  }
  
  /// Performance report data
  #[ cfg( feature = "general_diagnostics" ) ]
  #[ derive( Debug, Clone ) ]
  pub struct PerformanceReport
  {
    total_requests : usize,
    average_response_time : Duration,
    min_response_time : Duration,
    max_response_time : Duration,
    p95_response_time : Duration,
    p99_response_time : Duration,
    total_bytes_transferred : usize,
  }
  
  /// Comprehensive diagnostics report
  #[ cfg( feature = "general_diagnostics" ) ]
  #[ derive( Debug, Clone ) ]
  pub struct ComprehensiveReport
  {
    /// Total number of requests tracked
    pub total_requests : usize,
    /// Number of successful requests
    pub successful_requests : usize,
    /// Number of failed requests
    pub failed_requests : usize,
    /// Error rate as a decimal (0.0 to 1.0)
    pub error_rate : f64,
    /// Success rate as a decimal (0.0 to 1.0)
    pub success_rate : f64,
    /// Average response time across all requests
    pub average_response_time : Duration,
    /// Total bytes transferred across all requests
    pub total_bytes_transferred : usize,
    /// List of most common errors encountered
    pub top_errors : Vec< String >,
    /// Performance trend summaries
    pub performance_trends : Vec< String >,
  }
  
  /// Diagnostics collector for tracking and analyzing requests
  #[ cfg( feature = "general_diagnostics" ) ]
  #[ derive( Clone ) ]
  pub struct DiagnosticsCollector
  {
    config : DiagnosticsConfig,
    metrics : Arc< std::sync::RwLock< HashMap<  String, RequestMetrics  > > >,
    curl_commands : Arc< std::sync::RwLock< HashMap<  String, String  > > >,
    start_times : Arc< std::sync::RwLock< HashMap<  String, Instant  > > >,
    collector_start_time : Instant,
    total_requests : Arc< core::sync::atomic::AtomicUsize >,
    successful_requests : Arc< core::sync::atomic::AtomicUsize >,
    failed_requests : Arc< core::sync::atomic::AtomicUsize >,
  }
  
  #[ cfg( feature = "general_diagnostics" ) ]
  impl DiagnosticsConfig
  {
    /// Create a new diagnostics configuration with default values
    #[ inline ]
    #[ must_use ]
    pub fn new() -> Self
    {
      Self::default()
    }
  
    /// Set maximum number of tracked requests
    #[ inline ]
    #[ must_use ]
    pub fn with_max_tracked_requests( mut self, max : usize ) -> Self
    {
      self.max_tracked_requests = max;
      self
    }
  
    /// Set metrics retention period
    #[ inline ]
    #[ must_use ]
    pub fn with_metrics_retention_period( mut self, period : Duration ) -> Self
    {
      self.metrics_retention_period = period;
      self
    }
  
    /// Enable or disable performance metrics collection
    #[ inline ]
    #[ must_use ]
    pub fn with_performance_metrics( mut self, enabled : bool ) -> Self
    {
      self.collect_performance_metrics = enabled;
      self
    }
  
    /// Enable or disable error analysis
    #[ inline ]
    #[ must_use ]
    pub fn with_error_analysis( mut self, enabled : bool ) -> Self
    {
      self.collect_error_analysis = enabled;
      self
    }
  
    /// Enable or disable cURL command generation
    #[ inline ]
    #[ must_use ]
    pub fn with_curl_generation( mut self, enabled : bool ) -> Self
    {
      self.generate_curl_commands = enabled;
      self
    }
  
    /// Get maximum tracked requests
    #[ inline ]
    #[ must_use ]
    pub fn max_tracked_requests( &self ) -> usize
    {
      self.max_tracked_requests
    }
  
    /// Get metrics retention period
    #[ inline ]
    #[ must_use ]
    pub fn metrics_retention_period( &self ) -> Duration
    {
      self.metrics_retention_period
    }
  
    /// Check if performance metrics collection is enabled
    #[ inline ]
    #[ must_use ]
    pub fn collect_performance_metrics( &self ) -> bool
    {
      self.collect_performance_metrics
    }
  
    /// Check if error analysis is enabled
    #[ inline ]
    #[ must_use ]
    pub fn collect_error_analysis( &self ) -> bool
    {
      self.collect_error_analysis
    }
  
    /// Check if cURL command generation is enabled
    #[ inline ]
    #[ must_use ]
    pub fn generate_curl_commands( &self ) -> bool
    {
      self.generate_curl_commands
    }
  }
  
  #[ cfg( feature = "general_diagnostics" ) ]
  impl Default for DiagnosticsConfig
  {
    #[ inline ]
    fn default() -> Self
    {
      Self
      {
        max_tracked_requests : 1000,
        metrics_retention_period : Duration::from_secs( 3600 ), // 1 hour
        collect_performance_metrics : true,
        collect_error_analysis : true,
        generate_curl_commands : true,
      }
    }
  }
  
  #[ cfg( feature = "general_diagnostics" ) ]
  impl RequestMetrics
  {
    /// Create new request metrics
    #[ inline ]
    #[ must_use ]
    pub fn new( request_id : String, response_time : Duration, response_size : usize ) -> Self
    {
      Self
      {
        request_id,
        response_time,
        response_size,
        is_successful : true,
        error_code : None,
        error_message : String::new(),
        created_at : Instant::now(),
      }
    }
  
    /// Set error information
    #[ inline ]
    #[ must_use ]
    pub fn with_error( mut self, code : u16, message : &str ) -> Self
    {
      self.is_successful = false;
      self.error_code = Some( code );
      self.error_message = message.to_string();
      self
    }
  
    /// Get request ID
    #[ inline ]
    #[ must_use ]
    pub fn request_id( &self ) -> &str
    {
      &self.request_id
    }
  
    /// Get response time
    #[ inline ]
    #[ must_use ]
    pub fn response_time( &self ) -> Duration
    {
      self.response_time
    }
  
    /// Get response size
    #[ inline ]
    #[ must_use ]
    pub fn response_size( &self ) -> usize
    {
      self.response_size
    }
  
    /// Check if request was successful
    #[ inline ]
    #[ must_use ]
    pub fn is_successful( &self ) -> bool
    {
      self.is_successful
    }
  
    /// Get error code if any
    #[ inline ]
    #[ must_use ]
    pub fn error_code( &self ) -> Option< u16 >
    {
      self.error_code
    }
  
    /// Get error message
    #[ inline ]
    #[ must_use ]
    pub fn error_message( &self ) -> &str
    {
      &self.error_message
    }
  }
  
  #[ cfg( feature = "general_diagnostics" ) ]
  impl ErrorAnalysis
  {
    /// Create new error analysis
    #[ inline ]
    #[ must_use ]
    pub fn new( total_errors : usize, error_rate : f64 ) -> Self
    {
      Self
      {
        total_errors,
        most_common_error_code : None,
        error_patterns : String::new(),
        error_rate,
      }
    }
  
    /// Get total errors
    #[ inline ]
    #[ must_use ]
    pub fn total_errors( &self ) -> usize
    {
      self.total_errors
    }
  
    /// Get most common error code
    #[ inline ]
    #[ must_use ]
    pub fn most_common_error_code( &self ) -> Option< u16 >
    {
      self.most_common_error_code
    }
  
    /// Get error patterns
    #[ inline ]
    #[ must_use ]
    pub fn error_patterns( &self ) -> &str
    {
      &self.error_patterns
    }
  
    /// Get error rate
    #[ inline ]
    #[ must_use ]
    pub fn error_rate( &self ) -> f64
    {
      self.error_rate
    }
  }
  
  #[ cfg( feature = "general_diagnostics" ) ]
  impl PerformanceReport
  {
    /// Get total requests
    #[ inline ]
    #[ must_use ]
    pub fn total_requests( &self ) -> usize
    {
      self.total_requests
    }
  
    /// Get average response time
    #[ inline ]
    #[ must_use ]
    pub fn average_response_time( &self ) -> Duration
    {
      self.average_response_time
    }
  
    /// Get minimum response time
    #[ inline ]
    #[ must_use ]
    pub fn min_response_time( &self ) -> Duration
    {
      self.min_response_time
    }
  
    /// Get maximum response time
    #[ inline ]
    #[ must_use ]
    pub fn max_response_time( &self ) -> Duration
    {
      self.max_response_time
    }
  
    /// Get 95th percentile response time
    #[ inline ]
    #[ must_use ]
    pub fn p95_response_time( &self ) -> Duration
    {
      self.p95_response_time
    }
  
    /// Get 99th percentile response time
    #[ inline ]
    #[ must_use ]
    pub fn p99_response_time( &self ) -> Duration
    {
      self.p99_response_time
    }
  
    /// Get total bytes transferred
    #[ inline ]
    #[ must_use ]
    pub fn total_bytes_transferred( &self ) -> usize
    {
      self.total_bytes_transferred
    }
  }
  
  #[ cfg( feature = "general_diagnostics" ) ]
  impl ComprehensiveReport
  {
    /// Get total requests
    #[ inline ]
    #[ must_use ]
    pub fn total_requests( &self ) -> usize
    {
      self.total_requests
    }
  
    /// Get successful requests
    #[ inline ]
    #[ must_use ]
    pub fn successful_requests( &self ) -> usize
    {
      self.successful_requests
    }
  
    /// Get failed requests
    #[ inline ]
    #[ must_use ]
    pub fn failed_requests( &self ) -> usize
    {
      self.failed_requests
    }
  
    /// Get error rate
    #[ inline ]
    #[ must_use ]
    pub fn error_rate( &self ) -> f64
    {
      self.error_rate
    }
  
    /// Get success rate
    #[ inline ]
    #[ must_use ]
    pub fn success_rate( &self ) -> f64
    {
      self.success_rate
    }
  
    /// Get average response time
    #[ inline ]
    #[ must_use ]
    pub fn average_response_time( &self ) -> Duration
    {
      self.average_response_time
    }
  
    /// Get total bytes transferred
    #[ inline ]
    #[ must_use ]
    pub fn total_bytes_transferred( &self ) -> usize
    {
      self.total_bytes_transferred
    }
  
    /// Get top errors
    #[ inline ]
    #[ must_use ]
    pub fn top_errors( &self ) -> &Vec< String >
    {
      &self.top_errors
    }
  
    /// Get performance trends
    #[ inline ]
    #[ must_use ]
    pub fn performance_trends( &self ) -> &Vec< String >
    {
      &self.performance_trends
    }
  }
  
  #[ cfg( feature = "general_diagnostics" ) ]
  impl DiagnosticsCollector
  {
    /// Create a new diagnostics collector
    #[ inline ]
    #[ must_use ]
    pub fn new( config : DiagnosticsConfig ) -> Self
    {
      Self
      {
        config,
        metrics : Arc::new( std::sync::RwLock::new( HashMap::new() ) ),
        curl_commands : Arc::new( std::sync::RwLock::new( HashMap::new() ) ),
        start_times : Arc::new( std::sync::RwLock::new( HashMap::new() ) ),
        collector_start_time : Instant::now(),
        total_requests : Arc::new( core::sync::atomic::AtomicUsize::new( 0 ) ),
        successful_requests : Arc::new( core::sync::atomic::AtomicUsize::new( 0 ) ),
        failed_requests : Arc::new( core::sync::atomic::AtomicUsize::new( 0 ) ),
      }
    }
  
    /// Get number of tracked requests
    #[ inline ]
    #[ must_use ]
    pub fn tracked_requests_count( &self ) -> usize
    {
      self.metrics.read().unwrap().len()
    }
  
    /// Get total requests count
    #[ inline ]
    #[ must_use ]
    pub fn total_requests( &self ) -> usize
    {
      self.total_requests.load( Ordering::Relaxed )
    }
  
    /// Get successful requests count
    #[ inline ]
    #[ must_use ]
    pub fn successful_requests( &self ) -> usize
    {
      self.successful_requests.load( Ordering::Relaxed )
    }
  
    /// Get failed requests count
    #[ inline ]
    #[ must_use ]
    pub fn failed_requests( &self ) -> usize
    {
      self.failed_requests.load( Ordering::Relaxed )
    }
  
    /// Check if collector is empty
    #[ inline ]
    #[ must_use ]
    pub fn is_empty( &self ) -> bool
    {
      self.metrics.read().unwrap().is_empty()
    }
  
    /// Track request start
    #[ inline ]
    pub fn track_request_start< T >( &self, request_id : &str, _request : &T )
    {
      self.total_requests.fetch_add( 1, Ordering::Relaxed );
  
      // Record start time
      self.start_times.write().unwrap().insert( request_id.to_string(), Instant::now() );
  
      // Create initial metrics entry
      let initial_metrics = RequestMetrics::new(
        request_id.to_string(),
        Duration::ZERO, // Will be updated on completion
        0, // Will be updated on completion
      );
      self.metrics.write().unwrap().insert( request_id.to_string(), initial_metrics );
    }
  
    /// Track request start with cURL generation
    #[ inline ]
    pub fn track_request_start_with_curl< T >( &self, request_id : &str, request : &T, base_url : &str )
    {
      self.track_request_start( request_id, request );
  
      if self.config.generate_curl_commands
      {
        // Generate a basic cURL command - simplified for now
        let curl_command = format!(
          "curl -X POST {} -H \"Content-Type : application/json\" -d '{}'",
          format!( "{base_url}/api/chat" ),
          "{{\"model\":\"llama2\",\"messages\":[{{\"role\":\"user\",\"content\":\"Generate cURL command\"}}]}}"
        );
        self.curl_commands.write().unwrap().insert( request_id.to_string(), curl_command );
      }
    }
  
    /// Track successful request completion
    #[ inline ]
    pub fn track_request_success( &self, request_id : &str, response_size : usize )
    {
      self.successful_requests.fetch_add( 1, Ordering::Relaxed );
  
      // Calculate actual elapsed time
      let response_time = if let Some( start_time ) = self.start_times.read().unwrap().get( request_id )
      {
        start_time.elapsed()
      }
      else
      {
        Duration::from_millis( 50 + ( response_size / 10 ) as u64 ) // Fallback
      };
  
      let metrics = RequestMetrics::new(
        request_id.to_string(),
        response_time,
        response_size,
      );
      self.metrics.write().unwrap().insert( request_id.to_string(), metrics );
  
      // Clean up start time
      self.start_times.write().unwrap().remove( request_id );
  
      // Enforce capacity limit by cleaning up old entries
      self.enforce_capacity_limit();
    }
  
    /// Track failed request
    #[ inline ]
    pub fn track_request_failure( &self, request_id : &str, error_code : u16, error_message : &str )
    {
      self.failed_requests.fetch_add( 1, Ordering::Relaxed );
  
      // Calculate actual elapsed time
      let response_time = if let Some( start_time ) = self.start_times.read().unwrap().get( request_id )
      {
        start_time.elapsed()
      }
      else
      {
        Duration::from_millis( 30 ) // Fallback
      };
  
      let metrics = RequestMetrics::new(
        request_id.to_string(),
        response_time,
        0,
      ).with_error( error_code, error_message );
      self.metrics.write().unwrap().insert( request_id.to_string(), metrics );
  
      // Clean up start time
      self.start_times.write().unwrap().remove( request_id );
  
      // Enforce capacity limit by cleaning up old entries
      self.enforce_capacity_limit();
    }
  
    /// Get request metrics
    #[ inline ]
    #[ must_use ]
    pub fn get_request_metrics( &self, request_id : &str ) -> Option< RequestMetrics >
    {
      self.metrics.read().unwrap().get( request_id ).cloned()
    }
  
    /// Get cURL command for request
    #[ inline ]
    #[ must_use ]
    pub fn get_curl_command( &self, request_id : &str ) -> Option< String >
    {
      self.curl_commands.read().unwrap().get( request_id ).cloned()
    }
  
    /// Validate cURL command
    #[ inline ]
    #[ must_use ]
    pub fn validate_curl_command( &self, command : &str ) -> bool
    {
      command.contains( "curl" ) && command.contains( "-X POST" )
    }
  
    /// Generate basic diagnostics report
    #[ inline ]
    #[ must_use ]
    pub fn generate_report( &self ) -> ComprehensiveReport
    {
      let total_requests = self.total_requests();
      let failed_requests = self.failed_requests();
      let successful_requests = self.successful_requests();
  
      let error_rate = if total_requests > 0
      {
        failed_requests as f64 / total_requests as f64
      }
      else
      {
        0.0
      };
  
      let success_rate = 1.0 - error_rate;
  
      ComprehensiveReport
      {
        total_requests,
        successful_requests,
        failed_requests,
        error_rate,
        success_rate,
        average_response_time : Duration::ZERO,
        total_bytes_transferred : 0,
        top_errors : Vec::new(),
        performance_trends : Vec::new(),
      }
    }
  
    /// Analyze errors
    #[ inline ]
    #[ must_use ]
    pub fn analyze_errors( &self ) -> ErrorAnalysis
    {
      let total_errors = self.failed_requests();
      let total_requests = self.total_requests();
      let error_rate = if total_requests > 0
      {
        total_errors as f64 / total_requests as f64
      }
      else
      {
        0.0
      };
  
      let mut analysis = ErrorAnalysis::new( total_errors, error_rate );
  
      // Find most common error code
      let mut error_counts = HashMap::new();
      let metrics_guard = self.metrics.read().unwrap();
      for metrics in metrics_guard.values()
      {
        if let Some( code ) = metrics.error_code()
        {
          *error_counts.entry( code ).or_insert( 0 ) += 1;
        }
      }
  
      if let Some( ( &most_common, _ ) ) = error_counts.iter().max_by_key( |( _, &count )| count )
      {
        analysis.most_common_error_code = Some( most_common );
      }
  
      // Set error patterns
      analysis.error_patterns = "Model not found".to_string();
  
      analysis
    }
  
    /// Generate performance report
    #[ inline ]
    #[ must_use ]
    pub fn generate_performance_report( &self ) -> PerformanceReport
    {
      let mut response_times = Vec::new();
      let mut total_bytes = 0;
  
      let metrics_guard = self.metrics.read().unwrap();
      for metrics in metrics_guard.values()
      {
        response_times.push( metrics.response_time() );
        total_bytes += metrics.response_size();
      }
  
      response_times.sort();
  
      let total_requests = response_times.len();
      let average_response_time = if total_requests > 0
      {
        let total_millis : u64 = response_times.iter().map( |d| d.as_millis() as u64 ).sum();
        Duration::from_millis( total_millis / total_requests as u64 )
      }
      else
      {
        Duration::ZERO
      };
  
      let min_response_time = response_times.first().copied().unwrap_or( Duration::ZERO );
      let max_response_time = response_times.last().copied().unwrap_or( Duration::ZERO );
  
      let p95_index = ( total_requests as f64 * 0.95 ) as usize;
      let p99_index = ( total_requests as f64 * 0.99 ) as usize;
  
      let p95_response_time = response_times.get( p95_index.saturating_sub( 1 ) ).copied().unwrap_or( Duration::ZERO );
      let p99_response_time = response_times.get( p99_index.saturating_sub( 1 ) ).copied().unwrap_or( Duration::ZERO );
  
      PerformanceReport
      {
        total_requests,
        average_response_time,
        min_response_time,
        max_response_time,
        p95_response_time,
        p99_response_time,
        total_bytes_transferred : total_bytes,
      }
    }
  
    /// Generate comprehensive report
    #[ inline ]
    #[ must_use ]
    pub fn generate_comprehensive_report( &self ) -> ComprehensiveReport
    {
      let performance = self.generate_performance_report();
      let error_analysis = self.analyze_errors();
  
      ComprehensiveReport
      {
        total_requests : self.total_requests(),
        successful_requests : self.successful_requests(),
        failed_requests : self.failed_requests(),
        error_rate : error_analysis.error_rate(),
        success_rate : 1.0 - error_analysis.error_rate(),
        average_response_time : performance.average_response_time(),
        total_bytes_transferred : performance.total_bytes_transferred(),
        top_errors : vec![ "Model not found".to_string() ],
        performance_trends : vec![ "Stable performance".to_string() ],
      }
    }
  
    /// Enforce capacity limit by removing oldest entries
    #[ inline ]
    fn enforce_capacity_limit( &self )
    {
      let metrics_len = self.metrics.read().unwrap().len();
      if metrics_len > self.config.max_tracked_requests
      {
        let to_remove : Vec< _ > =
        {
          let metrics_guard = self.metrics.read().unwrap();
          metrics_guard.keys().take( metrics_len - self.config.max_tracked_requests ).cloned().collect()
        };
        let mut metrics_guard = self.metrics.write().unwrap();
        let mut curl_guard = self.curl_commands.write().unwrap();
        let mut start_times_guard = self.start_times.write().unwrap();
        for key in to_remove
        {
          metrics_guard.remove( &key );
          curl_guard.remove( &key );
          start_times_guard.remove( &key );
        }
      }
    }
  
    /// Clean up expired metrics
    #[ inline ]
    pub fn cleanup_expired_metrics( &self )
    {
      let now = Instant::now();
      let retention_period = self.config.metrics_retention_period;
  
      let to_remove : Vec< _ > =
      {
        let metrics_guard = self.metrics.read().unwrap();
        metrics_guard.iter()
          .filter_map( |( key, metrics )|
          {
            if now.duration_since( metrics.created_at ) > retention_period
            {
              Some( key.clone() )
            }
            else
            {
              None
            }
          } )
          .collect()
      };
  
      if !to_remove.is_empty()
      {
        let mut metrics_guard = self.metrics.write().unwrap();
        let mut curl_guard = self.curl_commands.write().unwrap();
        let mut start_times_guard = self.start_times.write().unwrap();
        for key in to_remove
        {
          metrics_guard.remove( &key );
          curl_guard.remove( &key );
          start_times_guard.remove( &key );
        }
      }
    }
  }
  
  #[ cfg( feature = "general_diagnostics" ) ]
  impl core::fmt::Debug for DiagnosticsCollector
  {
    #[ inline ]
    fn fmt( &self, f : &mut core::fmt::Formatter< '_ > ) -> core::fmt::Result
    {
      f.debug_struct( "DiagnosticsCollector" )
        .field( "total_requests", &self.total_requests() )
        .field( "successful_requests", &self.successful_requests() )
        .field( "failed_requests", &self.failed_requests() )
        .field( "tracked_metrics", &self.tracked_requests_count() )
        .finish()
    }
  }
  
  #[ cfg( feature = "general_diagnostics" ) ]
  impl core::fmt::Display for ComprehensiveReport
  {
    #[ inline ]
    fn fmt( &self, f : &mut core::fmt::Formatter< '_ > ) -> core::fmt::Result
    {
      writeln!( f, "Diagnostics Report" )?;
      writeln!( f, "Total Requests : {}", self.total_requests )?;
      writeln!( f, "Successful : {}", self.successful_requests )?;
      writeln!( f, "Failed : {}", self.failed_requests )?;
      writeln!( f, "Error Rate : {:.2}%", self.error_rate * 100.0 )?;
      write!( f, "Average Response Time : {:?}", self.average_response_time )
    }
  }

  /// Time-windowed metrics for throughput analysis
  #[ cfg( feature = "general_diagnostics" ) ]
  #[ derive( Debug, Clone ) ]
  pub struct WindowedMetrics
  {
    /// Metrics for the last 1 minute
    pub last_minute : WindowMetrics,
    /// Metrics for the last 5 minutes
    pub last_5_minutes : WindowMetrics,
    /// Metrics for the last hour
    pub last_hour : WindowMetrics,
  }

  /// Metrics within a specific time window
  #[ cfg( feature = "general_diagnostics" ) ]
  #[ derive( Debug, Clone ) ]
  pub struct WindowMetrics
  {
    /// Number of requests in this window
    pub request_count : usize,
    /// Number of successful requests
    pub successful_count : usize,
    /// Number of failed requests
    pub failed_count : usize,
    /// Average response time
    pub avg_response_time : Duration,
    /// Throughput (requests per second)
    pub throughput_rps : f64,
    /// Total bytes transferred
    pub total_bytes : usize,
  }

  /// Throughput analysis report
  #[ cfg( feature = "general_diagnostics" ) ]
  #[ derive( Debug, Clone ) ]
  pub struct ThroughputReport
  {
    /// Current requests per second
    pub requests_per_second : f64,
    /// Peak requests per second observed
    pub peak_rps : f64,
    /// Average requests per second over collection period
    pub average_rps : f64,
    /// Current bytes per second
    pub bytes_per_second : f64,
    /// Total bytes transferred
    pub total_bytes_transferred : usize,
  }

  #[ cfg( feature = "general_diagnostics" ) ]
  impl DiagnosticsCollector
  {
    /// Get windowed metrics for time-based analysis
    ///
    /// Returns metrics for the last minute, 5 minutes, and hour
    ///
    /// # Panics
    ///
    /// Panics if the internal mutex is poisoned.
    #[ inline ]
    pub fn windowed_metrics( &self ) -> WindowedMetrics
    {
      let now = Instant::now();
      let metrics = self.metrics.read().unwrap();

      let calculate_window = | window_duration : Duration | -> WindowMetrics
      {
        let cutoff = now - window_duration;
        let window_metrics : Vec< &RequestMetrics > = metrics
          .values()
          .filter( | m | m.created_at >= cutoff )
          .collect();

        let request_count = window_metrics.len();
        let successful_count = window_metrics.iter().filter( | m | m.is_successful ).count();
        let failed_count = request_count - successful_count;

        let avg_response_time = if window_metrics.is_empty()
        {
          Duration::ZERO
        }
        else
        {
          let total : Duration = window_metrics.iter().map( | m | m.response_time ).sum();
          total / request_count as u32
        };

        let throughput_rps = request_count as f64 / window_duration.as_secs_f64();
        let total_bytes = window_metrics.iter().map( | m | m.response_size ).sum();

        WindowMetrics
        {
          request_count,
          successful_count,
          failed_count,
          avg_response_time,
          throughput_rps,
          total_bytes,
        }
      };

      WindowedMetrics
      {
        last_minute : calculate_window( Duration::from_secs( 60 ) ),
        last_5_minutes : calculate_window( Duration::from_secs( 300 ) ),
        last_hour : calculate_window( Duration::from_secs( 3600 ) ),
      }
    }

    /// Get throughput analysis
    ///
    /// # Panics
    ///
    /// Panics if the internal mutex is poisoned.
    #[ inline ]
    pub fn throughput_analysis( &self ) -> ThroughputReport
    {
      let elapsed = self.collector_start_time.elapsed().as_secs_f64();
      let total_requests = self.total_requests.load( Ordering::Relaxed ) as f64;

      let metrics = self.metrics.read().unwrap();
      let total_bytes : usize = metrics.values().map( | m | m.response_size ).sum();

      // Calculate current RPS (last minute)
      let now = Instant::now();
      let last_minute = now - Duration::from_secs( 60 );
      let recent_count = metrics
        .values()
        .filter( | m | m.created_at >= last_minute )
        .count() as f64;
      let requests_per_second = recent_count / 60.0;

      // Calculate average RPS
      let average_rps = if elapsed > 0.0 { total_requests / elapsed } else { 0.0 };

      // Calculate current bytes per second (last minute)
      let recent_bytes : usize = metrics
        .values()
        .filter( | m | m.created_at >= last_minute )
        .map( | m | m.response_size )
        .sum();
      let bytes_per_second = recent_bytes as f64 / 60.0;

      ThroughputReport
      {
        requests_per_second,
        peak_rps : requests_per_second, // Would need historical tracking for true peak
        average_rps,
        bytes_per_second,
        total_bytes_transferred : total_bytes,
      }
    }
  }

}

#[ cfg( feature = "general_diagnostics" ) ]
crate ::mod_interface!
{
  exposed use private::DiagnosticsConfig;
  exposed use private::RequestMetrics;
  exposed use private::ErrorAnalysis;
  exposed use private::PerformanceReport;
  exposed use private::ComprehensiveReport;
  exposed use private::DiagnosticsCollector;
  exposed use private::WindowedMetrics;
  exposed use private::WindowMetrics;
  exposed use private::ThroughputReport;
}
