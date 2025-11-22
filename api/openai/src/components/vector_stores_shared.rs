//! Structures related to Vector Stores, including files, batches, and search results.

/// Define a private namespace for all its items.
mod private
{
  use serde::{ Deserialize, Serialize };
  use serde_json::Value;
  use crate::components::common::{ Metadata, ResponseError, VectorStoreFileAttributes };

  /// Represents the expiration policy for a vector store.
  #[ derive( Debug, Deserialize, Clone, PartialEq ) ]
  #[ non_exhaustive ]
  pub struct VectorStoreExpirationAfter
  {
    /// Anchor timestamp (`last_active_at`).
    pub anchor : String,
    /// Number of days after the anchor time the store expires (1-365).
    pub days : i32,
  }

  /// Represents the counts of files in different statuses within a vector store or batch.
  #[ derive( Debug, Deserialize, Clone, PartialEq ) ]
  #[ non_exhaustive ]
  pub struct VectorStoreFileCounts
  {
    /// The number of files that were cancelled.
    pub cancelled : i32,
    /// The number of files successfully processed.
    pub completed : i32,
    /// The number of files that failed processing.
    pub failed : i32,
    /// The number of files currently being processed.
    pub in_progress : i32,
    /// The total number of files.
    pub total : i32,
  }

  /// Represents a vector store object.
  /// A vector store is a collection of processed files can be used by the `file_search` tool.
  ///
  /// # Used By
  /// - `/vector_stores` (GET - in `ListVectorStoresResponse`, POST response)
  /// - `/vector_stores/{vector_store_id}` (GET, POST response)
  /// - `ListVectorStoresResponse`
  #[ derive( Debug, Deserialize, Clone, PartialEq ) ]
  #[ non_exhaustive ]
  pub struct VectorStoreObject
  {
    /// The Unix timestamp (in seconds) for when the vector store was created.
    pub created_at : i64,
    /// The expiration policy for the vector store.
    pub expires_after : Option< VectorStoreExpirationAfter >,
    /// The Unix timestamp (in seconds) for when the vector store will expire.
    pub expires_at : Option< i64 >,
    /// Counts of files in different statuses within the vector store.
    pub file_counts : VectorStoreFileCounts,
    /// The identifier, which can be referenced in API endpoints.
    pub id : String,
    /// The Unix timestamp (in seconds) for when the vector store was last active.
    pub last_active_at : Option< i64 >,
    /// Set of 16 key-value pairs attached to the object.
    pub metadata : Option< Metadata >,
    /// The name of the vector store.
    pub name : Option< String >,
    /// The object type, which is always `vector_store`.
    pub object : String,
    /// The status of the vector store (`expired`, `in_progress`, or `completed`).
    pub status : String,
    /// The total number of bytes used by the files in the vector store.
    pub usage_bytes : i64,
  }

  /// Response containing a list of vector stores.
  ///
  /// # Used By
  /// - `/vector_stores` (GET)
  #[ derive( Debug, Deserialize, Clone, PartialEq ) ]
  #[ non_exhaustive ]
  pub struct ListVectorStoresResponse
  {
    /// A list of vector store objects.
    pub data : Vec< VectorStoreObject >,
    /// The ID of the first vector store in the list.
    pub first_id : String,
    /// Indicates whether there are more vector stores available.
    pub has_more : bool,
    /// The ID of the last vector store in the list.
    pub last_id : String,
    /// The object type, always "list".
    pub object : String,
  }

  /// Represents the static chunking strategy configuration.
  ///
  /// # Used By
  /// - `StaticChunkingStrategyResponseParam`
  /// - `StaticChunkingStrategyRequestParam`
  #[ derive( Debug, Serialize, Deserialize, Clone, PartialEq ) ]
  #[ non_exhaustive ]
  pub struct StaticChunkingStrategy
  {
    /// The number of tokens that overlap between chunks (default 400). Must not exceed half of `max_chunk_size_tokens`.
    pub chunk_overlap_tokens : i32,
    /// The maximum number of tokens in each chunk (100-4096, default 800).
    pub max_chunk_size_tokens : i32,
  }

