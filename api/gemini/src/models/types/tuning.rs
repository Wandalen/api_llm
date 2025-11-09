//! Model tuning and fine-tuning types for the Gemini API.

use serde::{ Deserialize, Serialize };

/// Request to create a tuned model.
#[ derive( Debug, Clone, Serialize, Deserialize ) ]
#[ serde( rename_all = "camelCase" ) ]
pub struct CreateTunedModelRequest
{
  /// The tuned model to create.
  pub tuned_model : TunedModel,

  /// Optional tuned model ID.
  #[ serde( skip_serializing_if = "Option::is_none" ) ]
  pub tuned_model_id : Option< String >,
}

/// Tuned model information.
#[ derive( Debug, Clone, Serialize, Deserialize ) ]
#[ serde( rename_all = "camelCase" ) ]
pub struct TunedModel
{
  /// The tuned model name.
  pub name : String,

  /// Human-readable display name.
  #[ serde( skip_serializing_if = "Option::is_none" ) ]
  pub display_name : Option< String >,

  /// Description of the tuned model.
  #[ serde( skip_serializing_if = "Option::is_none" ) ]
  pub description : Option< String >,

  /// The base model being tuned.
  pub base_model : String,

  /// Current state of the tuned model.
  #[ serde( skip_serializing_if = "Option::is_none" ) ]
  pub state : Option< String >,

  /// Creation timestamp.
  #[ serde( skip_serializing_if = "Option::is_none" ) ]
  pub create_time : Option< String >,

  /// Last update timestamp.
  #[ serde( skip_serializing_if = "Option::is_none" ) ]
  pub update_time : Option< String >,

  /// Tuning task configuration.
  #[ serde( skip_serializing_if = "Option::is_none" ) ]
  pub tuning_task : Option< TuningTask >,

  /// Tuned model source information.
  #[ serde( skip_serializing_if = "Option::is_none" ) ]
  pub tuned_model_source : Option< TunedModelSource >,

  /// Temperature for the tuned model.
  #[ serde( skip_serializing_if = "Option::is_none" ) ]
  pub temperature : Option< f64 >,

  /// Top-P for the tuned model.
  #[ serde( skip_serializing_if = "Option::is_none" ) ]
  pub top_p : Option< f64 >,

  /// Top-K for the tuned model.
  #[ serde( skip_serializing_if = "Option::is_none" ) ]
  pub top_k : Option< i32 >,
}

/// Tuning task configuration and parameters.
#[ derive( Debug, Clone, Serialize, Deserialize ) ]
#[ serde( rename_all = "camelCase" ) ]
pub struct TuningTask
{
  /// Start time of the tuning task.
  #[ serde( skip_serializing_if = "Option::is_none" ) ]
  pub start_time : Option< String >,

  /// Completion time of the tuning task.
  #[ serde( skip_serializing_if = "Option::is_none" ) ]
  pub complete_time : Option< String >,

  /// Snapshots taken during tuning.
  #[ serde( skip_serializing_if = "Option::is_none" ) ]
  pub snapshots : Option< Vec< TuningSnapshot > >,

  /// Training data for the tuning task.
  #[ serde( skip_serializing_if = "Option::is_none" ) ]
  pub training_data : Option< Dataset >,

  /// Hyperparameters for tuning.
  #[ serde( skip_serializing_if = "Option::is_none" ) ]
  pub hyperparameters : Option< Hyperparameters >,
}

/// Tuning snapshot containing model state.
#[ derive( Debug, Clone, Serialize, Deserialize ) ]
#[ serde( rename_all = "camelCase" ) ]
pub struct TuningSnapshot
{
  /// Step number for this snapshot.
  #[ serde( skip_serializing_if = "Option::is_none" ) ]
  pub step : Option< i32 >,

  /// Epoch number for this snapshot.
  #[ serde( skip_serializing_if = "Option::is_none" ) ]
  pub epoch : Option< i32 >,

  /// Mean loss at this point.
  #[ serde( skip_serializing_if = "Option::is_none" ) ]
  pub mean_loss : Option< f64 >,

  /// Compute time for this step.
  #[ serde( skip_serializing_if = "Option::is_none" ) ]
  pub compute_time : Option< String >,
}

/// Dataset configuration for tuning.
#[ derive( Debug, Clone, Serialize, Deserialize ) ]
#[ serde( rename_all = "camelCase" ) ]
pub struct Dataset
{
  /// Examples in the dataset.
  #[ serde( skip_serializing_if = "Option::is_none" ) ]
  pub examples : Option< TuningExamples >,
}

/// Tuning examples container.
#[ derive( Debug, Clone, Serialize, Deserialize ) ]
#[ serde( rename_all = "camelCase" ) ]
pub struct TuningExamples
{
  /// List of tuning examples.
  pub examples : Vec< TuningExample >,
}

/// Individual tuning example.
#[ derive( Debug, Clone, Serialize, Deserialize ) ]
#[ serde( rename_all = "camelCase" ) ]
pub struct TuningExample
{
  /// Input text for the example.
  #[ serde( skip_serializing_if = "Option::is_none" ) ]
  pub text_input : Option< String >,

  /// Expected output for the example.
  #[ serde( skip_serializing_if = "Option::is_none" ) ]
  pub output : Option< String >,
}

/// Hyperparameters for model tuning.
#[ derive( Debug, Clone, Serialize, Deserialize ) ]
#[ serde( rename_all = "camelCase" ) ]
pub struct Hyperparameters
{
  /// Learning rate for training.
  #[ serde( skip_serializing_if = "Option::is_none" ) ]
  pub learning_rate : Option< f64 >,

  /// Number of training epochs.
  #[ serde( skip_serializing_if = "Option::is_none" ) ]
  pub epoch_count : Option< i32 >,

  /// Batch size for training.
  #[ serde( skip_serializing_if = "Option::is_none" ) ]
  pub batch_size : Option< i32 >,

  /// Learning rate multiplier.
  #[ serde( skip_serializing_if = "Option::is_none" ) ]
  pub learning_rate_multiplier : Option< f64 >,
}

/// Source information for tuned models.
#[ derive( Debug, Clone, Serialize, Deserialize ) ]
#[ serde( rename_all = "camelCase" ) ]
pub struct TunedModelSource
{
  /// The base tuned model if this is derived from another tuned model.
  #[ serde( skip_serializing_if = "Option::is_none" ) ]
  pub tuned_model : Option< String >,

  /// The base model name.
  #[ serde( skip_serializing_if = "Option::is_none" ) ]
  pub base_model : Option< String >,
}

/// Response from listing tuned models.
#[ derive( Debug, Clone, Serialize, Deserialize ) ]
#[ serde( rename_all = "camelCase" ) ]
pub struct ListTunedModelsResponse
{
  /// List of tuned models.
  pub tuned_models : Vec< TunedModel >,

  /// Token for retrieving the next page of results.
  #[ serde( skip_serializing_if = "Option::is_none" ) ]
  pub next_page_token : Option< String >,
}

/// Request to list tuned models.
#[ derive( Debug, Clone, Serialize, Deserialize ) ]
#[ serde( rename_all = "camelCase" ) ]
pub struct ListTunedModelsRequest
{
  /// Maximum number of models to return.
  #[ serde( skip_serializing_if = "Option::is_none" ) ]
  pub page_size : Option< i32 >,

  /// Token for retrieving a specific page of results.
  #[ serde( skip_serializing_if = "Option::is_none" ) ]
  pub page_token : Option< String >,

  /// Filter for tuned models.
  #[ serde( skip_serializing_if = "Option::is_none" ) ]
  pub filter : Option< String >,
}
