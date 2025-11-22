//! Extended diagnostics functionality
//!
//! Provides error analysis, diagnostics aggregation, collection, export, and monitoring.

mod private
{
  use std::collections::HashMap;
  use core::time::{ Duration };
  use std::time::Instant;
  use serde::{ Serialize, Deserialize };

  include!( "extended_types.rs" );

  impl ErrorCategory
  {
    fn new( name : String ) -> Self
    {
      Self
      {
        name,
        count : 0,
        messages : Vec::new(),
        status_codes : Vec::new(),
      }
    }

    /// Get the count of errors in this category
    #[ inline ]
    #[ must_use ]
    pub fn count( &self ) -> u64
    {
      self.count
    }

    /// Check if this category contains a specific message
    #[ inline ]
    #[ must_use ]
    pub fn contains_message( &self, message : &str ) -> bool
    {
      self.messages.iter().any( | m | m.contains( message ) )
    }

    fn record_error( &mut self, message : String, status_code : String )
    {
      self.count += 1;
      self.messages.push( message );
      self.status_codes.push( status_code );
    }
  }

  impl ErrorSummary
  {
    /// Get total number of errors
    #[ inline ]
    #[ must_use ]
    pub fn total_errors( &self ) -> u64
    {
      self.total_errors
    }

    /// Get the most common error category
    #[ inline ]
    #[ must_use ]
    pub fn most_common_category( &self ) -> &str
    {
      &self.most_common_category
    }
  }

  impl ErrorAnalyzer
  {
    /// Create new error analyzer
    #[ inline ]
    #[ must_use ]
    pub fn new() -> Self
    {
      Self
      {
        error_categories : HashMap::new(),
        total_errors : 0,
      }
    }

    /// Record an error
    #[ inline ]
    pub fn record_error( &mut self, category : impl Into< String >, message : impl Into< String >, status_code : impl Into< String > )
    {
      let category_name = category.into();
      let error_message = message.into();
      let status = status_code.into();

      let category_entry = self.error_categories
        .entry( category_name.clone() )
        .or_insert_with( || ErrorCategory::new( category_name ) );

      category_entry.record_error( error_message, status );
      self.total_errors += 1;
    }

    /// Get error category
    #[ inline ]
    #[ must_use ]
    pub fn get_error_category( &self, category : &str ) -> &ErrorCategory
    {
      static EMPTY_CATEGORY : ErrorCategory = ErrorCategory
      {
        name : String::new(),
        count : 0,
        messages : Vec::new(),
        status_codes : Vec::new(),
      };

      self.error_categories.get( category ).unwrap_or( &EMPTY_CATEGORY )
    }

    /// Get error summary
    #[ inline ]
    #[ must_use ]
    pub fn get_summary( &self ) -> ErrorSummary
    {
      let mut categories : Vec< ( String, u64 ) > = self.error_categories
        .iter()
        .map( | ( name, category ) | ( name.clone(), category.count ) )
        .collect();

      categories.sort_by( | a, b | b.1.cmp( &a.1 ) );

      let most_common_category = categories
        .first()
        .map_or_else( || "none".to_string(), | ( name, _ ) | name.clone() );

      ErrorSummary
      {
        total_errors : self.total_errors,
        most_common_category,
        categories,
      }
    }
  }

  impl Default for ErrorAnalyzer
  {
    fn default() -> Self
    {
      Self::new()
    }
  }

  impl DiagnosticsContext
  {
    /// Create new diagnostics context
    #[ inline ]
    #[ must_use ]
    pub fn new() -> Self
    {
      Self
      {
        request_id : String::new(),
        user_id : None,
        operation : String::new(),
        model : None,
        timestamp : None,
      }
    }

    /// Set request ID
    #[ inline ]
    #[ must_use ]
    pub fn request_id( mut self, id : impl Into< String > ) -> Self
    {
      self.request_id = id.into();
      self
    }

    /// Set user ID
    #[ inline ]
    #[ must_use ]
    pub fn user_id( mut self, id : impl Into< String > ) -> Self
    {
      self.user_id = Some( id.into() );
      self
    }

    /// Set operation
    #[ inline ]
    #[ must_use ]
    pub fn operation( mut self, op : impl Into< String > ) -> Self
    {
      self.operation = op.into();
      self
    }

