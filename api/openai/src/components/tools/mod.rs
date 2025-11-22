//! This module defines various tool-related structures used across the `OpenAI` API.

mod private
{
  use serde::{ Deserialize, Serialize };
  use former::Former;
  use crate::components::common::Coordinate;
  use crate::components::output::{ ComputerScreenshotImage, FileSearchResultItem };

  // ============================================================================
  // Common structures
  // ============================================================================

  /// Represents the available tools that can be used by models or assistants.
  ///
  /// # Used By
  /// - `CreateChatCompletionRequest`
  /// - `CreateResponseRequest`
  /// - `AssistantObject`
  /// - `RunObject`
  /// - `CreateRunRequest`
  /// - `CreateThreadAndRunRequest`
  /// - `MessageAttachment`
  /// - `FineTuneChatRequestInput`
  /// - `FineTunePreferenceInputData`
  /// - `RealtimeSession`
  /// - `RealtimeResponseCreateParams`
  #[ derive( Debug, Serialize, Deserialize, Clone, PartialEq ) ]
  #[ serde( tag = "type" ) ]
  #[ non_exhaustive ]
  pub enum Tool
  {
    /// A tool for controlling a virtual computer environment.
    #[ serde( rename = "computer_use_preview" ) ]
    ComputerUse( ComputerTool ),
    /// A tool for searching attached files.
    #[ serde( rename = "file_search" ) ]
    FileSearch( FileSearchTool ),
    /// A custom function defined by the user.
    #[ serde( rename = "function" ) ]
    Function( FunctionTool ),
    /// A tool for searching the web.
    #[ serde( rename = "web_search_preview" ) ]
    WebSearch( WebSearchTool ),
  }

  /// Represents the choice of which tool the model should use.
  ///
  /// # Used By
  /// - `CreateChatCompletionRequest`
  /// - `CreateResponseRequest`
  /// - `ResponseProperties` (within `common.rs`)
  #[ derive( Debug, Serialize, Deserialize, Clone, PartialEq ) ]
  #[ serde( untagged ) ]
  #[ non_exhaustive ]
  pub enum ToolChoice
  {
    /// Specifies a specific function to call.
    Function
    {
      /// The function details.
      function : ToolChoiceFunction,
      /// The type, always "function".
      r#type : String,
    },
    /// A string indicating the mode ("none", "auto", "required").
    String( String ),
  }

  /// Specifies a function name for the `ToolChoice::Function` variant.
  ///
  /// # Used By
  /// - `ToolChoice::Function`
  /// - `AssistantsNamedToolChoice` (within `assistants_shared.rs`)
  /// - `ChatCompletionNamedToolChoice` (within `chat_shared.rs`)
  #[ derive( Debug, Serialize, Deserialize, Clone, PartialEq ) ]
  #[ non_exhaustive ]
  pub struct ToolChoiceFunction
  {
    /// The name of the function to call.
    pub name : String,
  }

  // ============================================================================
  // Computer Use tool structures
  // ============================================================================

  /// Defines the computer use tool.
  /// A tool that controls a virtual computer. Learn more about the [computer tool](/docs/guides/tools-computer-use).
  ///
  /// # Used By
  /// - `Tool::ComputerUse` (within `tools.rs`)
  #[ derive( Debug, Serialize, Deserialize, Clone, PartialEq, Former ) ] // Added Serialize, Former
  #[ non_exhaustive ]
  pub struct ComputerTool
  {
    // Type is implicitly "computer_use_preview" via parent enum tag
    /// The height of the computer display.
    pub display_height : f64,
    /// The width of the computer display.
    pub display_width : f64,
    /// The type of computer environment to control (`mac`, `windows`, `ubuntu`, `browser`).
    pub environment : String,
  }

