//! Simple non-interactive test of `HuggingFace` API inference
//! This example tests the API without requiring user input

use api_huggingface::
{
  client::Client,
  inference::Inference,
  environment::{ HuggingFaceEnvironmentImpl, HuggingFaceEnvironment, EnvironmentInterface },
  providers::ChatMessage,
  Secret,
};

fn load_api_key() -> Result< String, Box< dyn std::error::Error > >
{
  println!( "🔍 TOKEN LOADING DEBUG INFO:" );
  println!( "=============================" );

  // Check environment variable first
  println!( "🔍 Checking environment variable HUGGINGFACE_API_KEY..." );
  let env_result = std::env::var("HUGGINGFACE_API_KEY");
  match &env_result
  {
  Ok(token) => {
      println!( "✅ Found HUGGINGFACE_API_KEY in environment" );
      println!( "📝 Token length : {} characters", token.len() );
      println!( "📝 Token prefix : {}...", &token[..core::cmp::min(10, token.len())] );
      println!( "📝 Token source : ENVIRONMENT VARIABLE" );
  },
  Err(e) => {
      println!( "❌ HUGGINGFACE_API_KEY not found in environment : {e:?}" );
  }
  }

  // Load API key from environment or workspace secrets using workspace_tools
  let api_key = env_result.or_else(|_| {
  println!( "🔍 Environment variable not found, trying workspace secrets..." );

  use workspace_tools as workspace;
  println!( "🔍 Initializing workspace..." );
  let workspace = workspace::workspace()
      .map_err(|e| {
  println!( "❌ Failed to initialize workspace : {e:?}" );
  std::env::VarError::NotPresent
      })?;

  println!( "✅ Workspace initialized successfully" );
  println!( "🔍 Current working directory : {}", std::env::current_dir().unwrap_or_else(|_| std::path::PathBuf::from("unknown")).display() );

  // WORKSPACE_TOOLS: Load project-specific secrets using proper API
  println!( "🔍 Loading from project secrets using workspace_tools..." );
  println!( "🔍 Workspace path : {:?}", std::env::var("WORKSPACE_PATH").unwrap_or_else(|_| "not set".to_string()) );

  let secrets_result = workspace.load_secrets_from_file("-secrets.sh");
  match &secrets_result
  {
      Ok(secrets) => {
  println!( "✅ Successfully loaded {} secrets using workspace_tools", secrets.len() );
  let secret_file_path = workspace.secret_file("-secrets.sh");
  println!( "📝 Loaded from absolute path : {}", secret_file_path.display() );

  if let Some(token) = secrets.get("HUGGINGFACE_API_KEY")
  {
          println!( "✅ Found HUGGINGFACE_API_KEY in project secrets" );
          println!( "📝 Token length : {} characters", token.len() );
          println!( "📝 Full token : {token}" );
          println!( "📝 Token source : workspace_tools from {}", secret_file_path.display() );
          return Ok(token.clone());
  }
  println!( "❌ HUGGINGFACE_API_KEY not found in project secrets" );
  println!( "📝 Available keys : {:?}", secrets.keys().collect::< Vec< _ > >() );
      },
      Err(e) => {
  println!( "❌ Failed to load project secrets : {e:?}" );
      }
  }

  // Fallback to workspace root secret/-secrets.sh
  let workspace_secrets_path = "-secrets.sh";
  let expected_workspace_absolute = std::path::Path::new("/home/user1/pro/secret").join(workspace_secrets_path);
  println!( "🔍 Attempting fallback to absolute path : {}", expected_workspace_absolute.display() );
  println!( "🔍 Workspace file exists check : {}", expected_workspace_absolute.exists() );

  let fallback_result = workspace.load_secrets_from_file(workspace_secrets_path);
  match &fallback_result
  {
      Ok(secrets) => {
  println!( "✅ Successfully loaded secrets from absolute path : {}", expected_workspace_absolute.display() );
  println!( "📝 Number of secrets loaded : {}", secrets.len() );
  println!( "📝 Available secret keys : {:?}", secrets.keys().collect::< Vec< _ > >() );

  if let Some(token) = secrets.get("HUGGINGFACE_API_KEY")
  {
          println!( "✅ Found HUGGINGFACE_API_KEY in absolute path : {}", expected_workspace_absolute.display() );
          println!( "📝 Token length : {} characters", token.len() );
          println!( "📝 Full token : {token}" );
          println!( "📝 Token source : {}", expected_workspace_absolute.display() );
          return Ok(token.clone());
  }
  println!( "❌ HUGGINGFACE_API_KEY not found in absolute path : {}", expected_workspace_absolute.display() );
      },
      Err(e) => {
  println!( "❌ Failed to load absolute path {}: {e:?}", expected_workspace_absolute.display() );
      }
  }

  println!( "❌ No token found in any source" );
  Err(std::env::VarError::NotPresent)
  })
  .map_err(|_| "HUGGINGFACE_API_KEY not found in environment or workspace secrets (./secret/-secrets.sh or -secrets.sh)")?;

  println!( "🎉 FINAL TOKEN LOADED SUCCESSFULLY!" );
  println!( "📝 Final token length : {} characters", api_key.len() );
  println!( "📝 Final full token : {api_key}" );
  println!();

  Ok( api_key )
}

