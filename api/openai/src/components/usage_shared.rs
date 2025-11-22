//! Structures related to API Usage and Costs endpoints.

/// Define a private namespace for all its items.
mod private
{
  // Serde imports
  use serde::Deserialize;

  /// Represents the monetary value and currency for a cost item.
  ///
  /// # Used By
  /// - `CostsResult`
  #[ derive( Debug, Deserialize, Clone, PartialEq ) ]
  #[ non_exhaustive ]
  pub struct CostsResultAmount
  {
    /// Lowercase ISO-4217 currency code (e.g., "usd").
    pub currency : String,
    /// The numeric value of the cost.
    pub value : f64,
  }

  /// Represents the aggregated cost details for a specific time bucket and grouping.
  ///
  /// # Used By
  /// - `UsageResult::Costs`
  #[ derive( Debug, Deserialize, Clone, PartialEq ) ]
  #[ non_exhaustive ]
  pub struct CostsResult
  {
    /// The monetary value and currency.
    pub amount : CostsResultAmount,
    /// The invoice line item if grouped by `line_item`.
    pub line_item : Option< String >,
    /// Object type, always "organization.costs.result".
    pub object : String,
    /// The project ID if grouped by `project_id`.
    pub project_id : Option< String >,
  }

  /// Represents aggregated usage details for the Audio Speeches API within a time bucket.
  ///
  /// # Used By
  /// - `UsageResult::AudioSpeeches`
  #[ derive( Debug, Deserialize, Clone, PartialEq ) ]
  #[ non_exhaustive ]
  pub struct UsageAudioSpeechesResult
  {
    /// The API key ID if grouped by `api_key_id`.
    pub api_key_id : Option< String >,
    /// The number of characters processed.
    pub characters : i32,
    /// The model name if grouped by `model`.
    pub model : Option< String >,
    /// The count of requests made to the model.
    pub num_model_requests : i32,
    /// Object type, always "`organization.usage.audio_speeches.result`".
    pub object : String,
    /// The project ID if grouped by `project_id`.
    pub project_id : Option< String >,
    /// The user ID if grouped by `user_id`.
    pub user_id : Option< String >,
  }

  /// Represents aggregated usage details for the Audio Transcriptions API within a time bucket.
  ///
  /// # Used By
  /// - `UsageResult::AudioTranscriptions`
  #[ derive( Debug, Deserialize, Clone, PartialEq ) ]
  #[ non_exhaustive ]
  pub struct UsageAudioTranscriptionsResult
  {
    /// The API key ID if grouped by `api_key_id`.
    pub api_key_id : Option< String >,
    /// The model name if grouped by `model`.
    pub model : Option< String >,
    /// The count of requests made to the model.
    pub num_model_requests : i32,
    /// Object type, always "`organization.usage.audio_transcriptions.result`".
    pub object : String,
    /// The project ID if grouped by `project_id`.
    pub project_id : Option< String >,
    /// The number of seconds processed.
    pub seconds : i32,
    /// The user ID if grouped by `user_id`.
    pub user_id : Option< String >,
  }

  /// Represents aggregated usage details for Code Interpreter sessions within a time bucket.
  ///
  /// # Used By
  /// - `UsageResult::CodeInterpreter`
  #[ derive( Debug, Deserialize, Clone, PartialEq ) ]
  #[ non_exhaustive ]
  pub struct UsageCodeInterpreterSessionsResult
  {
    /// The number of code interpreter sessions used.
    pub num_sessions : i32, // Corrected field name from 'sessions' to 'num_sessions' based on example
    /// Object type, always "`organization.usage.code_interpreter_sessions.result`".
    pub object : String,
    /// The project ID if grouped by `project_id`.
    pub project_id : Option< String >,
  }

  /// Represents aggregated usage details for the Completions API within a time bucket.
  ///
  /// # Used By
  /// - `UsageResult::Completions`
  #[ derive( Debug, Deserialize, Clone, PartialEq ) ]
  #[ non_exhaustive ]
  pub struct UsageCompletionsResult
  {
    /// The API key ID if grouped by `api_key_id`.
    pub api_key_id : Option< String >,
    /// Whether the usage result is for batch jobs if grouped by `batch`.
    pub batch : Option< bool >,
    /// Aggregated number of audio input tokens used (including cached).
    pub input_audio_tokens : Option< i32 >,
    /// Aggregated number of text input tokens used (including cached).
    pub input_tokens : i32,
    /// The model name if grouped by `model`.
    pub model : Option< String >,
    /// The count of requests made to the model.
    pub num_model_requests : i32,
    /// Object type, always "organization.usage.completions.result".
    pub object : String,
    /// Aggregated number of audio output tokens used.
    pub output_audio_tokens : Option< i32 >,
    /// Aggregated number of text output tokens used.
    pub output_tokens : i32,
    /// The project ID if grouped by `project_id`.
    pub project_id : Option< String >,
    /// Aggregated number of cached text input tokens.
    pub input_cached_tokens : Option< i32 >,
    /// The user ID if grouped by `user_id`.
    pub user_id : Option< String >,
  }

  /// Represents aggregated usage details for the Embeddings API within a time bucket.
  ///
  /// # Used By
  /// - `UsageResult::Embeddings`
  #[ derive( Debug, Deserialize, Clone, PartialEq ) ]
  #[ non_exhaustive ]
  pub struct UsageEmbeddingsResult
  {
    /// The API key ID if grouped by `api_key_id`.
    pub api_key_id : Option< String >,
    /// The aggregated number of input tokens used.
    pub input_tokens : i32,
    /// The model name if grouped by `model`.
    pub model : Option< String >,
    /// The count of requests made to the model.
    pub num_model_requests : i32,
    /// Object type, always "organization.usage.embeddings.result".
    pub object : String,
    /// The project ID if grouped by `project_id`.
    pub project_id : Option< String >,
    /// The user ID if grouped by `user_id`.
    pub user_id : Option< String >,
  }

