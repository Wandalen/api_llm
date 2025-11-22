//! Delta and streaming-related types and structures for the Assistants API.

/// Define a private namespace for streaming-related items.
mod private
{
  // Use full paths from crate root for components
  use crate::components::common::Error;

  // Import types from other modules
  use crate::components::assistants_shared::message::
  {
    MessageObject,
    ImageFileContent,
    FilePathAnnotation,
    ImageUrlContent
  };
  use crate::components::assistants_shared::thread::ThreadObject;
  use crate::components::assistants_shared::run::
  {
    RunObject,
    RunStepObject,
    MessageCreationDetails,
    ImageFileId,
  };

  // Add serde imports
  use serde::{ Deserialize };
  use serde_json::Value;

  // --- Delta Objects for Streaming ---

  /// Represents a delta for an image file content part during streaming.
  ///
  /// # Used By
  /// - `MessageDeltaContent`
  #[ derive( Debug, Deserialize, Clone, PartialEq ) ]
  pub struct MessageDeltaContentImageFileObject
  {
    /// The index of the content part in the message.
    pub index : i32,
    /// Always `image_file`.
    pub r#type : String,
    /// Details of the image file.
    pub image_file : Option< ImageFileContent >,
  }

  /// Represents a delta for a file citation annotation during streaming.
  ///
  /// # Used By
  /// - `MessageDeltaTextAnnotation`
  #[ derive( Debug, Deserialize, Clone, PartialEq ) ]
  pub struct MessageDeltaContentTextAnnotationsFileCitationObject
  {
    /// The index of the annotation in the text content part.
    pub index : i32,
    /// Always `file_citation`.
    pub r#type : String,
    /// The text in the message content that needs to be replaced.
    pub text : Option< String >,
    /// Details of the file citation.
    pub file_citation : Option< FileCitationAnnotationDetails >,
    /// Start index of the citation in the text.
    pub start_index : Option< i32 >,
    /// End index of the citation in the text.
    pub end_index : Option< i32 >,
  }

  /// Details for a file citation annotation delta.
  ///
  /// # Used By
  /// - `MessageDeltaContentTextAnnotationsFileCitationObject`
  #[ derive( Debug, Deserialize, Clone, PartialEq ) ]
  pub struct FileCitationAnnotationDetails
  {
    /// The ID of the specific File the citation is from.
    pub file_id : Option< String >,
    /// The specific quote in the file.
    pub quote : Option< String >,
  }

  /// Represents a delta for a file path annotation during streaming.
  ///
  /// # Used By
  /// - `MessageDeltaTextAnnotation`
  #[ derive( Debug, Deserialize, Clone, PartialEq ) ]
  pub struct MessageDeltaContentTextAnnotationsFilePathObject
  {
    /// The index of the annotation in the text content part.
    pub index : i32,
    /// Always `file_path`.
    pub r#type : String,
    /// The text in the message content that needs to be replaced.
    pub text : Option< String >,
    /// Details of the file path.
    pub file_path : Option< FilePathAnnotation >,
    /// Start index of the file path in the text.
    pub start_index : Option< i32 >,
    /// End index of the file path in the text.
    pub end_index : Option< i32 >,
  }

  /// Represents the delta for a text annotation (file citation or file path).
  ///
  /// # Used By
  /// - `MessageDeltaTextContent`
  #[ derive( Debug, Deserialize, Clone, PartialEq ) ]
  #[ serde( untagged ) ]
  pub enum MessageDeltaTextAnnotation
  {
    /// File citation annotation delta.
    FileCitation( MessageDeltaContentTextAnnotationsFileCitationObject ),
    /// File path annotation delta.
    FilePath( MessageDeltaContentTextAnnotationsFilePathObject ),
  }

  /// Represents the delta for text content within a message.
  ///
  /// # Used By
  /// - `MessageDeltaContentTextObject`
  #[ derive( Debug, Deserialize, Clone, PartialEq ) ]
  pub struct MessageDeltaTextContent
  {
    /// The text delta.
    pub value : Option< String >,
    /// Annotations associated with the delta.
    pub annotations : Option< Vec< MessageDeltaTextAnnotation > >,
  }

  /// Represents a delta for a text content part during streaming.
  ///
  /// # Used By
  /// - `MessageDeltaContent`
  #[ derive( Debug, Deserialize, Clone, PartialEq ) ]
  pub struct MessageDeltaContentTextObject
  {
    /// The index of the content part in the message.
    pub index : i32,
    /// Always `text`.
    pub r#type : String,
    /// The text content delta.
    pub text : Option< MessageDeltaTextContent >,
  }

