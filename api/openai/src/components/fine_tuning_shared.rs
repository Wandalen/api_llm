//! Structures shared across the Fine-tuning API, including jobs, checkpoints, and events.

/// Define a private namespace for all its items.
mod private
{
  // Use full paths from crate root for components
  use crate::components::common::{ Metadata, ParallelToolCalls };
  use crate::components::chat_shared::ChatCompletionResponseMessage;
  // Corrected import : ChatCompletionTool is in tools.rs
  use crate::components::chat_shared::ChatCompletionTool;

  // Serde imports
  use serde::{ Serialize, Deserialize };
  use serde_json::Value;

  /// Represents an error that occurred during a fine-tuning job.
  ///
  /// # Used By
  /// - `FineTuningJob`
  #[ derive( Debug, Serialize, Deserialize, Clone, PartialEq ) ] // Added Serialize
  pub struct FineTuningJobError
  {
    /// A machine-readable error code.
    pub code : String,
    /// A human-readable error message.
    pub message : String,
    /// The parameter that was invalid, usually `training_file` or `validation_file`. Null if not parameter-specific.
    pub param : Option< String >,
  }

  /// Represents the hyperparameters used for a fine-tuning job (legacy format).
  /// This structure is deprecated in favor of specifying hyperparameters within the `method` field.
  ///
  /// # Used By
  /// - `FineTuningJob` (as deprecated field)
  /// - `FineTuneSupervisedMethod`
  #[ derive( Debug, Serialize, Deserialize, Clone, PartialEq ) ] // Added Serialize
  pub struct FineTuningJobHyperparameters
  {
    /// Number of examples in each batch (`auto` or integer 1-256).
    pub batch_size : Value, // Represents oneOf : string("auto"), integer
    /// Scaling factor for the learning rate (`auto` or number > 0).
    pub learning_rate_multiplier : Value, // Represents oneOf : string("auto"), number
    /// The number of epochs to train the model for (`auto` or integer 1-50).
    pub n_epochs : Value, // Represents oneOf : string("auto"), integer
  }

  /// Represents the configuration for Weights & Biases integration.
  ///
  /// # Used By
  /// - `FineTuningIntegration`
  #[ derive( Debug, Serialize, Deserialize, Clone, PartialEq ) ] // Added Serialize
  pub struct FineTuningIntegrationWandb
  {
    /// The name of the project that the new run will be created under.
    pub project : String,
    /// A display name to set for the run. If not set, the Job ID is used.
    #[ serde( skip_serializing_if = "Option::is_none" ) ]
    pub name : Option< String >,
    /// The entity (team or username) to associate with the run. Defaults to the registered API key's default entity.
    #[ serde( skip_serializing_if = "Option::is_none" ) ]
    pub entity : Option< String >,
    /// A list of tags to be attached to the newly created run.
    #[ serde( skip_serializing_if = "Option::is_none" ) ]
    pub tags : Option< Vec< String > >,
  }

  /// Represents an integration configuration for a fine-tuning job.
  ///
  /// # Used By
  /// - `FineTuningJob`
  /// - `CreateFineTuningJobRequest` (within `requests/fine_tuning.rs` - *assuming*)
  #[ derive( Debug, Serialize, Deserialize, Clone, PartialEq ) ] // Added Serialize
  pub struct FineTuningIntegration
  {
    /// The type of integration (currently only "wandb").
    pub r#type : String,
    /// The settings for Weights & Biases integration.
    pub wandb : FineTuningIntegrationWandb,
  }

  /// Configuration for the DPO (Direct Preference Optimization) fine-tuning method.
  ///
  /// # Used By
  /// - `FineTuneMethod::DPO`
  #[ derive( Debug, Serialize, Deserialize, Clone, PartialEq ) ] // Added Serialize
  pub struct FineTuneDPOMethod
  {
    /// The hyperparameters specific to the DPO method.
    pub hyperparameters : DPOHyperparameters,
  }