  /// Represents aggregated usage details for the Images API within a time bucket.
  ///
  /// # Used By
  /// - `UsageResult::Images`
  #[ derive( Debug, Deserialize, Clone, PartialEq ) ]
  #[ non_exhaustive ]
  pub struct UsageImagesResult
  {
    /// The API key ID if grouped by `api_key_id`.
    pub api_key_id : Option< String >,
    /// The number of images processed.
    pub images : i32,
    /// The model name if grouped by `model`.
    pub model : Option< String >,
    /// The count of requests made to the model.
    pub num_model_requests : i32,
    /// Object type, always "organization.usage.images.result".
    pub object : String,
    /// The project ID if grouped by `project_id`.
    pub project_id : Option< String >,
    /// The image size if grouped by `size` (e.g., "1024x1024").
    pub size : Option< String >,
    /// The source of the usage if grouped by `source` (e.g., "image.generation").
    pub source : Option< String >,
    /// The user ID if grouped by `user_id`.
    pub user_id : Option< String >,
  }

  /// Represents aggregated usage details for the Moderations API within a time bucket.
  ///
  /// # Used By
  /// - `UsageResult::Moderations`
  #[ derive( Debug, Deserialize, Clone, PartialEq ) ]
  #[ non_exhaustive ]
  pub struct UsageModerationsResult
  {
    /// The API key ID if grouped by `api_key_id`.
    pub api_key_id : Option< String >,
    /// The aggregated number of input tokens used.
    pub input_tokens : i32,
    /// The model name if grouped by `model`.
    pub model : Option< String >,
    /// The count of requests made to the model.
    pub num_model_requests : i32,
    /// Object type, always "organization.usage.moderations.result".
    pub object : String,
    /// The project ID if grouped by `project_id`.
    pub project_id : Option< String >,
    /// The user ID if grouped by `user_id`.
    pub user_id : Option< String >,
  }

  /// Represents aggregated usage details for Vector Stores within a time bucket.
  ///
  /// # Used By
  /// - `UsageResult::VectorStores`
  #[ derive( Debug, Deserialize, Clone, PartialEq ) ]
  #[ non_exhaustive ]
  pub struct UsageVectorStoresResult
  {
    /// Object type, always "`organization.usage.vector_stores.result`".
    pub object : String,
    /// The project ID if grouped by `project_id`.
    pub project_id : Option< String >,
    /// The vector stores usage in bytes.
    pub usage_bytes : i64,
  }

  /// Represents the aggregated usage or cost result within a time bucket, varying by endpoint.
  ///
  /// # Used By
  /// - `UsageTimeBucket`
  #[ derive( Debug, Deserialize, Clone, PartialEq ) ]
  #[ serde( untagged ) ]
  #[ non_exhaustive ]
  pub enum UsageResult
  {
    /// Audio Speeches API usage details.
    AudioSpeeches( UsageAudioSpeechesResult ),
    /// Audio Transcriptions API usage details.
    AudioTranscriptions( UsageAudioTranscriptionsResult ),
    /// Code Interpreter sessions usage details.
    CodeInterpreter( UsageCodeInterpreterSessionsResult ),
    /// Completions API usage details.
    Completions( UsageCompletionsResult ),
    /// Costs details.
    Costs( CostsResult ),
    /// Embeddings API usage details.
    Embeddings( UsageEmbeddingsResult ),
    /// Images API usage details.
    Images( UsageImagesResult ),
    /// Moderations API usage details.
    Moderations( UsageModerationsResult ),
    /// Vector Stores usage details.
    VectorStores( UsageVectorStoresResult ),
  }

  /// Represents a time bucket containing aggregated usage or cost results.
  ///
  /// # Used By
  /// - `UsageResponse`
  #[ derive( Debug, Deserialize, Clone, PartialEq ) ]
  #[ non_exhaustive ]
  pub struct UsageTimeBucket
  {
    /// List of time buckets containing usage/cost data.
    pub data : Vec< UsageResult >,
    /// End timestamp of the bucket (Unix seconds).
    pub end_time : i64,
    /// Indicates if more data is available for pagination.
    pub has_more : bool,
    /// Pagination cursor for the next page.
    pub next_page : Option< String >,
    /// Object type, always "bucket".
    pub object : String,
    /// Start timestamp of the bucket (Unix seconds).
    pub start_time : i64,
  }

  /// Represents the paginated response from a Usage or Costs API endpoint.
  ///
  /// # Used By
  /// - `/organization/costs` (GET)
  /// - `/organization/usage/*` (GET endpoints)
  #[ derive( Debug, Deserialize, Clone, PartialEq ) ]
  #[ non_exhaustive ]
  pub struct UsageResponse
  {
    /// List of time buckets containing usage/cost data.
    pub data : Vec< UsageTimeBucket >,
    /// Indicates if more data is available for pagination.
    pub has_more : bool,
    /// Pagination cursor for the next page.
    pub next_page : Option< String >,
    /// Object type, always "page".
    pub object : String,
  }
} // end mod private

crate ::mod_interface!
{
  exposed use
  {
    CostsResultAmount,
    CostsResult,
    UsageAudioSpeechesResult,
    UsageAudioTranscriptionsResult,
    UsageCodeInterpreterSessionsResult,
    UsageCompletionsResult,
    UsageEmbeddingsResult,
    UsageImagesResult,
    UsageModerationsResult,
    UsageVectorStoresResult,
    UsageResult,
    UsageTimeBucket,
    UsageResponse
  };
}
