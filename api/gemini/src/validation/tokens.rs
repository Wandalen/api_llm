//! Token and model discovery validation functions

use super::*;

/// Validate a batch count tokens request.
///
/// # Arguments
///
/// * `request` - The batch count tokens request to validate
///
/// # Returns
///
/// Returns `Ok(())` if the request is valid, or a validation error.
///
/// # Errors
///
/// Returns `ValidationError` if the request is invalid, such as:
/// - Empty requests collection
/// - Individual request validation failures
#[ inline ]
pub fn validate_batch_count_tokens_request( request : &BatchCountTokensRequest ) -> Result< (), ValidationError >
{
  if request.requests.is_empty()
  {
    return Err( ValidationError::EmptyCollection {
      field : "requests".to_string(),
      context : "BatchCountTokensRequest".to_string(),
    } );
  }

  if request.requests.len() > MAX_BATCH_TOKEN_REQUESTS
  {
    return Err( ValidationError::CollectionTooLarge {
      field : "requests".to_string(),
      size : request.requests.len(),
      max : MAX_BATCH_TOKEN_REQUESTS,
    } );
  }

  // Validate each individual request
  for ( i, count_request ) in request.requests.iter().enumerate()
  {
    validate_count_tokens_request( count_request )
      .map_err( |e| ValidationError::InvalidFieldValue {
        field : format!( "requests[{i}]" ),
        value : "CountTokensRequest".to_string(),
        reason : e.to_string(),
      } )?;
  }

  Ok( () )
}

/// Validate a count tokens request.
///
/// # Arguments
///
/// * `request` - The count tokens request to validate
///
/// # Returns
///
/// Returns `Ok(())` if the request is valid, or a validation error.
///
/// # Errors
///
/// Returns `ValidationError` if the request is invalid, such as:
/// - Empty contents collection
/// - Individual content validation failures
#[ inline ]
pub fn validate_count_tokens_request( request : &CountTokensRequest ) -> Result< (), ValidationError >
{
  if request.contents.is_empty()
  {
    return Err( ValidationError::EmptyCollection {
      field : "contents".to_string(),
      context : "CountTokensRequest".to_string(),
    } );
  }

  // Validate each content item
  for ( i, content ) in request.contents.iter().enumerate()
  {
    validate_content( content )
      .map_err( |e| ValidationError::InvalidFieldValue {
        field : format!( "contents[{i}]" ),
        value : "Content".to_string(),
        reason : e.to_string(),
      } )?;
  }

  Ok( () )
}

/// Validate an analyze tokens request.
///
/// # Arguments
///
/// * `request` - The analyze tokens request to validate
///
/// # Returns
///
/// Returns `Ok(())` if the request is valid, or a validation error.
pub fn validate_analyze_tokens_request( request : &AnalyzeTokensRequest ) -> Result< (), ValidationError >
{
  if request.contents.is_empty()
  {
    return Err( ValidationError::EmptyCollection {
      field : "contents".to_string(),
      context : "AnalyzeTokensRequest".to_string(),
    } );
  }

  // Validate each content item
  for ( i, content ) in request.contents.iter().enumerate()
  {
    validate_content( content )
      .map_err( |e| ValidationError::InvalidFieldValue {
        field : format!( "contents[{i}]" ),
        value : "Content".to_string(),
        reason : e.to_string(),
      } )?;
  }

  Ok( () )
}

/// Validate a compare models request.
///
/// # Arguments
///
/// * `request` - The compare models request to validate
///
/// # Returns
///
/// Returns `Ok(())` if the request is valid, or a validation error.
pub fn validate_compare_models_request( request : &CompareModelsRequest ) -> Result< (), ValidationError >
{
  if request.model_names.is_empty()
  {
    return Err( ValidationError::EmptyCollection {
      field : "model_names".to_string(),
      context : "CompareModelsRequest".to_string(),
    } );
  }

  if request.model_names.len() > MAX_MODELS_TO_COMPARE
  {
    return Err( ValidationError::CollectionTooLarge {
      field : "model_names".to_string(),
      size : request.model_names.len(),
      max : MAX_MODELS_TO_COMPARE,
    } );
  }

  // Validate each model name
  for ( i, model_name ) in request.model_names.iter().enumerate()
  {
    if model_name.trim().is_empty()
    {
      return Err( ValidationError::RequiredFieldMissing {
        field : format!( "model_names[{}]", i ),
        context : "CompareModelsRequest".to_string(),
      } );
    }

    validate_model_name( model_name )
      .map_err( |e| ValidationError::InvalidFieldValue {
        field : format!( "model_names[{}]", i ),
        value : model_name.clone(),
        reason : e.to_string(),
      } )?;
  }

  Ok( () )
}

