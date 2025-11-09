//! Tests for request validation functionality

use api_huggingface::
{
  components::input::InferenceParameters,
  error::HuggingFaceError,
};

/// Test `InferenceParameters` validation ranges
#[ test ]
fn test_inference_parameters_temperature_validation()
{
  // Valid temperature ranges
  let valid_params = InferenceParameters::new()
  .with_temperature( 0.1 )
  .validate();
  assert!( valid_params.is_ok() );

  let valid_params = InferenceParameters::new()
  .with_temperature( 1.0 )
  .validate();
  assert!( valid_params.is_ok() );

  let valid_params = InferenceParameters::new()
  .with_temperature( 2.0 )
  .validate();
  assert!( valid_params.is_ok() );

  // Invalid temperature ranges
  let invalid_params = InferenceParameters::new()
  .with_temperature( -0.1 )
  .validate();
  assert!( invalid_params.is_err() );
  if let Err( HuggingFaceError::Validation( msg ) ) = invalid_params
  {
  assert!( msg.to_lowercase().contains( "temperature" ) );
  assert!( msg.contains( "0.0" ) );
  assert!( msg.contains( "2.0" ) );
  }
  else
  {
  panic!( "Expected validation error for negative temperature" );
  }

  let invalid_params = InferenceParameters::new()
  .with_temperature( 2.1 )
  .validate();
  assert!( invalid_params.is_err() );
  if let Err( HuggingFaceError::Validation( msg ) ) = invalid_params
  {
  assert!( msg.to_lowercase().contains( "temperature" ) );
  }
  else
  {
  panic!( "Expected validation error for high temperature" );
  }
}

/// Test `max_new_tokens` validation
#[ test ]
fn test_inference_parameters_max_tokens_validation()
{
  // Valid max_new_tokens
  let valid_params = InferenceParameters::new()
  .with_max_new_tokens( 1 )
  .validate();
  assert!( valid_params.is_ok() );

  let valid_params = InferenceParameters::new()
  .with_max_new_tokens( 4096 )
  .validate();
  assert!( valid_params.is_ok() );

  // Invalid max_new_tokens (zero)
  let invalid_params = InferenceParameters::new()
  .with_max_new_tokens( 0 )
  .validate();
  assert!( invalid_params.is_err() );
  if let Err( HuggingFaceError::Validation( msg ) ) = invalid_params
  {
  assert!( msg.contains( "max_new_tokens" ) );
  assert!( msg.contains( "greater than 0" ) );
  }
  else
  {
  panic!( "Expected validation error for zero max_new_tokens" );
  }

  // Invalid max_new_tokens (too large)
  let invalid_params = InferenceParameters::new()
  .with_max_new_tokens( 10000 )
  .validate();
  assert!( invalid_params.is_err() );
  if let Err( HuggingFaceError::Validation( msg ) ) = invalid_params
  {
  assert!( msg.contains( "max_new_tokens" ) );
  assert!( msg.contains( "8192" ) );
  }
  else
  {
  panic!( "Expected validation error for excessive max_new_tokens" );
  }
}

/// Test `top_p` validation
#[ test ]
fn test_inference_parameters_top_p_validation()
{
  // Valid top_p values
  let valid_params = InferenceParameters::new()
  .with_top_p( 0.0 )
  .validate();
  assert!( valid_params.is_ok() );

  let valid_params = InferenceParameters::new()
  .with_top_p( 1.0 )
  .validate();
  assert!( valid_params.is_ok() );

  // Invalid top_p values
  let invalid_params = InferenceParameters::new()
  .with_top_p( -0.1 )
  .validate();
  assert!( invalid_params.is_err() );

  let invalid_params = InferenceParameters::new()
  .with_top_p( 1.1 )
  .validate();
  assert!( invalid_params.is_err() );
  if let Err( HuggingFaceError::Validation( msg ) ) = invalid_params
  {
  assert!( msg.to_lowercase().contains( "top_p" ) );
  assert!( msg.contains( "0.0" ) );
  assert!( msg.contains( "1.0" ) );
  }
  else
  {
  panic!( "Expected validation error for invalid top_p" );
  }
}

/// Test input text validation
#[ test ]
fn test_input_text_validation()
{
  use api_huggingface::validation::validate_input_text;

  // Valid input text
  let valid_text = "Hello, world!";
  assert!( validate_input_text( valid_text ).is_ok() );

  let medium_text = "a".repeat( 1000 );
  assert!( validate_input_text( &medium_text ).is_ok() );

  // Empty input should be invalid
  let empty_text = "";
  let result = validate_input_text( empty_text );
  assert!( result.is_err() );
  if let Err( HuggingFaceError::Validation( msg ) ) = result
  {
  assert!( msg.contains( "empty" ) );
  }
  else
  {
  panic!( "Expected validation error for empty input" );
  }

  // Extremely long input should be invalid
  let long_text = "a".repeat( 100_000 );
  let result = validate_input_text( &long_text );
  assert!( result.is_err() );
  if let Err( HuggingFaceError::Validation( msg ) ) = result
  {
  assert!( msg.contains( "too long" ) );
  assert!( msg.contains( "50000" ) );
  }
  else
  {
  panic!( "Expected validation error for excessively long input" );
  }

  // Non-UTF8 sequences should be handled gracefully
  // (This test ensures we handle encoding properly)
  let unicode_text = "Hello 🌍! 你好世界 مرحبا بالعالم";
  assert!( validate_input_text( unicode_text ).is_ok() );
}

