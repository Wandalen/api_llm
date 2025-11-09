//! Cached Content functionality for the Ollama API client
//!
//! This module provides advanced content caching capabilities including:
//! - Intelligent content caching for repeated queries
//! - Cache invalidation and management operations
//! - Performance optimization through content-aware caching
//! - Memory-efficient storage with intelligent eviction
//!
//! All functionality follows the "Thin Client, Rich API" governing principle,
//! providing explicit control with transparent cache management operations.

use serde::{ Serialize, Deserialize };
use std::collections::HashMap;
use std::time::{ Duration, Instant };

/// Request structure for caching content operations
#[ derive( Debug, Clone, Serialize, Deserialize ) ]
pub struct CachedContentRequest
{
  /// Unique identifier for the content to cache
  pub content_id : String,
  /// Model name associated with this content
  pub model : String,
  /// Content to be cached
  pub content : String,
  /// Content type for intelligent caching decisions
  pub content_type : ContentType,
  /// Custom cache TTL for this content
  pub ttl : Option< Duration >,
  /// Cache priority level
  pub priority : CachePriority,
  /// Additional metadata for cache optimization
  pub metadata : Option< serde_json::Value >,
}

/// Response structure for cached content operations
#[ derive( Debug, Clone, Serialize, Deserialize ) ]
pub struct CachedContentResponse
{
  /// Unique identifier for the cached content
  pub content_id : String,
  /// Cached content data
  pub content : String,
  /// Model name associated with the content
  pub model : String,
  /// Content type
  pub content_type : ContentType,
  /// When the content was cached
  pub cached_at : u64, // Unix timestamp
  /// When the content expires (if applicable)
  pub expires_at : Option< u64 >,
  /// Number of times this content has been accessed
  pub access_count : u64,
  /// Cache hit performance metrics
  pub performance_metrics : Option< CachePerformanceMetrics >,
  /// Additional cached metadata
  pub metadata : Option< serde_json::Value >,
}

/// Content types for intelligent caching decisions
#[ derive( Debug, Clone, PartialEq, Serialize, Deserialize ) ]
pub enum ContentType
{
  /// Chat conversation content
  ChatConversation,
  /// Model response content
  ModelResponse,
  /// System instruction content
  SystemInstruction,
  /// User prompt content
  UserPrompt,
  /// Function call result content
  FunctionCallResult,
  /// Embeddings data content
  EmbeddingsData,
  /// Generated code content
  GeneratedCode,
  /// Document or text content
  DocumentContent,
  /// Custom content type
  Custom( String ),
}

/// Cache priority levels for intelligent management
#[ derive( Debug, Clone, PartialEq, Serialize, Deserialize ) ]
pub enum CachePriority
{
  /// Low priority - first to be evicted
  Low,
  /// Normal priority - standard caching behavior
  Normal,
  /// High priority - kept longer in cache
  High,
  /// Critical priority - only evicted when absolutely necessary
  Critical,
}

/// Configuration for content caching behavior
#[ derive( Debug, Clone, Serialize, Deserialize ) ]
pub struct ContentCacheConfig
{
  /// Maximum number of cached content items
  max_content_items : usize,
  /// Default TTL for cached content
  default_ttl : Duration,
  /// Maximum size per content item (bytes)
  max_content_size : usize,
  /// Total memory limit for cache (bytes)
  memory_limit : usize,
  /// Enable intelligent cache management
  intelligent_management : bool,
  /// Auto-cleanup interval
  cleanup_interval : Duration,
  /// Performance tracking enabled
  performance_tracking : bool,
}

/// Request structure for cache invalidation operations
#[ derive( Debug, Clone, Serialize, Deserialize ) ]
pub struct CacheInvalidationRequest
{
  /// Specific content IDs to invalidate (if any)
  pub content_ids : Option< Vec< String > >,
  /// Model name to invalidate all content for (if any)
  pub model : Option< String >,
  /// Content type to invalidate (if any)
  pub content_type : Option< ContentType >,
  /// Invalidate content older than this timestamp
  pub older_than : Option< u64 >,
  /// Priority levels to invalidate
  pub priorities : Option< Vec< CachePriority > >,
  /// Force invalidation (ignore critical priority)
  pub force : bool,
}