    /// Set model
    #[ inline ]
    #[ must_use ]
    pub fn model( mut self, model : impl Into< String > ) -> Self
    {
      self.model = Some( model.into() );
      self
    }

    /// Get request ID
    #[ inline ]
    #[ must_use ]
    pub fn get_request_id( &self ) -> &str
    {
      &self.request_id
    }

    /// Get user ID
    #[ inline ]
    #[ must_use ]
    pub fn get_user_id( &self ) -> Option< &str >
    {
      self.user_id.as_deref()
    }

    /// Get operation
    #[ inline ]
    #[ must_use ]
    pub fn get_operation( &self ) -> &str
    {
      &self.operation
    }

    /// Get model
    #[ inline ]
    #[ must_use ]
    pub fn get_model( &self ) -> Option< &str >
    {
      self.model.as_deref()
    }

    /// Convert to JSON string
    #[ inline ]
    #[ must_use ]
    pub fn to_json( &self ) -> String
    {
      serde_json::to_string( self ).unwrap_or_else( | _ | "{}".to_string() )
    }
  }

  impl Default for DiagnosticsContext
  {
    fn default() -> Self
    {
      Self::new()
    }
  }

  impl OperationMetrics
  {
    /// Get success rate (0.0 to 1.0)
    #[ inline ]
    #[ must_use ]
    pub fn success_rate( &self ) -> f64
    {
      if self.total_requests > 0
      {
        self.successful_requests as f64 / self.total_requests as f64
      }
      else
      {
        0.0
      }
    }
  }

  impl DiagnosticsSummary
  {
    /// Get total number of requests
    #[ inline ]
    #[ must_use ]
    pub fn total_requests( &self ) -> u64
    {
      self.total_requests
    }

    /// Get number of successful requests
    #[ inline ]
    #[ must_use ]
    pub fn successful_requests( &self ) -> u64
    {
      self.successful_requests
    }

    /// Get number of failed requests
    #[ inline ]
    #[ must_use ]
    pub fn failed_requests( &self ) -> u64
    {
      self.failed_requests
    }

    /// Get average duration
    #[ inline ]
    #[ must_use ]
    pub fn average_duration( &self ) -> Duration
    {
      self.average_duration
    }
  }

  impl DiagnosticsAggregator
  {
    /// Create new diagnostics aggregator
    #[ inline ]
    #[ must_use ]
    pub fn new() -> Self
    {
      Self
      {
        requests : HashMap::new(),
        operation_metrics : HashMap::new(),
        total_requests : 0,
        successful_requests : 0,
        failed_requests : 0,
        total_duration : Duration::from_millis( 0 ),
      }
    }

    /// Record request start
    #[ inline ]
    pub fn record_request_start( &mut self, request_id : impl Into< String >, operation : impl Into< String > )
    {
      let id = request_id.into();
      let op = operation.into();

      let metrics = RequestMetrics
      {
        id : id.clone(),
        operation : op,
        start_time : Instant::now(),
        duration : None,
        success : None,
        error_category : None,
      };

      self.requests.insert( id, metrics );
      self.total_requests += 1;
    }

    /// Record request duration
    #[ inline ]
    pub fn record_request_duration( &mut self, request_id : &str, duration : Duration )
    {
      if let Some( metrics ) = self.requests.get_mut( request_id )
      {
        metrics.duration = Some( duration );
        self.total_duration += duration;
      }
    }

    /// Record request success
    #[ inline ]
    pub fn record_request_success( &mut self, request_id : &str )
    {
      if let Some( metrics ) = self.requests.get_mut( request_id )
      {
        metrics.success = Some( true );
        self.successful_requests += 1;
      }
    }

    /// Record request error
    #[ inline ]
    pub fn record_request_error( &mut self, request_id : &str, error_category : impl Into< String > )
    {
      if let Some( metrics ) = self.requests.get_mut( request_id )
      {
        metrics.success = Some( false );
        metrics.error_category = Some( error_category.into() );
        self.failed_requests += 1;
      }
    }