  /// Hyperparameters specific to the DPO fine-tuning method.
  ///
  /// # Used By
  /// - `FineTuneDPOMethod`
  #[ derive( Debug, Serialize, Deserialize, Clone, PartialEq ) ] // Added Serialize
  pub struct DPOHyperparameters
  {
    /// The beta value for the DPO method (`auto` or number > 0, <= 2).
    pub beta : Value, // Represents oneOf : string("auto"), number
    /// Number of examples in each batch (`auto` or integer 1-256).
    pub batch_size : Value, // Represents oneOf : string("auto"), integer
    /// Scaling factor for the learning rate (`auto` or number > 0).
    pub learning_rate_multiplier : Value, // Represents oneOf : string("auto"), number
    /// The number of epochs to train the model for (`auto` or integer 1-50).
    pub n_epochs : Value, // Represents oneOf : string("auto"), integer
  }

  /// Configuration for the supervised fine-tuning method.
  ///
  /// # Used By
  /// - `FineTuneMethod::Supervised`
  #[ derive( Debug, Serialize, Deserialize, Clone, PartialEq ) ] // Added Serialize
  pub struct FineTuneSupervisedMethod
  {
    /// The hyperparameters used for the supervised fine-tuning job.
    pub hyperparameters : FineTuningJobHyperparameters,
  }

  /// Represents the method used for fine-tuning (supervised or DPO).
  ///
  /// # Used By
  /// - `FineTuningJob`
  /// - `CreateFineTuningJobRequest` (within `requests/fine_tuning.rs` - *assuming*)
  #[ derive( Debug, Serialize, Deserialize, Clone, PartialEq ) ] // Added Serialize
  #[ serde( tag = "type" ) ]
  pub enum FineTuneMethod
  {
    /// Supervised fine-tuning method.
    #[ serde( rename = "supervised" ) ]
    Supervised( FineTuneSupervisedMethod ),
    /// Direct Preference Optimization (DPO) fine-tuning method.
    #[ serde( rename = "dpo" ) ]
    DPO( FineTuneDPOMethod ),
  }

  /// The `fine_tuning.job` object represents a fine-tuning job that has been created through the API.
  ///
  /// # Used By
  /// - `/fine_tuning/jobs` (GET - in `ListPaginatedFineTuningJobsResponse`, POST response)
  /// - `/fine_tuning/jobs/{fine_tuning_job_id}` (GET, POST response)
  /// - `/fine_tuning/jobs/{fine_tuning_job_id}/cancel` (POST response)
  /// - `FineTuningJobCheckpoint`
  #[ derive( Debug, Serialize, Deserialize, Clone, PartialEq ) ] // Added Serialize
  pub struct FineTuningJob
  {
    /// The object identifier, which can be referenced in the API endpoints.
    pub id : String,
    /// The Unix timestamp (in seconds) for when the fine-tuning job was created.
    pub created_at : i64,
    /// For fine-tuning jobs that have `failed`, this will contain more information on the cause of the failure.
    pub error : Option< FineTuningJobError >,
    /// The name of the fine-tuned model that is being created. Null if the job is still running.
    pub fine_tuned_model : Option< String >,
    /// The Unix timestamp (in seconds) for when the fine-tuning job was finished. Null if the job is still running.
    pub finished_at : Option< i64 >,
    /// Deprecated : The hyperparameters used for the fine-tuning job. See `method`.
    #[ deprecated( note = "Use method.hyperparameters instead" ) ]
    pub hyperparameters : Option< FineTuningJobHyperparameters >,
    /// The base model that is being fine-tuned.
    pub model : String,
    /// The object type, which is always "`fine_tuning.job`".
    pub object : String,
    /// The organization that owns the fine-tuning job.
    pub organization_id : String,
    /// The compiled results file ID(s) for the fine-tuning job.
    pub result_files : Vec< String >,
    /// The current status of the fine-tuning job.
    pub status : String, // Enum : validating_files, queued, running, succeeded, failed, cancelled
    /// The total number of billable tokens processed by this job. Null if the job is still running.
    pub trained_tokens : Option< i64 >,
    /// The file ID used for training.
    pub training_file : String,
    /// The file ID used for validation. Null if no validation file was provided.
    pub validation_file : Option< String >,
    /// A list of integrations to enable for this fine-tuning job.
    #[ serde( skip_serializing_if = "Option::is_none" ) ]
    pub integrations : Option< Vec< FineTuningIntegration > >,
    /// The seed used for the fine-tuning job.
    pub seed : i32,
    /// The Unix timestamp (in seconds) for when the job is estimated to finish. Null if not running.
    #[ serde( skip_serializing_if = "Option::is_none" ) ]
    pub estimated_finish : Option< i64 >,
    /// The fine-tuning method used.
    #[ serde( skip_serializing_if = "Option::is_none" ) ]
    pub method : Option< FineTuneMethod >,
    /// Set of 16 key-value pairs attached to the object.
    #[ serde( skip_serializing_if = "Option::is_none" ) ]
    pub metadata : Option< Metadata >,
  }

