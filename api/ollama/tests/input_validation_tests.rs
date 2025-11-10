//! Tests for input validation functionality
//!
//! Verifies that the validation framework correctly catches invalid requests
//! before they reach the network layer.

#[ cfg( all( test, feature = "input_validation" ) ) ]
mod tests
{
  use api_ollama::
  {
    ChatRequest,
    GenerateRequest,
    input_validation ::{ Validate, ValidationError },
  };

  /// Test validation of empty model name
  #[ test ]
  fn test_validate_empty_model()
  {
    let request = GenerateRequest
    {
      model : String::new(),
      prompt : "test".to_string(),
      stream : None,
      options : None,
    };

    let result = request.validate();
    assert!( result.is_err() );

    if let Err( errors ) = result
    {
      assert_eq!( errors.len(), 1 );
      assert_eq!( errors[ 0 ].field, "model" );
      assert!( errors[ 0 ].message.contains( "cannot be empty" ) );
    }
  }

  /// Test validation of invalid model name characters
  #[ test ]
  fn test_validate_invalid_model_chars()
  {
    let request = GenerateRequest
    {
      model : "model@invalid!chars".to_string(),
      prompt : "test".to_string(),
      stream : None,
      options : None,
    };

    let result = request.validate();
    assert!( result.is_err() );

    if let Err( errors ) = result
    {
      assert_eq!( errors.len(), 1 );
      assert_eq!( errors[ 0 ].field, "model" );
      assert!( errors[ 0 ].message.contains( "invalid characters" ) );
    }
  }

  /// Test validation of empty prompt
  #[ test ]
  fn test_validate_empty_prompt()
  {
    let request = GenerateRequest
    {
      model : "llama2".to_string(),
      prompt : String::new(),
      stream : None,
      options : None,
    };

    let result = request.validate();
    assert!( result.is_err() );

    if let Err( errors ) = result
    {
      assert_eq!( errors.len(), 1 );
      assert_eq!( errors[ 0 ].field, "prompt" );
      assert!( errors[ 0 ].message.contains( "cannot be empty" ) );
    }
  }

  /// Test validation of temperature out of range
  #[ test ]
  fn test_validate_temperature_out_of_range()
  {
    let options = serde_json::json!
    ({
      "temperature" : 3.0,
    });

    let request = GenerateRequest
    {
      model : "llama2".to_string(),
      prompt : "test".to_string(),
      stream : None,
      options : Some( options ),
    };

    let result = request.validate();
    assert!( result.is_err() );

    if let Err( errors ) = result
    {
      assert_eq!( errors.len(), 1 );
      assert_eq!( errors[ 0 ].field, "options.temperature" );
      assert!( errors[ 0 ].message.contains( "out of range" ) );
    }
  }

  /// Test validation of `top_p` out of range
  #[ test ]
  fn test_validate_top_p_out_of_range()
  {
    let options = serde_json::json!
    ({
      "top_p" : 1.5,
    });

    let request = GenerateRequest
    {
      model : "llama2".to_string(),
      prompt : "test".to_string(),
      stream : None,
      options : Some( options ),
    };

    let result = request.validate();
    assert!( result.is_err() );

    if let Err( errors ) = result
    {
      assert_eq!( errors.len(), 1 );
      assert_eq!( errors[ 0 ].field, "options.top_p" );
      assert!( errors[ 0 ].message.contains( "out of range" ) );
    }
  }

  /// Test validation of valid request
  #[ test ]
  fn test_validate_valid_request()
  {
    let options = serde_json::json!
    ({
      "temperature" : 0.7,
      "top_p" : 0.9,
    });

    let request = GenerateRequest
    {
      model : "llama2".to_string(),
      prompt : "Tell me a story".to_string(),
      stream : None,
      options : Some( options ),
    };

    let result = request.validate();
    assert!( result.is_ok() );
  }

  /// Test validation with multiple errors
  #[ test ]
  fn test_validate_multiple_errors()
  {
    let options = serde_json::json!
    ({
      "temperature" : 3.0,
      "top_p" : 1.5,
    });

    let request = GenerateRequest
    {
      model : String::new(),
      prompt : String::new(),
      stream : None,
      options : Some( options ),
    };

    let result = request.validate();
    assert!( result.is_err() );

    if let Err( errors ) = result
    {
      assert_eq!( errors.len(), 4 ); // model, prompt, temperature, top_p
    }
  }

  /// Test `ChatRequest` validation
  #[ test ]
  #[ cfg( feature = "vision_support" ) ]
  fn test_validate_chat_request()
  {
    let request = ChatRequest
    {
      model : "llama2".to_string(),
      messages : vec![],  // Empty messages
      stream : None,
      options : None,
      tools : None,
      tool_messages : None,
    };

    let result = request.validate();
    assert!( result.is_err() );

    if let Err( errors ) = result
    {
      assert_eq!( errors.len(), 1 );
      assert_eq!( errors[ 0 ].field, "messages" );
      assert!( errors[ 0 ].message.contains( "cannot be empty" ) );
    }
  }

  /// Test `ValidationError` display
  #[ test ]
  fn test_validation_error_display()
  {
    let error = ValidationError
    {
      field : "temperature".to_string(),
      message : "Value out of range".to_string(),
      value : "3.0".to_string(),
      constraint : "[0.0, 2.0]".to_string(),
    };

    let display = format!( "{error}" );
    assert!( display.contains( "temperature" ) );
    assert!( display.contains( "Value out of range" ) );
    assert!( display.contains( "3.0" ) );
    assert!( display.contains( "[0.0, 2.0]" ) );
  }
}
