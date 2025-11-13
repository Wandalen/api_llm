//! Model Comparison for A/B Testing
//!
//! Compare multiple models side-by-side with the same input.

/// Define a private namespace for all its items.
mod private
{
  use crate::
  {
    client::Client,
    components::chat_shared::{ ChatCompletionRequest, CreateChatCompletionResponse },
    environment::{ OpenaiEnvironment, EnvironmentInterface },
    error::Result,
  };
  use std::time::Instant;

  /// Result from comparing a single model
  #[ derive( Debug, Clone ) ]
  pub struct ModelComparisonResult
  {
    /// Model name that was tested
    pub model_name : String,
    /// The response from the model
    pub response : CreateChatCompletionResponse,
    /// Response time in milliseconds
    pub response_time_ms : u64,
    /// Whether the request succeeded
    pub success : bool,
    /// Error message if request failed
    pub error_message : Option< String >,
    /// Total tokens used
    pub total_tokens : Option< i32 >,
  }

  /// Results from comparing multiple models
  #[ derive( Debug, Clone ) ]
  pub struct ComparisonResults
  {
    /// Individual model results
    pub results : Vec< ModelComparisonResult >,
    /// Total time for all comparisons in milliseconds
    pub total_time_ms : u64,
    /// Name of the fastest model
    pub fastest_model : Option< String >,
    /// Name of the slowest model
    pub slowest_model : Option< String >,
  }

  impl ComparisonResults
  {
    /// Calculate success rate across all models
    #[ must_use ]
    #[ inline ]
    pub fn success_rate( &self ) -> f64
    {
      if self.results.is_empty()
      {
        return 0.0;
      }
      let successful = self.results.iter().filter( | r | r.success ).count();
      successful as f64 / self.results.len() as f64
    }

    /// Get average response time in milliseconds
    #[ must_use ]
    #[ inline ]
    pub fn average_response_time_ms( &self ) -> u64
    {
      if self.results.is_empty()
      {
        return 0;
      }
      let total : u64 = self.results.iter().map( | r | r.response_time_ms ).sum();
      total / self.results.len() as u64
    }

    /// Get total tokens used across all models
    #[ must_use ]
    #[ inline ]
    pub fn total_tokens_used( &self ) -> i32
    {
      self.results
        .iter()
        .filter_map( | r | r.total_tokens )
        .sum()
    }
  }

  /// Comparison mode for model testing
  #[ derive( Debug, Clone, Copy, PartialEq, Eq ) ]
  pub enum ComparisonMode
  {
    /// Run models sequentially (one after another)
    Sequential,
    /// Run models in parallel (all at once)
    Parallel,
  }

  /// Model comparator for A/B testing
  #[ derive( Debug ) ]
  pub struct ModelComparator< 'a, E >
  where
    E : OpenaiEnvironment + EnvironmentInterface + Send + Sync + 'static,
  {
    client : &'a Client< E >,
    mode : ComparisonMode,
  }

