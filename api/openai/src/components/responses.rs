// src/components/responses.rs
//! This module defines the data structures for requests and responses related to the `OpenAI` Responses API.
//! It includes the main `CreateResponseRequest` for generating model responses,
//! the `ResponseObject` object representing a generated response, and various
//! stream event structures for real-time response generation.
//!
//! For more details, refer to the [`OpenAI` Responses API documentation](https://platform.openai.com/docs/api-reference/responses).

/// Define a private namespace for all its items.
mod private
{
  // Use crate root for base access
  use crate::components;
  // Grouped imports relative to crate root
  use crate::components::common::{ ResponseError, ResponseUsage, Metadata, Reasoning, TextResponseFormatConfigurationOptions, Includable };
  use crate::components::input::{ InputItem, ListedInputItem };
  use crate::components::output::{ OutputItem, OutputContentPart, Annotation };
  use crate::components::tools::{ Tool, ToolChoice };

  // Serde and Former imports
  use serde::{ Serialize, Deserialize };
  use former::Former;

  // --- Request Structs ---

  /// Represents the input for the Create Response request.
  /// Can be a simple string or a list of input items (messages, etc.).
  ///
  /// # Used By
  /// - `CreateResponseRequest`
  #[ derive( Debug, Serialize, Deserialize, Clone, PartialEq ) ] // Added Serialize/Deserialize
  #[ serde( untagged ) ] // Allows serialization as string or array
  #[ non_exhaustive ]
  pub enum ResponseInput
  {
    /// A list of input items.
    Items( Vec< InputItem > ),
    /// A single string input.
    String( String ),
  }

  /// Represents the request body for creating a model response.
  /// Corresponds to the request body of `POST /responses`.
  ///
  /// # Used By
  /// - `api::responses::Responses::create`
  #[ derive( Debug, Serialize, Clone, PartialEq, Former ) ] // REMOVED Default
  // Add #[ former( init ) ] if you need a constructor-like builder initialization
  #[ non_exhaustive ]
  pub struct CreateResponseRequest
  {
    /// Text, image, or file inputs to the model.
    pub input : ResponseInput,
    /// Additional output data to include (e.g., "`file_search_call.results`").
    #[ serde( skip_serializing_if = "Option::is_none" ) ]
    pub include : Option< Vec< Includable > >,
    /// System instructions for the model.
    #[ serde( skip_serializing_if = "Option::is_none" ) ]
    pub instructions : Option< String >,
    /// Maximum output tokens allowed.
    #[ serde( skip_serializing_if = "Option::is_none" ) ]
    pub max_output_tokens : Option< i32 >,
    /// Metadata associated with the request.
    #[ serde( skip_serializing_if = "Option::is_none" ) ]
    pub metadata : Option< Metadata >,
    /// Model ID used to generate the response.
    // Use the wrapper type for requests to allow From< String > via derive
    pub model : components::common::ModelIdsResponses,
    /// Whether to allow parallel tool calls. Defaults to true.
    #[ serde( default = "default_parallel_tool_calls" ) ] // Add default
    pub parallel_tool_calls : bool,
    /// ID of the previous response in the conversation.
    #[ serde( skip_serializing_if = "Option::is_none" ) ]
    pub previous_response_id : Option< String >,
    /// Reasoning configuration.
    #[ serde( skip_serializing_if = "Option::is_none" ) ]
    pub reasoning : Option< Reasoning >,
    /// Whether to store the generated response. Defaults to true.
    #[ serde( skip_serializing_if = "Option::is_none" ) ]
    pub store : Option< bool >,
    /// Whether to stream the response. Defaults to false.
    #[ serde( skip_serializing_if = "Option::is_none" ) ]
    pub stream : Option< bool >,
    /// Sampling temperature.
    #[ serde( skip_serializing_if = "Option::is_none" ) ]
    pub temperature : Option< f32 >,
    /// Text response format configuration.
    #[ serde( skip_serializing_if = "Option::is_none" ) ]
    pub text : Option< TextResponseFormatConfigurationOptions >,
    /// Strategy for choosing tools.
    #[ serde( skip_serializing_if = "Option::is_none" ) ]
    pub tool_choice : Option< ToolChoice >,
    /// List of available tools.
    #[ serde( skip_serializing_if = "Option::is_none" ) ]
    pub tools : Option< Vec< Tool > >,
    /// Nucleus sampling probability.
    #[ serde( skip_serializing_if = "Option::is_none" ) ]
    pub top_p : Option< f32 >,
    /// Truncation strategy ("auto" or "disabled").
    #[ serde( skip_serializing_if = "Option::is_none" ) ]
    pub truncation : Option< String >,
    /// End-user identifier.
    #[ serde( skip_serializing_if = "Option::is_none" ) ]
    pub user : Option< String >,
  }

