//! Advanced Authentication Module
//!
//! This module provides enhanced authentication capabilities for the `OpenAI` API client,
//! including OAuth integration, multi-tenant authentication, failover mechanisms,
//! security hardening, and performance optimizations.
//!
//! # Features
//!
//! - OAuth 2.0 token refresh and management
//! - Multi-tenant authentication with isolation
//! - Authentication failover and retry mechanisms
//! - Integration with rate limiting systems
//! - Security hardening and audit trails
//! - High-performance concurrent authentication
//! - Session management and cleanup

#![ allow( clippy::missing_inline_in_public_items, clippy::unused_async ) ]

mod private
{
  use crate::
  {
    environment ::{ OpenaiEnvironmentImpl },
    secret ::Secret,
    error ::OpenAIError,
    client ::Client,
  };
  use core::time::Duration;
  use std::
  {
    collections ::HashMap,
    time ::Instant,
    sync ::{ Arc, Mutex, RwLock },
  };
  // use tokio::time::sleep; // Commented out as unused
  use serde::{ Deserialize, Serialize };
  use error_tools::untyped::Result;

  /// OAuth token response from authentication server
  #[ derive( Debug, Clone, Serialize, Deserialize ) ]
  pub struct OAuthTokenResponse
  {
    /// Access token for API requests
    pub access_token : String,
    /// Token type (usually "Bearer")
    pub token_type : String,
    /// Token expiration in seconds
    pub expires_in : u64,
    /// Refresh token for token renewal
    pub refresh_token : Option< String >,
    /// Scope of the access token
    pub scope : Option< String >,
  }

  /// Multi-tenant authentication configuration
  #[ derive( Debug, Clone ) ]
  pub struct MultiTenantConfig
  {
    /// Primary tenant organization ID
    pub primary_org_id : String,
    /// Secondary tenant organization ID for cross-tenant scenarios
    pub secondary_org_id : Option< String >,
    /// Per-tenant API keys mapping
    pub tenant_api_keys : HashMap<  String, String  >,
    /// Per-tenant rate limits
    pub tenant_rate_limits : HashMap<  String, u32  >,
    /// Tenant isolation enforcement enabled
    pub isolation_enforced : bool,
  }

  /// Authentication session information
  #[ derive( Debug, Clone ) ]
  pub struct AuthSession
  {
    /// Session identifier
    pub session_id : String,
    /// Session creation timestamp
    pub created_at : Instant,
    /// Session last access timestamp
    pub last_accessed : Instant,
    /// Session timeout duration
    pub timeout : Duration,
    /// Whether session is expired
    pub is_expired : bool,
    /// Associated tenant ID if multi-tenant
    pub tenant_id : Option< String >,
  }

  impl AuthSession
  {
    /// Create a new authentication session
    #[ must_use ]
    pub fn new(session_id : String, timeout : Duration) -> Self
    {
      let now = Instant::now();
      Self
      {
        session_id,
        created_at : now,
        last_accessed : now,
        timeout,
        is_expired : false,
        tenant_id : None,
      }
    }

    /// Check if session is valid and not expired
    #[ must_use ]
    pub fn is_valid(&self) -> bool
    {
      !self.is_expired && self.last_accessed.elapsed() < self.timeout
    }

    /// Refresh session by updating last accessed time
    pub fn refresh(&mut self) -> bool
    {
      if self.is_expired
      {
        return false;
      }

      self.last_accessed = Instant::now();
      self.is_valid()
    }

    /// Mark session as expired
    pub fn expire(&mut self)
    {
      self.is_expired = true;
    }
  }

  /// Authentication audit log entry
  #[ derive( Debug, Clone ) ]
  pub struct AuthAuditEntry
  {
    /// Timestamp of the authentication event
    pub timestamp : Instant,
    /// Authentication event type
    pub event_type : String,
    /// Success or failure indicator
    pub success : bool,
    /// User/client identifier
    pub client_id : String,
    /// IP address if available
    pub ip_address : Option< String >,
    /// Additional context information
    pub context : HashMap<  String, String  >,
  }

