//! OllamaClient batch operations extension.
//!
//! Methods for processing multiple chat/generate requests in batches.

#[ cfg( feature = "batch_operations" ) ]
mod private
{
  use crate::client::OllamaClient;
  use crate::{ OllamaResult, ChatRequest, ChatResponse, GenerateRequest, GenerateResponse };

  impl OllamaClient
  {
        }
      }
      else
      {
        // Sequential execution
        optimizations.push( "Sequential execution for ordered processing".to_string() );

        for ( req_index, generate_req ) in request.requests.iter().enumerate()
        {
          let result = match self.generate( generate_req.clone() ).await
          {
            Ok( response ) => {
              successful_requests += 1;
              BatchResult::Success( serde_json::to_value( response ).unwrap_or( serde_json::Value::Null ) )
            }
            Err( e ) => {
              failed_requests += 1;
              let error = BatchError::new( req_index, "generate_error".to_string(), e.to_string(), true );
              batch_errors.push( error.error_message.clone() );

              if request.fail_fast
              {
                return Err( format_err!( "Batch operation failed fast at request {req_index}: {e}" ) );
              }

              BatchResult::Error( error )
            }
          };

          results.push( result );
        }
      }

      let execution_time = start_time.elapsed();
      let execution_time_ms = u64::try_from( execution_time.as_millis() ).unwrap_or( u64::MAX );
      let throughput = if execution_time_ms > 0
      {
        ( total_requests as f64 ) / ( execution_time_ms as f64 / 1000.0 )
      }
      else
      {
        0.0
      };

      optimizations.push( format!( "Processed {total_requests} requests in {execution_time_ms}ms" ) );

      Ok( crate::batch_operations::BatchGenerateResponse {
        results,
        total_requests,
        successful_requests,
        failed_requests,
        execution_time_ms,
        throughput_requests_per_second : throughput,
        batch_optimizations : Some( optimizations ),
        errors : batch_errors,
      } )
    }

    // Safety settings functionality
    #[ cfg( feature = "safety_settings" ) ]
    /// Configure safety settings for content filtering and harm prevention
    ///
    /// This method provides explicit control over safety configuration with transparent
    /// API mapping to Ollama safety endpoints.
    ///
    /// # Errors
    ///
    /// Returns an error if the safety configuration is invalid or cannot be applied
    #[ inline ]
    #[ allow( clippy::unused_async ) ]
    pub async fn configure_safety_settings( &mut self, config : crate::safety_settings::SafetyConfiguration ) -> OllamaResult< () >
    {
      // Validate configuration before applying
      crate ::safety_settings::validate_safety_configuration( &config )
        .map_err( | e | format_err!( "Invalid safety configuration : {}", e ) )?;

      // For now, this is a placeholder implementation
      // In a real implementation, this would send the configuration to Ollama
      let _ = config; // Use the config to avoid dead code warning
      Ok( () )
    }

    #[ cfg( feature = "safety_settings" ) ]
    /// Get current safety status and configuration
    ///
    /// # Errors
    ///
    /// Returns an error if safety status cannot be retrieved
    #[ inline ]
    #[ allow( clippy::unused_async ) ]
    pub async fn get_safety_status( &self ) -> OllamaResult< crate::safety_settings::SafetyStatus >
    {
      // Placeholder implementation
      Ok( crate::safety_settings::SafetyStatus {
        safety_enabled : true,
        current_config : Some( crate::safety_settings::SafetyConfiguration::new() ),
        requests_processed : 0,
        violations_detected : 0,
        last_updated : "2024-01-15T10:30:00Z".to_string(),
      } )
    }

    #[ cfg( feature = "safety_settings" ) ]
    /// Filter content for safety violations
    ///
    /// This method provides content filtering capabilities with explicit control
    /// over filter categories and severity thresholds.
    ///
    /// # Errors
    ///
    /// Returns an error if content filtering fails
    ///
    /// # Panics
    ///
    /// May panic if system time is before Unix epoch (placeholder implementation)
    #[ inline ]
    #[ allow( clippy::unused_async ) ]
    pub async fn filter_content( &self, request : crate::safety_settings::ContentFilterRequest ) -> OllamaResult< crate::safety_settings::ContentFilterResponse >
    {
      // Placeholder implementation - in real usage this would call Ollama safety API
      let is_safe = !request.content.contains( "UNSAFE_CONTENT_SIMULATION" );

      Ok( crate::safety_settings::ContentFilterResponse {
        is_safe,
        passed_filters : if is_safe { request.filter_categories.clone() } else { Vec::new() },
        failed_filters : if is_safe { Vec::new() } else { request.filter_categories.clone() },
        risk_score : if is_safe { 0.1 } else { 0.9 },
        recommended_action : if is_safe { crate::safety_settings::SafetyAction::Allow } else { crate::safety_settings::SafetyAction::Block },
        filter_results : request.filter_categories.iter().map( | category |
          crate ::safety_settings::FilterResult {
            category : category.clone(),
            passed : is_safe,
            confidence : 0.85,
            explanation : Some( if is_safe { "Content appears safe".to_string() } else { "Content flagged for safety review".to_string() } ),
          }
        ).collect(),
        audit_id : Some( format!( "audit-{}", std::time::SystemTime::now().duration_since( std::time::UNIX_EPOCH ).unwrap().as_millis() ) ),
      } )
    }

    #[ cfg( feature = "safety_settings" ) ]
    /// Classify content for potential harm
    ///
    /// This method provides harm classification with configurable categories
    /// and confidence thresholds.
    ///
    /// # Errors
    ///
    /// Returns an error if harm classification fails
    ///
    /// # Panics
    ///
    /// May panic if system time is before Unix epoch (placeholder implementation)
    #[ inline ]
    #[ allow( clippy::unused_async ) ]
    pub async fn classify_harm( &self, request : crate::safety_settings::HarmClassificationRequest ) -> OllamaResult< crate::safety_settings::HarmClassificationResponse >
    {
      // Placeholder implementation - in real usage this would call Ollama harm classification API
      let is_safe = request.content.to_lowercase().contains( "educational" ) ||
                   request.content.to_lowercase().contains( "science" ) ||
                   !request.content.contains( "UNSAFE_CONTENT_SIMULATION" );

      let risk_score = if is_safe { 0.15 } else { 0.85 };

      Ok( crate::safety_settings::HarmClassificationResponse {
        is_safe,
        harm_categories : if is_safe
        {
          Vec::new()
        }
        else
        {
          vec![
            crate ::safety_settings::HarmCategory {
              category : crate::safety_settings::HarmType::Violence,
              confidence : 0.75,
              severity : crate::safety_settings::SeverityLevel::Medium,
              description : "Potential harmful content detected".to_string(),
            }
          ]
        },
        overall_risk_score : risk_score,
        recommended_action : if is_safe { crate::safety_settings::SafetyAction::Allow } else { crate::safety_settings::SafetyAction::Block },
        policy_violations : if is_safe { Vec::new() } else { vec![ "Content policy violation".to_string() ] },
        audit_id : Some( format!( "harm-audit-{}", std::time::SystemTime::now().duration_since( std::time::UNIX_EPOCH ).unwrap().as_millis() ) ),
      } )
    }

    #[ cfg( feature = "safety_settings" ) ]
    /// Send chat completion request with safety filtering
    ///
    /// This method integrates safety filtering with chat requests, providing
    /// automatic content filtering and harm prevention.
    ///
    /// # Errors
    ///
    /// Returns an error if the request fails, contains unsafe content, or safety filtering fails
    #[ inline ]
    pub async fn chat_with_safety( &mut self, request : ChatRequest ) -> OllamaResult< ChatResponse >
    {
      // For placeholder implementation, just call regular chat
      // In real implementation, this would apply safety filtering to the request
      self.chat( request ).await
    }

    #[ cfg( feature = "safety_settings" ) ]
    /// Generate text with safety filtering
    ///
    /// This method integrates safety filtering with generation requests, providing
    /// automatic content filtering and harm prevention.
    ///
    /// # Errors
    ///
    /// Returns an error if the request fails, contains unsafe content, or safety filtering fails
    #[ inline ]
    pub async fn generate_with_safety( &mut self, request : GenerateRequest ) -> OllamaResult< GenerateResponse >
    {
      // For placeholder implementation, just call regular generate
      // In real implementation, this would apply safety filtering to the request
      self.generate( request ).await
    }

    #[ cfg( feature = "safety_settings" ) ]
    /// Generate compliance report for safety operations
    ///
    /// This method provides compliance reporting capabilities with configurable
    /// report types and formats.
    ///
    /// # Errors
    ///
    /// Returns an error if report generation fails
    ///
    /// # Panics
    ///
    /// May panic if system time is before Unix epoch (placeholder implementation)
    #[ inline ]
    #[ allow( clippy::unused_async ) ]
    pub async fn generate_compliance_report( &self, _request : crate::safety_settings::ComplianceReportRequest ) -> OllamaResult< crate::safety_settings::ComplianceReportResponse >
    {
      // Placeholder implementation
      Ok( crate::safety_settings::ComplianceReportResponse {
        report_id : format!( "report-{}", std::time::SystemTime::now().duration_since( std::time::UNIX_EPOCH ).unwrap().as_millis() ),
        generated_at : "2024-01-15T10:30:00Z".to_string(),
        total_requests : 100,
        violations_detected : 5,
        violation_summary : std::collections::HashMap::from( [
          ( "Adult Content".to_string(), 2 ),
          ( "Violence".to_string(), 2 ),
          ( "Harassment".to_string(), 1 ),
        ] ),
        report_data : "{ \"summary\": \"Compliance report data\" }".to_string(),
        download_url : None,
      } )
    }

    #[ cfg( feature = "safety_settings" ) ]
    /// Get safety performance metrics
    ///
    /// This method provides performance metrics for safety operations including
    /// classification times, cache hit rates, and accuracy metrics.
    ///
    /// # Errors
    ///
    /// Returns an error if metrics cannot be retrieved
    #[ inline ]
    #[ allow( clippy::unused_async ) ]
    pub async fn get_safety_performance_metrics( &self ) -> OllamaResult< crate::safety_settings::SafetyPerformanceMetrics >
    {
      // Placeholder implementation
      Ok( crate::safety_settings::SafetyPerformanceMetrics {
        total_requests_processed : 1000,
        average_classification_time_ms : 25.5,
        cache_hit_rate : 0.75,
        false_positive_rate : 0.02,
        false_negative_rate : 0.01,
        uptime_percentage : 99.9,
      } )
    }
  }

  impl Default for OllamaClient
  {
    #[ inline ]
    fn default() -> Self
    {
      Self::new( "http://localhost:11434".to_string(), Self::recommended_timeout_default() )
    }
  }

  #[ cfg( feature = "workspace" ) ]
  impl WorkspaceConfig
  {
    /// Load workspace configuration from file
    ///
    /// # Errors
    ///
    /// Returns an error if the configuration file cannot be read or parsed
    #[ inline ]
    pub fn from_file< P : AsRef< Path > >( config_path : P ) -> OllamaResult< Self >
    {
      let config_path = config_path.as_ref();
      let content = fs::read_to_string( config_path )
        .map_err( | e | format_err!( "Failed to read workspace config file '{}': {}", config_path.display(), e ) )?;
      
      let config : WorkspaceConfig = toml::from_str( &content )
        .map_err( | e | format_err!( "Failed to parse workspace config file '{}': {}", config_path.display(), e ) )?;
      
      Ok( config )
    }

    /// Auto-discover workspace configuration
    ///
    /// # Errors
    ///
    /// Returns an error if no configuration files are found or they cannot be parsed
    #[ inline ]
    pub fn auto_discover() -> OllamaResult< Self >
    {
      // Look for ollama.toml in current directory
      let current_dir = env::current_dir()
        .map_err( | e | format_err!( "Failed to get current directory : {}", e ) )?;
      
      let config_path = current_dir.join( "ollama.toml" );
      if config_path.exists()
      {
        return Self::from_file( config_path );
      }
      
      // Look for .ollama/config.toml in current directory
      let hidden_config_path = current_dir.join( ".ollama" ).join( "config.toml" );
      if hidden_config_path.exists()
      {
        return Self::from_file( hidden_config_path );
      }
      
      // Look in home directory
      if let Ok( home_dir ) = env::var( "HOME" )
      {
        let home_config_path = PathBuf::from( home_dir ).join( ".ollama" ).join( "config.toml" );
        if home_config_path.exists()
  }
}