  /// Helper function for default value of `parallel_tool_calls`
  fn default_parallel_tool_calls() -> bool { true }

  // --- Response Structs ---

  /// Represents an output message from the model within a Response.
  /// Corresponds to the `OutputMessage` schema in the `OpenAPI` spec.
  ///
  /// # Used By
  /// - `OutputItem::Message` (within `output.rs`)
  #[ derive( Debug, Serialize, Deserialize, Clone, PartialEq ) ] // Added Serialize
  #[ non_exhaustive ]
  pub struct OutputMessage
  {
    /// The content parts of the message (e.g., text, refusal).
    pub content : Vec< OutputContentPart >, // Uses OutputContentPart from output.rs
    /// The unique ID of the output message.
    pub id : String,
    /// The role of the output message, always "assistant".
    pub role : String,
    /// The status of the message (`in_progress`, `completed`, `incomplete`).
    pub status : String,
  }


  /// Represents a response generated by the model via the Responses API.
  /// Corresponds to the `Response` schema in the `OpenAPI` spec.
  ///
  /// # Used By
  /// - `/responses` (POST response)
  /// - `/responses/{response_id}` (GET response)
  /// - `RealtimeServerEventResponseCreated` (within `realtime_shared.rs`)
  /// - `RealtimeServerEventResponseDone` (within `realtime_shared.rs`)
  /// - `RealtimeServerEventResponseFailed` (within `realtime_shared.rs`)
  /// - `RealtimeServerEventResponseIncomplete` (within `realtime_shared.rs`)
  /// - `RealtimeServerEventResponseInProgress` (within `realtime_shared.rs`)
  #[ derive( Debug, Serialize, Deserialize, Clone, PartialEq, Former ) ] // Added Former
  #[ non_exhaustive ]
  pub struct ResponseObject // Renamed from Response
  {
    /// Unix timestamp (in seconds) of when this Response was created.
    pub created_at : i64, // Changed from number to i64 for timestamp
    /// The error object if the response status is `failed`. Null otherwise.
    #[ serde( skip_serializing_if = "Option::is_none" ) ]
    pub error : Option< ResponseError >, // Use relative path common::ResponseError
    /// Unique identifier for this Response.
    pub id : String,
    /// Details about why the response is incomplete, if applicable. Null otherwise.
    #[ serde( skip_serializing_if = "Option::is_none" ) ]
    pub incomplete_details : Option< ResponseIncompleteDetails >, // Needs definition
    /// System instructions provided.
    #[ serde( skip_serializing_if = "Option::is_none" ) ]
    pub instructions : Option< String >,
    /// Maximum output tokens allowed.
    #[ serde( skip_serializing_if = "Option::is_none" ) ]
    pub max_output_tokens : Option< i32 >,
    /// Metadata associated with the response.
    #[ serde( skip_serializing_if = "Option::is_none" ) ]
    pub metadata : Option< Metadata >, // Use relative path common::Metadata
    /// Model ID used for the response. *** CHANGED TO String ***
    pub model : String,
    /// The object type, always "response".
    pub object : String,
    /// An array of content items generated by the model.
    pub output : Vec< OutputItem >, // Needs definition
    /// Whether parallel tool calls were enabled.
    #[ serde( default = "default_parallel_tool_calls" ) ] // Add default
    pub parallel_tool_calls : bool,
    /// ID of the previous response in the conversation.
    #[ serde( skip_serializing_if = "Option::is_none" ) ]
    pub previous_response_id : Option< String >,
    /// Reasoning configuration used.
    #[ serde( skip_serializing_if = "Option::is_none" ) ]
    pub reasoning : Option< Reasoning >, // Use relative path common::Reasoning
    /// The status of the response generation (`completed`, `failed`, `in_progress`, `incomplete`).
    pub status : String,
    /// Whether to stream the response. Defaults to false.
    #[ serde( skip_serializing_if = "Option::is_none" ) ]
    pub stream : Option< bool >,
    /// Sampling temperature.
    #[ serde( skip_serializing_if = "Option::is_none" ) ]
    pub temperature : Option< f32 >,
    /// Text response format configuration.
    #[ serde( skip_serializing_if = "Option::is_none" ) ]
    pub text : Option< TextResponseFormatConfigurationOptions >, // Use relative path common::TextResponseFormatConfigurationOptions
    /// Tool choice strategy used.
    #[ serde( skip_serializing_if = "Option::is_none" ) ]
    pub tool_choice : Option< ToolChoice >, // Use relative path tools::ToolChoice
    /// Tools available to the model.
    #[ serde( skip_serializing_if = "Option::is_none" ) ]
    pub tools : Option< Vec< Tool > >, // Use relative path tools::Tool
    /// Nucleus sampling probability.
    #[ serde( skip_serializing_if = "Option::is_none" ) ]
    pub top_p : Option< f32 >,
    /// Truncation strategy ("auto" or "disabled").
    #[ serde( skip_serializing_if = "Option::is_none" ) ]
    pub truncation : Option< String >, // Enum : auto, disabled
    /// Usage statistics for the request. Null if status is not `completed`.
    #[ serde( skip_serializing_if = "Option::is_none" ) ]
    pub usage : Option< ResponseUsage >, // Use relative path common::ResponseUsage
    /// End-user identifier.
    #[ serde( skip_serializing_if = "Option::is_none" ) ]
    pub user : Option< String >,
  }