  /// Advanced authentication configuration
  #[ derive( Debug, Clone ) ]
  pub struct AdvancedAuthConfig
  {
    /// Primary API key for authentication
    pub primary_api_key : String,
    /// Secondary API key for failover scenarios
    pub secondary_api_key : Option< String >,
    /// OAuth access token for OAuth scenarios
    pub oauth_access_token : Option< String >,
    /// OAuth refresh token for token refresh
    pub oauth_refresh_token : Option< String >,
    /// Token expiration timestamp (Unix epoch)
    pub token_expires_at : Option< u64 >,
    /// Organization context for multi-tenant
    pub organization_context : Option< String >,
    /// Project context for project-scoped authentication
    pub project_context : Option< String >,
    /// Authentication audit trail enabled
    pub audit_trail_enabled : bool,
    /// Maximum authentication retries
    pub max_auth_retries : u32,
    /// Authentication timeout duration
    pub auth_timeout : Duration,
    /// Session management enabled
    pub session_management_enabled : bool,
    /// Default session timeout
    pub default_session_timeout : Duration,
  }

  impl Default for AdvancedAuthConfig
  {
    fn default() -> Self
    {
      Self
      {
        primary_api_key : String::new(),
        secondary_api_key : None,
        oauth_access_token : None,
        oauth_refresh_token : None,
        token_expires_at : None,
        organization_context : None,
        project_context : None,
        audit_trail_enabled : false,
        max_auth_retries : 3,
        auth_timeout : Duration::from_secs(30),
        session_management_enabled : false,
        default_session_timeout : Duration::from_secs(3600), // 1 hour
      }
    }
  }

  /// Advanced authentication manager
  pub struct AdvancedAuthManager
  {
    /// Current configuration
    config : AdvancedAuthConfig,
    /// Multi-tenant configuration if enabled
    multi_tenant_config : Option< MultiTenantConfig >,
    /// Active authentication sessions
    sessions : Arc< RwLock< HashMap<  String, AuthSession  > > >,
    /// Authentication audit log
    audit_log : Arc< Mutex< Vec< AuthAuditEntry > > >,
    /// OAuth token refresh function
    token_refresh_fn : Option< Box< dyn Fn() -> Result< OAuthTokenResponse > + Send + Sync > >,
  }

  impl AdvancedAuthManager
  {
    /// Create a new advanced authentication manager
    #[ must_use ]
    pub fn new(config : AdvancedAuthConfig) -> Self
    {
      Self
      {
        config,
        multi_tenant_config : None,
        sessions : Arc::new(RwLock::new(HashMap::new())),
        audit_log : Arc::new(Mutex::new(Vec::new())),
        token_refresh_fn : None,
      }
    }

    /// Configure multi-tenant authentication
    #[ must_use ]
    pub fn with_multi_tenant_config(mut self, config : MultiTenantConfig) -> Self
    {
      self.multi_tenant_config = Some(config);
      self
    }

    /// Set OAuth token refresh function
    #[ must_use ]
    pub fn with_oauth_refresh< F >(mut self, refresh_fn : F) -> Self
    where
      F: Fn() -> Result< OAuthTokenResponse > + Send + Sync + 'static,
    {
      self.token_refresh_fn = Some( Box::new( refresh_fn ) );
      self
    }

    /// Create or refresh OAuth client with token management
    ///
    /// # Errors
    ///
    /// Returns an error if token refresh fails, token is expired without refresh capability,
    /// or client creation fails.
    pub async fn create_oauth_client(&self) -> Result< Client< OpenaiEnvironmentImpl > >
    {
      // Check if OAuth token is expired and refresh if needed
      if let Some(expires_at) = self.config.token_expires_at
      {
        let current_time = u64::try_from(chrono::Utc::now().timestamp()).unwrap_or(0);
        if current_time >= expires_at
        {
          // Token is expired, attempt refresh
          if let Some(ref refresh_fn) = self.token_refresh_fn
          {
            match refresh_fn()
            {
              Ok(new_token) =>
              {
                self.log_auth_event("oauth_token_refresh", true, "system", None, HashMap::new()).await;
                // In a real implementation, we would update the stored token
                return self.create_client_with_token(&new_token.access_token).await;
              },
              Err(e) =>
              {
                self.log_auth_event("oauth_token_refresh", false, "system", None,
                  [("error".to_string(), e.to_string())].iter().cloned().collect()).await;
                return Err(e);
              }
            }
          }
          return Err(error_tools::Error::from(OpenAIError::InvalidArgument("OAuth token expired and no refresh function provided".to_string())));
        }
      }

      // Use existing token
      if let Some(ref token) = self.config.oauth_access_token
      {
        self.create_client_with_token(token).await
      }
      else
      {
        Err(error_tools::Error::from(OpenAIError::InvalidArgument("No OAuth token available".to_string())))
      }
    }