/// Response structure for cache invalidation operations
#[ derive( Debug, Clone, Serialize, Deserialize ) ]
pub struct CacheInvalidationResponse
{
  /// Number of content items invalidated
  pub invalidated_count : u64,
  /// Content IDs that were invalidated
  pub invalidated_ids : Vec< String >,
  /// Total memory freed (bytes)
  pub memory_freed : usize,
  /// Operation duration in milliseconds
  pub duration_ms : u64,
  /// Whether all requested invalidations succeeded
  pub success : bool,
  /// Error messages (if any)
  pub errors : Vec< String >,
}

/// Performance metrics for cache operations
#[ derive( Debug, Clone, Serialize, Deserialize ) ]
pub struct CachePerformanceMetrics
{
  /// Cache hit ratio (0.0 to 1.0)
  pub hit_ratio : f64,
  /// Average retrieval time in microseconds
  pub avg_retrieval_time_us : u64,
  /// Average storage time in microseconds
  pub avg_storage_time_us : u64,
  /// Memory usage efficiency (0.0 to 1.0)
  pub memory_efficiency : f64,
  /// Cache pressure level (0.0 to 1.0)
  pub cache_pressure : f64,
  /// Number of evictions in current session
  pub evictions : u64,
}

/// Intelligent cache manager for advanced caching strategies
#[ derive( Debug, Clone ) ]
pub struct IntelligentCacheManager
{
  config : ContentCacheConfig,
  content_store : HashMap<  String, CachedContentResponse  >,
  access_patterns : HashMap<  String, AccessPattern  >,
  performance_metrics : CachePerformanceMetrics,
  #[ allow(dead_code) ]
  last_cleanup : Instant,
}

/// Access pattern tracking for intelligent cache decisions
#[ derive( Debug, Clone ) ]
struct AccessPattern
{
  access_count : u64,
  last_accessed : Instant,
  access_frequency : f64, // Accesses per hour
  #[ allow(dead_code) ]
  content_type : ContentType,
  priority : CachePriority,
}

impl CachedContentRequest
{
  /// Create a new cached content request
  #[ inline ]
  #[ must_use ]
  pub fn new( content_id : String, model : String, content : String, content_type : ContentType ) -> Self
  {
    Self
    {
      content_id,
      model,
      content,
      content_type,
      ttl : None,
      priority : CachePriority::Normal,
      metadata : None,
    }
  }

  /// Set the cache TTL
  #[ inline ]
  #[ must_use ]
  pub fn with_ttl( mut self, ttl : Duration ) -> Self
  {
    self.ttl = Some( ttl );
    self
  }

  /// Set the cache priority
  #[ inline ]
  #[ must_use ]
  pub fn with_priority( mut self, priority : CachePriority ) -> Self
  {
    self.priority = priority;
    self
  }

  /// Set metadata
  #[ inline ]
  #[ must_use ]
  pub fn with_metadata( mut self, metadata : serde_json::Value ) -> Self
  {
    self.metadata = Some( metadata );
    self
  }

  /// Estimate content size in bytes
  #[ inline ]
  #[ must_use ]
  pub fn estimate_size( &self ) -> usize
  {
    self.content_id.len() +
    self.model.len() +
    self.content.len() +
    std ::mem::size_of::< ContentType >() +
    std ::mem::size_of::< CachePriority >() +
    self.metadata.as_ref().map_or( 0, | m | m.to_string().len() )
  }
}

impl ContentType
{
  /// Get the default TTL for this content type
  #[ inline ]
  #[ must_use ]
  pub fn default_ttl( &self ) -> Duration
  {
    match self
    {
      ContentType::ChatConversation => Duration::from_secs( 3600 ), // 1 hour
      ContentType::ModelResponse => Duration::from_secs( 1800 ), // 30 minutes
      ContentType::SystemInstruction => Duration::from_secs( 7200 ), // 2 hours
      ContentType::UserPrompt => Duration::from_secs( 1200 ), // 20 minutes
      ContentType::FunctionCallResult => Duration::from_secs( 900 ), // 15 minutes
      ContentType::EmbeddingsData => Duration::from_secs( 14400 ), // 4 hours
      ContentType::GeneratedCode => Duration::from_secs( 2400 ), // 40 minutes
      ContentType::DocumentContent => Duration::from_secs( 10800 ), // 3 hours
      ContentType::Custom( _ ) => Duration::from_secs( 1800 ), // 30 minutes default
    }
  }