  /// Details on why a response is incomplete.
  /// Corresponds to the nested `incomplete_details` schema within `Response`.
  ///
  /// # Used By
  /// - `ResponseObject`
  #[ derive( Debug, Serialize, Deserialize, Clone, PartialEq, Former ) ] // Added Serialize, Former
  #[ non_exhaustive ]
  pub struct ResponseIncompleteDetails
  {
    /// The reason why the response is incomplete (e.g., `max_output_tokens`, `content_filter`).
    pub reason : String,
  }

  /// Represents a list of Response items, typically used for input items list.
  /// Corresponds to the `ResponseItemList` schema in the `OpenAPI` spec.
  ///
  /// # Used By
  /// - `/responses/{response_id}/input_items` (GET)
  #[ derive( Debug, Deserialize, Clone, PartialEq ) ] // Only Deserialize needed for list response
  #[ non_exhaustive ]
  pub struct ResponseItemList
  {
    /// A list of input items (messages) associated with the response.
    pub data : Vec< ListedInputItem >, // Use relative path input::ListedInputItem
    /// The identifier of the first item in the data array.
    pub first_id : Option< String >,
    /// Indicates whether there are more items available.
    pub has_more : bool,
    /// The identifier of the last item in the data array.
    pub last_id : Option< String >,
    /// The object type, always "list".
    pub object : String,
  }

  /// Event data for `response.content_part.added`.
  #[ derive( Debug, Serialize, Deserialize, Clone, PartialEq ) ]
  #[ non_exhaustive ]
  pub struct ResponseContentPartAddedEvent
  {
    /// The index of the content part that was added.
    pub content_index : u32,
    /// The ID of the output item that the content part was added to.
    pub item_id : String,
    /// The index of the output item that the content part was added to.
    pub output_index : u32,
    /// The content part that was added.
    pub part : OutputContentPart,
  }