  /// Response containing a paginated list of fine-tuning jobs.
  ///
  /// # Used By
  /// - `/fine_tuning/jobs` (GET)
  #[ derive( Debug, Deserialize, Clone, PartialEq ) ] // Only Deserialize needed
  pub struct ListPaginatedFineTuningJobsResponse
  {
    /// A list of fine-tuning job objects.
    pub data : Vec< FineTuningJob >,
    /// Indicates whether there are more jobs available.
    pub has_more : bool,
    /// The object type, always "list".
    pub object : String,
  }

  /// Metrics reported at a specific step number during a fine-tuning job.
  ///
  /// # Used By
  /// - `FineTuningJobCheckpoint`
  #[ derive( Debug, Serialize, Deserialize, Clone, PartialEq ) ] // Added Serialize
  pub struct FineTuningJobCheckpointMetrics
  {
    /// The step number associated with these metrics.
    pub step : Option< f64 >,
    /// Training loss at this step.
    pub train_loss : Option< f64 >,
    /// Mean token accuracy on the training data at this step.
    pub train_mean_token_accuracy : Option< f64 >,
    /// Validation loss at this step.
    pub valid_loss : Option< f64 >,
    /// Mean token accuracy on the validation data at this step.
    pub valid_mean_token_accuracy : Option< f64 >,
    /// Full validation loss computed at the end of the job.
    pub full_valid_loss : Option< f64 >,
    /// Full mean token accuracy on the validation data computed at the end of the job.
    pub full_valid_mean_token_accuracy : Option< f64 >,
  }

  /// Represents a model checkpoint for a fine-tuning job that is ready to use.
  ///
  /// # Used By
  /// - `/fine_tuning/jobs/{fine_tuning_job_id}/checkpoints` (GET - in `ListFineTuningJobCheckpointsResponse`)
  #[ derive( Debug, Serialize, Deserialize, Clone, PartialEq ) ] // Added Serialize
  pub struct FineTuningJobCheckpoint
  {
    /// The checkpoint identifier.
    pub id : String,
    /// The Unix timestamp (in seconds) for when the checkpoint was created.
    pub created_at : i64,
    /// The name of the fine-tuned checkpoint model that is created.
    pub fine_tuned_model_checkpoint : String,
    /// The step number that the checkpoint was created at.
    pub step_number : i32,
    /// Metrics at the step number during the fine-tuning job.
    pub metrics : FineTuningJobCheckpointMetrics,
    /// The name of the fine-tuning job that this checkpoint was created from.
    pub fine_tuning_job_id : String,
    /// The object type, which is always "`fine_tuning.job.checkpoint`".
    pub object : String,
  }

