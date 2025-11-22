//! Interactive chat example demonstrating Anthropic's Prompt Caching for cost optimization
//!
//! This example shows how to use prompt caching to reduce costs by ~90% on repeated context.
//! The system prompt and conversation history are cached, reducing input token costs significantly.
//!
//! # Usage
//!
//! ```bash
//! export ANTHROPIC_API_KEY="your-api-key"
//! cargo run --example claude_chat_cached_interactive
//! ```
//!
//! # Commands
//!
//! - Type your message and press Enter to chat
//! - `/quit` or `/exit` - End conversation and show cost summary
//! - `/clear` - Clear conversation (creates new cache)
//! - `/help` - Show available commands
//! - `/history` - Display conversation history
//! - `/cache` - Show detailed cache statistics
//! - `/cost` - Show cost comparison with/without caching

use api_claude::
{
  Client,
  CreateMessageRequest,
  SystemContent,
  CacheControl,
  messages::Message,
  secret::Secret,
};
use std::io::{ self, Write as IoWrite };

/// Cache statistics tracker
#[ derive( Default ) ]
struct CacheStats
{
  total_requests : usize,
  cache_creation_tokens : usize,
  cache_read_tokens : usize,
  regular_input_tokens : usize,
  output_tokens : usize,
}

impl CacheStats
{
  fn add_usage( &mut self, usage : &api_claude::Usage )
  {
    self.total_requests += 1;
    self.cache_creation_tokens += usage.cache_creation_input_tokens.unwrap_or( 0 ) as usize;
    self.cache_read_tokens += usage.cache_read_input_tokens.unwrap_or( 0 ) as usize;
    self.regular_input_tokens += usage.input_tokens as usize;
    self.output_tokens += usage.output_tokens as usize;
  }

  fn total_input_tokens( &self ) -> usize
  {
    self.cache_creation_tokens + self.cache_read_tokens + self.regular_input_tokens
  }

  fn cost_without_cache( &self ) -> f64
  {
    // Anthropic pricing (as of Oct 2024): $3/MTok input, $15/MTok output for Sonnet 3.5
    let input_cost = ( self.total_input_tokens() as f64 / 1_000_000.0 ) * 3.0;
    let output_cost = ( self.output_tokens as f64 / 1_000_000.0 ) * 15.0;
    input_cost + output_cost
  }

  fn cost_with_cache( &self ) -> f64
  {
    // Cache writes : $3.75/MTok, Cache reads : $0.30/MTok, Regular : $3/MTok, Output : $15/MTok
    let cache_write_cost = ( self.cache_creation_tokens as f64 / 1_000_000.0 ) * 3.75;
    let cache_read_cost = ( self.cache_read_tokens as f64 / 1_000_000.0 ) * 0.30;
    let regular_cost = ( self.regular_input_tokens as f64 / 1_000_000.0 ) * 3.0;
    let output_cost = ( self.output_tokens as f64 / 1_000_000.0 ) * 15.0;
    cache_write_cost + cache_read_cost + regular_cost + output_cost
  }

  fn savings_percentage( &self ) -> f64
  {
    let without = self.cost_without_cache();
    if without == 0.0
    {
      return 0.0;
    }
    let with = self.cost_with_cache();
    ( ( without - with ) / without ) * 100.0
  }

  fn print_summary( &self )
  {
    println!( "\n📊 Cache Statistics Summary" );
    println!( "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━" );
    println!( "Total Requests :           {}", self.total_requests );
    println!( "\nToken Usage:" );
    println!( "  Cache Creation :         {:>8} tokens", self.cache_creation_tokens );
    println!( "  Cache Reads :            {:>8} tokens", self.cache_read_tokens );
    println!( "  Regular Input :          {:>8} tokens", self.regular_input_tokens );
    println!( "  Output :                 {:>8} tokens", self.output_tokens );
    println!( "  Total Input :            {:>8} tokens", self.total_input_tokens() );
    println!( "\nCost Analysis:" );
    println!( "  Without Caching :        ${:.6}", self.cost_without_cache() );
    println!( "  With Caching :           ${:.6}", self.cost_with_cache() );
    println!( "  💰 Savings :             ${:.6} ({:.1}%)",
      self.cost_without_cache() - self.cost_with_cache(),
      self.savings_percentage()
    );
    println!( "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n" );
  }

  fn print_detailed( &self )
  {
    println!( "\n📈 Detailed Cache Statistics" );
    println!( "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━" );
    println!( "Cache Performance:" );
    if self.total_requests > 1
    {
      let cache_hit_rate = ( self.cache_read_tokens as f64 /
        ( self.cache_read_tokens + self.regular_input_tokens ) as f64 ) * 100.0;
      println!( "  Cache Hit Rate :         {cache_hit_rate:.1}%" );
      println!( "  Cache Misses :           {}", i32::from( self.cache_creation_tokens > 0 ) );
      println!( "  Cache Hits :             {}", if self.cache_read_tokens > 0 { self.total_requests - 1 } else { 0 } );
    }
    else
    {
      println!( "  Cache Status :           Initializing (first request)" );
    }
    println!( "\nPricing Breakdown:" );
    println!( "  Cache Write ($3.75/MTok): ${:.6}", ( self.cache_creation_tokens as f64 / 1_000_000.0 ) * 3.75 );
    println!( "  Cache Read ($0.30/MTok):  ${:.6}", ( self.cache_read_tokens as f64 / 1_000_000.0 ) * 0.30 );
    println!( "  Regular ($3.00/MTok):     ${:.6}", ( self.regular_input_tokens as f64 / 1_000_000.0 ) * 3.0 );
    println!( "  Output ($15.00/MTok):     ${:.6}", ( self.output_tokens as f64 / 1_000_000.0 ) * 15.0 );
    println!( "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n" );
  }
}