  impl< 'a, E > ModelComparator< 'a, E >
  where
    E : OpenaiEnvironment + EnvironmentInterface + Send + Sync + 'static,
  {
    /// Create a new model comparator
    #[ must_use ]
    #[ inline ]
    pub fn new( client : &'a Client< E > ) -> Self
    {
      Self
      {
        client,
        mode : ComparisonMode::Sequential,
      }
    }

    /// Set comparison mode
    #[ must_use ]
    #[ inline ]
    pub fn with_mode( mut self, mode : ComparisonMode ) -> Self
    {
      self.mode = mode;
      self
    }

    /// Compare multiple models with the same request
    ///
    /// # Errors
    ///
    /// Returns error if client fails to execute requests
    #[ inline ]
    pub async fn compare
    (
      &self,
      models : &[ String ],
      base_request : ChatCompletionRequest,
    ) -> Result< ComparisonResults >
    {
      let start_time = Instant::now();

      let results = match self.mode
      {
        ComparisonMode::Sequential =>
        {
          self.compare_sequential( models, base_request ).await?
        },
        ComparisonMode::Parallel =>
        {
          self.compare_parallel( models, base_request ).await?
        },
      };

      #[ allow( clippy::cast_possible_truncation ) ]
      let total_time_ms = start_time.elapsed().as_millis() as u64;

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

    /// Compare models sequentially
    async fn compare_sequential
    (
      &self,
      models : &[ String ],
      base_request : ChatCompletionRequest,
    ) -> Result< Vec< ModelComparisonResult > >
    {
      let mut results = Vec::new();

      for model_name in models
      {
        let mut request = base_request.clone();
        request.model.clone_from( model_name );

        let result = self.test_single_model( model_name, request ).await;
        results.push( result );
      }

      Ok( results )
    }

    /// Compare models in parallel
    async fn compare_parallel
    (
      &self,
      models : &[ String ],
      base_request : ChatCompletionRequest,
    ) -> Result< Vec< ModelComparisonResult > >
    {
      use futures::future::join_all;

      let futures = models
        .iter()
        .map( | model_name |
        {
          let mut request = base_request.clone();
          request.model.clone_from( model_name );
          self.test_single_model( model_name, request )
        } )
        .collect::< Vec< _ > >();

      let results = join_all( futures ).await;

      Ok( results )
    }

    /// Test a single model (instance method)
    async fn test_single_model
    (
      &self,
      model_name : &str,
      request : ChatCompletionRequest,
    ) -> ModelComparisonResult
    {
      use crate::ClientApiAccessors;

      let start_time = Instant::now();

      match self.client.chat().create( request ).await
      {
        Ok( response ) =>
        {
          #[ allow( clippy::cast_possible_truncation ) ]
          let response_time_ms = start_time.elapsed().as_millis() as u64;
          let total_tokens = response.usage.as_ref().map( | u | u.total_tokens );

          ModelComparisonResult
          {
            model_name : model_name.to_string(),
            response,
            response_time_ms,
            success : true,
            error_message : None,
            total_tokens,
          }
        },
        Err( e ) =>
        {
          #[ allow( clippy::cast_possible_truncation ) ]
          let response_time_ms = start_time.elapsed().as_millis() as u64;

          ModelComparisonResult
          {
            model_name : model_name.to_string(),
            response : CreateChatCompletionResponse
            {
              id : String::new(),
              choices : Vec::new(),
              created_at : 0,
              model : String::new(),
              object : String::from( "chat.completion" ),
              system_fingerprint : None,
              usage : None,
            },
            response_time_ms,
            success : false,
            error_message : Some( format!( "{e}" ) ),
            total_tokens : None,
          }
        }
      }
    }
  }

  /// Extension trait for Client to add comparator method
  impl< E > Client< E >
  where
    E : OpenaiEnvironment + EnvironmentInterface + Send + Sync + 'static,
  {
    /// Create a model comparator for A/B testing
    #[ must_use ]
    #[ inline ]
    pub fn comparator( &self ) -> ModelComparator< '_, E >
    {
      ModelComparator::new( self )
    }
  }

  #[ cfg( test ) ]
  mod tests
  {
    use super::*;

    fn create_empty_response() -> CreateChatCompletionResponse
    {
      CreateChatCompletionResponse
      {
        id : String::new(),
        choices : Vec::new(),
        created_at : 0,
        model : String::new(),
        object : String::from( "chat.completion" ),
        system_fingerprint : None,
        usage : None,
      }
    }

    #[ test ]
    fn test_comparison_results_success_rate()
    {
      let results = ComparisonResults
      {
        results : vec![
          ModelComparisonResult
          {
            model_name : "model1".to_string(),
            response : create_empty_response(),
            response_time_ms : 100,
            success : true,
            error_message : None,
            total_tokens : Some( 50 ),
          },
          ModelComparisonResult
          {
            model_name : "model2".to_string(),
            response : create_empty_response(),
            response_time_ms : 200,
            success : false,
            error_message : Some( "Error".to_string() ),
            total_tokens : None,
          },
        ],
        total_time_ms : 300,
        fastest_model : Some( "model1".to_string() ),
        slowest_model : Some( "model2".to_string() ),
      };

      assert!( ( results.success_rate() - 0.5 ).abs() < f64::EPSILON, "Expected success rate of 0.5, got {}", results.success_rate() );
    }

    #[ test ]
    fn test_comparison_results_average_response_time()
    {
      let results = ComparisonResults
      {
        results : vec![
          ModelComparisonResult
          {
            model_name : "model1".to_string(),
            response : create_empty_response(),
            response_time_ms : 100,
            success : true,
            error_message : None,
            total_tokens : Some( 50 ),
          },
          ModelComparisonResult
          {
            model_name : "model2".to_string(),
            response : create_empty_response(),
            response_time_ms : 200,
            success : true,
            error_message : None,
            total_tokens : Some( 60 ),
          },
        ],
        total_time_ms : 300,
        fastest_model : Some( "model1".to_string() ),
        slowest_model : Some( "model2".to_string() ),
      };

      assert_eq!( results.average_response_time_ms(), 150 );
    }

    #[ test ]
    fn test_comparison_results_total_tokens()
    {
      let results = ComparisonResults
      {
        results : vec![
          ModelComparisonResult
          {
            model_name : "model1".to_string(),
            response : create_empty_response(),
            response_time_ms : 100,
            success : true,
            error_message : None,
            total_tokens : Some( 50 ),
          },
          ModelComparisonResult
          {
            model_name : "model2".to_string(),
            response : create_empty_response(),
            response_time_ms : 200,
            success : true,
            error_message : None,
            total_tokens : Some( 60 ),
          },
        ],
        total_time_ms : 300,
        fastest_model : Some( "model1".to_string() ),
        slowest_model : Some( "model2".to_string() ),
      };

      assert_eq!( results.total_tokens_used(), 110 );
    }

    #[ test ]
    fn test_comparison_mode()
    {
      assert_eq!( ComparisonMode::Sequential, ComparisonMode::Sequential );
      assert_eq!( ComparisonMode::Parallel, ComparisonMode::Parallel );
      assert_ne!( ComparisonMode::Sequential, ComparisonMode::Parallel );
    }
  }
}

crate::mod_interface!
{
  exposed use
  {
    ModelComparisonResult,
    ComparisonResults,
    ComparisonMode,
    ModelComparator,
  };
}
