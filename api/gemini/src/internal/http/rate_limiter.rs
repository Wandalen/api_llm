//! Rate limiting implementation for HTTP requests

use std::sync::{ Arc, Mutex };
use std::time::{ Duration, Instant };
use std::collections::VecDeque;
use reqwest::{ Client, Method };
use serde::Serialize;
use serde::Deserialize;

use crate::error::Error;

#[ cfg( feature = "logging" ) ]
use tracing::{ warn, debug };

/// Rate limiting configuration extracted from client for HTTP layer usage
#[ derive( Debug, Clone ) ]
pub struct RateLimitingConfig
{
  /// Maximum requests per second
  pub requests_per_second : f64,
  /// Token bucket size (burst capacity)
  pub bucket_size : usize,
  /// Rate limiting algorithm to use
  pub algorithm : String,
  /// Whether to collect metrics
  pub enable_metrics : bool,
}

/// Rate limiting algorithms
#[ derive( Debug, Clone ) ]
pub enum RateLimiter
{
  /// Token bucket algorithm with refill rate
  TokenBucket
  {
    /// Current available tokens
    tokens : f64,
    /// Last refill timestamp
    last_refill : Instant,
  },
  /// Sliding window algorithm with request timestamps
  SlidingWindow
  {
    /// Queue of request timestamps
    requests : VecDeque< Instant >,
  },
}

/// Rate limiting metrics for monitoring
#[ derive( Debug, Clone ) ]
pub struct RateLimitingMetrics
{
  /// Total number of requests processed
  pub total_requests : u64,
  /// Number of requests that were rate limited
  pub limited_requests : u64,
  /// Number of requests allowed
  pub allowed_requests : u64,
  /// Current rate limiter state
  pub current_algorithm : String,
  /// Current available tokens (for token bucket)
  pub available_tokens : f64,
  /// Requests in current window (for sliding window)
  pub window_requests : usize,
}

/// Rate limiting instance with state management
#[ derive( Debug ) ]
pub struct RateLimit
{
  config : RateLimitingConfig,
  limiter : Arc< Mutex< RateLimiter > >,
  metrics : Arc< Mutex< RateLimitingMetrics > >,
}

impl RateLimit
{
  /// Create a new rate limiter with the given configuration
  pub fn new( config : RateLimitingConfig ) -> Self
  {
    let limiter = match config.algorithm.as_str()
    {
      "sliding_window" => RateLimiter::SlidingWindow {
        requests : VecDeque::new(),
      },
      _ => RateLimiter::TokenBucket {
        tokens : config.bucket_size as f64,
        last_refill : Instant::now(),
      },
    };

    Self {
      config : config.clone(),
      limiter : Arc::new( Mutex::new( limiter ) ),
      metrics : Arc::new( Mutex::new( RateLimitingMetrics {
        total_requests : 0,
        limited_requests : 0,
        allowed_requests : 0,
        current_algorithm : config.algorithm,
        available_tokens : config.bucket_size as f64,
        window_requests : 0,
      } ) ),
    }
  }

  /// Check if a request should be allowed based on rate limits
  pub async fn should_allow_request( &self ) -> bool
  {
    let mut limiter = self.limiter.lock().unwrap();
    let mut metrics = self.metrics.lock().unwrap();

    metrics.total_requests += 1;

    let allowed = match &mut *limiter
    {
      RateLimiter::TokenBucket { tokens, last_refill } => {
        let now = Instant::now();
        let elapsed = now.duration_since( *last_refill ).as_secs_f64();

        // Refill tokens based on elapsed time
        *tokens += elapsed * self.config.requests_per_second;
        *tokens = tokens.min( self.config.bucket_size as f64 );
        *last_refill = now;

        if *tokens >= 1.0
        {
          *tokens -= 1.0;
          metrics.available_tokens = *tokens;
          true
        } else {
          #[ cfg( feature = "logging" ) ]
          debug!( "Rate limit exceeded : {} tokens available", *tokens );
          metrics.available_tokens = *tokens;
          false
        }
      },
      RateLimiter::SlidingWindow { requests } => {
        let now = Instant::now();
        let window_duration = Duration::from_secs_f64( 1.0 / self.config.requests_per_second * self.config.bucket_size as f64 );

        // Remove old requests outside the window
        while let Some( &front_time ) = requests.front()
        {
          if now.duration_since( front_time ) > window_duration
          {
            requests.pop_front();
          } else {
            break;
          }
        }

        if requests.len() < self.config.bucket_size
        {
          requests.push_back( now );
          metrics.window_requests = requests.len();
          true
        } else {
          #[ cfg( feature = "logging" ) ]
          debug!( "Rate limit exceeded : {} requests in window", requests.len() );
          metrics.window_requests = requests.len();
          false
        }
      }
    };

    if allowed
    {
      metrics.allowed_requests += 1;
    } else {
      metrics.limited_requests += 1;
    }

    allowed
  }

  /// Get current rate limiting metrics
  pub fn get_metrics( &self ) -> RateLimitingMetrics
  {
    self.metrics.lock().unwrap().clone()
  }

  /// Reset rate limiter state (useful for testing)
  pub fn reset( &self )
  {
    let mut limiter = self.limiter.lock().unwrap();
    let mut metrics = self.metrics.lock().unwrap();

    match &mut *limiter
    {
      RateLimiter::TokenBucket { tokens, last_refill } => {
        *tokens = self.config.bucket_size as f64;
        *last_refill = Instant::now();
        metrics.available_tokens = *tokens;
      },
      RateLimiter::SlidingWindow { requests } => {
        requests.clear();
        metrics.window_requests = 0;
      }
    }

    metrics.total_requests = 0;
    metrics.limited_requests = 0;
    metrics.allowed_requests = 0;
  }
}

/// Execute an HTTP request with rate limiting protection
pub async fn execute_with_rate_limiting< T, R >
(
  client : &Client,
  method : Method,
  url : &str,
  api_key : &str,
  body : Option< &T >,
  config : &super::HttpConfig,
  rate_limiter : Option< &RateLimit >,
)
-> Result< R, Error >
where
  T: Serialize,
  R: for< 'de > Deserialize< 'de >,
{
  let Some( rl ) = rate_limiter else {
    // No rate limiter configured, use normal execution
    return super::execute( client, method, url, api_key, body, config ).await;
  };

  // Check if request should be allowed
  if !rl.should_allow_request().await
  {
    #[ cfg( feature = "logging" ) ]
    warn!( "Request rate limited" );

    return Err( Error::RateLimited(
      "Rate limit exceeded".to_string()
    ) );
  }

  // Execute the request
  super ::execute( client, method, url, api_key, body, config ).await
}