  /// Represents a delta for a refusal content part during streaming.
  ///
  /// # Used By
  /// - `MessageDeltaContent`
  #[ derive( Debug, Deserialize, Clone, PartialEq ) ]
  pub struct MessageDeltaContentRefusalObject
  {
    /// The index of the refusal part in the message.
    pub index : i32,
    /// Always `refusal`.
    pub r#type : String,
    /// The refusal text delta.
    pub refusal : Option< String >,
  }

  /// Represents a delta for an image URL content part during streaming.
  ///
  /// # Used By
  /// - `MessageDeltaContent`
  #[ derive( Debug, Deserialize, Clone, PartialEq ) ]
  pub struct MessageDeltaContentImageUrlObject
  {
    /// The index of the content part in the message.
    pub index : i32,
    /// Always `image_url`.
    pub r#type : String,
    /// Image URL details.
    pub image_url : Option< ImageUrlContent >,
  }

  /// Represents the delta for different types of message content during streaming.
  ///
  /// # Used By
  /// - `MessageDelta`
  #[ derive( Debug, Deserialize, Clone, PartialEq ) ]
  #[ serde( untagged ) ]
  pub enum MessageDeltaContent
  {
    /// Image file delta.
    ImageFile( MessageDeltaContentImageFileObject ),
    /// Text delta.
    Text( MessageDeltaContentTextObject ),
    /// Refusal delta.
    Refusal( MessageDeltaContentRefusalObject ),
    /// Image URL delta.
    ImageUrl( MessageDeltaContentImageUrlObject ),
  }

  /// Represents the changes in a message during streaming.
  ///
  /// # Used By
  /// - `MessageDeltaObject`
  #[ derive( Debug, Deserialize, Clone, PartialEq ) ]
  pub struct MessageDelta
  {
    /// The role of the message author, if changed.
    pub role : Option< String >,
    /// The content parts that have changed.
    pub content : Option< Vec< MessageDeltaContent > >,
  }

  /// Represents a message delta event data during streaming.
  ///
  /// # Used By
  /// - `AssistantStreamEvent::ThreadMessageDelta`
  #[ derive( Debug, Deserialize, Clone, PartialEq ) ]
  pub struct MessageDeltaObject
  {
    /// The identifier of the message being modified.
    pub id : String,
    /// The object type, always `thread.message.delta`.
    pub object : String,
    /// The delta containing the changed fields.
    pub delta : MessageDelta,
  }

  /// Represents a delta for a message creation step detail during streaming.
  ///
  /// # Used By
  /// - `RunStepDeltaDetails::MessageCreation`
  #[ derive( Debug, Deserialize, Clone, PartialEq ) ]
  pub struct RunStepDeltaStepDetailsMessageCreationObject
  {
    /// Always `message_creation`.
    pub r#type : String,
    /// Details about the message creation, if changed.
    pub message_creation : Option< MessageCreationDetails >,
  }

  /// Represents a delta for a Code Interpreter log output during streaming.
  ///
  /// # Used By
  /// - `RunStepDeltaCodeInterpreterOutput::Logs`
  #[ derive( Debug, Deserialize, Clone, PartialEq ) ]
  pub struct RunStepDeltaStepDetailsToolCallsCodeOutputLogsObject
  {
    /// The index of the output in the outputs array.
    pub index : i32,
    /// Always `logs`.
    pub r#type : String,
    /// The text output delta.
    pub logs : Option< String >,
  }

  /// Represents a delta for a Code Interpreter image output during streaming.
  ///
  /// # Used By
  /// - `RunStepDeltaCodeInterpreterOutput::Image`
  #[ derive( Debug, Deserialize, Clone, PartialEq ) ]
  pub struct RunStepDeltaStepDetailsToolCallsCodeOutputImageObject
  {
    /// The index of the output in the outputs array.
    pub index : i32,
    /// Always `image`.
    pub r#type : String,
    /// Image file details, if changed.
    pub image : Option< ImageFileId >,
  }