  /// Represents different actions the computer use tool can perform.
  ///
  /// # Used By
  /// - `ComputerToolCall`
  #[ derive( Debug, Serialize, Deserialize, Clone, PartialEq ) ]
  #[ serde( tag = "type" ) ]
  #[ non_exhaustive ]
  pub enum ComputerAction
  {
    /// A click action.
    #[ serde( rename = "click" ) ]
    Click
    {
      /// Which mouse button was pressed (`left`, `right`, `wheel`, `back`, `forward`).
      button : String,
      /// The x-coordinate where the click occurred.
      x : i64,
      /// The y-coordinate where the click occurred.
      y : i64,
    },
    /// A double click action.
    #[ serde( rename = "double_click" ) ]
    DoubleClick
    {
      /// The x-coordinate where the double click occurred.
      x : i64,
      /// The y-coordinate where the double click occurred.
      y : i64,
    },
    /// A drag action.
    #[ serde( rename = "drag" ) ]
    Drag
    {
      /// An array of coordinates representing the path of the drag action.
      path : Vec< Coordinate >,
    },
    /// A collection of keypresses the model would like to perform.
    #[ serde( rename = "keypress" ) ]
    KeyPress
    {
      /// The combination of keys the model is requesting to be pressed.
      keys : Vec< String >,
    },
    /// A mouse move action.
    #[ serde( rename = "move" ) ]
    Move
    {
      /// The x-coordinate to move to.
      x : i64,
      /// The y-coordinate to move to.
      y : i64,
    },
    /// A screenshot action.
    #[ serde( rename = "screenshot" ) ]
    Screenshot {},
    /// A scroll action.
    #[ serde( rename = "scroll" ) ]
    Scroll
    {
      /// The x-coordinate where the scroll occurred.
      x : i64,
      /// The y-coordinate where the scroll occurred.
      y : i64,
      /// The horizontal scroll distance.
      scroll_x : i64,
      /// The vertical scroll distance.
      scroll_y : i64,
    },
    /// An action to type in text.
    #[ serde( rename = "type" ) ]
    Type
    {
      /// The text to type.
      text : String,
    },
    /// A wait action.
    #[ serde( rename = "wait" ) ]
    Wait {},
  }

  /// Represents a safety check associated with a computer tool call.
  ///
  /// # Used By
  /// - `ComputerToolCall`
  /// - `ComputerToolCallOutput`
  #[ derive( Debug, Serialize, Deserialize, Clone, PartialEq ) ] // Added Serialize
  #[ non_exhaustive ]
  pub struct ComputerToolCallSafetyCheck
  {
    /// The type code of the pending safety check.
    pub code : String,
    /// The ID of the pending safety check.
    pub id : String,
    /// Details about the pending safety check.
    pub message : String,
  }

  /// Represents a call to the computer use tool.
  /// See the [computer use guide](/docs/guides/tools-computer-use) for more information.
  ///
  /// # Used By
  /// - `ResponseOutputItem::ComputerCall` (within `responses.rs`)
  #[ derive( Debug, Serialize, Deserialize, Clone, PartialEq ) ] // Added Serialize
  #[ non_exhaustive ]
  pub struct ComputerToolCall
  {
    /// The specific action requested (e.g., click, type, scroll).
    pub action : ComputerAction,
    /// An identifier used when responding to the tool call with output.
    pub call_id : String,
    /// The unique ID of the computer call.
    pub id : String,
    /// The pending safety checks for the computer call.
    #[ serde( default ) ]
    pub pending_safety_checks : Vec< ComputerToolCallSafetyCheck >,
    /// The status of the item (`in_progress`, `completed`, `incomplete`). Populated when returned via API.
    pub status : String,
  }

  /// Represents the output returned from a computer tool call action.
  ///
  /// # Used By
  /// - `InputItem::ComputerCallOutput` (within `responses.rs`)
  #[ derive( Debug, Serialize, Deserialize, Clone, PartialEq ) ]
  #[ non_exhaustive ]
  pub struct ComputerToolCallOutput
  {
    /// The safety checks reported by the API that have been acknowledged by the developer.
    #[ serde( default ) ]
    pub acknowledged_safety_checks : Vec< ComputerToolCallSafetyCheck >,
    /// The ID of the computer tool call that produced the output.
    pub call_id : String,
    /// The unique ID of the computer call tool output. Populated when returned via API.
    #[ serde( skip_serializing_if = "Option::is_none" ) ]
    pub id : Option< String >,
    /// The output, typically a screenshot image.
    pub output : ComputerScreenshotImage,
    /// The status of the message input (`in_progress`, `completed`, `incomplete`). Populated when returned via API.
    #[ serde( skip_serializing_if = "Option::is_none" ) ]
    pub status : Option< String >,
  }

