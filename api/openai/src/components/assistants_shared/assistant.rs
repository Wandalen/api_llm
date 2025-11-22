//! Assistant-related types and structures for the Assistants API.

/// Define a private namespace for assistant-related items.
mod private
{
  // Use full paths from crate root for components
  use crate::components::common::
  {
    Metadata,
    TextResponseFormatConfiguration,
    ResponseFormatJsonObject,
    ResponseFormatJsonSchema,
  };
  use crate::components::tools::{ Tool, ToolChoiceFunction, FileSearchRankingOptions };

  // Add serde imports
  use serde::{ Serialize, Deserialize };

  /// Represents the supported models for Assistants. Placeholder type.
  #[ derive( Debug, Serialize, Deserialize, Clone, PartialEq ) ]
  pub struct AssistantSupportedModels
  {
    /// The model identifier string.
    pub value : String,
  }

  /// Specifies the format that the model must output. Compatible with GPT-4o, GPT-4 Turbo, and newer GPT-3.5 Turbo models.
  /// Setting to `{ "type": "json_schema", "json_schema": {...} }` enables Structured Outputs.
  /// Setting to `{ "type": "json_object" }` enables JSON mode.
  /// `auto` is the default value.
  ///
  /// # Used By
  /// - `AssistantObject`
  /// - `CreateAssistantRequest`
  /// - `ModifyAssistantRequest`
  /// - `RunObject`
  /// - `CreateRunRequest`
  /// - `CreateThreadAndRunRequest`
  #[ derive( Debug, Serialize, Deserialize, Clone, PartialEq ) ]
  #[ serde( untagged ) ] // Allows deserialization from string "auto" or the object types
  pub enum AssistantsApiResponseFormatOption
  {
    /// Default value, lets the model decide. String value must be "auto".
    Auto( String ),
    /// Text format configuration.
    Text( TextResponseFormatConfiguration ),
    /// JSON object format configuration.
    JsonObject( ResponseFormatJsonObject ),
    /// JSON schema format configuration.
    JsonSchema( ResponseFormatJsonSchema ),
  }

  /// Controls which (if any) tool is called by the model in the Assistants API.
  /// `none` means the model will not call any tools.
  /// `auto` is the default value and means the model can pick.
  /// `required` means the model must call one or more tools.
  /// Specifying a particular tool forces the model to call that tool.
  ///
  /// # Used By
  /// - `CreateRunRequest`
  /// - `CreateThreadAndRunRequest`
  /// - `RunObject`
  #[ derive( Debug, Serialize, Deserialize, Clone, PartialEq ) ]
  #[ serde( untagged ) ] // Allows deserialization from string or object
  pub enum AssistantsApiToolChoiceOption
  {
    /// String options : "none", "auto", "required".
    String( String ),
    /// Specifies a specific tool to call.
    Named( AssistantsNamedToolChoice ),
  }

  /// Specifies a tool the model should use within the Assistants API. Use to force the model to call a specific tool.
  ///
  /// # Used By
  /// - `AssistantsApiToolChoiceOption::Named`
  #[ derive( Debug, Serialize, Deserialize, Clone, PartialEq ) ]
  pub struct AssistantsNamedToolChoice
  {
    /// The type of the tool (`function`, `code_interpreter`, `file_search`).
    pub r#type : String,
    /// The function details if type is `function`.
    pub function : Option< ToolChoiceFunction >,
  }

  /// Controls for how a thread will be truncated prior to the run.
  ///
  /// # Used By
  /// - `RunObject`
  /// - `CreateRunRequest`
  /// - `CreateThreadAndRunRequest`
  #[ derive( Debug, Serialize, Deserialize, Clone, PartialEq ) ] // Added Serialize
  pub struct TruncationObject
  {
    /// The truncation strategy type (`auto` or `last_messages`).
    pub r#type : String,
    /// The number of most recent messages to keep when `type` is `last_messages`.
    pub last_messages : Option< i32 >,
  }