  /// Represents the delta for Code Interpreter output (logs or image).
  ///
  /// # Used By
  /// - `RunStepDeltaCodeInterpreter`
  #[ derive( Debug, Deserialize, Clone, PartialEq ) ]
  #[ serde( untagged ) ]
  pub enum RunStepDeltaCodeInterpreterOutput
  {
    /// Log output delta.
    Logs( RunStepDeltaStepDetailsToolCallsCodeOutputLogsObject ),
    /// Image output delta.
    Image( RunStepDeltaStepDetailsToolCallsCodeOutputImageObject ),
  }

  /// Represents the delta for Code Interpreter details during streaming.
  ///
  /// # Used By
  /// - `RunStepDeltaStepDetailsToolCallsCodeObject`
  #[ derive( Debug, Deserialize, Clone, PartialEq ) ]
  pub struct RunStepDeltaCodeInterpreter
  {
    /// The input code delta.
    pub input : Option< String >,
    /// The output deltas.
    pub outputs : Option< Vec< RunStepDeltaCodeInterpreterOutput > >,
  }

  /// Represents a delta for a Code Interpreter tool call during streaming.
  ///
  /// # Used By
  /// - `RunStepDeltaToolCall::Code`
  #[ derive( Debug, Deserialize, Clone, PartialEq ) ]
  pub struct RunStepDeltaStepDetailsToolCallsCodeObject
  {
    /// The index of the tool call in the tool calls array.
    pub index : i32,
    /// The ID of the tool call, if changed.
    pub id : Option< String >,
    /// Always `code_interpreter`.
    pub r#type : String,
    /// The Code Interpreter details delta.
    pub code_interpreter : Option< RunStepDeltaCodeInterpreter >,
  }

  /// Represents a delta for a File Search tool call during streaming.
  ///
  /// # Used By
  /// - `RunStepDeltaToolCall::FileSearch`
  #[ derive( Debug, Deserialize, Clone, PartialEq ) ]
  pub struct RunStepDeltaStepDetailsToolCallsFileSearchObject
  {
    /// The index of the tool call in the tool calls array.
    pub index : i32,
    /// The ID of the tool call, if changed.
    pub id : Option< String >,
    /// Always `file_search`.
    pub r#type : String,
    /// File search details delta (currently always empty).
    pub file_search : Option< Value >,
  }

  /// Represents the delta for a function call within a run step delta.
  ///
  /// # Used By
  /// - `RunStepDeltaStepDetailsToolCallsFunctionObject`
  #[ derive( Debug, Deserialize, Clone, PartialEq ) ]
  pub struct RunStepDeltaFunction
  {
    /// The name of the function, if changed.
    pub name : Option< String >,
    /// The arguments delta.
    pub arguments : Option< String >,
    /// The output delta.
    pub output : Option< String >,
  }

  /// Represents a delta for a Function tool call during streaming.
  ///
  /// # Used By
  /// - `RunStepDeltaToolCall::Function`
  #[ derive( Debug, Deserialize, Clone, PartialEq ) ]
  pub struct RunStepDeltaStepDetailsToolCallsFunctionObject
  {
    /// The index of the tool call in the tool calls array.
    pub index : i32,
    /// The ID of the tool call object, if changed.
    pub id : Option< String >,
    /// Always `function`.
    pub r#type : String,
    /// The function details delta.
    pub function : Option< RunStepDeltaFunction >,
  }

  /// Represents the delta for a specific tool call type within a run step.
  ///
  /// # Used By
  /// - `RunStepDeltaStepDetailsToolCallsObject`
  #[ derive( Debug, Deserialize, Clone, PartialEq ) ]
  #[ serde( untagged ) ]
  pub enum RunStepDeltaToolCall
  {
    /// Code Interpreter tool call delta.
    Code( RunStepDeltaStepDetailsToolCallsCodeObject ),
    /// File Search tool call delta.
    FileSearch( RunStepDeltaStepDetailsToolCallsFileSearchObject ),
    /// Function tool call delta.
    Function( RunStepDeltaStepDetailsToolCallsFunctionObject ),
  }

  /// Represents the delta for the tool calls part of a run step's details.
  ///
  /// # Used By
  /// - `RunStepDeltaDetails::ToolCalls`
  #[ derive( Debug, Deserialize, Clone, PartialEq ) ]
  pub struct RunStepDeltaStepDetailsToolCallsObject
  {
    /// Always `tool_calls`.
    pub r#type : String,
    /// An array of tool call deltas.
    pub tool_calls : Option< Vec< RunStepDeltaToolCall > >,
  }