  // ============================================================================
  // File Search tool structures
  // ============================================================================

  /// Represents ranking options for file search results.
  ///
  /// # Used By
  /// - `FileSearchTool`
  /// - `AssistantFileSearchSettings` (within `assistants_shared.rs`)
  /// - `RunStepDetailsToolCallsFileSearchRankingOptionsObject` (within `assistants_shared.rs`)
  #[ derive( Debug, Serialize, Deserialize, Clone, PartialEq, Default, Former ) ]
  #[ non_exhaustive ]
  pub struct FileSearchRankingOptions
  {
    /// The ranker to use for the file search (`auto` or `default_2024_08_21`). Defaults to `auto`.
    #[ serde( skip_serializing_if = "Option::is_none" ) ]
    pub ranker : Option< String >,
    /// The score threshold for the file search (0 to 1). Defaults to 0.
    #[ serde( skip_serializing_if = "Option::is_none" ) ]
    pub score_threshold : Option< f64 >,
  }

  /// Defines a file search tool.
  ///
  /// # Used By
  /// - `Tool::FileSearch` (within `tools.rs`)
  #[ derive( Debug, Serialize, Deserialize, Clone, PartialEq, Default ) ] // Removed Former
  #[ non_exhaustive ]
  pub struct FileSearchTool; // Removed brackets

  /// Represents a call to the file search tool, including queries and results.
  ///
  /// # Used By
  /// - `ResponseOutputItem::FileSearchCall` (within `output.rs`) // Corrected path
  #[ derive( Debug, Serialize, Deserialize, Clone, PartialEq ) ] // Added Serialize
  #[ non_exhaustive ]
  pub struct FileSearchToolCall
  {
    /// The unique ID of the file search tool call.
    pub id : String,
    /// The queries used to search for files.
    #[ serde( default ) ]
    pub queries : Vec< String >,
    /// The results of the file search tool call. Null if the call failed or is still in progress.
    #[ serde( skip_serializing_if = "Option::is_none" ) ]
    pub results : Option< Vec< FileSearchResultItem > >, // Uses the imported type
    /// The status of the file search tool call (`in_progress`, `searching`, `completed`, `failed`).
    pub status : String,
  }

  // ============================================================================
  // Function Calling tool structures
  // ============================================================================

  /// Represents the parameters for a function tool, described as a JSON Schema object.
  /// See the [guide](https://platform.openai.com/docs/guides/function-calling) for examples, and the
  /// [JSON Schema reference](https://json-schema.org/understanding-json-schema/) for
  /// documentation about the format. Omitting `parameters` defines a function with an empty parameter list.
  ///
  /// # Used By
  /// - `FunctionTool`
  /// - `AssistantToolsFunction` (within `assistants_shared.rs`)
  /// - `ChatCompletionFunctions` (within `chat_shared.rs`)
  #[ derive( Debug, Serialize, Deserialize, Clone, PartialEq, Default ) ]
  #[ serde( transparent ) ]
  #[ non_exhaustive ]
  pub struct FunctionParameters( pub serde_json::Value );

  impl FunctionParameters
  {
    /// Creates a new `FunctionParameters` from a JSON value.
    #[ must_use ]
    #[ inline ]
    pub fn new( value : serde_json::Value ) -> Self
    {
      Self( value )
    }
  }