fn print_help()
{
  println!( "\n📖 Available Commands:" );
  println!( "  /quit, /exit  - End conversation and show cost summary" );
  println!( "  /clear        - Clear conversation (creates new cache)" );
  println!( "  /help         - Show this help message" );
  println!( "  /history      - Display conversation history" );
  println!( "  /cache        - Show detailed cache statistics" );
  println!( "  /cost         - Show cost comparison with/without caching" );
  println!();
}

#[ tokio::main( flavor = "current_thread" ) ]
#[ allow( clippy::too_many_lines ) ]
async fn main() -> Result< (), Box< dyn core::error::Error > >
{
  println!( "💬 Claude Cached Interactive Chat" );
  println!( "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━" );
  println!( "Using Anthropic's Prompt Caching for cost optimization" );
  println!( "Type /help for available commands\n" );

  // Load API key
  let secret = Secret::load_from_env( "ANTHROPIC_API_KEY" )?;
  let client = Client::new( secret );

  // Large system prompt to demonstrate caching benefits
  let system_prompt = r"You are Claude, an expert AI assistant created by Anthropic.

Your capabilities include:
- Deep analysis and reasoning
- Technical problem solving
- Creative writing and ideation
- Code review and debugging
- Mathematical and logical reasoning
- Multilingual communication

Guidelines:
- Be concise but thorough in your responses
- Ask clarifying questions when needed
- Admit uncertainty rather than guessing
- Provide examples to illustrate concepts
- Break down complex topics into understandable parts
- Cite sources when referencing specific information

This system prompt is cached to reduce costs on subsequent requests.
The cache lasts for 5 minutes and significantly reduces input token costs.";

  let mut conversation : Vec< Message > = Vec::new();
  let mut stats = CacheStats::default();

  loop
  {
    // Get user input
    print!( "\n > " );
    io::stdout().flush()?;

    let mut input = String::new();
    io::stdin().read_line( &mut input )?;
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
        conversation.clear();
        stats = CacheStats::default();
        println!( "✨ Conversation cleared. New cache will be created on next request." );
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
        for ( i, msg ) in conversation.iter().enumerate()
        {
          let text = msg.content.first()
            .and_then( | c | c.text().map( str::to_string ) )
            .unwrap_or_else( || "[No text]".to_string() );
          let preview_len = text.len().min( 80 );
          println!( "{}. {:?}: {}...", i + 1, msg.role, &text[ ..preview_len ] );
        }
        continue;
      },
      "/cache" =>
      {
        stats.print_detailed();
        continue;
      },
      "/cost" =>
      {
        stats.print_summary();
        continue;
      },
      "" => continue,
      _ => {},
    }

    // Add user message
    conversation.push( Message::user( input.to_string() ) );

    // Create request with caching
    let request = CreateMessageRequest
    {
      model : "claude-sonnet-4-5-20250929".to_string(),
      max_tokens : 1024,
      messages : conversation.clone(),
      system : Some( vec![ SystemContent
      {
        r#type : "text".to_string(),
        text : system_prompt.to_string(),
        cache_control : Some( CacheControl::ephemeral() ),
      } ] ),
      temperature : None,
      stream : None,
      tools : None,
      tool_choice : None,
    };

    // Send request
    match client.create_message( request ).await
    {
      Ok( response ) =>
      {
        // Extract assistant response
        let assistant_text = response.content
          .first()
          .and_then( | c | c.text.clone() )
          .unwrap_or_else( || "[No response]".to_string() );

        println!( "\n🤖 Claude : {assistant_text}" );

        // Update statistics
        stats.add_usage( &response.usage );

        // Show cache status for this request
        let cache_read = response.usage.cache_read_input_tokens.unwrap_or( 0 );
        let cache_created = response.usage.cache_creation_input_tokens.unwrap_or( 0 );

        if cache_created > 0
        {
          println!( "\n✨ Cache created ({cache_created} tokens)" );
        }
        else if cache_read > 0
        {
          println!( "\n⚡ Cache hit! ({cache_read} tokens read from cache, ~90% cost savings)" );
        }

        // Add assistant response to conversation
        conversation.push( Message::assistant( assistant_text ) );
      },
      Err( e ) =>
      {
        eprintln!( "\n❌ Error : {e}" );
        conversation.pop(); // Remove failed user message
      },
    }
  }

  Ok(())
}