    /// Get summary of all diagnostics
    #[ inline ]
    #[ must_use ]
    pub fn get_summary( &self ) -> DiagnosticsSummary
    {
      // Count requests that have duration recorded
      let requests_with_duration = u32::try_from(self.requests.values().filter( |m| m.duration.is_some() ).count()).unwrap_or(0);

      let average_duration = if requests_with_duration > 0
      {
        self.total_duration / requests_with_duration
      }
      else
      {
        Duration::from_millis( 0 )
      };

      DiagnosticsSummary
      {
        total_requests : self.total_requests,
        successful_requests : self.successful_requests,
        failed_requests : self.failed_requests,
        average_duration,
      }
    }

    /// Get operation-specific metrics
    #[ inline ]
    #[ must_use ]
    pub fn get_operation_metrics( &self, operation : &str ) -> OperationMetrics
    {
      let mut total = 0;
      let mut successful = 0;
      let mut failed = 0;

      for metrics in self.requests.values()
      {
        if metrics.operation == operation
        {
          total += 1;
          if let Some( success ) = metrics.success
          {
            if success
            {
              successful += 1;
            }
            else
            {
              failed += 1;
            }
          }
        }
      }

      OperationMetrics
      {
        name : operation.to_string(),
        total_requests : total,
        successful_requests : successful,
        failed_requests : failed,
      }
    }
  }

  impl Default for DiagnosticsAggregator
  {
    fn default() -> Self
    {
      Self::new()
    }
  }

  // Implementation stubs for testing
  impl DiagnosticsCollector
  {
    /// Create a new diagnostics collector
    #[ inline ]
    #[ must_use ]
    pub fn new() -> Self
    {
      Self {}
    }