  /// Get the cache priority for this content type
  #[ inline ]
  #[ must_use ]
  pub fn default_priority( &self ) -> CachePriority
  {
    match self
    {
      ContentType::SystemInstruction => CachePriority::High,
      ContentType::EmbeddingsData => CachePriority::High,
      ContentType::DocumentContent => CachePriority::Normal,
      ContentType::GeneratedCode => CachePriority::Normal,
      ContentType::ChatConversation => CachePriority::Normal,
      ContentType::ModelResponse => CachePriority::Low,
      ContentType::UserPrompt => CachePriority::Low,
      ContentType::FunctionCallResult => CachePriority::Normal,
      ContentType::Custom( _ ) => CachePriority::Normal,
    }
  }
}

impl CachePriority
{
  /// Get the eviction weight (lower values are evicted first)
  #[ inline ]
  #[ must_use ]
  pub fn eviction_weight( &self ) -> u32
  {
    match self
    {
      CachePriority::Low => 1,
      CachePriority::Normal => 10,
      CachePriority::High => 100,
      CachePriority::Critical => 1000,
    }
  }
}

impl ContentCacheConfig
{
  /// Create a new content cache configuration with defaults
  #[ inline ]
  #[ must_use ]
  pub fn new() -> Self
  {
    Self
    {
      max_content_items : 1000,
      default_ttl : Duration::from_secs( 1800 ), // 30 minutes
      max_content_size : 1024 * 1024, // 1MB per item
      memory_limit : 100 * 1024 * 1024, // 100MB total
      intelligent_management : true,
      cleanup_interval : Duration::from_secs( 300 ), // 5 minutes
      performance_tracking : true,
    }
  }

  /// Set maximum content items
  #[ inline ]
  #[ must_use ]
  pub fn with_max_items( mut self, max_items : usize ) -> Self
  {
    self.max_content_items = max_items;
    self
  }

  /// Set default TTL
  #[ inline ]
  #[ must_use ]
  pub fn with_default_ttl( mut self, ttl : Duration ) -> Self
  {
    self.default_ttl = ttl;
    self
  }

  /// Set memory limit
  #[ inline ]
  #[ must_use ]
  pub fn with_memory_limit( mut self, limit : usize ) -> Self
  {
    self.memory_limit = limit;
    self
  }

  /// Enable or disable intelligent management
  #[ inline ]
  #[ must_use ]
  pub fn with_intelligent_management( mut self, enabled : bool ) -> Self
  {
    self.intelligent_management = enabled;
    self
  }

  /// Set cleanup interval
  #[ inline ]
  #[ must_use ]
  pub fn with_cleanup_interval( mut self, interval : Duration ) -> Self
  {
    self.cleanup_interval = interval;
    self
  }

  /// Getters for configuration values
  #[ inline ]
  #[ must_use ]
  pub fn max_content_items( &self ) -> usize { self.max_content_items }

  /// Get the default TTL for cached content
  #[ inline ]
  #[ must_use ]
  pub fn default_ttl( &self ) -> Duration { self.default_ttl }

  /// Get the maximum content size for cached items
  #[ inline ]
  #[ must_use ]
  pub fn max_content_size( &self ) -> usize { self.max_content_size }

  /// Get the memory limit for the cache
  #[ inline ]
  #[ must_use ]
  pub fn memory_limit( &self ) -> usize { self.memory_limit }

  /// Check if intelligent cache management is enabled
  #[ inline ]
  #[ must_use ]
  pub fn intelligent_management( &self ) -> bool { self.intelligent_management }

  /// Get the cleanup interval for the cache
  #[ inline ]
  #[ must_use ]
  pub fn cleanup_interval( &self ) -> Duration { self.cleanup_interval }

  /// Check if performance tracking is enabled
  #[ inline ]
  #[ must_use ]
  pub fn performance_tracking( &self ) -> bool { self.performance_tracking }
}

impl Default for ContentCacheConfig
{
  fn default() -> Self
  {
    Self::new()
  }
}

impl IntelligentCacheManager
{
  /// Create a new intelligent cache manager
  #[ inline ]
  #[ must_use ]
  pub fn new( config : ContentCacheConfig ) -> Self
  {
    Self
    {
      config,
      content_store : HashMap::new(),
      access_patterns : HashMap::new(),
      performance_metrics : CachePerformanceMetrics
      {
        hit_ratio : 0.0,
        avg_retrieval_time_us : 0,
        avg_storage_time_us : 0,
        memory_efficiency : 1.0,
        cache_pressure : 0.0,
        evictions : 0,
      },
      last_cleanup : Instant::now(),
    }
  }