/// Validate a get recommendations request.
///
/// # Arguments
///
/// * `request` - The get recommendations request to validate
///
/// # Returns
///
/// Returns `Ok(())` if the request is valid, or a validation error.
pub fn validate_get_recommendations_request( request : &GetRecommendationsRequest ) -> Result< (), ValidationError >
{
  if request.use_case.trim().is_empty()
  {
    return Err( ValidationError::RequiredFieldMissing {
      field : "use_case".to_string(),
      context : "GetRecommendationsRequest".to_string(),
    } );
  }

  if request.use_case.len() < 10
  {
    return Err( ValidationError::InvalidFieldValue {
      field : "use_case".to_string(),
      value : request.use_case.clone(),
      reason : "Use case description should be at least 10 characters".to_string(),
    } );
  }

  // Validate budget constraints if provided
  if let Some( budget ) = request.budget_constraints
  {
    if budget < 0.0
    {
      return Err( ValidationError::ValueOutOfRange {
        field : "budget_constraints".to_string(),
        value : budget,
        min : Some( 0.0 ),
        max : None,
      } );
    }
  }

  Ok( () )
}

/// Validate an advanced filter request.
///
/// # Arguments
///
/// * `request` - The advanced filter request to validate
///
/// # Returns
///
/// Returns `Ok(())` if the request is valid, or a validation error.
pub fn validate_advanced_filter_request( request : &AdvancedFilterRequest ) -> Result< (), ValidationError >
{
  // Validate cost constraints
  if let Some( max_cost ) = request.max_cost_per_1k
  {
    if max_cost < 0.0
    {
      return Err( ValidationError::ValueOutOfRange {
        field : "max_cost_per_1k".to_string(),
        value : max_cost,
        min : Some( 0.0 ),
        max : None,
      } );
    }
  }

  // Validate quality score constraints
  if let Some( min_quality ) = request.min_quality_score
  {
    if min_quality < 0.0 || min_quality > 1.0
    {
      return Err( ValidationError::ValueOutOfRange {
        field : "min_quality_score".to_string(),
        value : min_quality,
        min : Some( 0.0 ),
        max : Some( 1.0 ),
      } );
    }
  }

  // Validate response time constraints
  if let Some( max_response_time ) = request.max_response_time
  {
    if max_response_time <= 0.0
    {
      return Err( ValidationError::ValueOutOfRange {
        field : "max_response_time".to_string(),
        value : max_response_time,
        min : Some( 0.001 ),
        max : None,
      } );
    }
  }

  Ok( () )
}

/// Validate a model status request.
///
/// # Arguments
///
/// * `request` - The model status request to validate
///
/// # Returns
///
/// Returns `Ok(())` if the request is valid, or a validation error.
pub fn validate_model_status_request( request : &ModelStatusRequest ) -> Result< (), ValidationError >
{
  if request.model_names.is_empty()
  {
    return Err( ValidationError::EmptyCollection {
      field : "model_names".to_string(),
      context : "ModelStatusRequest".to_string(),
    } );
  }

  if request.model_names.len() > MAX_MODEL_STATUS_REQUESTS
  {
    return Err( ValidationError::CollectionTooLarge {
      field : "model_names".to_string(),
      size : request.model_names.len(),
      max : MAX_MODEL_STATUS_REQUESTS,
    } );
  }

  // Validate each model name
  for ( i, model_name ) in request.model_names.iter().enumerate()
  {
    if model_name.trim().is_empty()
    {
      return Err( ValidationError::RequiredFieldMissing {
        field : format!( "model_names[{}]", i ),
        context : "ModelStatusRequest".to_string(),
      } );
    }

    validate_model_name( model_name )
      .map_err( |e| ValidationError::InvalidFieldValue {
        field : format!( "model_names[{}]", i ),
        value : model_name.clone(),
        reason : e.to_string(),
      } )?;
  }

  Ok( () )
}
