//! Basic Chat Example ⭐
//!
//! **Complexity**: ⭐ (Basic - Perfect for beginners)
//!
//! This example demonstrates the simplest possible interaction with the `HuggingFace`
//! Inference API: asking a single question and receiving a single answer.
//!
//! **What You'll Learn:**
//! - How to initialize the `HuggingFace` client
//! - How to configure basic parameters (temperature, max tokens)
//! - How to make a single inference request
//! - How to handle the response
//!
//! **Usage:**
//! ```bash
//! export HUGGINGFACE_API_KEY="your_api_key_here"
//! cargo run --example chat --features="full"
//! ```

use api_huggingface::
{
  Client,
  environment::HuggingFaceEnvironmentImpl,
  components::
  {
  input::InferenceParameters,
  models::Models,
  },
  secret::Secret,
};

#[ tokio::main ]
async fn main() -> Result< (), Box< dyn std::error::Error > >
{
  println!( "🤗 HuggingFace Basic Chat Example" );
  println!( "==================================\n" );

  // Step 1: Load API key from environment variable
  // The API key grants access to HuggingFace's inference endpoints
  let api_key = Secret::load_from_env( "HUGGINGFACE_API_KEY" )?;
  println!( "✓ API key loaded successfully" );

  // Step 2: Create the HuggingFace client
  // The client manages authentication and HTTP communication
  let env = HuggingFaceEnvironmentImpl::build( api_key, None )?;
  let client = Client::build( env )?;
  println!( "✓ Client initialized\n" );

  // Step 3: Configure inference parameters
  // These parameters control how the AI generates responses
  let params = InferenceParameters::new()
  .with_temperature( 0.7 )        // Controls randomness (0.0 = deterministic, 1.0 = creative)
  .with_max_new_tokens( 150 )     // Maximum length of generated response
  .with_top_p( 0.9 );              // Nucleus sampling for response diversity

  println!( "📋 Configuration:" );
  println!( "   Temperature: 0.7" );
  println!( "   Max tokens: 150" );
  println!( "   Top-p: 0.9\n" );

  // Step 4: Select a model
  // We're using Kimi-K2 - the recommended model for HuggingFace's new Router API
  // Excellent for reasoning, math, and conversational tasks
  let model = Models::kimi_k2_instruct();
  println!( "🤖 Model: {model}\n" );

  // Step 5: Ask a question
  // This is a static, predefined question for educational purposes
  let question = "What is artificial intelligence and how does it work?";
  println!( "❓ Question: {question}\n" );

  // Step 6: Send request and receive response
  // The client sends an HTTP request to HuggingFace's API
  println!( "⏳ Sending request to HuggingFace API..." );

  match client.inference().create_with_parameters( question, model, params ).await
  {
  Ok( response ) =>
  {
      // Extract the generated text from the response
      let answer = response.extract_text_or_default( "No response generated" );

      println!( "\n💬 Answer:" );
      println!( "─────────────────────────────────────────" );
      println!( "{answer}" );
      println!( "─────────────────────────────────────────\n" );
  },
  Err( e ) =>
  {
      // If something goes wrong, display a helpful error message
      eprintln!( "❌ Error: {e}" );
      eprintln!( "\n💡 Troubleshooting:" );
      eprintln!( "   • Verify your HUGGINGFACE_API_KEY is set correctly" );
      eprintln!( "   • Check your internet connection" );
      eprintln!( "   • Ensure you have access to the Inference API" );
      return Err( Box::new( e ) as Box< dyn std::error::Error > );
  }
  }

  println!( "✅ Chat example completed successfully!\n" );

  Ok( () )
}
