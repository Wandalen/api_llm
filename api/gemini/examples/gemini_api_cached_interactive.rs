//! Interactive chat with server-side cached content and token management.
//!
//! This example demonstrates advanced Gemini API features:
//! - Interactive chat loop with conversation history
//! - Server-side cached content for context efficiency
//! - Token counting for cache optimization
//! - Real streaming responses with cached context
//! - Cache lifecycle management (create/update/delete)
//! - Cost optimization through intelligent caching
//!
//! ## Features Demonstrated
//!
//! 1. **Server-side Caching**: Creates cached content to store conversation context
//! 2. **Token Management**: Counts tokens to optimize cache usage and costs
//! 3. **Interactive Chat**: Real-time conversation with the AI
//! 4. **Streaming Responses**: Live streaming of AI responses (with streaming feature)
//! 5. **Cache Optimization**: Automatically manages cache size and updates
//! 6. **Cost Efficiency**: Reduces token usage through smart caching
//!
//! ## Usage
//!
//! ```bash
//! # With real streaming (recommended)
//! cargo run --example gemini_chat_cached_interactive --features streaming
//!
//! # With all features
//! cargo run --example gemini_chat_cached_interactive --features full
//!
//! # Basic version without streaming
//! cargo run --example gemini_chat_cached_interactive
//! ```
//!
//! ## Commands
//!
//! - Type messages normally for conversation
//! - `!tokens` - Show current token usage
//! - `!cache info` - Show cache information
//! - `!cache clear` - Clear and recreate cache
//! - `!help` - Show available commands
//! - `quit`, `exit`, `bye` - End conversation and cleanup
//!
//! ## Requirements
//!
//! - Valid GEMINI_API_KEY environment variable
//! - Interactive terminal (not for automated testing)

#[ cfg( feature = "streaming" ) ]
use futures::StreamExt;
use api_gemini::{
  client ::Client,
  models ::*,
  error ::Error,
};
use std::io::{self, Write};
use std::time::{SystemTime, UNIX_EPOCH};

/// Configuration for the cached chat session
struct ChatConfig
{
  /// Maximum tokens to keep in cache before optimization
  max_cache_tokens: i32,
  /// Maximum conversation turns to keep in memory
  max_conversation_turns: usize,
  /// Cache TTL in seconds
  cache_ttl_seconds: u32,
  /// Model to use for the conversation
  model_name: String,
}

impl Default for ChatConfig
{
  fn default() -> Self
  {
    Self {
      max_cache_tokens: 8000,
      max_conversation_turns: 20,
      cache_ttl_seconds: 3600, // 1 hour
      model_name: "gemini-1.5-flash-latest".to_string(),
    }
  }
}

/// Manages the cached conversation state
struct CachedChatSession
{
  client: Client,
  config: ChatConfig,
  conversation_history: Vec< Content >,
  cache_id: Option< String >,
  total_tokens_used: i32,
  cached_tokens: i32,
}

impl CachedChatSession
{
  /// Create a new cached chat session
  async fn new(client: Client) -> Result< Self, Error > 
  {
    let config = ChatConfig::default();

    Ok(Self {
      client,
      config,
      conversation_history: Vec::new(),
      cache_id: None,
      total_tokens_used: 0,
      cached_tokens: 0,
    })
  }

  /// Initialize the conversation with a system prompt and create initial cache
  async fn initialize(&mut self) -> Result< (), Error > 
  {
    println!("🔄 Initializing cached conversation session...");

    // Create initial system context
    let system_content = Content {
      role: "user".to_string(),
      parts : vec![Part {
        text: Some(
        "You are a helpful AI assistant. This is the beginning of our conversation. \
        Please be concise but informative in your responses. You can discuss any topic \
        the user is interested in.".to_string()
        ),
        inline_data: None,
        function_call: None,
        function_response: None,
        ..Default::default()
      }],
    };

    self.conversation_history.push(system_content.clone());

    // Create cached content for the initial context
    self.create_cache().await?;

    println!("✅ Cache initialized successfully!");
    Ok(())
  }