  /// Event data for `response.content_part.done`.
  #[ derive( Debug, Serialize, Deserialize, Clone, PartialEq ) ]
  #[ non_exhaustive ]
  pub struct ResponseContentPartDoneEvent
  {
    /// The index of the content part that is done.
    pub content_index : u32,
    /// The ID of the output item that the content part was added to.
    pub item_id : String,
    /// The index of the output item that the content part was added to.
    pub output_index : u32,
    /// The content part that is done.
    pub part : OutputContentPart,
  }

  /// Event data for `response.created`.
  #[ derive( Debug, Serialize, Deserialize, Clone, PartialEq ) ]
  #[ non_exhaustive ]
  pub struct ResponseCreatedEvent
  {
    /// The response that was created.
    pub response : ResponseObject, // Changed from Response
  }

  /// Event data for `error`.
  #[ derive( Debug, Serialize, Deserialize, Clone, PartialEq ) ]
  #[ non_exhaustive ]
  pub struct ResponseErrorEvent
  {
    /// The error code.
    #[ serde( skip_serializing_if = "Option::is_none" ) ]
    pub code : Option< String >,
    /// The error message.
    pub message : String,
    /// The error parameter.
    #[ serde( skip_serializing_if = "Option::is_none" ) ]
    pub param : Option< String >,
  }

  /// Event data for `response.failed`.
  #[ derive( Debug, Serialize, Deserialize, Clone, PartialEq ) ]
  #[ non_exhaustive ]
  pub struct ResponseFailedEvent
  {
    /// The response that failed.
    pub response : ResponseObject, // Changed from Response
  }

  /// Event data for `response.file_search_call.completed`.
  #[ derive( Debug, Serialize, Deserialize, Clone, PartialEq ) ]
  #[ non_exhaustive ]
  pub struct ResponseFileSearchCallCompletedEvent
  {
    /// The ID of the output item that the file search call is initiated.
    pub item_id : String,
    /// The index of the output item that the file search call is initiated.
    pub output_index : u32,
  }

  /// Event data for `response.file_search_call.in_progress`.
  #[ derive( Debug, Serialize, Deserialize, Clone, PartialEq ) ]
  #[ non_exhaustive ]
  pub struct ResponseFileSearchCallInProgressEvent
  {
    /// The ID of the output item that the file search call is initiated.
    pub item_id : String,
    /// The index of the output item that the file search call is initiated.
    pub output_index : u32,
  }

  /// Event data for `response.file_search_call.searching`.
  #[ derive( Debug, Serialize, Deserialize, Clone, PartialEq ) ]
  #[ non_exhaustive ]
  pub struct ResponseFileSearchCallSearchingEvent
  {
    /// The ID of the output item that the file search call is initiated.
    pub item_id : String,
    /// The index of the output item that the file search call is searching.
    pub output_index : u32,
  }

  /// Event data for `response.function_call_arguments.delta`.
  #[ derive( Debug, Serialize, Deserialize, Clone, PartialEq ) ]
  #[ non_exhaustive ]
  pub struct ResponseFunctionCallArgumentsDeltaEvent
  {
    /// The function-call arguments delta that is added.
    pub delta : String,
    /// The ID of the output item that the function-call arguments delta is added to.
    pub item_id : String,
    /// The index of the output item that the function-call arguments delta is added to.
    pub output_index : u32,
    /// The call ID of the function.
    pub call_id : String,
  }

  /// Event data for `response.function_call_arguments.done`.
  #[ derive( Debug, Serialize, Deserialize, Clone, PartialEq ) ]
  #[ non_exhaustive ]
  pub struct ResponseFunctionCallArgumentsDoneEvent
  {
    /// The function-call arguments that are finalized.
    pub arguments : String,
    /// The call ID of the function.
    pub call_id : String,
    /// The ID of the output item that the function-call arguments are finalized.
    pub item_id : String,
    /// The index of the output item that the function-call arguments are finalized.
    pub output_index : u32,
  }

  /// Event data for `response.incomplete`.
  #[ derive( Debug, Serialize, Deserialize, Clone, PartialEq ) ]
  #[ non_exhaustive ]
  pub struct ResponseIncompleteEvent
  {
    /// The response that was incomplete.
    pub response : ResponseObject, // Changed from Response
  }