  /// Store content in the cache
  #[ inline ]
  pub fn store_content( &mut self, request : CachedContentRequest ) -> Result< (), String >
  {
    let start_time = Instant::now();

    // Check content size limits
    let content_size = request.estimate_size();
    if content_size > self.config.max_content_size
    {
      return Err( format!( "Content size {} exceeds limit {}", content_size, self.config.max_content_size ) );
    }

    // Check if we need to make room
    if self.needs_eviction( content_size )
    {
      self.intelligent_eviction( content_size )?;
    }

    // Create cached content response
    let cached_content = CachedContentResponse
    {
      content_id : request.content_id.clone(),
      content : request.content,
      model : request.model,
      content_type : request.content_type.clone(),
      cached_at : std::time::SystemTime::now().duration_since( std::time::UNIX_EPOCH ).unwrap().as_secs(),
      expires_at : request.ttl.map( | ttl |
        std ::time::SystemTime::now().duration_since( std::time::UNIX_EPOCH ).unwrap().as_secs() + ttl.as_secs()
      ),
      access_count : 0,
      performance_metrics : None,
      metadata : request.metadata,
    };

    // Store content
    self.content_store.insert( request.content_id.clone(), cached_content );

    // Update access patterns
    self.access_patterns.insert( request.content_id, AccessPattern
    {
      access_count : 0,
      last_accessed : Instant::now(),
      access_frequency : 0.0,
      content_type : request.content_type,
      priority : request.priority,
    } );

    // Update performance metrics
    self.performance_metrics.avg_storage_time_us = start_time.elapsed().as_micros() as u64;

    Ok( () )
  }

  /// Retrieve content from the cache
  #[ inline ]
  pub fn retrieve_content( &mut self, content_id : &str ) -> Option< CachedContentResponse >
  {
    let start_time = Instant::now();

    if let Some( mut content ) = self.content_store.get( content_id ).cloned()
    {
      // Check if content has expired
      if let Some( expires_at ) = content.expires_at
      {
        let now = std::time::SystemTime::now().duration_since( std::time::UNIX_EPOCH ).unwrap().as_secs();
        if now > expires_at
        {
          // Content expired, remove it
          self.content_store.remove( content_id );
          self.access_patterns.remove( content_id );
          return None;
        }
      }

      // Update access patterns
      if let Some( pattern ) = self.access_patterns.get_mut( content_id )
      {
        pattern.access_count += 1;
        pattern.last_accessed = Instant::now();

        // Calculate access frequency (accesses per hour)
        let hours_since_first_access = pattern.last_accessed.duration_since( Instant::now() - Duration::from_secs( 3600 ) ).as_secs_f64() / 3600.0;
        if hours_since_first_access > 0.0
        {
          pattern.access_frequency = pattern.access_count as f64 / hours_since_first_access;
        }
      }

      // Update content access count
      content.access_count += 1;
      self.content_store.insert( content_id.to_string(), content.clone() );

      // Update performance metrics
      self.performance_metrics.avg_retrieval_time_us = start_time.elapsed().as_micros() as u64;

      Some( content )
    }
    else
    {
      None
    }
  }

