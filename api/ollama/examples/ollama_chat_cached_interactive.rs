//! Interactive chat example demonstrating request caching for performance optimization
//!
//! This example shows how to use request caching to reduce latency and improve throughput
//! by caching identical requests. The conversation demonstrates cache hits when you ask
//! the same or similar questions multiple times.
//!
//! # Usage
//!
//! ```bash
//! # Ensure Ollama is running locally with llama3.2:3b model
//! cargo run --example ollama_chat_cached_interactive --all-features
//! ```
//!
//! # Commands
//!
//! - Type your message and press Enter to chat
//! - `/quit` or `/exit` - End conversation and show cache summary
//! - `/clear` - Clear conversation and cache
//! - `/help` - Show available commands
//! - `/history` - Display conversation history
//! - `/cache` - Show detailed cache statistics
//! - `/perf` - Show performance metrics
//!
//! # How Caching Works
//!
//! Request caching in Ollama works at the HTTP request level:
//! - **Cache Miss**: First time making a request (slower, creates cache entry)
//! - **Cache Hit**: Identical request found in cache (faster, reads from cache)
//! - **Cache Eviction**: Oldest entries removed when cache is full
//!
//! The cache uses request content as the key, so identical prompts will hit the cache.
//! This is most effective for:
//! - Repeated questions (FAQ-style interactions)
//! - Deterministic responses (temperature = 0)
//! - Multi-user scenarios where users ask similar questions
//!
//! **Complexity**: ⭐⭐⭐⭐ (Advanced)

#![ allow( clippy::std_instead_of_core ) ] // std required for Instant which isn't in core

#[ cfg( feature = "streaming" ) ]
use futures_util::StreamExt;
use api_ollama::{ OllamaClient, ChatRequest, ChatMessage, MessageRole, RequestCacheConfig };
use std::io::{ self, Write as IoWrite };
use std::collections::HashMap;
use std::time::{ Duration, Instant };

/// Cache and performance statistics tracker
#[ derive( Default ) ]
struct SessionStats
{
  total_requests : usize,
  cached_requests : usize,
  uncached_requests : usize,
  cache_response_times : Vec< Duration >,
  uncached_response_times : Vec< Duration >,
  cache_hits : u64,
  cache_misses : u64,
  cache_evictions : u64,
}

impl SessionStats
{
  /// Record a request with its response time and cache status
  fn record_request( &mut self, response_time : Duration, was_cached : bool )
  {
    self.total_requests += 1;

    if was_cached
    {
      self.cached_requests += 1;
      self.cache_response_times.push( response_time );
    }
    else
    {
      self.uncached_requests += 1;
      self.uncached_response_times.push( response_time );
    }
  }

  /// Update cache statistics from client
  fn update_cache_stats( &mut self, hits : u64, misses : u64, evictions : u64 )
  {
    self.cache_hits = hits;
    self.cache_misses = misses;
    self.cache_evictions = evictions;
  }

  /// Calculate average response time for cached requests
  fn avg_cached_time( &self ) -> Duration
  {
    if self.cache_response_times.is_empty()
    {
      return Duration::ZERO;
    }

    let total : Duration = self.cache_response_times.iter().sum();
    #[ allow( clippy::cast_possible_truncation ) ] // Response count won't exceed u32 in practice
    let len = self.cache_response_times.len() as u32;
    total / len
  }

  /// Calculate average response time for uncached requests
  fn avg_uncached_time( &self ) -> Duration
  {
    if self.uncached_response_times.is_empty()
    {
      return Duration::ZERO;
    }

    let total : Duration = self.uncached_response_times.iter().sum();
    #[ allow( clippy::cast_possible_truncation ) ] // Response count won't exceed u32 in practice
    let len = self.uncached_response_times.len() as u32;
    total / len
  }

  /// Calculate cache hit rate as percentage
  fn cache_hit_rate( &self ) -> f64
  {
    let total = self.cache_hits + self.cache_misses;
    if total == 0
    {
      return 0.0;
    }
    ( self.cache_hits as f64 / total as f64 ) * 100.0
  }