  /// Event data for `response.in_progress`.
  #[ derive( Debug, Serialize, Deserialize, Clone, PartialEq ) ]
  #[ non_exhaustive ]
  pub struct ResponseInProgressEvent
  {
    /// The response that is in progress.
    pub response : ResponseObject, // Changed from Response
  }

  /// Event data for `response.output_item.added`.
  #[ derive( Debug, Serialize, Deserialize, Clone, PartialEq ) ]
  #[ non_exhaustive ]
  pub struct ResponseOutputItemAddedEvent
  {
    /// The output item that was added.
    pub item : OutputItem,
    /// The index of the output item that was added.
    pub output_index : u32,
  }

  /// Event data for `response.output_item.done`.
  #[ derive( Debug, Serialize, Deserialize, Clone, PartialEq ) ]
  #[ non_exhaustive ]
  pub struct ResponseOutputItemDoneEvent
  {
    /// The output item that was marked done.
    pub item : OutputItem,
    /// The index of the output item that was marked done.
    pub output_index : u32,
  }

  /// Event data for `response.refusal.delta`.
  #[ derive( Debug, Serialize, Deserialize, Clone, PartialEq ) ]
  #[ non_exhaustive ]
  pub struct ResponseRefusalDeltaEvent
  {
    /// The index of the content part that the refusal text is added to.
    pub content_index : u32,
    /// The refusal text that is added.
    pub delta : String,
    /// The ID of the output item that the refusal text is added to.
    pub item_id : String,
    /// The index of the output item that the refusal text is added to.
    pub output_index : u32,
  }

  /// Event data for `response.refusal.done`.
  #[ derive( Debug, Serialize, Deserialize, Clone, PartialEq ) ]
  #[ non_exhaustive ]
  pub struct ResponseRefusalDoneEvent
  {
    /// The index of the content part that the refusal text is finalized.
    pub content_index : u32,
    /// The ID of the output item that the refusal text is finalized.
    pub item_id : String,
    /// The index of the output item that the refusal text is finalized.
    pub output_index : u32,
    /// The refusal text that is finalized.
    pub refusal : String,
  }

  /// Event data for `response.output_text.annotation.added`.
  #[ derive( Debug, Serialize, Deserialize, Clone, PartialEq ) ]
  #[ non_exhaustive ]
  pub struct ResponseTextAnnotationDeltaEvent
  {
    /// The annotation that was added.
    pub annotation : Annotation,
    /// The index of the annotation that was added.
    pub annotation_index : u32,
    /// The index of the content part that the text annotation was added to.
    pub content_index : u32,
    /// The ID of the output item that the text annotation was added to.
    pub item_id : String,
    /// The index of the output item that the text annotation was added to.
    pub output_index : u32,
  }

  /// Event data for `response.output_text.delta`.
  #[ derive( Debug, Serialize, Deserialize, Clone, PartialEq ) ]
  #[ non_exhaustive ]
  pub struct ResponseTextDeltaEvent
  {
    /// The index of the content part that the text delta was added to.
    pub content_index : u32,
    /// The text delta that was added.
    pub delta : String,
    /// The ID of the output item that the text delta was added to.
    pub item_id : String,
    /// The index of the output item that the text delta was added to.
    pub output_index : u32,
  }

  /// Event data for `response.output_text.done`.
  #[ derive( Debug, Serialize, Deserialize, Clone, PartialEq ) ]
  #[ non_exhaustive ]
  pub struct ResponseTextDoneEvent
  {
    /// The index of the content part that the text content is finalized.
    pub content_index : u32,
    /// The ID of the output item that the text content is finalized.
    pub item_id : String,
    /// The index of the output item that the text content is finalized.
    pub output_index : u32,
    /// The text content that is finalized.
    pub text : String,
  }

  /// Event data for `response.web_search_call.completed`.
  #[ derive( Debug, Serialize, Deserialize, Clone, PartialEq ) ]
  #[ non_exhaustive ]
  pub struct ResponseWebSearchCallCompletedEvent
  {
    /// Unique ID for the output item associated with the web search call.
    pub item_id : String,
    /// The index of the output item that the web search call is associated with.
    pub output_index : u32,
  }