async fn test_inference_api< E >( inference : &Inference< E > ) -> Result< (), Box< dyn std::error::Error > >
where
  E : HuggingFaceEnvironment + EnvironmentInterface + Send + Sync + 'static + Clone,
{
  println!( "🧪 Testing simple inference with facebook/bart-large-cnn..." );

  let test_input = "x=13";

  let pro_models = [
  "meta-llama/Meta-Llama-3-8B-Instruct",
  "meta-llama/Llama-2-7b-chat-hf",
  "mistralai/Mistral-7B-Instruct-v0.2",
  "codellama/CodeLlama-7b-Instruct-hf",
  "microsoft/DialoGPT-medium",
  "HuggingFaceH4/zephyr-7b-beta",
  "NousResearch/Nous-Hermes-2-Mixtral-8x7B-DPO",
  "facebook/bart-large-cnn", // Known working fallback
  ];

  println!( "🔍 Testing Pro Plan Model Availability..." );
  println!();

  for (i, model) in pro_models.iter().enumerate()
  {
  println!( "🧪 Test {}: {}", i + 1, model );
  println!( "📤 Input : {test_input:?}" );
  println!( "🌐 API Endpoint : https://api-inference.huggingface.co/models/{model}" );
  println!( "📝 Request Model : {model}" );

  match inference.create( test_input, model ).await
  {
      Ok( response ) =>
      {
  println!( "✅ SUCCESS! Model {model} is available" );
  if let Some(text) = response.extract_text()
  {
          println!( "📝 Response : \"{}\"", text.chars().take(100).collect::< String >() );
  }
  println!( "🎉 WORKING MODEL FOUND: {model}" );
      },
      Err( e ) =>
      {
  println!( "❌ FAILED: {model} - {e}" );
      }
  }

  println!( "{}", "=".repeat( 80 ) );
  println!();
  }

  Ok( () )
}

async fn test_providers_api< E >( client : &Client< E > ) -> Result< (), Box< dyn std::error::Error > >
where
  E : HuggingFaceEnvironment + EnvironmentInterface + Send + Sync + 'static + Clone,
{
  println!( "\n🚀 Testing NEW Providers API for Pro Models..." );
  println!( "===============================================" );
  println!( "This API provides access to proper conversational models for Pro plan users" );
  println!();

  let providers = client.providers();
  let pro_models_for_providers = [
  "meta-llama/Meta-Llama-3-8B-Instruct",
  "mistralai/Mistral-7B-Instruct-v0.2",
  "codellama/CodeLlama-7b-Instruct-hf",
  ];

  let math_question = "If x = 13, what is x * 3?";

  for (i, model) in pro_models_for_providers.iter().enumerate()
  {
  println!( "🧪 Pro Test {}: {}", i + 1, model );
  println!( "📤 Math Question : \"{math_question}\"" );
  println!( "🌐 API Endpoint : https://api-inference.huggingface.co/v1/chat/completions" );
  println!( "📝 Request Model : {model}" );

  match providers.math_completion( model, math_question ).await
  {
      Ok( response ) =>
      {
  println!( "✅ SUCCESS! Pro model {model} responded via Providers API" );
  if let Some( choice ) = response.choices.first()
  {
          println!( "📝 Pro Response : \"{}\"", choice.message.content.chars().take(200).collect::< String >() );
          println!( "🎉 WORKING PRO MODEL FOUND: {model}" );

          // Test simple conversation too
          println!( "\n🗣️  Testing conversational capability..." );
          let messages = vec![
      ChatMessage { role : "system".to_string(), content : "You are a helpful assistant.".to_string() },
      ChatMessage { role : "user".to_string(), content : "Hello! How are you?".to_string() }
          ];

          match providers.chat_completion( model, messages, Some(100), Some(0.7), Some(0.9) ).await
          {
      Ok( chat_response ) =>
      {
              if let Some( chat_choice ) = chat_response.choices.first()
              {
        println!( "💬 Conversation Response : \"{}\"", chat_choice.message.content.chars().take(150).collect::< String >() );
              }
      },
      Err( e ) =>
      {
              println!( "❌ Conversation test failed : {e}" );
      }
          }

          break;
  }
      },
      Err( e ) =>
      {
  println!( "❌ FAILED: {model} - {e}" );
      }
  }

  println!( "{}", "=".repeat( 80 ) );
  println!();
  }

  Ok( () )
}

#[ tokio::main ]
async fn main() -> Result< (), Box< dyn std::error::Error > >
{
  println!( "🧪 HuggingFace API Test - Non-Interactive Example" );
  println!( "===============================================" );

  let api_key = load_api_key()?;

  // Build client
  let secret_key = Secret::new( api_key );
  let environment = HuggingFaceEnvironmentImpl::build( secret_key, None )?;
  let client = Client::build( environment )?;
  let inference = Inference::new( &client );

  println!( "✅ Client initialized successfully" );
  println!();

  // Test standard inference API
  test_inference_api( &inference ).await?;

  // Test new Providers API for Pro models
  test_providers_api( &client ).await?;

  println!( "\n📊 Summary:" );
  println!( "• Standard Inference API (/models/{{model}}) - Limited to free tier models like BART" );
  println!( "• Providers API (/v1/chat/completions) - Access to Pro plan models" );
  println!( "• Pro models provide proper conversational AI and mathematical reasoning" );
  println!( "• Free tier users automatically fall back to BART with enhanced prompting" );

  Ok( () )
}