  /// Calculate performance improvement from caching
  fn performance_improvement( &self ) -> f64
  {
    let uncached = self.avg_uncached_time().as_secs_f64();
    if uncached == 0.0
    {
      return 0.0;
    }

    let cached = self.avg_cached_time().as_secs_f64();
    let improvement = ( ( uncached - cached ) / uncached ) * 100.0;
    improvement.max( 0.0 )
  }

  /// Print summary statistics
  fn print_summary( &self )
  {
    println!( "\n📊 Cache Statistics Summary" );
    println!( "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━" );
    println!( "Total Requests :           {}", self.total_requests );
    println!( "\nCache Performance:" );
    println!( "  Cache Hits :             {:>8}", self.cache_hits );
    println!( "  Cache Misses :           {:>8}", self.cache_misses );
    println!( "  Cache Evictions :        {:>8}", self.cache_evictions );
    println!( "  Hit Rate :               {:>7.1}%", self.cache_hit_rate() );

    if !self.cache_response_times.is_empty() || !self.uncached_response_times.is_empty()
    {
      println!( "\nPerformance Metrics:" );
      if !self.uncached_response_times.is_empty()
      {
        println!( "  Uncached Avg Time :      {:>7.2}s", self.avg_uncached_time().as_secs_f64() );
      }
      if !self.cache_response_times.is_empty()
      {
        println!( "  Cached Avg Time :        {:>7.2}s", self.avg_cached_time().as_secs_f64() );
      }
      if !self.cache_response_times.is_empty() && !self.uncached_response_times.is_empty()
      {
        println!( "  ⚡ Speed Improvement :    {:>7.1}%", self.performance_improvement() );
      }
    }
    println!( "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n" );
  }

  /// Print detailed statistics
  fn print_detailed( &self )
  {
    println!( "\n📈 Detailed Cache Statistics" );
    println!( "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━" );
    println!( "Request Breakdown:" );
    println!( "  Total Requests :         {}", self.total_requests );
    println!( "  Cached Requests :        {}", self.cached_requests );
    println!( "  Uncached Requests :      {}", self.uncached_requests );

    println!( "\nCache Statistics:" );
    println!( "  Hits :                   {}", self.cache_hits );
    println!( "  Misses :                 {}", self.cache_misses );
    println!( "  Evictions :              {}", self.cache_evictions );
    println!( "  Hit Rate :               {:.1}%", self.cache_hit_rate() );

    if !self.cache_response_times.is_empty()
    {
      println!( "\nCached Response Times:" );
      println!( "  Count :                  {}", self.cache_response_times.len() );
      println!( "  Average :                {:.2}s", self.avg_cached_time().as_secs_f64() );
      if let Some( min ) = self.cache_response_times.iter().min()
      {
        println!( "  Minimum :                {:.2}s", min.as_secs_f64() );
      }
      if let Some( max ) = self.cache_response_times.iter().max()
      {
        println!( "  Maximum :                {:.2}s", max.as_secs_f64() );
      }
    }

    if !self.uncached_response_times.is_empty()
    {
      println!( "\nUncached Response Times:" );
      println!( "  Count :                  {}", self.uncached_response_times.len() );
      println!( "  Average :                {:.2}s", self.avg_uncached_time().as_secs_f64() );
      if let Some( min ) = self.uncached_response_times.iter().min()
      {
        println!( "  Minimum :                {:.2}s", min.as_secs_f64() );
      }
      if let Some( max ) = self.uncached_response_times.iter().max()
      {
        println!( "  Maximum :                {:.2}s", max.as_secs_f64() );
      }
    }

    if !self.cache_response_times.is_empty() && !self.uncached_response_times.is_empty()
    {
      println!( "\nPerformance Comparison:" );
      println!( "  Speed Improvement :      {:.1}%", self.performance_improvement() );
      println!( "  Time Saved (avg):       {:.2}s",
        self.avg_uncached_time().as_secs_f64() - self.avg_cached_time().as_secs_f64()
      );
    }
    println!( "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n" );
  }
}