  /// Represents the static chunking strategy as returned in responses.
  ///
  /// # Used By
  /// - `ChunkingStrategyResponse`
  #[ derive( Debug, Deserialize, Clone, PartialEq ) ]
  #[ non_exhaustive ]
  pub struct StaticChunkingStrategyResponseParam
  {
    /// The static chunking configuration details.
    pub r#static : StaticChunkingStrategy,
    /// Always `static`.
    pub r#type : String,
  }

  /// Represents an unknown or older chunking strategy in responses.
  ///
  /// # Used By
  /// - `ChunkingStrategyResponse`
  #[ derive( Debug, Deserialize, Clone, PartialEq ) ]
  #[ non_exhaustive ]
  pub struct OtherChunkingStrategyResponseParam
  {
    /// Always `other`.
    pub r#type : String,
  }

  /// Represents the chunking strategy used for a file in a vector store (response format).
  ///
  /// # Used By
  /// - `VectorStoreFileObject`
  #[ derive( Debug, Deserialize, Clone, PartialEq ) ]
  #[ serde( untagged ) ]
  #[ non_exhaustive ]
  pub enum ChunkingStrategyResponse
  {
    /// Other/unknown chunking strategy.
    Other( OtherChunkingStrategyResponseParam ),
    /// Static chunking strategy details.
    Static( StaticChunkingStrategyResponseParam ),
  }

  /// Represents a file attached to a vector store.
  ///
  /// # Used By
  /// - `/vector_stores/{vector_store_id}/files` (GET - in `ListVectorStoreFilesResponse`, POST response)
  /// - `/vector_stores/{vector_store_id}/files/{file_id}` (GET, POST response)
  /// - `ListVectorStoreFilesResponse`
  #[ derive( Debug, Deserialize, Clone, PartialEq ) ]
  #[ non_exhaustive ]
  pub struct VectorStoreFileObject
  {
    /// Attributes associated with the file.
    pub attributes : Option< VectorStoreFileAttributes >,
    /// The strategy used to chunk the file.
    pub chunking_strategy : Option< ChunkingStrategyResponse >,
    /// The Unix timestamp (in seconds) for when the vector store file was created.
    pub created_at : i64,
    /// The identifier, which can be referenced in API endpoints.
    pub id : String,
    /// The last error associated with this vector store file. Will be `null` if there are no errors.
    pub last_error : Option< ResponseError >,
    /// The object type, which is always `vector_store.file`.
    pub object : String,
    /// The status of the vector store file (`in_progress`, `completed`, `cancelled`, or `failed`).
    pub status : String,
    /// The total vector store usage in bytes. Note that this may be different from the original file size.
    pub usage_bytes : i64,
    /// The ID of the vector store that the File is attached to.
    pub vector_store_id : String,
  }

  /// Response containing a list of vector store files.
  ///
  /// # Used By
  /// - `/vector_stores/{vector_store_id}/files` (GET)
  /// - `/vector_stores/{vector_store_id}/file_batches/{batch_id}/files` (GET)
  #[ derive( Debug, Deserialize, Clone, PartialEq ) ]
  #[ non_exhaustive ]
  pub struct ListVectorStoreFilesResponse
  {
    /// A list of vector store file objects.
    pub data : Vec< VectorStoreFileObject >,
    /// The ID of the first file in the list.
    pub first_id : String,
    /// Indicates whether there are more files available.
    pub has_more : bool,
    /// The ID of the last file in the list.
    pub last_id : String,
    /// The object type, always "list".
    pub object : String,
  }