  /// Represents the Code Interpreter tool for Assistants.
  ///
  /// # Used By
  /// - `AssistantObject.tools`
  /// - `CreateAssistantRequest.tools`
  /// - `ModifyAssistantRequest.tools`
  /// - `CreateRunRequest.tools`
  /// - `CreateThreadAndRunRequest.tools`
  /// - `MessageAttachment.tools`
  #[ derive( Debug, Serialize, Deserialize, Clone, PartialEq ) ]
  pub struct AssistantToolsCode
  {
    // Type is implicitly "code_interpreter" via parent enum tag
  }

  /// Represents the File Search tool for Assistants, with optional overrides.
  ///
  /// # Used By
  /// - `AssistantObject.tools`
  /// - `CreateAssistantRequest.tools`
  /// - `ModifyAssistantRequest.tools`
  /// - `CreateRunRequest.tools`
  /// - `CreateThreadAndRunRequest.tools`
  #[ derive( Debug, Serialize, Deserialize, Clone, PartialEq ) ]
  pub struct AssistantToolsFileSearch
  {
    /// Optional settings to override the default file search behavior.
    #[ serde( skip_serializing_if = "Option::is_none" ) ]
    pub file_search : Option< AssistantFileSearchSettings >,
  }

  /// Settings to override the File Search tool behavior.
  ///
  /// # Used By
  /// - `AssistantToolsFileSearch`
  #[ derive( Debug, Serialize, Deserialize, Clone, PartialEq ) ]
  pub struct AssistantFileSearchSettings
  {
    /// The maximum number of results the file search tool should output.
    #[ serde( skip_serializing_if = "Option::is_none" ) ]
    pub max_num_results : Option< i32 >,
    /// Ranking options for the file search results.
    #[ serde( skip_serializing_if = "Option::is_none" ) ]
    pub ranking_options : Option< FileSearchRankingOptions >,
  }

  /// Represents the File Search tool type marker, used when attaching files to messages.
  ///
  /// # Used By
  /// - `MessageAttachment.tools`
  #[ derive( Debug, Serialize, Deserialize, Clone, PartialEq ) ]
  pub struct AssistantToolsFileSearchTypeOnly
  {
    // Type is implicitly "file_search" via parent enum tag
  }

  /// Represents the Function tool for Assistants.
  ///
  /// # Used By
  /// - `AssistantObject.tools`
  /// - `CreateAssistantRequest.tools`
  /// - `ModifyAssistantRequest.tools`
  /// - `CreateRunRequest.tools`
  /// - `CreateThreadAndRunRequest.tools`
  #[ derive( Debug, Serialize, Deserialize, Clone, PartialEq ) ]
  pub struct AssistantToolsFunction
  {
    /// The function definition.
    pub function : crate::components::tools::FunctionTool,
  }

  /// Represents an `assistant` that can call the model and use tools.
  ///
  /// # Used By
  /// - `/assistants` (GET - in `ListAssistantsResponse`, POST response)
  /// - `/assistants/{assistant_id}` (GET, POST response)
  /// - `MessageObject`
  /// - `RunObject`
  /// - `RunStepObject`
  #[ derive( Debug, Serialize, Deserialize, Clone, PartialEq ) ] // Added Serialize
  pub struct AssistantObject
  {
    /// The identifier, which can be referenced in API endpoints.
    pub id : String,
    /// The object type, which is always `assistant`.
    pub object : String,
    /// The Unix timestamp (in seconds) for when the assistant was created.
    pub created_at : i64,
    /// The name of the assistant. The maximum length is 256 characters.
    pub name : Option< String >,
    /// The description of the assistant. The maximum length is 512 characters.
    pub description : Option< String >,
    /// ID of the model to use.
    pub model : String,
    /// The system instructions that the assistant uses. The maximum length is 256,000 characters.
    pub instructions : Option< String >,
    /// A list of tool enabled on the assistant. Maximum 128 tools.
    pub tools : Vec< Tool >,
    /// A set of resources used by the assistant's tools.
    pub tool_resources : Option< ToolResources >,
    /// Set of 16 key-value pairs attached to the object.
    pub metadata : Option< Metadata >,
    /// Sampling temperature between 0 and 2. Defaults to 1.
    pub temperature : Option< f32 >,
    /// Nucleus sampling probability. Defaults to 1.
    pub top_p : Option< f32 >,
    /// The response format specified for the assistant.
    pub response_format : Option< AssistantsApiResponseFormatOption >,
  }