  /// Defines a function tool that the model can call.
  ///
  /// # Used By
  /// - `Tool::Function` (within `tools.rs`)
  /// - `AssistantToolsFunction` (within `assistants_shared.rs`)
  /// - `ChatCompletionTool` (within `chat_shared.rs`)
  #[ derive( Debug, Serialize, Deserialize, Clone, PartialEq, Former ) ]
  #[ non_exhaustive ]
  pub struct FunctionTool
  {
    // Type is implicitly "function" via parent enum tag
    /// An optional description of what the function does, used by the model to choose when and how to call the function.
    #[ serde( skip_serializing_if = "Option::is_none" ) ]
    pub description : Option< String >,
    /// The name of the function to be called. Must be a-z, A-Z, 0-9, or contain underscores and dashes, with a maximum length of 64.
    pub name : String,
    /// The parameters the function accepts, described as a JSON Schema object.
    pub parameters : FunctionParameters,
    /// Whether to enable strict schema adherence when generating the function call. Defaults to false.
    #[ serde( skip_serializing_if = "Option::is_none" ) ]
    pub strict : Option< bool >,
  }

  /// Represents a call to a function tool, generated by the model.
  ///
  /// # Used By
  /// - `ResponseOutputItem::FunctionCall` (within `responses.rs`)
  /// - `RealtimeConversationItem` (within `realtime_shared.rs`)
  /// - `RunStepToolCall::Function` (within `assistants_shared.rs`)
  #[ derive( Debug, Serialize, Deserialize, Clone, PartialEq ) ] // Added Serialize
  #[ non_exhaustive ]
  pub struct FunctionToolCall
  {
    /// A JSON string of the arguments to pass to the function.
    pub arguments : String,
    /// The unique ID of the function tool call generated by the model.
    pub call_id : String,
    /// The unique ID of the function tool call. Populated when returned via API.
    pub id : String,
    /// The name of the function to run.
    pub name : String,
    /// The status of the item (`in_progress`, `completed`, `incomplete`). Populated when returned via API.
    pub status : String,
  }

  /// Represents the output returned from a function tool call, to be sent back to the model.
  ///
  /// # Used By
  /// - `InputItem::FunctionCallOutput` (within `responses.rs`)
  /// - `RealtimeConversationItem` (within `realtime_shared.rs`)
  #[ derive( Debug, Serialize, Deserialize, Clone, PartialEq ) ]
  #[ non_exhaustive ]
  pub struct FunctionToolCallOutput
  {
    /// The unique ID of the function tool call generated by the model that this output corresponds to.
    pub call_id : String,
    /// The unique ID of the function tool call output. Populated when returned via API.
    #[ serde( skip_serializing_if = "Option::is_none" ) ]
    pub id : Option< String >,
    /// A JSON string of the output of the function tool call.
    pub output : String,
    /// The status of the item (`in_progress`, `completed`, `incomplete`). Populated when returned via API.
    #[ serde( skip_serializing_if = "Option::is_none" ) ]
    pub status : Option< String >,
  }

  // ============================================================================
  // Web Search tool structures
  // ============================================================================

  /// Defines a web search tool.
  /// This tool searches the web for relevant results to use in a response.
  /// Learn more about the [web search tool](/docs/guides/tools-web-search).
  ///
  /// # Used By
  /// - `Tool::WebSearch` (within `tools.rs`)
  #[ derive( Debug, Serialize, Deserialize, Clone, PartialEq, Former, Default ) ] // Added Former back
  #[ serde( default ) ] // Use defaults for missing fields
  #[ non_exhaustive ]
  pub struct WebSearchTool
  {
    /// The amount of context window space to use for search results
    #[ serde( skip_serializing_if = "Option::is_none" ) ]
    pub search_context_size : Option< String >,
    /// User location information for search personalization
    #[ serde( skip_serializing_if = "Option::is_none" ) ]
    pub user_location : Option< WebSearchUserLocation >,
  }