/// Test model identifier validation
#[ test ]
fn test_model_identifier_validation()
{
  use api_huggingface::validation::validate_model_identifier;

  // Valid model identifiers
  assert!( validate_model_identifier( "gpt2" ).is_ok() );
  assert!( validate_model_identifier( "meta-llama/Llama-2-7b-hf" ).is_ok() );
  assert!( validate_model_identifier( "microsoft/DialoGPT-medium" ).is_ok() );
  assert!( validate_model_identifier( "sentence-transformers/all-MiniLM-L6-v2" ).is_ok() );

  // Invalid model identifiers
  let long_model = "a".repeat( 300 );
  let invalid_models = vec![
  "",                           // empty
  " ",                          // whitespace only
  "model with spaces",          // spaces in name
  "model\nwith\nnewlines",     // newlines
  "/leading-slash",             // leading slash
  "trailing-slash/",            // trailing slash
  "double//slash",              // double slash
  &long_model,                  // too long
  ];

  for invalid_model in invalid_models
  {
  let result = validate_model_identifier( invalid_model );
  assert!( result.is_err(), "Model '{invalid_model}' should be invalid" );
  if let Err( HuggingFaceError::Validation( msg ) ) = result
  {
      assert!( msg.to_lowercase().contains( "model" ) );
  }
  else
  {
      panic!( "Expected validation error for model '{invalid_model}'" );
  }
  }
}

/// Test batch input validation
#[ test ]
fn test_batch_input_validation()
{
  use api_huggingface::validation::validate_batch_inputs;

  // Valid batch inputs
  let valid_batch = vec![ "Hello".to_string(), "World".to_string() ];
  assert!( validate_batch_inputs( &valid_batch ).is_ok() );

  // Empty batch should be invalid
  let empty_batch : Vec< String > = vec![];
  let result = validate_batch_inputs( &empty_batch );
  assert!( result.is_err() );
  if let Err( HuggingFaceError::Validation( msg ) ) = result
  {
  assert!( msg.contains( "empty" ) );
  }
  else
  {
  panic!( "Expected validation error for empty batch" );
  }

  // Too many inputs should be invalid
  let large_batch : Vec< String > = ( 0..1001 ).map( | i | format!( "input_{i}" ) ).collect();
  let result = validate_batch_inputs( &large_batch );
  assert!( result.is_err() );
  if let Err( HuggingFaceError::Validation( msg ) ) = result
  {
  assert!( msg.to_lowercase().contains( "too many" ) );
  assert!( msg.contains( "1000" ) );
  }
  else
  {
  panic!( "Expected validation error for excessive batch size" );
  }

  // Batch with invalid individual inputs should fail
  let invalid_batch = vec![ "Valid input".to_string(), String::new() ];
  let result = validate_batch_inputs( &invalid_batch );
  assert!( result.is_err() );
}

/// Test stop sequences validation
#[ test ]
fn test_stop_sequences_validation()
{
  // Valid stop sequences
  let valid_params = InferenceParameters::new()
  .with_stop_sequences( vec![ "\n".to_string(), "END".to_string() ] )
  .validate();
  assert!( valid_params.is_ok() );

  // Too many stop sequences should be invalid
  let many_stops : Vec< String > = ( 0..20 ).map( | i | format!( "stop_{i}" ) ).collect();
  let invalid_params = InferenceParameters::new()
  .with_stop_sequences( many_stops )
  .validate();
  assert!( invalid_params.is_err() );
  if let Err( HuggingFaceError::Validation( msg ) ) = invalid_params
  {
  assert!( msg.contains( "stop" ) );
  assert!( msg.contains( "10" ) );
  }
  else
  {
  panic!( "Expected validation error for too many stop sequences" );
  }

  // Empty stop sequences should be invalid
  let empty_stops = vec![ String::new() ];
  let invalid_params = InferenceParameters::new()
  .with_stop_sequences( empty_stops )
  .validate();
  assert!( invalid_params.is_err() );
}

/// Test comprehensive parameter validation
#[ test ]
fn test_multiple_validation_errors()
{
  // Test that multiple validation errors are reported
  let invalid_params = InferenceParameters::new()
  .with_temperature( -1.0 )           // Invalid temperature
  .with_max_new_tokens( 0 )           // Invalid max tokens
  .with_top_p( 2.0 )                  // Invalid top_p
  .validate();
  
  assert!( invalid_params.is_err() );
  if let Err( HuggingFaceError::Validation( msg ) ) = invalid_params
  {
  // Should contain multiple error messages
  assert!( msg.to_lowercase().contains( "temperature" ) );
  assert!( msg.to_lowercase().contains( "max_new_tokens" ) );
  assert!( msg.to_lowercase().contains( "top_p" ) );
  }
  else
  {
  panic!( "Expected validation error with multiple issues" );
  }
}

/// Test default parameters are valid
#[ test ]
fn test_default_parameters_valid()
{
  let default_params = InferenceParameters::default();
  assert!( default_params.validate().is_ok() );

  let new_params = InferenceParameters::new();
  assert!( new_params.validate().is_ok() );
}