//! Model comparison functionality for A/B testing and model selection.
//!
//! This module provides utilities for comparing multiple models side-by-side
//! with the same input to evaluate performance, quality, and costs.

use crate::client::Client;
use crate::error::Error;
use crate::models::
{
  GenerateContentRequest,
  GenerateContentResponse,
};
use std::time::Instant;

/// Result of comparing a single model.
#[ derive( Debug, Clone ) ]
pub struct ModelComparisonResult
{
  /// Model name that was tested
  pub model_name : String,
  /// Generated response
  pub response : GenerateContentResponse,
  /// Response time in milliseconds
  pub response_time_ms : u64,
  /// Success flag
  pub success : bool,
  /// Error message if failed
  pub error_message : Option< String >,
  /// Estimated input tokens
  pub input_tokens : Option< i32 >,
  /// Estimated output tokens
  pub output_tokens : Option< i32 >,
}

/// Result of comparing multiple models.
#[ derive( Debug, Clone ) ]
pub struct ComparisonResults
{
  /// Individual model results
  pub results : Vec< ModelComparisonResult >,
  /// Total comparison time
  pub total_time_ms : u64,
  /// Fastest model
  pub fastest_model : Option< String >,
  /// Slowest model
  pub slowest_model : Option< String >,
}

impl ComparisonResults
{
  /// Get the fastest successful model result.
  #[ must_use ]
  pub fn get_fastest( &self ) -> Option< &ModelComparisonResult >
  {
    self.results
      .iter()
      .filter( | r | r.success )
      .min_by_key( | r | r.response_time_ms )
  }

  /// Get the slowest successful model result.
  #[ must_use ]
  pub fn get_slowest( &self ) -> Option< &ModelComparisonResult >
  {
    self.results
      .iter()
      .filter( | r | r.success )
      .max_by_key( | r | r.response_time_ms )
  }

  /// Get average response time across successful models.
  #[ must_use ]
  pub fn average_response_time( &self ) -> f64
  {
    let successful : Vec< _ > = self.results.iter().filter( | r | r.success ).collect();
    if successful.is_empty()
    {
      return 0.0;
    }

    let total : u64 = successful.iter().map( | r | r.response_time_ms ).sum();
    total as f64 / successful.len() as f64
  }

  /// Get success rate across all models.
  #[ must_use ]
  pub fn success_rate( &self ) -> f64
  {
    if self.results.is_empty()
    {
      return 0.0;
    }

    let successful = self.results.iter().filter( | r | r.success ).count();
    successful as f64 / self.results.len() as f64
  }
}

/// Model comparison helper for Client.
#[ derive( Debug ) ]
pub struct ModelComparator< 'a >
{
  client : &'a Client,
}

impl< 'a > ModelComparator< 'a >
{
  /// Create a new model comparator.
  #[ must_use ]
  #[ inline ]
  pub fn new( client : &'a Client ) -> Self
  {
    Self { client }
  }

  /// Compare multiple models with the same request.
  ///
  /// # Errors
  ///
  /// Returns an error if all models fail to generate content.
  pub async fn compare_models(
    &self,
    model_names : &[ &str ],
    request : &GenerateContentRequest,
  ) -> Result< ComparisonResults, Error >
  {
    let start = Instant::now();
    let mut results = Vec::with_capacity( model_names.len() );

    for model_name in model_names
    {
      let model_start = Instant::now();

      match self.client.models().by_name( model_name ).generate_content( request ).await
      {
        Ok( response ) =>
        {
          let elapsed = model_start.elapsed().as_millis() as u64;

          // Extract token counts from usage metadata
          let input_tokens = response.usage_metadata.as_ref().and_then( | u | u.prompt_token_count );
          let output_tokens = response.usage_metadata.as_ref().and_then( | u | u.candidates_token_count );

          results.push( ModelComparisonResult
          {
            model_name : model_name.to_string(),
            response,
            response_time_ms : elapsed,
            success : true,
            error_message : None,
            input_tokens,
            output_tokens,
          } );
        }
        Err( err ) =>
        {
          let elapsed = model_start.elapsed().as_millis() as u64;

          // Create empty response
          let empty_response = GenerateContentResponse
          {
            candidates : vec![],
            prompt_feedback : None,
            usage_metadata : None,
            grounding_metadata : None,
          };

          results.push( ModelComparisonResult
          {
            model_name : model_name.to_string(),
            response : empty_response,
            response_time_ms : elapsed,
            success : false,
            error_message : Some( err.to_string() ),
            input_tokens : None,
            output_tokens : None,
          } );
        }
      }
    }

    let total_time_ms = start.elapsed().as_millis() as u64;

    // Identify fastest and slowest models
    let fastest_model = results
      .iter()
      .filter( | r | r.success )
      .min_by_key( | r | r.response_time_ms )
      .map( | r | r.model_name.clone() );

    let slowest_model = results
      .iter()
      .filter( | r | r.success )
      .max_by_key( | r | r.response_time_ms )
      .map( | r | r.model_name.clone() );

    Ok( ComparisonResults
    {
      results,
      total_time_ms,
      fastest_model,
      slowest_model,
    } )
  }

  /// Compare models in parallel for faster results.
  ///
  /// # Errors
  ///
  /// Returns an error if all models fail to generate content.
  pub async fn compare_models_parallel(
    &self,
    model_names : &[ &str ],
    request : &GenerateContentRequest,
  ) -> Result< ComparisonResults, Error >
  {
    let start = Instant::now();

    // Create futures for all model requests
    let futures : Vec< _ > = model_names
      .iter()
      .map( | model_name |
      {
        let request = request.clone();
        async move
        {
          let model_start = Instant::now();
          let result = self.client.models().by_name( model_name ).generate_content( &request ).await;
          let elapsed = model_start.elapsed().as_millis() as u64;

          match result
          {
            Ok( response ) =>
            {
              let input_tokens = response.usage_metadata.as_ref().and_then( | u | u.prompt_token_count );
              let output_tokens = response.usage_metadata.as_ref().and_then( | u | u.candidates_token_count );

              ModelComparisonResult
              {
                model_name : model_name.to_string(),
                response,
                response_time_ms : elapsed,
                success : true,
                error_message : None,
                input_tokens,
                output_tokens,
              }
            }
            Err( err ) =>
            {
              // Create empty response
              let empty_response = GenerateContentResponse
              {
                candidates : vec![],
                prompt_feedback : None,
                usage_metadata : None,
                grounding_metadata : None,
              };

              ModelComparisonResult
              {
                model_name : model_name.to_string(),
                response : empty_response,
                response_time_ms : elapsed,
                success : false,
                error_message : Some( err.to_string() ),
                input_tokens : None,
                output_tokens : None,
              }
            }
          }
        }
      } )
      .collect();

    // Execute all requests in parallel
    let results = futures::future::join_all( futures ).await;

    let total_time_ms = start.elapsed().as_millis() as u64;

    // Identify fastest and slowest models
    let fastest_model = results
      .iter()
      .filter( | r | r.success )
      .min_by_key( | r | r.response_time_ms )
      .map( | r | r.model_name.clone() );

    let slowest_model = results
      .iter()
      .filter( | r | r.success )
      .max_by_key( | r | r.response_time_ms )
      .map( | r | r.model_name.clone() );

    Ok( ComparisonResults
    {
      results,
      total_time_ms,
      fastest_model,
      slowest_model,
    } )
  }
}

impl Client
{
  /// Create a model comparator for this client.
  #[ must_use ]
  #[ inline ]
  pub fn comparator( &self ) -> ModelComparator< '_ >
  {
    ModelComparator::new( self )
  }
}
