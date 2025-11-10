//! Model Comparison for A/B Testing
//!
//! Compare multiple Claude models side-by-side with the same request to evaluate
//! response quality, speed, and token usage.

mod private
{
  use super::super::{ Client, CreateMessageRequest, CreateMessageResponse };
  use crate::error::{ AnthropicError, AnthropicResult };
  use std::time::Instant;

  /// Result from comparing a single model
  #[ derive( Debug, Clone ) ]
  pub struct ModelComparisonResult
  {
    /// Model name that was tested
    pub model_name : String,
    /// Response from the model (empty if error occurred)
    pub response : CreateMessageResponse,
    /// Response time in milliseconds
    pub response_time_ms : u64,
    /// Whether the request succeeded
    pub success : bool,
    /// Error message if request failed
    pub error_message : Option< String >,
    /// Input tokens used
    pub input_tokens : Option< u32 >,
    /// Output tokens used
    pub output_tokens : Option< u32 >,
  }

  /// Results from comparing multiple models
  #[ derive( Debug, Clone ) ]
  pub struct ComparisonResults
  {
    /// Individual model results
    pub results : Vec< ModelComparisonResult >,
    /// Total time for all comparisons in milliseconds
    pub total_time_ms : u64,
    /// Fastest model name (if any succeeded)
    pub fastest_model : Option< String >,
    /// Slowest model name (if any succeeded)
    pub slowest_model : Option< String >,
  }

  impl ComparisonResults
  {
    /// Calculate success rate across all models
    #[ must_use ]
    pub fn success_rate( &self ) -> f64
    {
      if self.results.is_empty()
      {
        return 0.0;
      }
      let successful = self.results.iter().filter( | r | r.success ).count();
      ( successful as f64 ) / ( self.results.len() as f64 )
    }

    /// Get average response time for successful requests
    #[ must_use ]
    pub fn average_response_time_ms( &self ) -> Option< u64 >
    {
      let successful_times : Vec< u64 > = self.results
        .iter()
        .filter( | r | r.success )
        .map( | r | r.response_time_ms )
        .collect();

      if successful_times.is_empty()
      {
        None
      }
      else
      {
        Some( successful_times.iter().sum::< u64 >() / successful_times.len() as u64 )
      }
    }

    /// Get total input tokens across all successful requests
    #[ must_use ]
    pub fn total_input_tokens( &self ) -> u32
    {
      self.results
        .iter()
        .filter_map( | r | r.input_tokens )
        .sum()
    }

    /// Get total output tokens across all successful requests
    #[ must_use ]
    pub fn total_output_tokens( &self ) -> u32
    {
      self.results
        .iter()
        .filter_map( | r | r.output_tokens )
        .sum()
    }
  }

  /// Model comparator for A/B testing
  #[ derive( Debug ) ]
  pub struct ModelComparator< 'a >
  {
    client : &'a Client,
  }