  /// Represents a batch operation for adding files to a vector store.
  ///
  /// # Used By
  /// - `/vector_stores/{vector_store_id}/file_batches` (POST response)
  /// - `/vector_stores/{vector_store_id}/file_batches/{batch_id}` (GET response)
  /// - `/vector_stores/{vector_store_id}/file_batches/{batch_id}/cancel` (POST response)
  #[ derive( Debug, Deserialize, Clone, PartialEq ) ]
  #[ non_exhaustive ]
  pub struct VectorStoreFileBatchObject
  {
    /// The Unix timestamp (in seconds) for when the vector store files batch was created.
    pub created_at : i64,
    /// Counts of files in different statuses within the batch.
    pub file_counts : VectorStoreFileCounts,
    /// The identifier, which can be referenced in API endpoints.
    pub id : String,
    /// The object type, which is always `vector_store.file_batch`.
    pub object : String,
    /// The status of the vector store files batch (`in_progress`, `completed`, `cancelled` or `failed`).
    pub status : String,
    /// The ID of the vector store that the File is attached to.
    pub vector_store_id : String,
  }

  /// Represents the auto chunking strategy parameter for requests.
  ///
  /// # Used By
  /// - `ChunkingStrategyRequestParam`
  #[ derive( Debug, Serialize, Deserialize, Clone, PartialEq ) ]
  #[ non_exhaustive ]
  pub struct AutoChunkingStrategyRequestParam
  {
    /// Always `auto`.
    pub r#type : String,
  }

  /// Represents the static chunking strategy parameter for requests.
  ///
  /// # Used By
  /// - `ChunkingStrategyRequestParam`
  #[ derive( Debug, Serialize, Deserialize, Clone, PartialEq ) ]
  #[ non_exhaustive ]
  pub struct StaticChunkingStrategyRequestParam
  {
    /// The static chunking configuration details.
    pub r#static : StaticChunkingStrategy,
    /// Always `static`.
    pub r#type : String,
  }

  /// Represents the chunking strategy parameter for requests (auto or static).
  ///
  /// # Used By
  /// - `CreateVectorStoreRequest` (within `requests/vector_stores.rs` - *assuming*)
  /// - `CreateVectorStoreFileRequest` (within `requests/vector_stores.rs` - *assuming*)
  /// - `CreateVectorStoreFileBatchRequest` (within `requests/vector_stores.rs` - *assuming*)
  #[ derive( Debug, Serialize, Deserialize, Clone, PartialEq ) ]
  #[ serde( untagged ) ]
  #[ non_exhaustive ]
  pub enum ChunkingStrategyRequestParam
  {
    /// Use the default auto chunking strategy.
    Auto( AutoChunkingStrategyRequestParam ),
    /// Use a custom static chunking strategy.
    Static( StaticChunkingStrategyRequestParam ),
  }

  /// Represents the content of a search result chunk.
  ///
  /// # Used By
  /// - `VectorStoreSearchResultItem`
  /// - `VectorStoreFileContentResponse`
  #[ derive( Debug, Deserialize, Clone, PartialEq ) ]
  #[ non_exhaustive ]
  pub struct VectorStoreSearchResultContentObject
  {
    /// The text content.
    pub text : String,
    /// The type of content (currently only "text").
    pub r#type : String,
  }

  /// Represents a single search result item from a vector store search.
  ///
  /// # Used By
  /// - `VectorStoreSearchResultsPage`
  #[ derive( Debug, Deserialize, Clone, PartialEq ) ]
  #[ non_exhaustive ]
  pub struct VectorStoreSearchResultItem
  {
    /// Attributes associated with the file.
    pub attributes : Option< VectorStoreFileAttributes >,
    /// Content chunks from the file.
    pub content : Vec< VectorStoreSearchResultContentObject >,
    /// The ID of the vector store file.
    pub file_id : String,
    /// The name of the vector store file.
    pub filename : String,
    /// The similarity score for the result (0 to 1).
    pub score : f64,
  }

