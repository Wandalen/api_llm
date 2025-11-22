//! Interactive Chat with Response Caching ⭐⭐⭐⭐
//!
//! **Complexity**: ⭐⭐⭐⭐ (Advanced - Production-ready cached chat)
//!
//! This example demonstrates building an interactive chat interface with intelligent
//! response caching to reduce API costs and improve response times for repeated queries.
//!
//! **What You'll Learn:**
//! - How to integrate caching into a chat application
//! - How to cache responses based on normalized user input
//! - How to configure cache TTL for different use cases
//! - How to display cache statistics for monitoring
//! - How to balance cache freshness vs. API cost
//! - Production patterns for cached chat applications
//!
//! **Usage:**
//! ```bash
//! export HUGGINGFACE_API_KEY="your_api_key_here"
//!
//! cargo run --example chat_cached_interactive --features="full"
//! ```
//!
//! **Cache Behavior:**
//! - Responses are cached for 5 minutes by default
//! - Cache keys are normalized (lowercase, trimmed) for better hit rates
//! - Cache statistics shown after each interaction
//! - Type `/stats` to see detailed cache metrics
//! - Type `/clear` to clear the cache
//! - Type `/exit` or `/quit` to end the session

use api_huggingface::
{
  Client,
  environment::HuggingFaceEnvironmentImpl,
  cache::{ Cache, CacheConfig },
  components::input::InferenceParameters,
  secret::Secret,
};
use std::io::{ self, Write as IoWrite };
use core::time::Duration;

/// Normalize user input for cache key generation
///
/// This improves cache hit rates by treating similar queries as identical.
fn normalize_query( query : &str ) -> String
{
  query.trim().to_lowercase()
}

#[ tokio::main ]
async fn main() -> Result< (), Box< dyn std::error::Error > >
{
  println!( "🤖 HuggingFace Cached Interactive Chat" );
  println!( "======================================\n" );

  // Load API key from environment
  let api_key = Secret::load_from_env( "HUGGINGFACE_API_KEY" )?;

  // Initialize client
  let env = HuggingFaceEnvironmentImpl::build( api_key, None )?;
  let client = Client::build( env )?;

  // Configure cache with 5-minute TTL and max 100 entries
  let cache_config = CacheConfig
  {
  max_entries : 100,
  default_ttl : Some( Duration::from_secs( 300 ) ), // 5 minutes
  };

  // Create cache for storing responses
  let cache : Cache< String, String > = Cache::new( cache_config );

  // Configure inference parameters
  let params = InferenceParameters::new()
  .with_temperature( 0.7 )
  .with_max_new_tokens( 200 );

  // Use a fast, efficient model for chat
  let model = "moonshotai/Kimi-K2-Instruct-0905:groq";

  println!( "ℹ️  Cache configured : 100 entries max, 5-minute TTL" );
  println!( "ℹ️  Model : {model}" );
  println!( "\n💡 Commands:" );
  println!( "   /stats  - Show cache statistics" );
  println!( "   /clear  - Clear cache" );
  println!( "   /exit   - Exit chat\n" );

  // Main chat loop
  loop
  {
  // Prompt for user input
  print!( "You : " );
  io::stdout().flush()?;

  let mut user_input = String::new();
  io::stdin().read_line( &mut user_input )?;
  let user_input = user_input.trim();

  // Handle empty input
  if user_input.is_empty()
  {
      continue;
  }

  // Handle special commands
  match user_input
  {
      "/exit" | "/quit" =>
      {
  println!( "\n👋 Goodbye!" );
  break;
      }
      "/stats" =>
      {
  let stats = cache.stats().await;
  println!( "\n📊 Cache Statistics:" );
  println!( "   Total requests : {}", stats.total_requests() );
  println!( "   Cache hits : {}", stats.hits );
  println!( "   Cache misses : {}", stats.misses );
  println!( "   Hit rate : {:.1}%", stats.hit_rate() * 100.0 );
  println!( "   Current entries : {}", stats.entries );
  println!( "   Evictions : {}\n", stats.evictions );
  continue;
      }
      "/clear" =>
      {
  cache.clear().await;
  println!( "✅ Cache cleared\n" );
  continue;
      }
      _ => {}
  }

  // Normalize query for cache lookup
  let cache_key = normalize_query( user_input );

  // Check cache first
  if let Some( cached_response ) = cache.get( &cache_key ).await
  {
      println!( "Assistant (cached): {cached_response}" );

      let stats = cache.stats().await;
      println!( "💾 Cache hit! Hit rate : {:.1}%\n", stats.hit_rate() * 100.0 );
      continue;
  }

  // Cache miss - make API call
  print!( "Assistant : " );
  io::stdout().flush()?;

  match client.inference().create_with_parameters(
      user_input,
      model,
      params.clone()
  ).await
  {
      Ok( response ) =>
      {
  let response_text = response.extract_text_or_default( "" );
  println!( "{response_text}" );

  // Cache the response
  cache.insert( cache_key, response_text, None ).await;

  let stats = cache.stats().await;
  println!( "🌐 API call made. Hit rate : {:.1}%\n", stats.hit_rate() * 100.0 );
      }
      Err( e ) =>
      {
  println!( "❌ Error : {e}\n" );
      }
  }
  }

  // Display final statistics
  let final_stats = cache.stats().await;
  println!( "\n📊 Final Cache Statistics:" );
  println!( "   Total requests : {}", final_stats.total_requests() );
  println!( "   Cache hits : {}", final_stats.hits );
  println!( "   Cache misses : {}", final_stats.misses );
  println!( "   Final hit rate : {:.1}%", final_stats.hit_rate() * 100.0 );
  println!( "   Entries cached : {}", final_stats.entries );

  // Calculate cost savings (assuming $0.50 per 1000 tokens, ~200 tokens per response)
  let api_calls_saved = final_stats.hits;
  let estimated_savings = api_calls_saved as f64 * 0.0001; // $0.0001 per response
  println!( "   💰 Estimated savings : ${estimated_savings:.4}\n" );

  Ok( () )
}