  /// Create server-side cached content
  async fn create_cache(&mut self) -> Result< (), Error > 
  {
    let timestamp = SystemTime::now()
    .duration_since(UNIX_EPOCH)
    .unwrap()
    .as_secs();

    let cache_request = CreateCachedContentRequest {
      model: self.config.model_name.clone(),
      contents: self.conversation_history.clone(),
    ttl : Some(format!("{}s", self.config.cache_ttl_seconds)),
      expire_time: None,
    display_name : Some(format!("Interactive Chat Session {}", timestamp)),
      system_instruction : Some(Content {
        role: "system".to_string(),
        parts : vec![Part {
          text: Some(
          "You are engaging in an interactive conversation. Maintain context \
          from previous messages and provide helpful, accurate responses.".to_string()
          ),
          inline_data: None,
          function_call: None,
          function_response: None,
          ..Default::default()
        }],
      }),
      tools: None,
      tool_config: None,
    };

    let response = self.client.cached_content().create(&cache_request).await?;
    self.cache_id = Some(response.name.clone());

    // Count tokens in the cached content
    self.update_token_counts().await?;

println!("📦 Created cache : {} (ID: {})",
    response.display_name.unwrap_or_else(|| "Unnamed".to_string()),
    response.name
    );

    Ok(())
  }

  /// Update cached content with new conversation history
  async fn update_cache(&mut self) -> Result< (), Error > 
  {
    if let Some(cache_id) = &self.cache_id
    {
      let update_request = UpdateCachedContentRequest {
      ttl : Some(format!("{}s", self.config.cache_ttl_seconds)),
        expire_time: None,
      };

      let _response = self.client.cached_content().update(cache_id, &update_request).await?;
      self.update_token_counts().await?;

      println!("🔄 Cache updated successfully");
    }
    Ok(())
  }

  /// Count tokens in current conversation and update cached token count
  async fn update_token_counts(&mut self) -> Result< (), Error > 
  {
    let count_request = CountTokensRequest {
      contents: self.conversation_history.clone(),
      generate_content_request: None,
    };

    let count_response = self.client
    .models()
    .count_tokens(&self.config.model_name, &count_request)
    .await?;

    self.total_tokens_used = count_response.total_tokens;
    self.cached_tokens = count_response.cached_content_token_count.unwrap_or(0);

    Ok(())
  }

  /// Add a user message to the conversation
  fn add_user_message(&mut self, message: String)
  {
    let content = Content {
      role: "user".to_string(),
      parts : vec![Part {
        text: Some(message),
        inline_data: None,
        function_call: None,
        function_response: None,
        ..Default::default()
      }],
    };
    self.conversation_history.push(content);
  }

  /// Add an AI response to the conversation
  fn add_ai_response(&mut self, response: Content)
  {
    self.conversation_history.push(response);
  }

  /// Generate AI response using cached content
  async fn generate_response(&mut self) -> Result< String, Error > 
  {
    let request = GenerateContentRequest {
      contents: self.conversation_history.clone(),
      generation_config : Some(GenerationConfig {
        temperature: Some(0.7),
        max_output_tokens: Some(1024),
        top_p: Some(0.9),
        top_k: Some(40),
        candidate_count: Some(1),
        stop_sequences: None,
      }),
      safety_settings: None,
      tools: None,
      tool_config: None,
      system_instruction: None,
      cached_content: self.cache_id.clone(),
    };

    #[ cfg( feature = "streaming" ) ]
    {
      self.generate_streaming_response(request).await
    }

    #[ cfg( not( feature = "streaming" ) ) ]
    {
      self.generate_regular_response(request).await
    }
  }

  /// Generate streaming response
  #[ cfg( feature = "streaming" ) ]
  async fn generate_streaming_response(&mut self, request: GenerateContentRequest) -> Result< String, Error > 
  {
    let stream = self.client
    .models()
    .by_name(&self.config.model_name)
    .generate_content_stream(&request)
    .await?;

    let mut full_response = String::new();
    let mut response_content: Option< Content > = None;

    futures ::pin_mut!(stream);

    while let Some(chunk) = stream.next().await
    {
      match chunk
      {
        Ok(streaming_response) => {
          if let Some(candidates) = streaming_response.candidates
          {
            if let Some(candidate) = candidates.first()
            {
              if let Some(part) = candidate.content.parts.first()
              {
                if let Some(text) = &part.text
                {
                print!("{}", text);
                  io ::stdout().flush().unwrap();
                  full_response.push_str(text);
                }
              }

              if response_content.is_none()
              {
                response_content = Some(candidate.content.clone());
              }
            }
          }
        }
        Err(e) => {
          return Err(e);
        }
      }
    }

    // Add response to conversation history
    if let Some(content) = response_content
    {
      self.add_ai_response(content);
    } else if !full_response.is_empty()
    {
      let content = Content {
        role: "model".to_string(),
        parts : vec![Part {
          text: Some(full_response.clone()),
          inline_data: None,
          function_call: None,
          function_response: None,
          ..Default::default()
        }],
      };
      self.add_ai_response(content);
    }

    Ok(full_response)
  }

