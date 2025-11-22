//! Example : Text generation with `HuggingFace` Inference API
//!
//! This example demonstrates basic text generation using the `HuggingFace` API.
//! 
//! Usage:
//! ```bash
//! export HUGGINGFACE_API_KEY="your_api_key_here"
//! cargo run --example inference_create --features="full"
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
  // Initialize tracing for debugging
  tracing_subscriber::fmt::init();
  
  println!( "🤗 HuggingFace Inference API Example" );
  
  // Load API key from environment
  let api_key = Secret::load_from_env( "HUGGINGFACE_API_KEY" )?;
  println!( "✓ API key loaded from environment" );
  
  // Create environment and client
  let env = HuggingFaceEnvironmentImpl::build( api_key, None )?;
  let client = Client::build( env )?;
  println!( "✓ Client initialized" );
  
  // Configure inference parameters
  let params = InferenceParameters::new()
  .with_temperature( 0.7 )
  .with_max_new_tokens( 100 )
  .with_top_p( 0.9 );
  
  let model = Models::llama_3_3_70b_instruct();
  println!( "🤖 Using model : {model}" );
  
  // Example prompts
  let prompts = [
  "What is the capital of France?",
  "Explain quantum computing in simple terms.",
  "Write a haiku about artificial intelligence.",
  ];
  
  for ( i, prompt ) in prompts.iter().enumerate()
  {
  println!( "\n📝 Example {}: {prompt}", i + 1 );
  
  match client.inference().create_with_parameters( *prompt, model, params.clone() ).await
  {
      Ok( response ) =>
      {
  let text = response.extract_text_or_default( "No response generated" );
  println!( "💬 Response : {text}" );
      },
      Err( e ) =>
      {
  eprintln!( "❌ Error : {e}" );
      }
  }
  }
  
  println!( "\n✅ Example completed!" );
  Ok( () )
}