  /// Represents the delta for the details of a run step.
  ///
  /// # Used By
  /// - `RunStepDelta`
  #[ derive( Debug, Deserialize, Clone, PartialEq ) ]
  #[ serde( untagged ) ]
  pub enum RunStepDeltaDetails
  {
    /// Delta for a message creation step.
    MessageCreation( RunStepDeltaStepDetailsMessageCreationObject ),
    /// Delta for a tool calls step.
    ToolCalls( RunStepDeltaStepDetailsToolCallsObject ),
  }

  /// Represents the changes in a run step during streaming.
  ///
  /// # Used By
  /// - `RunStepDeltaObject`
  #[ derive( Debug, Deserialize, Clone, PartialEq ) ]
  pub struct RunStepDelta
  {
    /// The delta for the step details.
    pub step_details : Option< RunStepDeltaDetails >,
  }

  /// Represents a run step delta event data during streaming.
  ///
  /// # Used By
  /// - `AssistantStreamEvent::ThreadRunStepDelta`
  #[ derive( Debug, Deserialize, Clone, PartialEq ) ]
  pub struct RunStepDeltaObject
  {
    /// The identifier of the run step being modified.
    pub id : String,
    /// The object type, always `thread.run.step.delta`.
    pub object : String,
    /// The delta containing the changed fields.
    pub delta : RunStepDelta,
  }

  /// Represents the different types of events emitted during an Assistant stream.
  ///
  /// # Used By
  /// - Streaming responses from `/threads/runs` (POST), `/threads/{thread_id}/runs` (POST), `/threads/{thread_id}/runs/{run_id}/submit_tool_outputs` (POST)
  #[ derive( Debug, Deserialize, Clone, PartialEq ) ]
  #[ serde( tag = "event" ) ]
  pub enum AssistantStreamEvent
  {
    /// Occurs when a new thread is created.
    #[ serde( rename = "thread.created" ) ]
    ThreadCreated
    {
      /// The created `ThreadObject`.
      data : ThreadObject
    },
    /// Occurs when a new run is created.
    #[ serde( rename = "thread.run.created" ) ]
    ThreadRunCreated
    {
      /// The created `RunObject`.
      data : RunObject
    },
    /// Occurs when a run moves to a `queued` status.
    #[ serde( rename = "thread.run.queued" ) ]
    ThreadRunQueued
    {
      /// The `RunObject` in the queued state.
      data : RunObject
    },
    /// Occurs when a run moves to an `in_progress` status.
    #[ serde( rename = "thread.run.in_progress" ) ]
    ThreadRunInProgress
    {
      /// The `RunObject` in the in-progress state.
      data : RunObject
    },
    /// Occurs when a run moves to a `requires_action` status.
    #[ serde( rename = "thread.run.requires_action" ) ]
    ThreadRunRequiresAction
    {
      /// The `RunObject` requiring action.
      data : RunObject
    },
    /// Occurs when a run is completed.
    #[ serde( rename = "thread.run.completed" ) ]
    ThreadRunCompleted
    {
      /// The completed `RunObject`.
      data : RunObject
    },
    /// Occurs when a run ends with status `incomplete`.
    #[ serde( rename = "thread.run.incomplete" ) ]
    ThreadRunIncomplete
    {
      /// The incomplete `RunObject`.
      data : RunObject
    },
    /// Occurs when a run fails.
    #[ serde( rename = "thread.run.failed" ) ]
    ThreadRunFailed
    {
      /// The failed `RunObject`.
      data : RunObject
    },
    /// Occurs when a run moves to a `cancelling` status.
    #[ serde( rename = "thread.run.cancelling" ) ]
    ThreadRunCancelling
    {
      /// The `RunObject` in the cancelling state.
      data : RunObject
    },
    /// Occurs when a run is cancelled.
    #[ serde( rename = "thread.run.cancelled" ) ]
    ThreadRunCancelled
    {
      /// The cancelled `RunObject`.
      data : RunObject
    },
    /// Occurs when a run expires.
    #[ serde( rename = "thread.run.expired" ) ]
    ThreadRunExpired
    {
      /// The expired `RunObject`.
      data : RunObject
    },
    /// Occurs when a run step is created.
    #[ serde( rename = "thread.run.step.created" ) ]
    ThreadRunStepCreated
    {
      /// The created `RunStepObject`.
      data : RunStepObject
    },
    /// Occurs when a run step moves to an `in_progress` state.
    #[ serde( rename = "thread.run.step.in_progress" ) ]
    ThreadRunStepInProgress
    {
      /// The `RunStepObject` in the in-progress state.
      data : RunStepObject
    },
    /// Occurs when parts of a run step are being streamed.
    #[ serde( rename = "thread.run.step.delta" ) ]
    ThreadRunStepDelta
    {
      /// The `RunStepDeltaObject` containing the changes.
      data : RunStepDeltaObject
    },
    /// Occurs when a run step is completed.
    #[ serde( rename = "thread.run.step.completed" ) ]
    ThreadRunStepCompleted
    {
      /// The completed `RunStepObject`.
      data : RunStepObject
    },
    /// Occurs when a run step fails.
    #[ serde( rename = "thread.run.step.failed" ) ]
    ThreadRunStepFailed
    {
      /// The failed `RunStepObject`.
      data : RunStepObject
    },
    /// Occurs when a run step is cancelled.
    #[ serde( rename = "thread.run.step.cancelled" ) ]
    ThreadRunStepCancelled
    {
      /// The cancelled `RunStepObject`.
      data : RunStepObject
    },
    /// Occurs when a run step expires.
    #[ serde( rename = "thread.run.step.expired" ) ]
    ThreadRunStepExpired
    {
      /// The expired `RunStepObject`.
      data : RunStepObject
    },
    /// Occurs when a message is created.
    #[ serde( rename = "thread.message.created" ) ]
    ThreadMessageCreated
    {
      /// The created `MessageObject`.
      data : MessageObject
    },
    /// Occurs when a message moves to an `in_progress` state.
    #[ serde( rename = "thread.message.in_progress" ) ]
    ThreadMessageInProgress
    {
      /// The `MessageObject` in the in-progress state.
      data : MessageObject
    },
    /// Occurs when parts of a Message are being streamed.
    #[ serde( rename = "thread.message.delta" ) ]
    ThreadMessageDelta
    {
      /// The `MessageDeltaObject` containing the changes.
      data : MessageDeltaObject
    },
    /// Occurs when a message is completed.
    #[ serde( rename = "thread.message.completed" ) ]
    ThreadMessageCompleted
    {
      /// The completed `MessageObject`.
      data : MessageObject
    },
    /// Occurs when a message ends before it is completed.
    #[ serde( rename = "thread.message.incomplete" ) ]
    ThreadMessageIncomplete
    {
      /// The incomplete `MessageObject`.
      data : MessageObject
    },
    /// Occurs when an error occurs during streaming.
    #[ serde( rename = "error" ) ]
    Error
    {
      /// The `Error` object containing details.
      data : Error
    },
    /// Occurs when a stream ends.
    #[ serde( rename = "done" ) ]
    Done
    {
      /// The data, always the string "\[DONE\]".
      data : String
    },
  }
}