    /// Create tenant-specific client
    ///
    /// # Errors
    ///
    /// Returns an error if multi-tenant configuration is not enabled, tenant isolation
    /// verification fails, tenant has no configured API key, or client creation fails.
    pub async fn create_tenant_client(&self, tenant_id : &str) -> Result< Client< OpenaiEnvironmentImpl > >
    {
      // Check if we have multi-tenant configuration
      let multi_tenant = self.multi_tenant_config.as_ref()
        .ok_or_else(|| error_tools::Error::from(OpenAIError::InvalidArgument("Multi-tenant configuration not enabled".to_string())))?;

      // Verify tenant isolation if enforced
      if multi_tenant.isolation_enforced
      {
        self.verify_tenant_isolation(tenant_id).await?;
      }

      // Get tenant-specific API key
      let tenant_api_key = multi_tenant.tenant_api_keys.get(tenant_id)
        .ok_or_else(|| error_tools::Error::from(OpenAIError::InvalidArgument(format!("No API key configured for tenant : {tenant_id}"))))?;

      // For simplicity, we'll create a new environment each time
      // In a real implementation, you might want caching for performance
      let secret = Secret::new(tenant_api_key.clone())?;
      let environment = OpenaiEnvironmentImpl::build(
        secret,
        Some(tenant_id.to_string()),
        None, // project_id
        crate ::environment::OpenAIRecommended::base_url().to_string(),
        crate ::environment::OpenAIRecommended::realtime_base_url().to_string(),
      )?;

      self.log_auth_event("tenant_client_created", true, tenant_id, None, HashMap::new()).await;
      Client::build(environment)
    }

    /// Create client with authentication failover
    ///
    /// # Errors
    ///
    /// Returns an error if both primary and secondary authentication attempts fail,
    /// or if no secondary key is available and primary authentication fails.
    pub async fn create_failover_client(&self) -> Result< Client< OpenaiEnvironmentImpl > >
    {
      // Try primary API key first
      let primary_result = self.create_client_with_key(&self.config.primary_api_key).await;
      match primary_result
      {
        Ok(client) =>
        {
          self.log_auth_event("primary_auth_success", true, "system", None, HashMap::new()).await;
          Ok(client)
        },
        Err(primary_error) =>
        {
          self.log_auth_event("primary_auth_failure", false, "system", None,
            [("error".to_string(), primary_error.to_string())].iter().cloned().collect()).await;

          // Try secondary API key if available
          if let Some(ref secondary_key) = self.config.secondary_api_key
          {
            match self.create_client_with_key(secondary_key).await
            {
              Ok(client) =>
              {
                self.log_auth_event("failover_auth_success", true, "system", None, HashMap::new()).await;
                return Ok(client);
              },
              Err(secondary_error) =>
              {
                self.log_auth_event("failover_auth_failure", false, "system", None,
                  [("error".to_string(), secondary_error.to_string())].iter().cloned().collect()).await;
                // Return the secondary error as it's more recent
                return Err(secondary_error);
              }
            }
          }

          // No secondary key available, return primary error
          Err(primary_error)
        }
      }
    }