  /// User location information for web search personalization
  ///
  /// # Used By
  /// - `WebSearchTool`
  #[ derive( Debug, Serialize, Deserialize, Clone, PartialEq, Former, Default ) ]
  #[ serde( default ) ]
  #[ non_exhaustive ]
  pub struct WebSearchUserLocation
  {
    /// Type of location (e.g., "approximate")
    #[ serde( skip_serializing_if = "Option::is_none" ) ]
    pub r#type : Option< String >,
    /// City name
    #[ serde( skip_serializing_if = "Option::is_none" ) ]
    pub city : Option< String >,
    /// Country code
    #[ serde( skip_serializing_if = "Option::is_none" ) ]
    pub country : Option< String >,
    /// Region/state
    #[ serde( skip_serializing_if = "Option::is_none" ) ]
    pub region : Option< String >,
    /// Timezone
    #[ serde( skip_serializing_if = "Option::is_none" ) ]
    pub timezone : Option< String >,
  }


  /// Represents a call to the web search tool.
  /// The results of a web search tool call. See the
  /// [web search guide](/docs/guides/tools-web-search) for more information.
  ///
  /// # Used By
  /// - `ResponseOutputItem::WebSearchCall` (within `responses.rs`)
  #[ derive( Debug, Serialize, Deserialize, Clone, PartialEq ) ] // Added Serialize
  #[ non_exhaustive ]
  pub struct WebSearchToolCall
  {
    /// The unique ID of the web search tool call.
    pub id : String,
    /// The status of the web search tool call (`in_progress`, `searching`, `completed`, `failed`).
    pub status : String,
    // Note : The actual search results are typically included in the subsequent assistant message annotations, not directly in this call object.
  }

  /// Represents web search context size options.
  /// High level guidance for the amount of context window space to use for the search.
  ///
  /// # Used By
  /// - `CreateChatCompletionRequest.web_search_options` (within `requests/chat.rs` - *assuming*)
  #[ derive( Debug, Serialize, Deserialize, Clone, PartialEq ) ]
  #[ non_exhaustive ]
  pub struct WebSearchContextSize
  {
    /// The context size setting (`low`, `medium`, or `high`). Defaults to `medium`.
    pub value : String, // Enum : low, medium, high
  }

  /// Represents approximate location parameters for the web search.
  ///
  /// # Used By
  /// - `CreateChatCompletionRequest.web_search_options` (within `requests/chat.rs` - *assuming*)
  /// - `WebSearchTool` (as `user_location` field in some contexts, though not directly in the tool definition schema)
  #[ derive( Debug, Serialize, Deserialize, Clone, PartialEq ) ]
  #[ non_exhaustive ]
  pub struct WebSearchLocation
  {
    /// Free text input for the city (e.g., "San Francisco").
    pub city : Option< String >,
    /// The two-letter ISO country code (e.g., "US").
    #[ serde( skip_serializing_if = "Option::is_none" ) ]
    pub country : Option< String >,
    /// Free text input for the region (e.g., "California").
    #[ serde( skip_serializing_if = "Option::is_none" ) ]
    pub region : Option< String >,
    /// The IANA timezone (e.g., "`America/Los_Angeles`").
    #[ serde( skip_serializing_if = "Option::is_none" ) ]
    pub timezone : Option< String >,
  }
}

crate ::mod_interface!
{
  // Common tool structures
  exposed use { Tool, ToolChoice, ToolChoiceFunction };

  // Computer Use tool
  exposed use
  {
    ComputerTool,
    ComputerAction,
    ComputerToolCallSafetyCheck,
    ComputerToolCall,
    ComputerToolCallOutput,
  };

  // File Search tool
  exposed use
  {
    FileSearchRankingOptions,
    FileSearchTool,
    FileSearchToolCall,
  };

  // Function Calling tool
  exposed use
  {
    FunctionParameters,
    FunctionTool,
    FunctionToolCall,
    FunctionToolCallOutput,
  };

  // Web Search tool
  exposed use
  {
    WebSearchTool,
    WebSearchToolCall,
    WebSearchContextSize,
    WebSearchLocation,
    WebSearchUserLocation,
  };

  // Re-export types used by tool structures
  own use crate::components::common::Coordinate;
  own use crate::components::output::{ ComputerScreenshotImage, FileSearchResultItem };
}