  /// Event data for `response.web_search_call.in_progress`.
  #[ derive( Debug, Serialize, Deserialize, Clone, PartialEq ) ]
  #[ non_exhaustive ]
  pub struct ResponseWebSearchCallInProgressEvent
  {
    /// Unique ID for the output item associated with the web search call.
    pub item_id : String,
    /// The index of the output item that the web search call is associated with.
    pub output_index : u32,
  }

  /// Event data for `response.web_search_call.searching`.
  #[ derive( Debug, Serialize, Deserialize, Clone, PartialEq ) ]
  #[ non_exhaustive ]
  pub struct ResponseWebSearchCallSearchingEvent
  {
    /// Unique ID for the output item associated with the web search call.
    pub item_id : String,
    /// The index of the output item that the web search call is associated with.
    pub output_index : u32,
  }

  /// Event data for `response.completed`.
  #[ derive( Debug, Serialize, Deserialize, Clone, PartialEq ) ]
  #[ non_exhaustive ]
  pub struct ResponseCompletedEvent
  {
    /// The response that was completed.
    pub response : ResponseObject,
    /// The sequence number of this event in the stream.
    #[ serde( skip_serializing_if = "Option::is_none" ) ]
    pub sequence_number : Option< u32 >,
  }

  /// Represents an event emitted when streaming a Response.
  /// Corresponds to the `ResponseStreamEvent` schema in the `OpenAPI` spec.
  ///
  /// # Used By
  /// - `api::responses::Responses::create_stream`
  #[ derive( Debug, Serialize, Deserialize, Clone, PartialEq ) ]
  #[ serde( tag = "type" ) ]
  #[ non_exhaustive ]
  pub enum ResponseStreamEvent
  {
    /// Emitted when a new content part is added.
    #[ serde( rename = "response.content_part.added" ) ]
    ResponseContentPartAdded( ResponseContentPartAddedEvent ),
    /// Emitted when a content part is done.
    #[ serde( rename = "response.content_part.done" ) ]
    ResponseContentPartDone( ResponseContentPartDoneEvent ),
    /// Emitted when a response is created.
    #[ serde( rename = "response.created" ) ]
    ResponseCreated( ResponseCreatedEvent ),
    /// Emitted when an error occurs.
    #[ serde( rename = "error" ) ]
    ResponseErrorEvent( ResponseErrorEvent ),
    /// Emitted when a response fails.
    #[ serde( rename = "response.failed" ) ]
    ResponseFailed( ResponseFailedEvent ),
    /// Emitted when a file search call is completed (results found).
    #[ serde( rename = "response.file_search_call.completed" ) ]
    ResponseFileSearchCallCompleted( ResponseFileSearchCallCompletedEvent ),
    /// Emitted when a file search call is initiated.
    #[ serde( rename = "response.file_search_call.in_progress" ) ]
    ResponseFileSearchCallInProgress( ResponseFileSearchCallInProgressEvent ),
    /// Emitted when a file search is currently searching.
    #[ serde( rename = "response.file_search_call.searching" ) ]
    ResponseFileSearchCallSearching( ResponseFileSearchCallSearchingEvent ),
    /// Emitted when function-call arguments are finalized.
    #[ serde( rename = "response.function_call_arguments.done" ) ]
    ResponseFunctionCallArgumentsDone( ResponseFunctionCallArgumentsDoneEvent ),
    /// Emitted when there is a partial function-call arguments delta.
    #[ serde( rename = "response.function_call_arguments.delta" ) ]
    ResponseFunctionCallArgumentsDelta( ResponseFunctionCallArgumentsDeltaEvent ),
    /// Emitted when a response finishes as incomplete.
    #[ serde( rename = "response.incomplete" ) ]
    ResponseIncomplete( ResponseIncompleteEvent ),
    /// Emitted when the response is in analysis.
    #[ serde( rename = "response.in_analysis" ) ]
    ResponseInAnalysis( ResponseInProgressEvent ),
    /// Emitted when the response is in progress.
    #[ serde( rename = "response.in_progress" ) ]
    ResponseInProgress( ResponseInProgressEvent ),
    /// Emitted when a new output item is added.
    #[ serde( rename = "response.output_item.added" ) ]
    ResponseOutputItemAdded( ResponseOutputItemAddedEvent ),
    /// Emitted when an output item is marked done.
    #[ serde( rename = "response.output_item.done" ) ]
    ResponseOutputItemDone( ResponseOutputItemDoneEvent ),
    /// Emitted when refusal text is finalized.
    #[ serde( rename = "response.refusal.done" ) ]
    ResponseRefusalDone( ResponseRefusalDoneEvent ),
    /// Emitted when there is a partial refusal text.
    #[ serde( rename = "response.refusal.delta" ) ]
    ResponseRefusalDelta( ResponseRefusalDeltaEvent ),
    /// Emitted when text annotation is added.
    #[ serde( rename = "response.output_text.annotation.added" ) ]
    ResponseTextAnnotationDelta( ResponseTextAnnotationDeltaEvent ),
    /// Emitted when there is an additional text delta.
    #[ serde( rename = "response.output_text.delta" ) ]
    ResponseTextDelta( ResponseTextDeltaEvent ),
    /// Emitted when text content is finalized.
    #[ serde( rename = "response.output_text.done" ) ]
    ResponseTextDone( ResponseTextDoneEvent ),
    /// Emitted when a web search call is completed.
    #[ serde( rename = "response.web_search_call.completed" ) ]
    ResponseWebSearchCallCompleted( ResponseWebSearchCallCompletedEvent ),
    /// Emitted when a web search call is initiated.
    #[ serde( rename = "response.web_search_call.in_progress" ) ]
    ResponseWebSearchCallInProgress( ResponseWebSearchCallInProgressEvent ),
    /// Emitted when a web search call is executing.
    #[ serde( rename = "response.web_search_call.searching" ) ]
    ResponseWebSearchCallSearching( ResponseWebSearchCallSearchingEvent ),
    /// Emitted when the model response is complete.
    #[ serde( rename = "response.completed" ) ]
    ResponseCompleted( ResponseCompletedEvent ),
  }
} // end mod private