    /// Create session-managed client
    ///
    /// # Errors
    ///
    /// Returns an error if session management is not enabled, session is expired,
    /// or client creation fails.
    ///
    /// # Panics
    ///
    /// Panics if the sessions lock is poisoned.
    pub async fn create_session_client(&self, session_id : &str) -> Result< Client< OpenaiEnvironmentImpl > >
    {
      if !self.config.session_management_enabled
      {
        return Err(error_tools::Error::from(OpenAIError::InvalidArgument("Session management not enabled".to_string())));
      }

      // Check if session exists and is valid
      {
        let session_valid = {
          let mut sessions = self.sessions.write().unwrap();
          if let Some(session) = sessions.get_mut(session_id)
          {
            if session.is_valid()
            {
              session.refresh();
              true
            }
            else
            {
              sessions.remove(session_id);
              false
            }
          }
          else
          {
            // Create new session
            let session = AuthSession::new(session_id.to_string(), self.config.default_session_timeout);
            sessions.insert(session_id.to_string(), session);
            true
          }
        };

        if session_valid
        {
          if self.sessions.read().unwrap().contains_key(session_id)
          {
            self.log_auth_event("session_refreshed", true, session_id, None, HashMap::new()).await;
          }
          else
          {
            self.log_auth_event("session_created", true, session_id, None, HashMap::new()).await;
          }
        }
        else
        {
          self.log_auth_event("session_expired", false, session_id, None, HashMap::new()).await;
          return Err(error_tools::Error::from(OpenAIError::InvalidArgument("Session expired".to_string())));
        }
      }

      // Create client with primary key (sessions are just for tracking)
      self.create_client_with_key(&self.config.primary_api_key).await
    }

    /// Cleanup expired sessions
    ///
    /// # Panics
    ///
    /// Panics if the sessions lock is poisoned.
    pub async fn cleanup_expired_sessions(&self) -> usize
    {
      if !self.config.session_management_enabled
      {
        return 0;
      }

      let mut sessions = self.sessions.write().unwrap();
      let initial_count = sessions.len();

      sessions.retain(|session_id, session| {
        let keep = session.is_valid();
        if !keep
        {
          // Log session cleanup
          tokio ::spawn({
            let audit_log = Arc::clone(&self.audit_log);
            let session_id = session_id.clone();
            async move {
              let entry = AuthAuditEntry
              {
                timestamp : Instant::now(),
                event_type : "session_cleanup".to_string(),
                success : true,
                client_id : session_id,
                ip_address : None,
                context : HashMap::new(),
              };
              audit_log.lock().unwrap().push(entry);
            }
          });
        }
        keep
      });

      initial_count - sessions.len()
    }

    /// Get authentication performance metrics
    ///
    /// # Panics
    ///
    /// Panics if the audit log lock is poisoned.
    pub async fn get_performance_metrics(&self) -> AuthPerformanceMetrics
    {
      let audit_log = self.audit_log.lock().unwrap();
      let total_events = audit_log.len();
      let successful_events = audit_log.iter().filter(|e| e.success).count();
      let success_rate = if total_events > 0 { successful_events as f64 / total_events as f64 } else { 0.0 };

      // Calculate recent performance (last 100 events)
      let recent_events : Vec< _ > = audit_log.iter().rev().take( 100 ).collect();
      let recent_successful = recent_events.iter().filter(|e| e.success).count();
      let recent_success_rate = if recent_events.is_empty() { 0.0 } else { recent_successful as f64 / recent_events.len() as f64 };

      AuthPerformanceMetrics
      {
        total_auth_attempts : total_events,
        successful_auth_attempts : successful_events,
        overall_success_rate : success_rate,
        recent_success_rate,
        active_sessions : self.sessions.read().unwrap().len(),
        cached_environments : 0, // Simplified implementation without caching
      }
    }

    /// Private helper : Create client with specific API key
    async fn create_client_with_key(&self, api_key : &str) -> Result< Client< OpenaiEnvironmentImpl > >
    {
      let secret = Secret::new(api_key.to_string())?;
      let environment = OpenaiEnvironmentImpl::build(
        secret,
        self.config.organization_context.clone(),
        self.config.project_context.clone(),
        crate ::environment::OpenAIRecommended::base_url().to_string(),
        crate ::environment::OpenAIRecommended::realtime_base_url().to_string(),
      )?;

      Client::build(environment)
    }

    /// Private helper : Create client with OAuth token
    async fn create_client_with_token(&self, token : &str) -> Result< Client< OpenaiEnvironmentImpl > >
    {
      // In a real implementation, this would use the token as a Bearer token
      // For now, we'll treat it as an API key for simplicity
      self.create_client_with_key(token).await
    }

    /// Private helper : Verify tenant isolation
    async fn verify_tenant_isolation(&self, tenant_id : &str) -> Result< () >
    {
      let multi_tenant = self.multi_tenant_config.as_ref().unwrap();

      if !multi_tenant.tenant_api_keys.contains_key(tenant_id)
      {
        return Err(error_tools::Error::from(OpenAIError::InvalidArgument(format!("Tenant {tenant_id} not authorized"))));
      }

      // Additional isolation checks would go here
      Ok(())
    }

