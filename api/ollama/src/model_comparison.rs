//! Model Comparison for A/B Testing
//!
//! Compare multiple Ollama models side-by-side with the same request to evaluate
//! response quality, speed, and token usage.

#[ cfg( feature = "enabled" ) ]
mod private
{
  use super::super::{ OllamaClient, OllamaResult, ChatRequest, ChatResponse };
  use std::time::Instant;

  /// Result from comparing a single model
  #[ derive( Debug, Clone ) ]
  pub struct ModelComparisonResult
  {
    /// Model name that was tested
    pub model_name : String,
    /// Response from the model
    pub response : ChatResponse,
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
    client : &'a mut OllamaClient,
  }

  impl< 'a > ModelComparator< 'a >
  {
    /// Create new model comparator
    #[ must_use ]
    pub fn new( client : &'a mut OllamaClient ) -> Self
    {
      Self { client }
    }

    /// Compare multiple models sequentially with the same request
    ///
    /// # Errors
    ///
    /// Returns error only if no models provided
    pub async fn compare_models
    (
      &mut self,
      model_names : &[ impl AsRef< str > ],
      base_request : &ChatRequest,
    ) -> OllamaResult< ComparisonResults >
    {
      if model_names.is_empty()
      {
        return Err( error_tools::untyped::format_err!( "At least one model required" ) );
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

        match self.client.chat( request ).await
        {
          Ok( response ) =>
          {
            let elapsed = request_start.elapsed().as_millis() as u64;

            results.push( ModelComparisonResult
            {
              model_name : model_str.to_string(),
              input_tokens : response.prompt_eval_count,
              output_tokens : response.eval_count,
              response,
              response_time_ms : elapsed,
              success : true,
              error_message : None,
            } );
          },
          Err( err ) =>
          {
            let elapsed = request_start.elapsed().as_millis() as u64;

            // Create empty response for failed request
            let empty_response = ChatResponse
            {
              #[ cfg( feature = "vision_support" ) ]
              message : crate::messages::ChatMessage
              {
                role : crate::messages::MessageRole::Assistant,
                content : String::new(),
                images : None,
                #[ cfg( feature = "tool_calling" ) ]
                tool_calls : None,
              },
              #[ cfg( not( feature = "vision_support" ) ) ]
              message : None,
              done : false,
              done_reason : None,
              model : Some( model_str.to_string() ),
              created_at : None,
              total_duration : None,
              load_duration : None,
              prompt_eval_count : None,
              prompt_eval_duration : None,
              eval_count : None,
              eval_duration : None,
            };

            results.push( ModelComparisonResult
            {
              model_name : model_str.to_string(),
              response : empty_response,
              response_time_ms : elapsed,
              success : false,
              error_message : Some( format!( "{:?}", err ) ),
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
  }

  impl OllamaClient
  {
    /// Create a model comparator for this client
    #[ must_use ]
    #[ inline ]
    pub fn comparator( &mut self ) -> ModelComparator< '_ >
    {
      ModelComparator::new( self )
    }
  }
}

#[ cfg( feature = "enabled" ) ]
crate::mod_interface!
{
  exposed use
  {
    ModelComparisonResult,
    ComparisonResults,
    ModelComparator,
  };
}
