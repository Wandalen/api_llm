//! List available models example.
//!
//! Demonstrates how to retrieve and display available Grok models.
//!
//! Run with:
//! ```bash
//! cargo run --example list_models --features integration
//! ```

use api_xai::{ Client, XaiEnvironmentImpl, Secret, ClientApiAccessors };

#[ tokio::main ]
async fn main() -> Result< (), Box< dyn core::error::Error > >
{
  // Setup client
  let secret = Secret::load_with_fallbacks( "XAI_API_KEY" )?;
  let env = XaiEnvironmentImpl::new( secret )?;
  let client = Client::build( env )?;

  println!( "🚀 XAI Grok API - List Models Example\n" );

  // List all models
  println!( "📤 Fetching available models...\n" );
  let models_response = client.models().list().await?;

  println!( "✅ Found {} models:\n", models_response.data.len() );

  // Display model information
  for ( idx, model ) in models_response.data.iter().enumerate()
  {
    println!( "{}. Model ID: {}", idx + 1, model.id );
    println!( "   - Object : {}", model.object );
    println!( "   - Created : {}", model.created );
    println!( "   - Owned by : {}", model.owned_by );
    println!();
  }

  // Get specific model details
  if let Some( first_model ) = models_response.data.first()
  {
    println!( "📋 Fetching details for : {}\n", first_model.id );

    match client.models().get( &first_model.id ).await
    {
      Ok( model_details ) =>
      {
        println!( "✅ Model Details:" );
        println!( "   - ID: {}", model_details.id );
        println!( "   - Object : {}", model_details.object );
        println!( "   - Created : {}", model_details.created );
        println!( "   - Owned by : {}", model_details.owned_by );
      }
      Err( e ) =>
      {
        println!( "⚠️  Could not fetch model details : {e}" );
      }
    }
  }

  Ok( () )
}