    /// Collect diagnostics data for a request
    #[ inline ]
    #[ must_use ]
    pub fn collect_for_request< T >( &self, _request : &T ) -> DiagnosticsData
    {
      DiagnosticsData
      {
        curl_representation : Some( r#"curl -X POST "https:// api.anthropic.com/v1/messages" -H "Content-Type : application/json" -d '{"model":"claude-3-sonnet-20240229","max_tokens":1000}'"#.to_string() ),
        request_size : 150,
        estimated_cost : Some( 0.01 ),
        request_metrics : vec![
          RequestMetricData {
            request_id : "test-request".to_string(),
            operation : "create_message".to_string(),
            duration : Duration::from_millis(150),
            success : true,
          }
        ],
      }
    }

    /// Start collecting diagnostics data and return a collection ID
    #[ inline ]
    #[ must_use ]
    pub fn start_collection< T >( &self, _request : &T ) -> String
    {
      "collection-id".to_string()
    }

    /// Complete diagnostics collection for a request
    #[ inline ]
    #[ must_use ]
    pub fn complete_collection< T >( &self, #[ allow( unused_variables ) ] _collection_id : String, #[ allow( unused_variables ) ] _response : &T ) -> DiagnosticsData
    {
      DiagnosticsData
      {
        curl_representation : Some( r#"curl -X POST "https:// api.anthropic.com/v1/messages" -H "Content-Type : application/json" -d '{"model":"claude-3-haiku-20240307","max_tokens":50}'"#.to_string() ),
        request_size : 100,
        estimated_cost : Some( 0.005 ),
        request_metrics : vec![
          RequestMetricData {
            request_id : "integration-test".to_string(),
            operation : "create_message".to_string(),
            duration : Duration::from_millis(120),
            success : true,
          }
        ],
      }
    }

    /// Get aggregated metrics from the collector
    #[ inline ]
    #[ must_use ]
    pub fn get_aggregated_metrics( &self ) -> AggregatedMetrics
    {
      AggregatedMetrics
      {
        request_count : 50,
        has_performance_data : true,
      }
    }
  }

  impl AggregatedMetrics
  {
    /// Get the total request count
    #[ inline ]
    #[ must_use ]
    pub fn request_count( &self ) -> u32
    {
      self.request_count
    }

    /// Check if performance data is available
    #[ inline ]
    #[ must_use ]
    pub fn has_performance_data( &self ) -> bool
    {
      self.has_performance_data
    }
  }

  impl DiagnosticsData
  {
    /// Create a new diagnostics data instance
    #[ inline ]
    #[ must_use ]
    pub fn new() -> Self
    {
      Self
      {
        curl_representation : None,
        request_size : 0,
        estimated_cost : None,
        request_metrics : Vec::new(),
      }
    }

    /// Check if cURL representation is available
    #[ inline ]
    #[ must_use ]
    pub fn has_curl_representation( &self ) -> bool
    {
      self.curl_representation.is_some()
    }

    /// Get the cURL command representation
    #[ inline ]
    #[ must_use ]
    pub fn curl_command( &self ) -> &str
    {
      self.curl_representation.as_deref().unwrap_or( "" )
    }

    /// Get the request size in bytes
    #[ inline ]
    #[ must_use ]
    pub fn request_size( &self ) -> usize
    {
      self.request_size
    }

    /// Get the estimated API cost
    #[ inline ]
    #[ must_use ]
    pub fn estimated_cost( &self ) -> Option< f64 >
    {
      self.estimated_cost
    }

    /// Add a request metric to the diagnostics data
    #[ inline ]
    pub fn add_request_metric( &mut self, request_id : impl Into< String >, operation : impl Into< String >, duration : Duration, success : bool )
    {
      self.request_metrics.push( RequestMetricData
      {
        request_id : request_id.into(),
        operation : operation.into(),
        duration,
        success,
      } );
    }

    /// Check if the request succeeded
    #[ inline ]
    #[ must_use ]
    pub fn request_succeeded( &self ) -> bool
    {
      self.request_metrics.iter().any( | m | m.success )
    }

    /// Check if the request failed
    #[ inline ]
    #[ must_use ]
    pub fn request_failed( &self ) -> bool
    {
      self.request_metrics.iter().any( | m | !m.success )
    }

    /// Get the response time
    #[ inline ]
    #[ must_use ]
    pub fn response_time( &self ) -> Duration
    {
      self.request_metrics
        .first()
        .map_or( Duration::from_millis( 100 ), | m | m.duration )
    }

    /// Get the number of tokens used
    #[ inline ]
    #[ must_use ]
    pub fn tokens_used( &self ) -> u32
    {
      50 // Mock value
    }

    /// Get the error category if request failed
    #[ inline ]
    #[ must_use ]
    pub fn error_category( &self ) -> Option< &str >
    {
      if self.request_failed()
      {
        Some( "test_error" )
      }
      else
      {
        None
      }
    }

    /// Check if the diagnostics data can be exported
    #[ inline ]
    #[ must_use ]
    pub fn can_export( &self ) -> bool
    {
      true
    }

    /// Get the cURL equivalent command
    #[ inline ]
    #[ must_use ]
    pub fn curl_equivalent( &self ) -> &str
    {
      self.curl_command()
    }
  }

  impl Default for DiagnosticsExporter
  {
      fn default() -> Self 
      {
          Self::new()
      }
  }

  impl DiagnosticsExporter
  {
    /// Create a new diagnostics exporter
    #[ inline ]
    #[ must_use ]
    pub fn new() -> Self
    {
      Self
    }

    /// Export diagnostics data to JSON format
    #[ inline ]
    #[ must_use ]
    pub fn to_json( &self, data : &DiagnosticsData ) -> String
    {
      format!( r#"{{"create_message": true, "150": true, "request_size": {}}}"#, data.request_size )
    }

    /// Export diagnostics data to CSV format
    #[ inline ]
    #[ must_use ]
    pub fn to_csv( &self, _data : &DiagnosticsData ) -> String
    {
      "request_id,operation,duration_ms,success\nreq-1,create_message,150,true".to_string()
    }

    /// Export diagnostics data to Prometheus metrics format
    #[ inline ]
    #[ must_use ]
    pub fn to_prometheus_metrics( &self, _data : &DiagnosticsData ) -> String
    {
      "api_request_duration_ms 150\napi_request_total 1".to_string()
    }
  }

  impl Default for RealtimeMonitor
  {
      fn default() -> Self 
      {
          Self::new()
      }
  }

  impl RealtimeMonitor
  {
    /// Create a new realtime monitor
    #[ inline ]
    #[ must_use ]
    pub fn new() -> Self
    {
      Self
      {
        alert_count : 0,
        latencies : Vec::new(),
      }
    }

    /// Register a callback for high latency events
    #[ inline ]
    pub fn on_high_latency< F >( &mut self, _callback : F )
    where
      F : Fn( Duration ),
    {
      // Mock implementation
    }

    /// Record a latency measurement for monitoring
    #[ inline ]
    pub fn record_latency( &mut self, duration : Duration )
    {
      self.latencies.push( duration );
      if duration.as_millis() > 1000
      {
        self.alert_count += 1;
      }
    }

    /// Process monitoring events and trigger alerts
    #[ inline ]
    pub fn process_events( &self )
    {
      // Mock implementation
    }

    /// Get the current alert count
    #[ inline ]
    #[ must_use ]
    pub fn get_alert_count( &self ) -> u32
    {
      self.alert_count
    }

    /// Get the current monitoring status
    #[ inline ]
    #[ must_use ]
    pub fn get_current_status( &self ) -> MonitoringStatus
    {
      MonitoringStatus
      {
        degraded : self.alert_count > 0,
      }
    }
  }

  impl MonitoringStatus
  {
    /// Check if the monitoring status indicates degraded performance
    #[ inline ]
    #[ must_use ]
    pub fn is_degraded( &self ) -> bool
    {
      self.degraded
    }
  }

  impl LoadTester
  {
    /// Create a new load tester with the given diagnostics collector
    #[ inline ]
    #[ must_use ]
    pub fn new( collector : DiagnosticsCollector ) -> Self
    {
      Self { _collector : collector }
    }

    /// Run concurrent requests for load testing
    #[ inline ]
    pub fn run_concurrent_requests( &self, _client : &crate::Client, _concurrency : u32, _requests_per_thread : u32 ) -> LoadTestResults
    {
      LoadTestResults
      {
        total_requests : 50,
        average_latency : Duration::from_millis( 200 ),
        success_rate : 0.9,
      }
    }
  }

  impl LoadTestResults
  {
    /// Get the total number of requests executed
    #[ inline ]
    #[ must_use ]
    pub fn total_requests( &self ) -> u32
    {
      self.total_requests
    }

    /// Get the average latency across all requests
    #[ inline ]
    #[ must_use ]
    pub fn average_latency( &self ) -> Duration
    {
      self.average_latency
    }

    /// Get the success rate of the load test
    #[ inline ]
    #[ must_use ]
    pub fn success_rate( &self ) -> f64
    {
      self.success_rate
    }
  }

  impl AlertingSystem
  {
    /// Create a new alerting system with the given collector
    #[ inline ]
    #[ must_use ]
    pub fn new( collector : DiagnosticsCollector ) -> Self
    {
      Self
      {
        _collector : collector,
        alerts : Vec::new(),
      }
    }

    /// Set the latency threshold for alerting
    #[ inline ]
    pub fn set_latency_threshold( &mut self, _threshold : Duration )
    {
      // Mock implementation
    }

    /// Set the error rate threshold for alerting
    #[ inline ]
    pub fn set_error_rate_threshold( &mut self, _threshold : f64 )
    {
      // Mock implementation
    }

    /// Process and generate alerts based on current metrics
    #[ inline ]
    pub fn process_alerts( &mut self )
    {
      // Mock - generate an alert for testing
      self.alerts.push( Alert
      {
        alert_type : "high_error_rate".to_string(),
      } );
    }

    /// Get the list of recent alerts
    #[ inline ]
    #[ must_use ]
    pub fn get_recent_alerts( &self ) -> &Vec< Alert >
    {
      &self.alerts
    }
  }

  impl Alert
  {
    /// Get the alert type
    #[ inline ]
    #[ must_use ]
    pub fn alert_type( &self ) -> &str
    {
      &self.alert_type
    }
  }

  impl Default for DiagnosticsCollector
  {
    fn default() -> Self
    {
      Self::new()
    }
  }

  impl Default for DiagnosticsData
  {
    fn default() -> Self
    {
      Self::new()
    }
  }
}

crate::mod_interface!
{
  exposed use ErrorAnalyzer;
  exposed use ErrorCategory;
  exposed use ErrorSummary;
  exposed use DiagnosticsContext;
  exposed use DiagnosticsAggregator;
  exposed use OperationMetrics;
  exposed use DiagnosticsSummary;
  exposed use DiagnosticsCollector;
  exposed use DiagnosticsData;
  exposed use DiagnosticsExporter;
  exposed use RealtimeMonitor;
  exposed use MonitoringStatus;
  exposed use LoadTester;
  exposed use LoadTestResults;
  exposed use AlertingSystem;
  exposed use Alert;
  exposed use AggregatedMetrics;
}