  /// A set of resources that are used by the assistant's tools.
  ///
  /// # Used By
  /// - `AssistantObject`
  /// - `ThreadObject`
  /// - `CreateAssistantRequest`
  /// - `ModifyAssistantRequest`
  /// - `CreateThreadRequest`
  /// - `ModifyThreadRequest`
  /// - `CreateThreadAndRunRequest`
  #[ derive( Debug, Serialize, Deserialize, Clone, PartialEq ) ]
  pub struct ToolResources
  {
    /// Resources for the Code Interpreter tool.
    #[ serde( skip_serializing_if = "Option::is_none" ) ]
    pub code_interpreter : Option< CodeInterpreterResources >,
    /// Resources for the File Search tool.
    #[ serde( skip_serializing_if = "Option::is_none" ) ]
    pub file_search : Option< FileSearchResources >,
  }

  /// Resources for the Code Interpreter tool.
  ///
  /// # Used By
  /// - `ToolResources`
  #[ derive( Debug, Serialize, Deserialize, Clone, PartialEq ) ]
  pub struct CodeInterpreterResources
  {
    /// A list of file IDs made available to the Code Interpreter tool. Maximum 20 files.
    #[ serde( default, skip_serializing_if = "Option::is_none" ) ] // Default to empty vec if None during serialization
    pub file_ids : Option< Vec< String > >,
  }

  /// Resources for the File Search tool.
  ///
  /// # Used By
  /// - `ToolResources`
  #[ derive( Debug, Serialize, Deserialize, Clone, PartialEq ) ]
  pub struct FileSearchResources
  {
    /// The vector store IDs attached to the assistant or thread. Maximum 1 ID.
    #[ serde( skip_serializing_if = "Option::is_none" ) ]
    pub vector_store_ids : Option< Vec< String > >,
    // Note : 'vector_stores' field from CreateAssistantRequest/CreateThreadRequest is not part of the response object.
  }

  /// Response containing a list of assistants.
  ///
  /// # Used By
  /// - `/assistants` (GET)
  #[ derive( Debug, Deserialize, Clone, PartialEq ) ]
  pub struct ListAssistantsResponse
  {
    /// The object type, always "list".
    pub object : String,
    /// A list of assistant objects.
    pub data : Vec< AssistantObject >,
    /// The ID of the first assistant in the list.
    pub first_id : String,
    /// The ID of the last assistant in the list.
    pub last_id : String,
    /// Indicates whether there are more assistants available.
    pub has_more : bool,
  }

  /// Response structure for deletion operations of assistants.
  ///
  /// # Used By
  /// - `/assistants/{assistant_id}` (DELETE)
  #[ derive( Debug, Deserialize, Clone, PartialEq ) ]
  pub struct DeleteAssistantResponse
  {
    /// The ID of the deleted assistant.
    pub id : String,
    /// The object type, always "assistant.deleted".
    pub object : String,
    /// Whether the deletion was successful.
    pub deleted : bool,
  }
}

crate ::mod_interface!
{
  exposed use private::AssistantSupportedModels;
  exposed use private::AssistantsApiResponseFormatOption;
  exposed use private::AssistantsApiToolChoiceOption;
  exposed use private::AssistantsNamedToolChoice;
  exposed use private::TruncationObject;
  exposed use private::AssistantToolsCode;
  exposed use private::AssistantToolsFileSearch;
  exposed use private::AssistantFileSearchSettings;
  exposed use private::AssistantToolsFileSearchTypeOnly;
  exposed use private::AssistantToolsFunction;
  exposed use private::AssistantObject;
  exposed use private::ToolResources;
  exposed use private::CodeInterpreterResources;
  exposed use private::FileSearchResources;
  exposed use private::ListAssistantsResponse;
  exposed use private::DeleteAssistantResponse;
}