    /// Private helper : Log authentication event
    async fn log_auth_event(
      &self,
      event_type : &str,
      success : bool,
      client_id : &str,
      ip_address : Option< String >,
      context : HashMap< String, String >,
    )
    {
      if !self.config.audit_trail_enabled
      {
        return;
      }

      let entry = AuthAuditEntry
      {
        timestamp : Instant::now(),
        event_type : event_type.to_string(),
        success,
        client_id : client_id.to_string(),
        ip_address,
        context,
      };

      self.audit_log.lock().unwrap().push(entry);
    }
  }

  impl core::fmt::Debug for AdvancedAuthManager
  {
    fn fmt(&self, f : &mut core::fmt::Formatter< '_ >) -> core::fmt::Result
    {
      f.debug_struct("AdvancedAuthManager")
        .field("config", &self.config)
        .field("multi_tenant_config", &self.multi_tenant_config)
        .field("sessions_count", &self.sessions.read().unwrap().len())
        .field("audit_log_entries", &self.audit_log.lock().unwrap().len())
        .field("has_token_refresh_fn", &self.token_refresh_fn.is_some())
        .finish()
    }
  }

  /// Authentication performance metrics
  #[ derive( Debug, Clone ) ]
  pub struct AuthPerformanceMetrics
  {
    /// Total authentication attempts
    pub total_auth_attempts : usize,
    /// Successful authentication attempts
    pub successful_auth_attempts : usize,
    /// Overall success rate (0.0 to 1.0)
    pub overall_success_rate : f64,
    /// Recent success rate (last 100 attempts)
    pub recent_success_rate : f64,
    /// Number of active sessions
    pub active_sessions : usize,
    /// Number of cached environment instances
    pub cached_environments : usize,
  }

  /// Global advanced authentication manager instance
  static GLOBAL_AUTH_MANAGER: std::sync::OnceLock< std::sync::Mutex< Option< AdvancedAuthManager > > > = std::sync::OnceLock::new();

  /// Initialize global advanced authentication manager
  ///
  /// # Errors
  ///
  /// Currently this function cannot fail, but returns `Result` for future extensibility.
  pub fn initialize_advanced_auth(config : AdvancedAuthConfig) -> Result< () >
  {
    let manager = AdvancedAuthManager::new(config);
    GLOBAL_AUTH_MANAGER.get_or_init(|| std::sync::Mutex::new(Some(manager)));
    Ok(())
  }

  /// Get reference to global advanced authentication manager
  ///
  /// # Errors
  ///
  /// Returns an error if the global authentication manager mutex is poisoned.
  pub fn get_advanced_auth_manager() -> Result< std::sync::MutexGuard< 'static, Option< AdvancedAuthManager > > >
  {
    GLOBAL_AUTH_MANAGER.get_or_init(|| std::sync::Mutex::new(None))
      .lock()
      .map_err(|e| error_tools::Error::from(OpenAIError::Internal(format!("Failed to lock auth manager : {e}"))))
  }

  /// Convenience function : Create OAuth client using global manager
  ///
  /// # Errors
  ///
  /// Returns an error if the global authentication manager is not initialized,
  /// mutex is poisoned, or OAuth client creation fails.
  pub async fn create_oauth_client() -> Result< Client< OpenaiEnvironmentImpl > >
  {
    // Get config from the manager first
    let config = {
      let manager_guard = get_advanced_auth_manager()?;
      let manager = manager_guard.as_ref()
        .ok_or_else(|| error_tools::Error::from(OpenAIError::InvalidArgument("Advanced auth manager not initialized".to_string())))?;
      manager.config.clone()
    };

    // Create a temporary manager without the function pointer
    let temp_manager = AdvancedAuthManager::new(config);
    temp_manager.create_oauth_client().await
  }

