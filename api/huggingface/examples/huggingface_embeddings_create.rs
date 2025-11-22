//! Example : Embedding generation with `HuggingFace` API
//!
//! This example demonstrates embedding generation and similarity calculation.
//! 
//! Usage:
//! ```bash
//! export HUGGINGFACE_API_KEY="your_api_key_here"
//! cargo run --example embeddings_create --features="full"
//! ```

use api_huggingface::
{
  Client,
  environment::HuggingFaceEnvironmentImpl,
  components::
  {
  models::Models,
  },
  secret::Secret,
};

#[ tokio::main ]
async fn main() -> Result< (), Box< dyn std::error::Error > >
{
  // Initialize tracing for debugging
  tracing_subscriber::fmt::init();
  
  println!( "🤗 HuggingFace Embeddings API Example" );
  
  // Load API key from environment
  let api_key = Secret::load_from_env( "HUGGINGFACE_API_KEY" )?;
  println!( "✓ API key loaded from environment" );
  
  // Create environment and client
  let env = HuggingFaceEnvironmentImpl::build( api_key, None )?;
  let client = Client::build( env )?;
  println!( "✓ Client initialized" );
  
  let model = Models::all_minilm_l6_v2();
  println!( "🤖 Using embedding model : {model}" );
  
  // Example 1: Single embedding
  println!( "\n📝 Example 1: Single text embedding" );
  let text = "The quick brown fox jumps over the lazy dog.";
  println!( "Text : {text}" );
  
  match client.embeddings().create( text, model ).await
  {
  Ok( response ) =>
  {
      match response
      {
  api_huggingface::components::embeddings::EmbeddingResponse::Single( embeddings ) =>
  {
          if let Some( embedding ) = embeddings.first()
          {
      println!( "✓ Generated embedding with {} dimensions", embedding.len() );
      println!( "📊 First 5 values : {:?}", &embedding[ 0..5.min( embedding.len() ) ] );
          }
  },
  api_huggingface::components::embeddings::EmbeddingResponse::Batch( _ ) => println!( "Unexpected response format" ),
      }
  },
  Err( e ) =>
  {
      eprintln!( "❌ Error : {e}" );
  }
  }
  
  // Example 2: Batch embeddings
  println!( "\n📝 Example 2: Batch text embeddings" );
  let texts = vec!
  [
  "Artificial intelligence is transforming the world.".to_string(),
  "Machine learning algorithms can process vast amounts of data.".to_string(),
  "Natural language processing enables computers to understand human language.".to_string(),
  ];
  
  println!( "Texts : {texts:?}" );
  
  match client.embeddings().create_batch( texts.clone(), model ).await
  {
  Ok( response ) =>
  {
      match response
      {
  api_huggingface::components::embeddings::EmbeddingResponse::Single( embeddings ) =>
  {
          println!( "✓ Generated {} embeddings", embeddings.len() );
          for ( i, embedding ) in embeddings.iter().enumerate()
          {
      println!( "  Embedding {}: {} dimensions", i + 1, embedding.len() );
          }
  },
  api_huggingface::components::embeddings::EmbeddingResponse::Batch( _ ) => println!( "Unexpected response format" ),
      }
  },
  Err( e ) =>
  {
      eprintln!( "❌ Error : {e}" );
  }
  }
  
  // Example 3: Similarity calculation
  println!( "\n📝 Example 3: Similarity calculation" );
  let first_text = "I love programming in Rust";
  let second_text = "Rust is my favorite programming language";
  let third_text = "I enjoy cooking pasta";
  
  println!( "Text 1: {first_text}" );
  println!( "Text 2: {second_text}" );
  println!( "Text 3: {third_text}" );
  
  // Compare first_text with second_text (should be high similarity)
  match client.embeddings().similarity( first_text, second_text, model ).await
  {
  Ok( similarity ) =>
  {
      println!( "🎯 Similarity (Text1 vs Text2): {similarity:.4}" );
  },
  Err( e ) =>
  {
      eprintln!( "❌ Similarity error : {e}" );
  }
  }
  
  // Compare first_text with third_text (should be low similarity)
  match client.embeddings().similarity( first_text, third_text, model ).await
  {
  Ok( similarity ) =>
  {
      println!( "🎯 Similarity (Text1 vs Text3): {similarity:.4}" );
  },
  Err( e ) =>
  {
      eprintln!( "❌ Similarity error : {e}" );
  }
  }
  
  println!( "\n✅ Example completed!" );
  Ok( () )
}