  /// Generate regular (non-streaming) response
  #[ cfg( not( feature = "streaming" ) ) ]
  async fn generate_regular_response(&mut self, request: GenerateContentRequest) -> Result< String, Error > 
  {
    let response = self.client
    .models()
    .by_name(&self.config.model_name)
    .generate_content(&request)
    .await?;

    if let Some(candidate) = response.candidates.first()
    {
      if let Some(part) = candidate.content.parts.first()
      {
        if let Some(text) = &part.text
        {
          // Simulate streaming by printing words with delays
          let words: Vec< &str > = text.split_whitespace().collect();
          for (i, word) in words.iter().enumerate()
          {
          print!("{}", word);
            if i < words.len() - 1
            {
              print!(" ");
            }
            io ::stdout().flush().unwrap();
            tokio ::time::sleep(tokio::time::Duration::from_millis(80)).await;
          }

          self.add_ai_response(candidate.content.clone());
          return Ok(text.clone());
        }
      }
    }

    Err(Error::ApiError("No response generated".to_string()))
  }

  /// Check if cache optimization is needed
  async fn should_optimize_cache(&mut self) -> Result< bool, Error > 
  {
    self.update_token_counts().await?;

    Ok(self.total_tokens_used > self.config.max_cache_tokens ||
    self.conversation_history.len() > self.config.max_conversation_turns)
  }

  /// Optimize cache by removing older messages and updating cache
  async fn optimize_cache(&mut self) -> Result< (), Error > 
  {
  println!("\n🔄 Optimizing cache (current tokens : {})...", self.total_tokens_used);

    // Keep system message and recent conversation
    let keep_count = self.config.max_conversation_turns / 2;
    if self.conversation_history.len() > keep_count + 1
    {
      let mut optimized_history = vec![self.conversation_history[0].clone()]; // Keep system message

      // Keep the most recent messages
      let start_index = self.conversation_history.len().saturating_sub(keep_count);
      optimized_history.extend_from_slice(&self.conversation_history[start_index..]);

      self.conversation_history = optimized_history;
    }

    // Recreate cache with optimized history
    self.delete_cache().await?;
    self.create_cache().await?;

  println!("✅ Cache optimized! New token count : {}", self.total_tokens_used);
    Ok(())
  }

  /// Delete the current cache
  async fn delete_cache(&mut self) -> Result< (), Error > 
  {
    if let Some(cache_id) = &self.cache_id
    {
      self.client.cached_content().delete(cache_id).await?;
      self.cache_id = None;
      println!("🗑️ Cache deleted");
    }
    Ok(())
  }

  /// Show token usage information
  async fn show_token_info(&mut self) -> Result< (), Error > 
  {
    self.update_token_counts().await?;

    println!("\n📊 Token Usage Information:");
  println!("  Total tokens in conversation : {}", self.total_tokens_used);
  println!("  Cached tokens : {}", self.cached_tokens);
  println!("  Conversation turns : {}", self.conversation_history.len() - 1); // Exclude system message
  println!("  Cache ID: {}", self.cache_id.as_deref().unwrap_or("None"));
  println!("  Max cache tokens (before optimization): {}", self.config.max_cache_tokens);
    println!();

    Ok(())
  }

  /// Show cache information
  async fn show_cache_info(&mut self) -> Result< (), Error > 
  {
    if let Some(cache_id) = &self.cache_id
    {
      match self.client.cached_content().get(cache_id).await
      {
        Ok(cache_info) => {
          println!("\n📦 Cache Information:");
        println!("  Cache ID: {}", cache_info.name);
        println!("  Display Name : {}", cache_info.display_name.unwrap_or_else(|| "Unnamed".to_string()));
        println!("  Model : {}", cache_info.model);
        println!("  Expire Time : {}", cache_info.expire_time.as_deref().unwrap_or("No expiration"));
        println!("  Create Time : {}", cache_info.create_time.unwrap_or_else(|| "Unknown".to_string()));
        println!("  Update Time : {}", cache_info.update_time.unwrap_or_else(|| "Unknown".to_string()));
          println!();
        }
        Err(e) => {
        println!("❌ Error retrieving cache info : {}", e);
        }
      }
    } else {
      println!("❌ No active cache");
    }
    Ok(())
  }

