//! Circuit breaker feature configuration setters for ClientBuilder.

use core::time::Duration;
use super::ClientBuilder;

impl ClientBuilder
{
  /// Enables or disables circuit breaker functionality.
  ///
  /// When enabled, the client will track failures and open the circuit
  /// after reaching the failure threshold, preventing further requests
  /// until the timeout period expires.
  #[ must_use ]
  #[ inline ]
  pub fn enable_circuit_breaker( mut self, enable : bool ) -> Self
  {
    self.enable_circuit_breaker = enable;
    self
  }

  /// Sets the failure threshold for circuit breaker.
  ///
  /// The circuit will open after this many consecutive failures.
  /// Must be greater than 0 (validated during `build()`).
  ///
  /// # Arguments
  ///
  /// * `threshold` - Number of failures before opening the circuit
  #[ must_use ]
  #[ inline ]
  pub fn circuit_breaker_failure_threshold( mut self, threshold : u32 ) -> Self
  {
    self.circuit_breaker_failure_threshold = threshold;
    self
  }

  /// Sets the success threshold for circuit breaker recovery.
  ///
  /// The circuit will close after this many consecutive successes
  /// when in half-open state. Must be greater than 0 (validated during `build()`).
  ///
  /// # Arguments
  ///
  /// * `threshold` - Number of successes before closing the circuit
  #[ must_use ]
  #[ inline ]
  pub fn circuit_breaker_success_threshold( mut self, threshold : u32 ) -> Self
  {
    self.circuit_breaker_success_threshold = threshold;
    self
  }

  /// Sets the timeout for circuit breaker recovery.
  ///
  /// After this timeout expires, the circuit will transition from
  /// open to half-open state, allowing test requests through.
  /// Must be greater than 0 (validated during `build()`.
  ///
  /// # Arguments
  ///
  /// * `timeout` - Duration to wait before allowing test requests
  #[ must_use ]
  #[ inline ]
  pub fn circuit_breaker_timeout( mut self, timeout : Duration ) -> Self
  {
    self.circuit_breaker_timeout = timeout;
    self
  }

  /// Enables or disables circuit breaker metrics collection.
  ///
  /// When enabled, the client will collect metrics about circuit breaker state:
  /// - Failure/success counts
  /// - State transition timestamps
  /// - Circuit open/close events
  #[ must_use ]
  #[ inline ]
  pub fn enable_circuit_breaker_metrics( mut self, enable_metrics : bool ) -> Self
  {
    self.enable_circuit_breaker_metrics = enable_metrics;
    self
  }

  /// Enables or disables shared circuit breaker state.
  ///
  /// When enabled, circuit breaker state will be shared across
  /// multiple client instances. When disabled, each client has
  /// its own isolated circuit state.
  #[ must_use ]
  #[ inline ]
  pub fn circuit_breaker_shared_state( mut self, shared : bool ) -> Self
  {
    self.circuit_breaker_shared_state = shared;
    self
  }
}