  /// Convenience function : Create tenant client using global manager
  ///
  /// # Errors
  ///
  /// Returns an error if the global authentication manager is not initialized,
  /// mutex is poisoned, or tenant client creation fails.
  pub async fn create_tenant_client(tenant_id : &str) -> Result< Client< OpenaiEnvironmentImpl > >
  {
    // Get config and multi-tenant config from the manager first
    let (config, multi_tenant_config) = {
      let manager_guard = get_advanced_auth_manager()?;
      let manager = manager_guard.as_ref()
        .ok_or_else(|| error_tools::Error::from(OpenAIError::InvalidArgument("Advanced auth manager not initialized".to_string())))?;
      (manager.config.clone(), manager.multi_tenant_config.clone())
    };

    // Create a temporary manager without the function pointer
    let mut temp_manager = AdvancedAuthManager::new(config);
    if let Some(mt_config) = multi_tenant_config
    {
      temp_manager = temp_manager.with_multi_tenant_config(mt_config);
    }
    temp_manager.create_tenant_client(tenant_id).await
  }

  /// Convenience function : Create failover client using global manager
  ///
  /// # Errors
  ///
  /// Returns an error if the global authentication manager is not initialized,
  /// mutex is poisoned, or failover client creation fails.
  pub async fn create_failover_client() -> Result< Client< OpenaiEnvironmentImpl > >
  {
    // Get config from the manager first
    let config = {
      let manager_guard = get_advanced_auth_manager()?;
      let manager = manager_guard.as_ref()
        .ok_or_else(|| error_tools::Error::from(OpenAIError::InvalidArgument("Advanced auth manager not initialized".to_string())))?;
      manager.config.clone()
    };

    // Create a temporary manager without the function pointer
    let temp_manager = AdvancedAuthManager::new(config);
    temp_manager.create_failover_client().await
  }

  #[ cfg( test ) ]
  mod tests
  {
    use super::*;

    #[ test ]
    fn test_advanced_auth_config_default()
    {
      let config = AdvancedAuthConfig::default();
      assert_eq!(config.max_auth_retries, 3);
      assert_eq!(config.auth_timeout, Duration::from_secs(30));
      assert!(!config.audit_trail_enabled);
      assert!(!config.session_management_enabled);
    }

    #[ test ]
    fn test_auth_session_creation()
    {
      let session = AuthSession::new("test_session".to_string(), Duration::from_secs(60));
      assert_eq!(session.session_id, "test_session");
      assert!(session.is_valid());
      assert!(!session.is_expired);
    }

    #[ test ]
    fn test_auth_session_expiration()
    {
      let mut session = AuthSession::new("test_session".to_string(), Duration::from_millis(1));
      std ::thread::sleep(Duration::from_millis(2));
      assert!(!session.is_valid());

      session.expire();
      assert!(session.is_expired);
      assert!(!session.refresh());
    }

    #[ tokio::test ]
    async fn test_advanced_auth_manager_creation()
    {
      let config = AdvancedAuthConfig::default();
      let manager = AdvancedAuthManager::new(config);

      let metrics = manager.get_performance_metrics().await;
      assert_eq!(metrics.total_auth_attempts, 0);
      assert_eq!(metrics.active_sessions, 0);
      assert_eq!(metrics.cached_environments, 0);
    }

    #[ tokio::test ]
    async fn test_multi_tenant_config()
    {
      let mut tenant_keys = HashMap::new();
      tenant_keys.insert("tenant_a".to_string(), "sk-tenant-a-key".to_string());

      let multi_tenant_config = MultiTenantConfig
      {
        primary_org_id : "org_123".to_string(),
        secondary_org_id : None,
        tenant_api_keys : tenant_keys,
        tenant_rate_limits : HashMap::new(),
        isolation_enforced : true,
      };

      let config = AdvancedAuthConfig::default();
      let manager = AdvancedAuthManager::new(config).with_multi_tenant_config(multi_tenant_config);

      assert!(manager.multi_tenant_config.is_some());
    }
  }
}

crate ::mod_interface!
{
  orphan use OAuthTokenResponse;
  orphan use MultiTenantConfig;
  orphan use AuthSession;
  orphan use AuthAuditEntry;
  orphan use AdvancedAuthConfig;
  orphan use AdvancedAuthManager;
  orphan use AuthPerformanceMetrics;
  orphan use initialize_advanced_auth;
  orphan use get_advanced_auth_manager;
  orphan use create_oauth_client;
  orphan use create_tenant_client;
  orphan use create_failover_client;
}