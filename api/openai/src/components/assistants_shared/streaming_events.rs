//! Helper event types for Assistant streaming responses.
//!
//! This module contains specialized event enums and structs that wrap
//! the core streaming types for easier event handling and pattern matching.

/// Define a private namespace for streaming event items.
mod private
{
  use crate::components::common::Error;
  use crate::components::assistants_shared::message::MessageObject;
  use crate::components::assistants_shared::thread::ThreadObject;
  use crate::components::assistants_shared::run::{ RunObject, RunStepObject };
  use crate::components::assistants_shared::streaming::
  {
    MessageDeltaObject,
    RunStepDeltaObject,
  };

  use serde::{ Deserialize };

  /// Represents message-related events during streaming.
  #[ derive( Debug, Deserialize, Clone, PartialEq ) ]
  #[ serde( untagged ) ]
  pub enum MessageStreamEvent
  {
    /// Message created event.
    Created( MessageCreatedEvent ),
    /// Message in progress event.
    InProgress( MessageInProgressEvent ),
    /// Message delta event.
    Delta( MessageDeltaEvent ),
    /// Message completed event.
    Completed( MessageCompletedEvent ),
    /// Message incomplete event.
    Incomplete( MessageIncompleteEvent ),
  }

  /// Event data for when a message is created.
  #[ derive( Debug, Deserialize, Clone, PartialEq ) ]
  pub struct MessageCreatedEvent
  {
    /// Event type identifier (`thread.message.created`).
    pub event : String,
    /// The created message object.
    pub data : MessageObject,
  }

  /// Event data for when a message is in progress.
  #[ derive( Debug, Deserialize, Clone, PartialEq ) ]
  pub struct MessageInProgressEvent
  {
    /// Event type identifier (`thread.message.in_progress`).
    pub event : String,
    /// The message object in progress.
    pub data : MessageObject,
  }

  /// Event data for a message delta.
  #[ derive( Debug, Deserialize, Clone, PartialEq ) ]
  pub struct MessageDeltaEvent
  {
    /// Event type identifier (`thread.message.delta`).
    pub event : String,
    /// The message delta object.
    pub data : MessageDeltaObject,
  }

  /// Event data for when a message is completed.
  #[ derive( Debug, Deserialize, Clone, PartialEq ) ]
  pub struct MessageCompletedEvent
  {
    /// Event type identifier (`thread.message.completed`).
    pub event : String,
    /// The completed message object.
    pub data : MessageObject,
  }

  /// Event data for when a message is incomplete.
  #[ derive( Debug, Deserialize, Clone, PartialEq ) ]
  pub struct MessageIncompleteEvent
  {
    /// Event type identifier (`thread.message.incomplete`).
    pub event : String,
    /// The incomplete message object.
    pub data : MessageObject,
  }

  /// Represents run step-related events during streaming.
  #[ derive( Debug, Deserialize, Clone, PartialEq ) ]
  #[ serde( untagged ) ]
  pub enum RunStepStreamEvent
  {
    /// Run step created event.
    Created( RunStepCreatedEvent ),
    /// Run step in progress event.
    InProgress( RunStepInProgressEvent ),
    /// Run step delta event.
    Delta( RunStepDeltaEvent ),
    /// Run step completed event.
    Completed( RunStepCompletedEvent ),
    /// Run step failed event.
    Failed( RunStepFailedEvent ),
    /// Run step cancelled event.
    Cancelled( RunStepCancelledEvent ),
    /// Run step expired event.
    Expired( RunStepExpiredEvent ),
  }

  /// Event data for when a run step is created.
  #[ derive( Debug, Deserialize, Clone, PartialEq ) ]
  pub struct RunStepCreatedEvent
  {
    /// Event type identifier (`thread.run.step.created`).
    pub event : String,
    /// The created run step object.
    pub data : RunStepObject,
  }

  /// Event data for when a run step is in progress.
  #[ derive( Debug, Deserialize, Clone, PartialEq ) ]
  pub struct RunStepInProgressEvent
  {
    /// Event type identifier (`thread.run.step.in_progress`).
    pub event : String,
    /// The run step object in progress.
    pub data : RunStepObject,
  }