  /// Clear cache and recreate
  async fn clear_cache(&mut self) -> Result< (), Error > 
  {
    println!("🔄 Clearing and recreating cache...");
    self.delete_cache().await?;

    // Reset conversation but keep system message
    if !self.conversation_history.is_empty()
    {
      self.conversation_history = vec![self.conversation_history[0].clone()];
    }

    self.create_cache().await?;
    println!("✅ Cache cleared and recreated!");
    Ok(())
  }

  /// Show help information
  fn show_help(&self)
  {
    println!("\n📖 Available Commands:");
    println!("  !tokens           - Show current token usage");
    println!("  !cache info       - Show cache information");
    println!("  !cache clear      - Clear and recreate cache");
    println!("  !help             - Show this help message");
    println!("  quit/exit/bye     - End conversation");
    println!("\n💡 Tips:");
  println!("  - Cache automatically optimizes when reaching {} tokens", self.config.max_cache_tokens);
    println!("  - Cached content reduces token costs for repeated context");
    println!("  - Use !tokens to monitor your usage");
    println!();
  }

  /// Cleanup resources
  async fn cleanup(&mut self) -> Result< (), Error > 
  {
    if let Some(cache_id) = &self.cache_id
    {
      println!("🧹 Cleaning up cache...");
      self.client.cached_content().delete(cache_id).await?;
      println!("✅ Cache deleted successfully");
    }
    Ok(())
  }
}

#[ tokio::main ]
#[ allow( clippy::too_many_lines ) ]
async fn main() -> Result<(), Box< dyn core::error::Error > > 
{
  // Initialize client
  let client = match Client::new()
  {
    Ok(client) => client,
    Err(e) => {
    eprintln!("❌ Failed to create client : {}", e);
      eprintln!("💡 Make sure GEMINI_API_KEY environment variable is set");
      return Ok(());
    }
  };

  println!("🤖 Interactive Cached Chat with Gemini");
  println!("=====================================");
  println!("This demo showcases server-side caching with token management.");
  println!("Type '!help' for commands or start chatting!\n");

  // Initialize chat session
  let mut session = CachedChatSession::new(client).await?;

  // Initialize cache
  if let Err(e) = session.initialize().await
  {
  eprintln!("❌ Failed to initialize session : {}", e);
    return Ok(());
  }

  // Main chat loop
  loop
  {
    // Get user input
    print!("\n👤 You: ");
    io ::stdout().flush()?;

    let mut input = String::new();
    match io::stdin().read_line(&mut input)
    {
      Ok(0) => {
        println!("\n❌ No input available. Use this example in interactive terminal only.");
        break;
      }
    Ok(_) => {}
      Err(e) => {
      println!("\n❌ Error reading input : {}", e);
        break;
      }
    }

    let user_message = input.trim().to_string();

    // Handle empty input
    if user_message.is_empty()
    {
      continue;
    }

    // Handle exit commands
    if matches!(user_message.to_lowercase().as_str(), "quit" | "exit" | "bye")
    {
      println!("\n👋 Goodbye! Thanks for chatting!");
      break;
    }

    // Handle special commands
    match user_message.as_str()
    {
      "!help" => {
        session.show_help();
        continue;
      }
      "!tokens" => {
        if let Err(e) = session.show_token_info().await
        {
        println!("❌ Error showing token info : {}", e);
        }
        continue;
      }
      "!cache info" => {
        if let Err(e) = session.show_cache_info().await
        {
        println!("❌ Error showing cache info : {}", e);
        }
        continue;
      }
      "!cache clear" => {
        if let Err(e) = session.clear_cache().await
        {
        println!("❌ Error clearing cache : {}", e);
        }
        continue;
      }
    _ => {}
    }

    // Add user message to conversation
    session.add_user_message(user_message);

    // Check if cache optimization is needed
    if let Ok(true) = session.should_optimize_cache().await
    {
      if let Err(e) = session.optimize_cache().await
      {
      println!("⚠️ Cache optimization failed : {}", e);
      }
    }

    // Generate AI response
    print!("\n🤖 AI: ");
    io ::stdout().flush()?;

    match session.generate_response().await
    {
      Ok(_response) => {
        println!("\n");

        // Update cache after successful response
        if let Err(e) = session.update_cache().await
        {
        println!("⚠️ Cache update failed : {}", e);
        }
      }
      Err(e) => {
      println!("\n❌ Error generating response : {}", e);
        println!("💡 Please try again or type 'quit' to exit.");
      }
    }
  }

  // Cleanup
  if let Err(e) = session.cleanup().await
  {
  eprintln!("⚠️ Cleanup error : {}", e);
  }

  Ok(())
}