  /// Invalidate content based on criteria
  #[ inline ]
  pub fn invalidate_content( &mut self, request : CacheInvalidationRequest ) -> CacheInvalidationResponse
  {
    let start_time = Instant::now();
    let mut invalidated_ids = Vec::new();
    let mut memory_freed = 0usize;
    let mut errors = Vec::new();

    // Collect IDs to invalidate
    let mut ids_to_remove = Vec::new();

    for ( content_id, content ) in &self.content_store
    {
      let mut should_invalidate = false;

      // Check specific content IDs
      if let Some( ref target_ids ) = request.content_ids
      {
        if target_ids.contains( content_id )
        {
          should_invalidate = true;
        }
      }

      // Check model name
      if let Some( ref target_model ) = request.model
      {
        if content.model == *target_model
        {
          should_invalidate = true;
        }
      }

      // Check content type
      if let Some( ref target_type ) = request.content_type
      {
        if content.content_type == *target_type
        {
          should_invalidate = true;
        }
      }

      // Check age
      if let Some( older_than ) = request.older_than
      {
        if content.cached_at < older_than
        {
          should_invalidate = true;
        }
      }

      // Check priority levels
      if let Some( ref target_priorities ) = request.priorities
      {
        if let Some( pattern ) = self.access_patterns.get( content_id )
        {
          if target_priorities.contains( &pattern.priority )
          {
            should_invalidate = true;
          }
        }
      }

      // Check force flag and critical priority
      if should_invalidate
      {
        if let Some( pattern ) = self.access_patterns.get( content_id )
        {
          if pattern.priority == CachePriority::Critical && !request.force
          {
            should_invalidate = false;
            errors.push( format!( "Cannot invalidate critical content '{}' without force flag", content_id ) );
          }
        }
      }

      if should_invalidate
      {
        ids_to_remove.push( content_id.clone() );
        memory_freed += content.content.len() + content_id.len() + content.model.len();
      }
    }

    // Remove invalidated content
    for content_id in &ids_to_remove
    {
      self.content_store.remove( content_id );
      self.access_patterns.remove( content_id );
      invalidated_ids.push( content_id.clone() );
    }

    CacheInvalidationResponse
    {
      invalidated_count : invalidated_ids.len() as u64,
      invalidated_ids,
      memory_freed,
      duration_ms : start_time.elapsed().as_millis() as u64,
      success : errors.is_empty(),
      errors,
    }
  }

  /// Get current performance metrics
  #[ inline ]
  #[ must_use ]
  pub fn performance_metrics( &self ) -> &CachePerformanceMetrics
  {
    &self.performance_metrics
  }

  /// Get cache utilization information
  #[ inline ]
  #[ must_use ]
  pub fn cache_utilization( &self ) -> ( usize, usize, f64 )
  {
    let current_items = self.content_store.len();
    let max_items = self.config.max_content_items;
    let utilization = current_items as f64 / max_items as f64;
    ( current_items, max_items, utilization )
  }

  /// Check if eviction is needed
  #[ inline ]
  #[ must_use ]
  fn needs_eviction( &self, additional_size : usize ) -> bool
  {
    let current_items = self.content_store.len();
    let current_memory = self.estimate_memory_usage();

    current_items >= self.config.max_content_items ||
    current_memory + additional_size > self.config.memory_limit
  }

  /// Perform intelligent eviction
  #[ inline ]
  fn intelligent_eviction( &mut self, space_needed : usize ) -> Result< (), String >
  {
    let mut eviction_candidates = Vec::new();

    // Score each item for eviction (higher score = more likely to evict)
    for ( content_id, _content ) in &self.content_store
    {
      if let Some( pattern ) = self.access_patterns.get( content_id )
      {
        let age_factor = pattern.last_accessed.elapsed().as_secs() as f64 / 3600.0; // Hours since last access
        let frequency_factor = 1.0 / ( pattern.access_frequency + 1.0 ); // Lower frequency = higher score
        let priority_factor = 1.0 / pattern.priority.eviction_weight() as f64; // Lower priority = higher score

        let eviction_score = age_factor * frequency_factor * priority_factor;

        eviction_candidates.push( ( content_id.clone(), eviction_score ) );
      }
    }

    // Sort by eviction score (highest first)
    eviction_candidates.sort_by( | a, b | b.1.partial_cmp( &a.1 ).unwrap() );

    // Evict items until we have enough space
    let mut freed_space = 0usize;
    let mut evicted_count = 0usize;

    for ( content_id, _score ) in eviction_candidates
    {
      if let Some( content ) = self.content_store.get( &content_id )
      {
        freed_space += content.content.len() + content_id.len() + content.model.len();
        self.content_store.remove( &content_id );
        self.access_patterns.remove( &content_id );
        evicted_count += 1;

        if freed_space >= space_needed
        {
          break;
        }
      }
    }

    // Update performance metrics
    self.performance_metrics.evictions += evicted_count as u64;

    if freed_space >= space_needed
    {
      Ok( () )
    }
    else
    {
      Err( format!( "Could not free enough space : needed {}, freed {}", space_needed, freed_space ) )
    }
  }

  /// Estimate current memory usage
  #[ inline ]
  #[ must_use ]
  fn estimate_memory_usage( &self ) -> usize
  {
    self.content_store.iter().map( | ( id, content ) |
      id.len() + content.content.len() + content.model.len() +
      std ::mem::size_of::< CachedContentResponse >()
    ).sum()
  }
}