  /// Event data for a run step delta.
  #[ derive( Debug, Deserialize, Clone, PartialEq ) ]
  pub struct RunStepDeltaEvent
  {
    /// Event type identifier (`thread.run.step.delta`).
    pub event : String,
    /// The run step delta object.
    pub data : RunStepDeltaObject,
  }

  /// Event data for when a run step is completed.
  #[ derive( Debug, Deserialize, Clone, PartialEq ) ]
  pub struct RunStepCompletedEvent
  {
    /// Event type identifier (`thread.run.step.completed`).
    pub event : String,
    /// The completed run step object.
    pub data : RunStepObject,
  }

  /// Event data for when a run step fails.
  #[ derive( Debug, Deserialize, Clone, PartialEq ) ]
  pub struct RunStepFailedEvent
  {
    /// Event type identifier (`thread.run.step.failed`).
    pub event : String,
    /// The failed run step object.
    pub data : RunStepObject,
  }

  /// Event data for when a run step is cancelled.
  #[ derive( Debug, Deserialize, Clone, PartialEq ) ]
  pub struct RunStepCancelledEvent
  {
    /// Event type identifier (`thread.run.step.cancelled`).
    pub event : String,
    /// The cancelled run step object.
    pub data : RunStepObject,
  }

  /// Event data for when a run step expires.
  #[ derive( Debug, Deserialize, Clone, PartialEq ) ]
  pub struct RunStepExpiredEvent
  {
    /// Event type identifier (`thread.run.step.expired`).
    pub event : String,
    /// The expired run step object.
    pub data : RunStepObject,
  }

  /// Represents run-related events during streaming.
  #[ derive( Debug, Deserialize, Clone, PartialEq ) ]
  #[ serde( untagged ) ]
  pub enum RunStreamEvent
  {
    /// Run created event.
    Created( RunCreatedEvent ),
    /// Run queued event.
    Queued( RunQueuedEvent ),
    /// Run in progress event.
    InProgress( RunInProgressEvent ),
    /// Run requires action event.
    RequiresAction( RunRequiresActionEvent ),
    /// Run completed event.
    Completed( RunCompletedEvent ),
    /// Run incomplete event.
    Incomplete( RunIncompleteEvent ),
    /// Run failed event.
    Failed( RunFailedEvent ),
    /// Run cancelling event.
    Cancelling( RunCancellingEvent ),
    /// Run cancelled event.
    Cancelled( RunCancelledEvent ),
    /// Run expired event.
    Expired( RunExpiredEvent ),
  }

  /// Event data for when a run is created.
  #[ derive( Debug, Deserialize, Clone, PartialEq ) ]
  pub struct RunCreatedEvent
  {
    /// Event type identifier (`thread.run.created`).
    pub event : String,
    /// The created run object.
    pub data : RunObject,
  }

  /// Event data for when a run is queued.
  #[ derive( Debug, Deserialize, Clone, PartialEq ) ]
  pub struct RunQueuedEvent
  {
    /// Event type identifier (`thread.run.queued`).
    pub event : String,
    /// The queued run object.
    pub data : RunObject,
  }

  /// Event data for when a run is in progress.
  #[ derive( Debug, Deserialize, Clone, PartialEq ) ]
  pub struct RunInProgressEvent
  {
    /// Event type identifier (`thread.run.in_progress`).
    pub event : String,
    /// The run object in progress.
    pub data : RunObject,
  }

  /// Event data for when a run requires action.
  #[ derive( Debug, Deserialize, Clone, PartialEq ) ]
  pub struct RunRequiresActionEvent
  {
    /// Event type identifier (`thread.run.requires_action`).
    pub event : String,
    /// The run object requiring action.
    pub data : RunObject,
  }

  /// Event data for when a run is completed.
  #[ derive( Debug, Deserialize, Clone, PartialEq ) ]
  pub struct RunCompletedEvent
  {
    /// Event type identifier (`thread.run.completed`).
    pub event : String,
    /// The completed run object.
    pub data : RunObject,
  }

