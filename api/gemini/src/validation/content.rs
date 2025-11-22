//! Content generation validation functions

use super::*;

/// Validate enhanced generate content request with new features.
///
/// # Arguments
///
/// * `request` - The generate content request to validate
///
/// # Returns
///
/// Returns `Ok(())` if the request is valid, or a validation error.
pub fn validate_enhanced_generate_content_request( request : &GenerateContentRequest ) -> Result< (), ValidationError >
{
  // Basic validation
  if request.contents.is_empty()
  {
    return Err( ValidationError::EmptyCollection {
      field : "contents".to_string(),
      context : "GenerateContentRequest".to_string(),
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

  // Validate tool config if provided
  if let Some( tool_config ) = &request.tool_config
  {
    validate_tool_config( tool_config )
      .map_err( |e| ValidationError::InvalidFieldValue {
        field : "tool_config".to_string(),
        value : "ToolConfig".to_string(),
        reason : e.to_string(),
      } )?;
  }

  // Validate system instruction if provided
  if let Some( system_instruction ) = &request.system_instruction
  {
    validate_system_instruction( system_instruction )
      .map_err( |e| ValidationError::InvalidFieldValue {
        field : "system_instruction".to_string(),
        value : "SystemInstruction".to_string(),
        reason : e.to_string(),
      } )?;
  }

  // Validate tools if provided
  if let Some( tools ) = &request.tools
  {
    if tools.is_empty()
    {
      return Err( ValidationError::EmptyCollection {
        field : "tools".to_string(),
        context : "GenerateContentRequest".to_string(),
      } );
    }

    for ( i, tool ) in tools.iter().enumerate()
    {
      validate_tool( tool )
        .map_err( |e| ValidationError::InvalidFieldValue {
          field : format!( "tools[{}]", i ),
          value : "Tool".to_string(),
          reason : e.to_string(),
        } )?;
    }
  }

  Ok( () )
}

/// Validate tool.
///
/// # Arguments
///
/// * `tool` - The tool to validate
///
/// # Returns
///
/// Returns `Ok(())` if the tool is valid, or a validation error.
pub fn validate_tool( tool : &Tool ) -> Result< (), ValidationError >
{
  // Validate code execution tool if provided
  if let Some( code_execution_tool ) = &tool.code_execution_tool
  {
    validate_code_execution_tool( code_execution_tool )
      .map_err( |e| ValidationError::InvalidFieldValue {
        field : "code_execution_tool".to_string(),
        value : "CodeExecutionTool".to_string(),
        reason : e.to_string(),
      } )?;
  }

  // Validate function declarations if provided
  if let Some( function_declarations ) = &tool.function_declarations
  {
    if function_declarations.is_empty()
    {
      return Err( ValidationError::EmptyCollection {
        field : "function_declarations".to_string(),
        context : "Tool".to_string(),
      } );
    }

    for ( i, function_declaration ) in function_declarations.iter().enumerate()
    {
      validate_function_declaration( function_declaration )
        .map_err( |e| ValidationError::InvalidFieldValue {
          field : format!( "function_declarations[{}]", i ),
          value : "FunctionDeclaration".to_string(),
          reason : e.to_string(),
        } )?;
    }
  }

  Ok( () )
}

/// Validate function declaration.
///
/// # Arguments
///
/// * `declaration` - The function declaration to validate
///
/// # Returns
///
/// Returns `Ok(())` if the declaration is valid, or a validation error.
pub fn validate_function_declaration( declaration : &FunctionDeclaration ) -> Result< (), ValidationError >
{
  // Validate function name
  if declaration.name.trim().is_empty()
  {
    return Err( ValidationError::RequiredFieldMissing {
      field : "name".to_string(),
      context : "FunctionDeclaration".to_string(),
    } );
  }

  // Function names should be valid identifiers
  if !declaration.name.chars().all( |c| c.is_alphanumeric() || c == '_' )
  {
    return Err( ValidationError::InvalidFieldValue {
      field : "name".to_string(),
      value : declaration.name.clone(),
      reason : "Function name should contain only alphanumeric characters and underscores".to_string(),
    } );
  }

  Ok( () )
}
