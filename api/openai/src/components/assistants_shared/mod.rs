//! Structures shared across the Assistants API, including Assistants, Threads, Messages, Runs, and Steps.

// Declare submodules
pub mod assistant;
pub mod message;
pub mod thread;
pub mod run;
pub mod streaming;
pub mod streaming_events;

// Re-export assistant types
pub use assistant::
{
  AssistantSupportedModels,
  AssistantsApiResponseFormatOption,
  AssistantsApiToolChoiceOption,
  AssistantsNamedToolChoice,
  TruncationObject,
  AssistantToolsCode,
  AssistantToolsFileSearch,
  AssistantFileSearchSettings,
  AssistantToolsFileSearchTypeOnly,
  AssistantToolsFunction,
  AssistantObject,
  ToolResources,
  CodeInterpreterResources,
  FileSearchResources,
  ListAssistantsResponse,
  DeleteAssistantResponse,
};

// Re-export message types
pub use message::
{
  MessageContentImageFileObject,
  ImageFileContent,
  MessageContentImageUrlObject,
  ImageUrlContent,
  MessageContentTextAnnotationsFileCitationObject,
  FileCitationAnnotation,
  MessageContentTextAnnotationsFilePathObject,
  FilePathAnnotation,
  MessageContentTextObject,
  TextContent,
  MessageContentRefusalObject,
  MessageContent,
  MessageAttachment,
  MessageObject,
  IncompleteDetails,
  ListMessagesResponse,
};

// Re-export thread types
pub use thread::
{
  ThreadObject,
  ListThreadsResponse,
};

// Re-export run types
pub use run::
{
  RunLastError,
  RequiredAction,
  SubmitToolOutputs,
  RunToolCallObject,
  RunToolCallFunction,
  RunObject,
  ListRunsResponse,
  RunStepDetailsMessageCreationObject,
  MessageCreationDetails,
  RunStepDetailsToolCallsCodeOutputLogsObject,
  RunStepDetailsToolCallsCodeOutputImageObject,
  ImageFileId,
  CodeInterpreterOutput,
  CodeInterpreterDetails,
  RunStepDetailsToolCallsCodeObject,
  RunStepDetailsToolCallsFileSearchObject,
  RunStepDetailsToolCallsFunctionObject,
  FunctionCallDetails,
  RunStepToolCall,
  RunStepDetailsToolCallsObject,
  RunStepDetails,
  RunStepObject,
  ListRunStepsResponse,
};

// Re-export streaming types
pub use streaming::
{
  MessageDeltaContentImageFileObject,
  MessageDeltaContentTextAnnotationsFileCitationObject,
  FileCitationAnnotationDetails,
  MessageDeltaContentTextAnnotationsFilePathObject,
  MessageDeltaTextAnnotation,
  MessageDeltaTextContent,
  MessageDeltaContentTextObject,
  MessageDeltaContentRefusalObject,
  MessageDeltaContentImageUrlObject,
  MessageDeltaContent,
  MessageDelta,
  MessageDeltaObject,
  RunStepDeltaStepDetailsMessageCreationObject,
  RunStepDeltaStepDetailsToolCallsCodeOutputLogsObject,
  RunStepDeltaStepDetailsToolCallsCodeOutputImageObject,
  RunStepDeltaCodeInterpreterOutput,
  RunStepDeltaCodeInterpreter,
  RunStepDeltaStepDetailsToolCallsCodeObject,
  RunStepDeltaStepDetailsToolCallsFileSearchObject,
  RunStepDeltaFunction,
  RunStepDeltaStepDetailsToolCallsFunctionObject,
  RunStepDeltaToolCall,
  RunStepDeltaStepDetailsToolCallsObject,
  RunStepDeltaDetails,
  RunStepDelta,
  RunStepDeltaObject,
  AssistantStreamEvent,
};

// Re-export streaming event types
pub use streaming_events::
{
  MessageStreamEvent,
  MessageCreatedEvent,
  MessageInProgressEvent,
  MessageDeltaEvent,
  MessageCompletedEvent,
  MessageIncompleteEvent,
  RunStepStreamEvent,
  RunStepCreatedEvent,
  RunStepInProgressEvent,
  RunStepDeltaEvent,
  RunStepCompletedEvent,
  RunStepFailedEvent,
  RunStepCancelledEvent,
  RunStepExpiredEvent,
  RunStreamEvent,
  RunCreatedEvent,
  RunQueuedEvent,
  RunInProgressEvent,
  RunRequiresActionEvent,
  RunCompletedEvent,
  RunIncompleteEvent,
  RunFailedEvent,
  RunCancellingEvent,
  RunCancelledEvent,
  RunExpiredEvent,
  ThreadStreamEvent,
  ThreadCreatedEvent,
  DoneEvent,
  ErrorEvent,
};