  /// Represents a paginated response from a vector store search.
  ///
  /// # Used By
  /// - `/vector_stores/{vector_store_id}/search` (POST)
  #[ derive( Debug, Deserialize, Clone, PartialEq ) ]
  #[ non_exhaustive ]
  pub struct VectorStoreSearchResultsPage
  {
    /// The list of search result items.
    pub data : Vec< VectorStoreSearchResultItem >,
    /// Indicates if there are more results to fetch.
    pub has_more : bool,
    /// The token for the next page, if any.
    pub next_page : Option< String >,
    /// The object type, always `vector_store.search_results.page`.
    pub object : String,
    /// The query used for this search.
    pub search_query : Vec< String >,
  }

  /// Represents a filter comparing an attribute key to a value.
  ///
  /// # Used By
  /// - `Filter`
  #[ derive( Debug, Serialize, Deserialize, Clone, PartialEq ) ]
  #[ non_exhaustive ]
  pub struct ComparisonFilter
  {
    /// The key to compare against the value.
    pub key : String,
    /// The comparison operator (`eq`, `ne`, `gt`, `gte`, `lt`, `lte`). Defaults to `eq`.
    pub r#type : String,
    /// The value to compare against (string, number, or boolean).
    pub value : Value,
  }

  /// Represents a filter combining multiple filters using `and` or `or`.
  ///
  /// # Used By
  /// - `Filter`
  #[ non_exhaustive ]
  #[ derive( Debug, Serialize, Deserialize, Clone, PartialEq ) ]
  pub struct CompoundFilter
  {
    /// Array of filters to combine (can be `ComparisonFilter` or `CompoundFilter`).
    pub filters : Vec< Filter >,
    /// Type of operation (`and` or `or`).
    pub r#type : String,
  }

  /// Represents a filter used in vector store search requests.
  ///
  /// # Used By
  /// - `VectorStoreSearchRequest` (within `requests/vector_stores.rs` - *assuming*)
  /// - `CompoundFilter`
  #[ non_exhaustive ]
  #[ derive( Debug, Serialize, Deserialize, Clone, PartialEq ) ]
  #[ serde( untagged ) ]
  pub enum Filter
  {
    /// A simple comparison filter.
    Comparison( ComparisonFilter ),
    /// A filter combining other filters.
    Compound( CompoundFilter ),
  }

  /// Represents the response containing the parsed content of a vector store file.
  ///
  /// # Used By
  /// - `/vector_stores/{vector_store_id}/files/{file_id}/content` (GET)
  #[ non_exhaustive ]
  #[ derive( Debug, Deserialize, Clone, PartialEq ) ]
  pub struct VectorStoreFileContentResponse
  {
    /// Parsed content of the file.
    pub data : Vec< VectorStoreSearchResultContentObject >, // Reusing this as it matches the structure
    /// Indicates if there are more content pages to fetch.
    pub has_more : bool,
    /// The token for the next page, if any.
    pub next_page : Option< String >,
    /// The object type, always `vector_store.file_content.page`.
    pub object : String,
  }
}

crate ::mod_interface!
{
  exposed use
  {
    AutoChunkingStrategyRequestParam,
    ChunkingStrategyResponse,
    ChunkingStrategyRequestParam,
    ComparisonFilter,
    CompoundFilter,
    Filter,
    ListVectorStoreFilesResponse,
    ListVectorStoresResponse,
    OtherChunkingStrategyResponseParam,
    StaticChunkingStrategy,
    StaticChunkingStrategyRequestParam,
    StaticChunkingStrategyResponseParam,
    VectorStoreExpirationAfter,
    VectorStoreFileBatchObject,
    VectorStoreFileContentResponse,
    VectorStoreFileCounts,
    VectorStoreFileObject,
    VectorStoreObject,
    VectorStoreSearchResultContentObject,
    VectorStoreSearchResultItem,
    VectorStoreSearchResultsPage
  };
}