  impl< 'a > ModelComparator< 'a >
  {
    /// Create new model comparator
    #[ must_use ]
    pub fn new( client : &'a Client ) -> Self
    {
      Self { client }
    }

    /// Compare multiple models sequentially with the same request
    ///
    /// # Errors
    ///
    /// Returns error only if no models provided or catastrophic failure
    pub async fn compare_models
    (
      &self,
      model_names : &[ impl AsRef< str > ],
      base_request : &CreateMessageRequest,
    ) -> AnthropicResult< ComparisonResults >
    {
      if model_names.is_empty()
      {
        return Err( AnthropicError::InvalidArgument( "At least one model required".to_string() ) );
      }

      let start = Instant::now();
      let mut results = Vec::new();

      for model_name in model_names
      {
        let model_str = model_name.as_ref();

        // Clone request and update model
        let mut request = base_request.clone();
        request.model = model_str.to_string();

        let request_start = Instant::now();

        match self.client.create_message( request ).await
        {
          Ok( response ) => {
            #[ allow( clippy::cast_possible_truncation ) ]
            let elapsed = request_start.elapsed().as_millis().min( u128::from( u64::MAX ) ) as u64;

            results.push( ModelComparisonResult
            {
              model_name : model_str.to_string(),
              input_tokens : Some( response.usage.input_tokens ),
              output_tokens : Some( response.usage.output_tokens ),
              response,
              response_time_ms : elapsed,
              success : true,
              error_message : None,
            } );
          },
          Err( err ) => {
            #[ allow( clippy::cast_possible_truncation ) ]
            let elapsed = request_start.elapsed().as_millis().min( u128::from( u64::MAX ) ) as u64;

            // Create empty response for failed request
            let empty_response = CreateMessageResponse
            {
              id : String::new(),
              r#type : String::new(),
              role : String::new(),
              content : vec![],
              model : model_str.to_string(),
              stop_reason : None,
              stop_sequence : None,
              usage : crate::client::types::Usage
              {
                input_tokens : 0,
                output_tokens : 0,
                cache_creation_input_tokens : None,
                cache_read_input_tokens : None,
              },
            };

            results.push( ModelComparisonResult
            {
              model_name : model_str.to_string(),
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

      #[ allow( clippy::cast_possible_truncation ) ]
      let total_time_ms = start.elapsed().as_millis().min( u128::from( u64::MAX ) ) as u64;

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

    /// Compare multiple models in parallel (faster but uses more quota)
    ///
    /// # Errors
    ///
    /// Returns error only if no models provided or catastrophic failure
    pub async fn compare_models_parallel
    (
      &self,
      model_names : &[ impl AsRef< str > ],
      base_request : &CreateMessageRequest,
    ) -> AnthropicResult< ComparisonResults >
    {
      if model_names.is_empty()
      {
        return Err( AnthropicError::InvalidArgument( "At least one model required".to_string() ) );
      }

      let start = Instant::now();

      // Create futures for all model requests
      let futures : Vec< _ > = model_names
        .iter()
        .map( | model_name | async move {
          let model_str = model_name.as_ref();

          // Clone request and update model
          let mut request = base_request.clone();
          request.model = model_str.to_string();

          let request_start = Instant::now();

          match self.client.create_message( request ).await
          {
            Ok( response ) => {
              #[ allow( clippy::cast_possible_truncation ) ]
              let elapsed = request_start.elapsed().as_millis().min( u128::from( u64::MAX ) ) as u64;

              ModelComparisonResult
              {
                model_name : model_str.to_string(),
                input_tokens : Some( response.usage.input_tokens ),
                output_tokens : Some( response.usage.output_tokens ),
                response,
                response_time_ms : elapsed,
                success : true,
                error_message : None,
              }
            },
            Err( err ) => {
              #[ allow( clippy::cast_possible_truncation ) ]
              let elapsed = request_start.elapsed().as_millis().min( u128::from( u64::MAX ) ) as u64;

              // Create empty response for failed request
              let empty_response = CreateMessageResponse
              {
                id : String::new(),
                r#type : String::new(),
                role : String::new(),
                content : vec![],
                model : model_str.to_string(),
                stop_reason : None,
                stop_sequence : None,
                usage : crate::client::types::Usage
                {
                  input_tokens : 0,
                  output_tokens : 0,
                  cache_creation_input_tokens : None,
                  cache_read_input_tokens : None,
                },
              };

              ModelComparisonResult
              {
                model_name : model_str.to_string(),
                response : empty_response,
                response_time_ms : elapsed,
                success : false,
                error_message : Some( err.to_string() ),
                input_tokens : None,
                output_tokens : None,
              }
            }
          }
        } )
        .collect();

      // Execute all requests in parallel
      let results = futures::future::join_all( futures ).await;

      #[ allow( clippy::cast_possible_truncation ) ]
      let total_time_ms = start.elapsed().as_millis().min( u128::from( u64::MAX ) ) as u64;

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
    /// Create a model comparator for this client
    #[ must_use ]
    #[ inline ]
    pub fn comparator( &self ) -> ModelComparator< '_ >
    {
      ModelComparator::new( self )
    }
  }
}

crate::mod_interface!
{
  orphan use
  {
    ModelComparisonResult,
    ComparisonResults,
    ModelComparator,
  };
}