crate ::mod_interface!
{
  exposed use private::MessageDeltaContentImageFileObject;
  exposed use private::MessageDeltaContentTextAnnotationsFileCitationObject;
  exposed use private::FileCitationAnnotationDetails;
  exposed use private::MessageDeltaContentTextAnnotationsFilePathObject;
  exposed use private::MessageDeltaTextAnnotation;
  exposed use private::MessageDeltaTextContent;
  exposed use private::MessageDeltaContentTextObject;
  exposed use private::MessageDeltaContentRefusalObject;
  exposed use private::MessageDeltaContentImageUrlObject;
  exposed use private::MessageDeltaContent;
  exposed use private::MessageDelta;
  exposed use private::MessageDeltaObject;
  exposed use private::RunStepDeltaStepDetailsMessageCreationObject;
  exposed use private::RunStepDeltaStepDetailsToolCallsCodeOutputLogsObject;
  exposed use private::RunStepDeltaStepDetailsToolCallsCodeOutputImageObject;
  exposed use private::RunStepDeltaCodeInterpreterOutput;
  exposed use private::RunStepDeltaCodeInterpreter;
  exposed use private::RunStepDeltaStepDetailsToolCallsCodeObject;
  exposed use private::RunStepDeltaStepDetailsToolCallsFileSearchObject;
  exposed use private::RunStepDeltaFunction;
  exposed use private::RunStepDeltaStepDetailsToolCallsFunctionObject;
  exposed use private::RunStepDeltaToolCall;
  exposed use private::RunStepDeltaStepDetailsToolCallsObject;
  exposed use private::RunStepDeltaDetails;
  exposed use private::RunStepDelta;
  exposed use private::RunStepDeltaObject;
  exposed use private::AssistantStreamEvent;
}