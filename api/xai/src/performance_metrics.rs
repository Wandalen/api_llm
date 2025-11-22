mod private
{
  //! Performance metrics collection using Prometheus.
  //!
  //! This module provides client-side metrics collection for monitoring
  //! XAI API usage, performance, and costs.
  //!
  //! # Design Decisions
  //!
  //! ## Why Client-Side Metrics?
  //!
  //! The XAI Grok API does not provide built-in analytics or metrics.
  //! Client-side collection offers:
  //!
  //! 1. **Production Monitoring**: Track performance in live systems
  //! 2. **Cost Tracking**: Monitor token usage and API costs
  //! 3. **SLA Compliance**: Measure latencies and error rates
  //! 4. **Capacity Planning**: Understand usage patterns
  //!
  //! ## Why Prometheus?
  //!
  //! Prometheus is the industry standard for metrics:
  //!
  //! 1. **Ecosystem**: Integrates with `Grafana`, `AlertManager`, etc.
  //! 2. **Pull-Based**: No external dependencies for metric storage
  //! 3. **Standard**: CNCF graduated project
  //! 4. **Performance**: Efficient time-series storage
  //!
  //! ## Metrics Collected
  //!
  //! - **Request Count**: Total number of API requests
  //! - **Request Duration**: Latency distribution (histogram)
  //! - **Token Usage**: Total tokens consumed (input + output)
  //! - **Error Count**: Total number of failed requests
  //! - **Model Usage**: Requests per model (labels)
  //!
  //! ## Alternatives Considered
  //!
  //! - **`StatsD`**: Less ecosystem integration than `Prometheus`
  //! - **OpenTelemetry**: More complex, overkill for metrics-only
  //! - **Custom JSON Logging**: No built-in aggregation/querying

  use std::time::{ Duration, Instant };

  #[ cfg( feature = "performance_metrics" ) ]
  use prometheus::{ Counter, Histogram, HistogramOpts, Registry, Opts };

  /// Metrics collector for XAI API operations.
  ///
  /// Collects performance and usage metrics compatible with Prometheus.
  /// Metrics can be exposed via `/metrics` endpoint for scraping.
  ///
  /// # Metrics
  ///
  /// - `xai_requests_total` - Total number of requests
  /// - `xai_request_duration_seconds` - Request latency distribution
  /// - `xai_tokens_total` - Total tokens consumed
  /// - `xai_errors_total` - Total number of errors
  ///
  /// # Examples
  ///
  /// ```no_run
  /// # #[ cfg( feature = "performance_metrics") ]
  /// # {
  /// use api_xai::MetricsCollector;
  /// use std::time::Duration;
  ///
  /// // Create collector
  /// let metrics = MetricsCollector::new();
  ///
  /// // Record a successful request
  /// metrics.record_request
  /// (
  ///   Duration::from_millis( 250 ),
  ///   1500, // tokens
  ///   true  // success
  /// );
  ///
  /// // Record a failed request
  /// metrics.record_request
  /// (
  ///   Duration::from_millis( 100 ),
  ///   0,    // no tokens
  ///   false // failure
  /// );
  ///
  /// // Export metrics for Prometheus
  /// let prometheus_text = metrics.export();
  /// println!( "{}", prometheus_text );
  /// # }
  /// ```
  #[ cfg( feature = "performance_metrics" ) ]
  pub struct MetricsCollector
  {
    registry : Registry,
    requests_total : Counter,
    requests_duration : Histogram,
    tokens_total : Counter,
    errors_total : Counter,
  }

  #[ cfg( feature = "performance_metrics" ) ]
  impl std::fmt::Debug for MetricsCollector
  {
    fn fmt( &self, f : &mut std::fmt::Formatter< '_ > ) -> std::fmt::Result
    {
      f.debug_struct( "MetricsCollector" )
        .field( "registry", &"< Registry >" )
        .field( "requests_total", &"< Counter >" )
        .field( "requests_duration", &"< Histogram >" )
        .field( "tokens_total", &"< Counter >" )
        .field( "errors_total", &"< Counter >" )
        .finish()
    }
  }

  #[ cfg( feature = "performance_metrics" ) ]
  impl MetricsCollector
  {
    /// Creates a new metrics collector.
    ///
    /// Initializes all Prometheus metrics with default configuration.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # #[ cfg( feature = "performance_metrics") ]
    /// # {
    /// use api_xai::MetricsCollector;
    ///
    /// let metrics = MetricsCollector::new();
    /// # }
    /// ```
    ///
    /// # Panics
    ///
    /// Panics if metric registration fails (rare, only if metric names are invalid).
    pub fn new() -> Self
    {
      let registry = Registry::new();

      // Request counter
      let requests_total = Counter::with_opts
      (
        Opts::new
        (
          "xai_requests_total",
          "Total number of XAI API requests"
        )
      ).unwrap();

      // Request duration histogram
      let requests_duration = Histogram::with_opts
      (
        HistogramOpts::new
        (
          "xai_request_duration_seconds",
          "XAI API request duration in seconds"
        )
      ).unwrap();

      // Token counter
      let tokens_total = Counter::with_opts
      (
        Opts::new
        (
          "xai_tokens_total",
          "Total tokens consumed (input + output)"
        )
      ).unwrap();

      // Error counter
      let errors_total = Counter::with_opts
      (
        Opts::new
        (
          "xai_errors_total",
          "Total number of failed requests"
        )
      ).unwrap();

      // Register metrics
      registry.register( Box::new( requests_total.clone() ) ).unwrap();
      registry.register( Box::new( requests_duration.clone() ) ).unwrap();
      registry.register( Box::new( tokens_total.clone() ) ).unwrap();
      registry.register( Box::new( errors_total.clone() ) ).unwrap();

      Self
      {
        registry,
        requests_total,
        requests_duration,
        tokens_total,
        errors_total,
      }
    }

    /// Records a completed request.
    ///
    /// # Arguments
    ///
    /// * `duration` - Request execution time
    /// * `tokens` - Total tokens consumed (0 for errors)
    /// * `success` - Whether the request succeeded
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # #[ cfg( feature = "performance_metrics") ]
    /// # {
    /// use api_xai::MetricsCollector;
    /// use std::time::Duration;
    ///
    /// let metrics = MetricsCollector::new();
    ///
    /// // Record successful request
    /// metrics.record_request
    /// (
    ///   Duration::from_millis( 250 ),
    ///   1500,
    ///   true
    /// );
    /// # }
    /// ```
    pub fn record_request( &self, duration : Duration, tokens : u32, success : bool )
    {
      self.requests_total.inc();
      self.requests_duration.observe( duration.as_secs_f64() );

      if success
      {
        self.tokens_total.inc_by( f64::from(tokens) );
      }
      else
      {
        self.errors_total.inc();
      }
    }

    /// Exports metrics in Prometheus text format.
    ///
    /// Returns metrics formatted for Prometheus scraping.
    /// Expose this via an HTTP `/metrics` endpoint.
    ///
    /// # Returns
    ///
    /// Metrics in Prometheus exposition format.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # #[ cfg( feature = "performance_metrics") ]
    /// # {
    /// use api_xai::MetricsCollector;
    ///
    /// let metrics = MetricsCollector::new();
    ///
    /// // Export for Prometheus
    /// let prometheus_text = metrics.export();
    ///
    /// // Serve via HTTP (pseudo-code)
    /// // server.route("/metrics", || prometheus_text)
    /// # }
    /// ```
    ///
    /// # Panics
    ///
    /// Panics if encoding fails (rare, only on internal encoder errors).
    pub fn export( &self ) -> String
    {
      use prometheus::Encoder;

      let encoder = prometheus::TextEncoder::new();
      let metric_families = self.registry.gather();

      let mut buffer = Vec::new();
      encoder.encode( &metric_families, &mut buffer ).unwrap();

      String::from_utf8( buffer ).unwrap()
    }

    /// Returns the registry for advanced integration.
    ///
    /// Allows direct access to the Prometheus registry for custom
    /// metric registration or integration with other libraries.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # #[ cfg( feature = "performance_metrics") ]
    /// # {
    /// use api_xai::MetricsCollector;
    ///
    /// let metrics = MetricsCollector::new();
    /// let registry = metrics.registry();
    ///
    /// // Use registry for custom metrics
    /// # }
    /// ```
    pub fn registry( &self ) -> &Registry
    {
      &self.registry
    }
  }

  #[ cfg( feature = "performance_metrics" ) ]
  impl Default for MetricsCollector
  {
    fn default() -> Self
    {
      Self::new()
    }
  }

  /// A wrapper that automatically records metrics for operations.
  ///
  /// Provides automatic metric recording via RAII pattern.
  /// Records duration and outcome when dropped.
  ///
  /// # Examples
  ///
  /// ```no_run
  /// # #[ cfg( feature = "performance_metrics") ]
  /// # {
  /// use api_xai::{ MetricsCollector, MetricGuard };
  /// use std::sync::Arc;
  ///
  /// let metrics = Arc::new( MetricsCollector::new() );
  ///
  /// {
  ///   let _guard = MetricGuard::new( metrics.clone() );
  ///
  ///   // ... make API request ...
  ///
  ///   // When guard drops, metrics are recorded automatically
  /// }
  /// # }
  /// ```
  #[ cfg( feature = "performance_metrics" ) ]
  #[ derive( Debug ) ]
  pub struct MetricGuard
  {
    metrics : std::sync::Arc< MetricsCollector >,
    start : Instant,
    tokens : u32,
    success : bool,
  }

  #[ cfg( feature = "performance_metrics" ) ]
  impl MetricGuard
  {
    /// Creates a new metric guard.
    ///
    /// Starts timing immediately. Records metrics when dropped.
    ///
    /// # Arguments
    ///
    /// * `metrics` - The metrics collector to record to
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # #[ cfg( feature = "performance_metrics") ]
    /// # {
    /// use api_xai::{ MetricsCollector, MetricGuard };
    /// use std::sync::Arc;
    ///
    /// let metrics = Arc::new( MetricsCollector::new() );
    /// let guard = MetricGuard::new( metrics );
    /// # }
    /// ```
    pub fn new( metrics : std::sync::Arc< MetricsCollector > ) -> Self
    {
      Self
      {
        metrics,
        start : Instant::now(),
        tokens : 0,
        success : false,
      }
    }

    /// Sets the token count for this operation.
    ///
    /// # Arguments
    ///
    /// * `tokens` - Total tokens consumed
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # #[ cfg( feature = "performance_metrics") ]
    /// # {
    /// use api_xai::{ MetricsCollector, MetricGuard };
    /// use std::sync::Arc;
    ///
    /// let metrics = Arc::new( MetricsCollector::new() );
    /// let mut guard = MetricGuard::new( metrics );
    ///
    /// // After getting response
    /// guard.set_tokens( 1500 );
    /// # }
    /// ```
    pub fn set_tokens( &mut self, tokens : u32 )
    {
      self.tokens = tokens;
    }

    /// Marks the operation as successful.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # #[ cfg( feature = "performance_metrics") ]
    /// # {
    /// use api_xai::{ MetricsCollector, MetricGuard };
    /// use std::sync::Arc;
    ///
    /// let metrics = Arc::new( MetricsCollector::new() );
    /// let mut guard = MetricGuard::new( metrics );
    ///
    /// // After successful request
    /// guard.set_success();
    /// # }
    /// ```
    pub fn set_success( &mut self )
    {
      self.success = true;
    }
  }

  #[ cfg( feature = "performance_metrics" ) ]
  impl Drop for MetricGuard
  {
    fn drop( &mut self )
    {
      let duration = self.start.elapsed();
      self.metrics.record_request( duration, self.tokens, self.success );
    }
  }
}

#[ cfg( feature = "performance_metrics" ) ]
crate::mod_interface!
{
  exposed use
  {
    MetricsCollector,
    MetricGuard,
  };
}