fn print_help()
{
  println!( "\n📖 Available Commands:" );
  println!( "  /quit, /exit  - End conversation and show cache summary" );
  println!( "  /clear        - Clear conversation and cache" );
  println!( "  /help         - Show this help message" );
  println!( "  /history      - Display conversation history" );
  println!( "  /cache        - Show detailed cache statistics" );
  println!( "  /perf         - Show performance metrics" );
  println!();
}

#[ tokio::main ]
#[ allow( clippy::too_many_lines ) ]
async fn main() -> Result< (), Box< dyn core::error::Error > >
{
  println!( "💬 Ollama Cached Interactive Chat" );
  println!( "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━" );
  println!( "Using request caching for performance optimization" );
  println!( "Type /help for available commands\n" );

  // Fix(issue-eof-before-server-003): Check for EOF BEFORE setting up client
  // Root cause: EOF check was inside main loop after client setup, so empty stdin caused issues
  // Pitfall: Interactive examples should handle "no input" gracefully before initializing resources

  // Check for input availability before setting up client and resources
  print!( "\n > " );
  io ::stdout().flush()?;

  let mut first_input = String::new();
  let bytes_read = match io::stdin().read_line( &mut first_input )
  {
    Ok( n ) => n,
    Err( e ) =>
    {
      println!( "\nError reading input : {e}" );
      return Ok( () );
    }
  };

  // Handle EOF immediately - no resources needed if there's no input
  if bytes_read == 0
  {
    println!( "\n👋 No input available (EOF). Exiting gracefully." );
    println!( "Note : Use this example in interactive terminal only." );
    println!( "Run : cargo run --example ollama_chat_cached_interactive --all-features" );
    return Ok( () );
  }

  let first_input = first_input.trim();

  // Handle quit/exit on first input
  if first_input == "/quit" || first_input == "/exit"
  {
    println!( "👋 Goodbye!" );
    return Ok( () );
  }

  // Handle help on first input
  if first_input == "/help"
  {
    print_help();
    return Ok( () );
  }

  // Skip if first input is empty
  if first_input.is_empty()
  {
    println!( "👋 No input provided. Exiting." );
    return Ok( () );
  }

  // NOW set up resources (only after confirming we have input)
  // Initialize cache configuration with explicit settings
  // Following "Thin Client, Rich API" principle - all cache behavior is explicit
  let cache_config = RequestCacheConfig::new()
    .with_max_entries( 100 )
    .with_default_ttl( Duration::from_secs( 300 ) ) // 5 minute cache TTL
    .with_cleanup_interval( Duration::from_secs( 60 ) ); // Cleanup every minute

  // Create Ollama client with caching enabled
  let mut client = OllamaClient::new(
    "http://localhost:11434".to_string(),
    OllamaClient::recommended_timeout_default()
  ).with_request_cache( cache_config );

  // Verify cache is enabled
  if client.has_cache()
  {
    println!( "✅ Cache enabled (max : 100 entries, TTL: 5 minutes)\n" );
  }
  else
  {
    eprintln!( "Warning : Cache initialization failed" );
  }

  let mut conversation_history : Vec< ChatMessage > = Vec::new();
  let mut stats = SessionStats::default();

  // Configure Ollama parameters for deterministic responses
  // Lower temperature increases cache hit probability
  let mut options = HashMap::new();
  options.insert( "temperature".to_string(), serde_json::json!( 0.3 ) ); // Low temperature for consistency
  options.insert( "top_k".to_string(), serde_json::json!( 20 ) );
  options.insert( "top_p".to_string(), serde_json::json!( 0.8 ) );
  options.insert( "num_predict".to_string(), serde_json::json!( 512 ) );

  // Process first input (already validated as non-empty, non-quit, non-help)
  // Add user message to conversation history
  conversation_history.push( ChatMessage
  {
    role : MessageRole::User,
    content : first_input.to_string(),
    #[ cfg( feature = "vision_support" ) ]
    images : None,
    #[ cfg( feature = "tool_calling" ) ]
    tool_calls : None,
  });

  // Get cache stats before request to determine if this will be a cache hit
  let stats_before = client.cache_stats();

  // Create chat request
  #[ cfg( feature = "streaming" ) ]
  let request = ChatRequest
  {
    model : "llama3.2:3b".to_string(),
    messages : conversation_history.clone(),
    stream : Some( true ),
    options : Some( serde_json::Value::Object( options.clone().into_iter().collect() ) ),
    #[ cfg( feature = "tool_calling" ) ]
    tools : None,
    #[ cfg( feature = "tool_calling" ) ]
    tool_messages : None,
  };

  print!( "\n🤖 AI: " );
  io ::stdout().flush()?;

  // Track response time
  let start_time = Instant::now();

  // Use streaming if available
  #[ cfg( feature = "streaming" ) ]
  {
    match client.chat_stream( request.clone() ).await
    {
      Ok( mut stream ) =>
      {
        let mut full_response = String::new();

        while let Some( chunk ) = stream.next().await
        {
          match chunk
          {
            Ok( streaming_response ) =>
            {
              if !streaming_response.message.content.is_empty()
              {
                print!( "{}", streaming_response.message.content );
                io ::stdout().flush()?;
                full_response.push_str( &streaming_response.message.content );
              }

              if streaming_response.done
              {
                break;
              }
            }
            Err( e ) =>
            {
              println!( "\nStreaming error : {e}" );
              break;
            }
          }
        }

        let response_time = start_time.elapsed();
        println!( "\n" );

        // Get cache stats after request
        let stats_after = client.cache_stats();
        let was_cache_hit = stats_after.hits > stats_before.hits;

        // Update session statistics
        stats.record_request( response_time, was_cache_hit );
        stats.update_cache_stats( stats_after.hits, stats_after.misses, stats_after.evictions );

        // Show cache status
        if was_cache_hit
        {
          println!( "⚡ Cache hit! (responded in {:.2}s)", response_time.as_secs_f64() );
        }
        else
        {
          println!( "💾 Cache miss - response cached (took {:.2}s)", response_time.as_secs_f64() );
        }

        // Add AI response to conversation history
        if !full_response.is_empty()
        {
          conversation_history.push( ChatMessage
          {
            role : MessageRole::Assistant,
            content : full_response,
            #[ cfg( feature = "vision_support" ) ]
            images : None,
            #[ cfg( feature = "tool_calling" ) ]
            tool_calls : None,
          });
        }
      }
      Err( e ) =>
      {
        println!( "Error : {e}" );
        println!( "Please try again or type /quit to exit.\n" );
        conversation_history.pop(); // Remove failed user message
      }
    }
  }

  // Fallback for non-streaming builds
  #[ cfg( not( feature = "streaming" ) ) ]
  {
    let non_streaming_request = ChatRequest
    {
      model : "llama3.2:3b".to_string(),
      messages : conversation_history.clone(),
      stream : Some( false ),
      options : Some( serde_json::Value::Object( options.clone().into_iter().collect() ) ),
      #[ cfg( feature = "tool_calling" ) ]
      tools : None,
      #[ cfg( feature = "tool_calling" ) ]
      tool_messages : None,
    };

    match client.chat( non_streaming_request ).await
    {
      Ok( response ) =>
      {
        let response_time = start_time.elapsed();

        if !response.message.content.is_empty()
        {
          // Simulate streaming display
          let words : Vec< &str > = response.message.content.split_whitespace().collect();
          for ( i, word ) in words.iter().enumerate()
          {
            print!( "{}", word );
            if i < words.len() - 1
            {
              print!( " " );
            }
            io ::stdout().flush()?;
            tokio ::time::sleep( tokio::time::Duration::from_millis( 50 ) ).await;
          }
          println!( "\n" );

          // Get cache stats after request
          let stats_after = client.cache_stats();
          let was_cache_hit = stats_after.hits > stats_before.hits;

          // Update session statistics
          stats.record_request( response_time, was_cache_hit );
          stats.update_cache_stats( stats_after.hits, stats_after.misses, stats_after.evictions );

          // Show cache status
          if was_cache_hit
          {
            println!( "⚡ Cache hit! (responded in {:.2}s)", response_time.as_secs_f64() );
          }
          else
          {
            println!( "💾 Cache miss - response cached (took {:.2}s)", response_time.as_secs_f64() );
          }

          // Add AI response to conversation history
          conversation_history.push( ChatMessage
          {
            role : MessageRole::Assistant,
            content : response.message.content,
            #[ cfg( feature = "vision_support" ) ]
            images : None,
            #[ cfg( feature = "tool_calling" ) ]
            tool_calls : None,
          });
        }
        else
        {
          println!( "AI response contained no text." );
        }
      }
      Err( e ) =>
      {
        println!( "Error : {e}" );
        println!( "Please try again or type /quit to exit.\n" );
        conversation_history.pop(); // Remove failed user message
      }
    }
  }

  // Continue with normal loop for subsequent inputs
  loop
  {
    // Get user input
    print!( "\n > " );
    io ::stdout().flush()?;

    let mut input = String::new();
    let bytes_read = match io::stdin().read_line( &mut input )
    {
      Ok( n ) => n,
      Err( e ) =>
      {
        println!( "\nError reading input : {e}" );
        break;
      }
    };

    // Handle EOF (no input available in non-interactive mode)
    if bytes_read == 0
    {
      println!( "\n👋 No input available (EOF). Exiting gracefully." );
      println!( "Note : Use this example in interactive terminal only." );
      println!( "Run : cargo run --example ollama_chat_cached_interactive --all-features" );
      break;
    }

    let input = input.trim();

    // Handle commands
    match input
    {
      "/quit" | "/exit" =>
      {
        stats.print_summary();
        break;
      },
      "/clear" =>
      {
        conversation_history.clear();
        // Re-create client to clear cache (explicit cache management)
        client = OllamaClient::new(
          "http://localhost:11434".to_string(),
          OllamaClient::recommended_timeout_default()
        ).with_request_cache(
          RequestCacheConfig::new()
            .with_max_entries( 100 )
            .with_default_ttl( Duration::from_secs( 300 ) )
            .with_cleanup_interval( Duration::from_secs( 60 ) )
        );
        stats = SessionStats::default();
        println!( "✨ Conversation and cache cleared" );
        continue;
      },
      "/help" =>
      {
        print_help();
        continue;
      },
      "/history" =>
      {
        println!( "\n📜 Conversation History:" );
        if conversation_history.is_empty()
        {
          println!( "  (empty)" );
        }
        else
        {
          for ( i, msg ) in conversation_history.iter().enumerate()
          {
            let role = match msg.role
            {
              MessageRole::User => "You",
              MessageRole::Assistant => "AI",
              _ => "System",
            };
            let preview_len = msg.content.len().min( 60 );
            println!( "  {}. {}: {}...", i + 1, role, &msg.content[ ..preview_len ] );
          }
        }
        continue;
      },
      "/cache" =>
      {
        stats.print_detailed();
        continue;
      },
      "/perf" =>
      {
        stats.print_summary();
        continue;
      },
      "" => continue,
      _ => {},
    }

    // Add user message to conversation history
    conversation_history.push( ChatMessage
    {
      role : MessageRole::User,
      content : input.to_string(),
      #[ cfg( feature = "vision_support" ) ]
      images : None,
      #[ cfg( feature = "tool_calling" ) ]
      tool_calls : None,
    });

    // Get cache stats before request to determine if this will be a cache hit
    let stats_before = client.cache_stats();

    // Create chat request
    #[ cfg( feature = "streaming" ) ]
    let request = ChatRequest
    {
      model : "llama3.2:3b".to_string(),
      messages : conversation_history.clone(),
      stream : Some( true ),
      options : Some( serde_json::Value::Object( options.clone().into_iter().collect() ) ),
      #[ cfg( feature = "tool_calling" ) ]
      tools : None,
      #[ cfg( feature = "tool_calling" ) ]
      tool_messages : None,
    };

    print!( "\n🤖 AI: " );
    io ::stdout().flush()?;

    // Track response time
    let start_time = Instant::now();

    // Use streaming if available
    #[ cfg( feature = "streaming" ) ]
    {
      match client.chat_stream( request.clone() ).await
      {
        Ok( mut stream ) =>
        {
          let mut full_response = String::new();

          while let Some( chunk ) = stream.next().await
          {
            match chunk
            {
              Ok( streaming_response ) =>
              {
                if !streaming_response.message.content.is_empty()
                {
                  print!( "{}", streaming_response.message.content );
                  io ::stdout().flush()?;
                  full_response.push_str( &streaming_response.message.content );
                }

                if streaming_response.done
                {
                  break;
                }
              }
              Err( e ) =>
              {
                println!( "\nStreaming error : {e}" );
                break;
              }
            }
          }

          let response_time = start_time.elapsed();
          println!( "\n" );

          // Get cache stats after request
          let stats_after = client.cache_stats();
          let was_cache_hit = stats_after.hits > stats_before.hits;

          // Update session statistics
          stats.record_request( response_time, was_cache_hit );
          stats.update_cache_stats( stats_after.hits, stats_after.misses, stats_after.evictions );

          // Show cache status
          if was_cache_hit
          {
            println!( "⚡ Cache hit! (responded in {:.2}s)", response_time.as_secs_f64() );
          }
          else
          {
            println!( "💾 Cache miss - response cached (took {:.2}s)", response_time.as_secs_f64() );
          }

          // Add AI response to conversation history
          if !full_response.is_empty()
          {
            conversation_history.push( ChatMessage
            {
              role : MessageRole::Assistant,
              content : full_response,
              #[ cfg( feature = "vision_support" ) ]
              images : None,
              #[ cfg( feature = "tool_calling" ) ]
              tool_calls : None,
            });
          }
        }
        Err( e ) =>
        {
          println!( "Error : {e}" );
          println!( "Please try again or type /quit to exit.\n" );
          conversation_history.pop(); // Remove failed user message
        }
      }
    }

    // Fallback for non-streaming builds
    #[ cfg( not( feature = "streaming" ) ) ]
    {
      let non_streaming_request = ChatRequest
      {
        model : "llama3.2:3b".to_string(),
        messages : conversation_history.clone(),
        stream : Some( false ),
        options : Some( serde_json::Value::Object( options.clone().into_iter().collect() ) ),
        #[ cfg( feature = "tool_calling" ) ]
        tools : None,
        #[ cfg( feature = "tool_calling" ) ]
        tool_messages : None,
      };

      match client.chat( non_streaming_request ).await
      {
        Ok( response ) =>
        {
          let response_time = start_time.elapsed();

          if !response.message.content.is_empty()
          {
            // Simulate streaming display
            let words : Vec< &str > = response.message.content.split_whitespace().collect();
            for ( i, word ) in words.iter().enumerate()
            {
              print!( "{}", word );
              if i < words.len() - 1
              {
                print!( " " );
              }
              io ::stdout().flush()?;
              tokio ::time::sleep( tokio::time::Duration::from_millis( 50 ) ).await;
            }
            println!( "\n" );

            // Get cache stats after request
            let stats_after = client.cache_stats();
            let was_cache_hit = stats_after.hits > stats_before.hits;

            // Update session statistics
            stats.record_request( response_time, was_cache_hit );
            stats.update_cache_stats( stats_after.hits, stats_after.misses, stats_after.evictions );

            // Show cache status
            if was_cache_hit
            {
              println!( "⚡ Cache hit! (responded in {:.2}s)", response_time.as_secs_f64() );
            }
            else
            {
              println!( "💾 Cache miss - response cached (took {:.2}s)", response_time.as_secs_f64() );
            }

            // Add AI response to conversation history
            conversation_history.push( ChatMessage
            {
              role : MessageRole::Assistant,
              content : response.message.content,
              #[ cfg( feature = "vision_support" ) ]
              images : None,
              #[ cfg( feature = "tool_calling" ) ]
              tool_calls : None,
            });
          }
          else
          {
            println!( "AI response contained no text." );
          }
        }
        Err( e ) =>
        {
          println!( "Error : {e}" );
          println!( "Please try again or type /quit to exit.\n" );
          conversation_history.pop(); // Remove failed user message
        }
      }
    }
  }

  Ok( () )
}