  /// Event data for when a run is incomplete.
  #[ derive( Debug, Deserialize, Clone, PartialEq ) ]
  pub struct RunIncompleteEvent
  {
    /// Event type identifier (`thread.run.incomplete`).
    pub event : String,
    /// The incomplete run object.
    pub data : RunObject,
  }

  /// Event data for when a run fails.
  #[ derive( Debug, Deserialize, Clone, PartialEq ) ]
  pub struct RunFailedEvent
  {
    /// Event type identifier (`thread.run.failed`).
    pub event : String,
    /// The failed run object.
    pub data : RunObject,
  }

  /// Event data for when a run is cancelling.
  #[ derive( Debug, Deserialize, Clone, PartialEq ) ]
  pub struct RunCancellingEvent
  {
    /// Event type identifier (`thread.run.cancelling`).
    pub event : String,
    /// The cancelling run object.
    pub data : RunObject,
  }

  /// Event data for when a run is cancelled.
  #[ derive( Debug, Deserialize, Clone, PartialEq ) ]
  pub struct RunCancelledEvent
  {
    /// Event type identifier (`thread.run.cancelled`).
    pub event : String,
    /// The cancelled run object.
    pub data : RunObject,
  }

  /// Event data for when a run expires.
  #[ derive( Debug, Deserialize, Clone, PartialEq ) ]
  pub struct RunExpiredEvent
  {
    /// Event type identifier (`thread.run.expired`).
    pub event : String,
    /// The expired run object.
    pub data : RunObject,
  }

  /// Represents thread-related events during streaming.
  #[ derive( Debug, Deserialize, Clone, PartialEq ) ]
  #[ serde( untagged ) ]
  pub enum ThreadStreamEvent
  {
    /// Thread created event.
    Created( ThreadCreatedEvent ),
  }

  /// Event data for when a thread is created.
  #[ derive( Debug, Deserialize, Clone, PartialEq ) ]
  pub struct ThreadCreatedEvent
  {
    /// Event type identifier (`thread.created`).
    pub event : String,
    /// The created thread object.
    pub data : ThreadObject,
  }

  /// Event data for the stream termination message.
  #[ derive( Debug, Deserialize, Clone, PartialEq ) ]
  pub struct DoneEvent
  {
    /// Event type identifier (`done`).
    pub event : String,
    /// The data, always "\[DONE\]".
    pub data : String,
  }

  /// Event data for an error during streaming.
  #[ derive( Debug, Deserialize, Clone, PartialEq ) ]
  pub struct ErrorEvent
  {
    /// Event type identifier (`error`).
    pub event : String,
    /// The error details.
    pub data : Error,
  }
}

crate ::mod_interface!
{
  exposed use private::MessageStreamEvent;
  exposed use private::MessageCreatedEvent;
  exposed use private::MessageInProgressEvent;
  exposed use private::MessageDeltaEvent;
  exposed use private::MessageCompletedEvent;
  exposed use private::MessageIncompleteEvent;
  exposed use private::RunStepStreamEvent;
  exposed use private::RunStepCreatedEvent;
  exposed use private::RunStepInProgressEvent;
  exposed use private::RunStepDeltaEvent;
  exposed use private::RunStepCompletedEvent;
  exposed use private::RunStepFailedEvent;
  exposed use private::RunStepCancelledEvent;
  exposed use private::RunStepExpiredEvent;
  exposed use private::RunStreamEvent;
  exposed use private::RunCreatedEvent;
  exposed use private::RunQueuedEvent;
  exposed use private::RunInProgressEvent;
  exposed use private::RunRequiresActionEvent;
  exposed use private::RunCompletedEvent;
  exposed use private::RunIncompleteEvent;
  exposed use private::RunFailedEvent;
  exposed use private::RunCancellingEvent;
  exposed use private::RunCancelledEvent;
  exposed use private::RunExpiredEvent;
  exposed use private::ThreadStreamEvent;
  exposed use private::ThreadCreatedEvent;
  exposed use private::DoneEvent;
  exposed use private::ErrorEvent;
}