  /// Response containing a list of fine-tuning job checkpoints.
  ///
  /// # Used By
  /// - `/fine_tuning/jobs/{fine_tuning_job_id}/checkpoints` (GET)
  #[ derive( Debug, Deserialize, Clone, PartialEq ) ] // Only Deserialize needed
  pub struct ListFineTuningJobCheckpointsResponse
  {
    /// A list of fine-tuning job checkpoint objects.
    pub data : Vec< FineTuningJobCheckpoint >,
    /// The object type, always "list".
    pub object : String,
    /// The ID of the first checkpoint in the list, used for pagination.
    pub first_id : Option< String >,
    /// The ID of the last checkpoint in the list, used for pagination.
    pub last_id : Option< String >,
    /// Indicates whether there are more checkpoints available.
    pub has_more : bool,
  }

  /// Represents an event related to a fine-tuning job (e.g., status updates, metric reports).
  ///
  /// # Used By
  /// - `/fine_tuning/jobs/{fine_tuning_job_id}/events` (GET - in `ListFineTuningJobEventsResponse`)
  #[ derive( Debug, Serialize, Deserialize, Clone, PartialEq ) ] // Added Serialize
  pub struct FineTuningJobEvent
  {
    /// The object type, which is always "`fine_tuning.job.event`".
    pub object : String,
    /// The object identifier.
    pub id : String,
    /// The Unix timestamp (in seconds) for when the event was created.
    pub created_at : i64,
    /// The log level of the event (`info`, `warn`, `error`).
    pub level : String,
    /// The message of the event.
    pub message : String,
    /// The type of event (`message` or `metrics`).
    pub r#type : String,
    /// Additional data associated with the event (e.g., metrics).
    pub data : Option< Value >,
  }

  /// Response containing a list of fine-tuning job events.
  ///
  /// # Used By
  /// - `/fine_tuning/jobs/{fine_tuning_job_id}/events` (GET)
  #[ derive( Debug, Deserialize, Clone, PartialEq ) ] // Only Deserialize needed
  pub struct ListFineTuningJobEventsResponse
  {
    /// A list of fine-tuning event objects.
    pub data : Vec< FineTuningJobEvent >,
    /// The object type, always "list".
    pub object : String,
    /// Indicates whether there are more events available.
    pub has_more : bool,
  }

  /// Represents a permission for a fine-tuned model checkpoint.
  ///
  /// # Used By
  /// - `/fine_tuning/checkpoints/{permission_id}/permissions` (GET - in `ListFineTuningCheckpointPermissionResponse`, POST response)
  /// - `DeleteFineTuningCheckpointPermissionResponse` (within `common.rs`)
  #[ derive( Debug, Serialize, Deserialize, Clone, PartialEq ) ] // Added Serialize
  pub struct FineTuningCheckpointPermission
  {
    /// The permission identifier.
    pub id : String,
    /// The Unix timestamp (in seconds) for when the permission was created.
    pub created_at : i64,
    /// The project identifier that the permission is for.
    pub project_id : String,
    /// The object type, which is always "checkpoint.permission".
    pub object : String,
  }

  /// Response containing a list of fine-tuning checkpoint permissions.
  ///
  /// # Used By
  /// - `/fine_tuning/checkpoints/{permission_id}/permissions` (GET, POST response)
  #[ derive( Debug, Deserialize, Clone, PartialEq ) ] // Only Deserialize needed
  pub struct ListFineTuningCheckpointPermissionResponse
  {
    /// A list of fine-tuned model checkpoint permission objects.
    pub data : Vec< FineTuningCheckpointPermission >,
    /// The object type, always "list".
    pub object : String,
    /// The ID of the first permission in the list, used for pagination.
    pub first_id : Option< String >,
    /// The ID of the last permission in the list, used for pagination.
    pub last_id : Option< String >,
    /// Indicates whether there are more permissions available.
    pub has_more : bool,
  }