crate ::mod_interface!
{
  // Expose all structs defined in this module
  exposed use
  {
    ResponseInput,
    CreateResponseRequest,
    ResponseObject,
    ResponseIncompleteDetails,
    ResponseItemList,
    OutputMessage,
    ResponseStreamEvent,
    ResponseCreatedEvent,
    ResponseInProgressEvent,
    ResponseFailedEvent,
    ResponseIncompleteEvent,
    ResponseOutputItemAddedEvent,
    ResponseOutputItemDoneEvent,
    ResponseContentPartAddedEvent,
    ResponseContentPartDoneEvent,
    ResponseTextDeltaEvent,
    ResponseTextAnnotationDeltaEvent,
    ResponseTextDoneEvent,
    ResponseRefusalDeltaEvent,
    ResponseRefusalDoneEvent,
    ResponseFunctionCallArgumentsDeltaEvent,
    ResponseFunctionCallArgumentsDoneEvent,
    ResponseFileSearchCallInProgressEvent,
    ResponseFileSearchCallSearchingEvent,
    ResponseFileSearchCallCompletedEvent,
    ResponseWebSearchCallInProgressEvent,
    ResponseWebSearchCallSearchingEvent,
    ResponseWebSearchCallCompletedEvent,
    ResponseCompletedEvent,
    ResponseErrorEvent,
  };
  // Re-export types used by the exposed structs
  own use crate::components::
  {
    common ::
    {
      ResponseError,
      ResponseUsage,
      Metadata,
      Reasoning,
      TextResponseFormatConfigurationOptions,
      Includable,
      ListQuery, // Correctly placed ListQuery
    },
    input ::{ InputItem, ListedInputItem },
    output ::{ OutputItem, OutputContentPart, Annotation, FileSearchResultItem, ComputerScreenshotImage },
    tools ::{ Tool, ToolChoice },
  };
}