  /// Represents the input structure for a chat-based fine-tuning job using the supervised method.
  /// This is typically one line in the training JSONL file.
  ///
  /// # Used By
  /// - Training data files for supervised chat fine-tuning.
  #[ derive( Debug, Serialize, Deserialize, Clone, PartialEq ) ] // Added Serialize
  pub struct FineTuneChatRequestInput
  {
    /// A list of messages representing the conversation turn.
    pub messages : Vec< ChatCompletionResponseMessage >, // Assuming response message structure is used for training examples
    /// Optional list of tools available for this example.
    #[ serde( skip_serializing_if = "Option::is_none" ) ]
    pub tools : Option< Vec< ChatCompletionTool > >,
    /// Optional setting for parallel tool calls in this example.
    #[ serde( skip_serializing_if = "Option::is_none" ) ]
    pub parallel_tool_calls : Option< ParallelToolCalls >,
  }

  /// Represents the input structure for a completions-based fine-tuning job.
  /// This is typically one line in the training JSONL file.
  ///
  /// # Used By
  /// - Training data files for legacy completions fine-tuning.
  #[ derive( Debug, Serialize, Deserialize, Clone, PartialEq ) ] // Added Serialize
  pub struct FineTuneCompletionRequestInput
  {
    /// The input prompt for this training example.
    pub prompt : String,
    /// The desired completion for this training example.
    pub completion : String,
  }

  /// Represents the input structure for a preference-based (DPO) fine-tuning job.
  /// This is typically one line in the training JSONL file.
  ///
  /// # Used By
  /// - Training data files for DPO fine-tuning.
  #[ derive( Debug, Serialize, Deserialize, Clone, PartialEq ) ] // Added Serialize
  pub struct FineTunePreferenceRequestInput
  {
    /// The input messages for the preference comparison.
    pub input : FineTunePreferenceInputData,
    /// The preferred completion message for the output.
    pub preferred_completion : Vec< ChatCompletionResponseMessage >, // Assuming response message structure
    /// The non-preferred completion message for the output.
    pub non_preferred_completion : Vec< ChatCompletionResponseMessage >, // Assuming response message structure
  }

  /// Contains the input messages and optional tools for a DPO preference example.
  ///
  /// # Used By
  /// - `FineTunePreferenceRequestInput`
  #[ derive( Debug, Serialize, Deserialize, Clone, PartialEq ) ] // Added Serialize
  pub struct FineTunePreferenceInputData
  {
    /// The list of messages forming the input context.
    pub messages : Vec< ChatCompletionResponseMessage >, // Assuming response message structure
    /// Optional list of tools available for this example.
    #[ serde( skip_serializing_if = "Option::is_none" ) ]
    pub tools : Option< Vec< ChatCompletionTool > >,
    /// Optional setting for parallel tool calls in this example.
    #[ serde( skip_serializing_if = "Option::is_none" ) ]
    pub parallel_tool_calls : Option< ParallelToolCalls >,
  }

} // end mod private

crate ::mod_interface!
{
  exposed use
  {
    FineTuningJobError,
    FineTuningJobHyperparameters,
    FineTuningIntegrationWandb,
    FineTuningIntegration,
    FineTuneDPOMethod,
    DPOHyperparameters,
    FineTuneSupervisedMethod,
    FineTuneMethod,
    FineTuningJob,
    ListPaginatedFineTuningJobsResponse,
    FineTuningJobCheckpointMetrics,
    FineTuningJobCheckpoint,
    ListFineTuningJobCheckpointsResponse,
    FineTuningJobEvent,
    ListFineTuningJobEventsResponse,
    FineTuningCheckpointPermission,
    ListFineTuningCheckpointPermissionResponse,
    FineTuneChatRequestInput,
    FineTuneCompletionRequestInput,
    FineTunePreferenceRequestInput,
    FineTunePreferenceInputData
  };
}