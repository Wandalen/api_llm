//! # Use Case 3: Code Review and Programming Assistant
//! 
//! This example demonstrates using local LLMs as a programming assistant for code
//! review, bug detection, optimization suggestions, and code explanation. This is
//! particularly valuable for developers who need AI assistance while maintaining
//! code privacy and security.
//! 
//! ## Real-world applications:
//! - Automated code review and quality checks
//! - Bug detection and security vulnerability scanning
//! - Code optimization suggestions
//! - Documentation generation
//! - Code explanation for learning/onboarding
//! - Refactoring assistance
//! 
//! ## To run this example:
//! ```bash
//! # Make sure Ollama is running with a code-capable model
//! ollama pull codellama
//! cargo run --example code_assistant --all-features
//! ```

use api_ollama::{ OllamaClient, ChatRequest, ChatMessage, MessageRole };

const EXAMPLE_MERGE_SORT_CODE: &str = r"
fn merge_sort< T: Ord + Clone >( mut vec : Vec< T > ) -> Vec< T >
{
    if vec.len() <= 1
    {
        return vec;
    }

    let mid = vec.len() / 2;
    let right = vec.split_off(mid);
    let left = vec;

    merge(merge_sort(left), merge_sort(right))
}

fn merge< T: Ord + Clone >( left : Vec< T >, right : Vec< T > ) -> Vec< T >
{
    let mut result = Vec::with_capacity(left.len() + right.len());
    let mut left_iter = left.into_iter();
    let mut right_iter = right.into_iter();
    let mut left_item = left_iter.next();
    let mut right_item = right_iter.next();

    loop
    {
        match (left_item, right_item)
        {
            (Some(l), Some(r)) => {
                if l <= r
                {
                    result.push(l);
                    left_item = left_iter.next();
                }
                else

                {
                    result.push(r);
                    right_item = right_iter.next();
                }
            }
            (Some(l), None) => {
                result.push(l);
                result.extend(left_iter);
                break;
            }
            (None, Some(r)) => {
                result.push(r);
                result.extend(right_iter);
                break;
            }
            (None, None) => break,
        }
    }
    result
}";

async fn setup_client_and_model() -> Result< ( OllamaClient, String ), Box< dyn core::error::Error > >
{
  // Initialize client
  let mut client = OllamaClient::new( "http://localhost:11434".to_string(), OllamaClient::recommended_timeout_fast() );
  
  // Check if Ollama is available
  if !client.is_available().await
  {
    eprintln!( "❌ Ollama server is not available. Please start Ollama and try again." );
    std ::process::exit( 1 );
  }
  
  // Get available models - prefer code-specific models
  let models = client.list_models().await?;
  if models.models.is_empty()
  {
    eprintln!( "❌ No models available. Please install a model first." );
    eprintln!( "   For best results with code : ollama pull codellama" );
    eprintln!( "   Alternative : ollama pull llama3.2" );
    std ::process::exit( 1 );
  }
  
  // Try to find a code-optimized model, fall back to any available model
  let model_name = models.models
    .iter()
    .find( | m | m.name.to_lowercase().contains( "code" ) || m.name.to_lowercase().contains( "llama" ) )
    .map_or_else(|| models.models[ 0 ].name.clone(), | m | m.name.clone());
    
  println!( "✅ Using model : {model_name}" );
  
  Ok( ( client, model_name ) )
}

async fn analyze_code_samples(
  client : &mut OllamaClient,
  model_name : &str,
  code_samples : Vec< ( &str, &str ) >
) -> Result< (), Box< dyn core::error::Error > >
{
  // Analysis tasks for each code sample
  let analysis_tasks = [
    "Code Review : Identify potential bugs, error handling issues, and code quality problems.",
    "Security Analysis : Look for security vulnerabilities, unsafe practices, or potential attack vectors.",
    "Performance Optimization : Suggest performance improvements and more efficient approaches.",
    "Best Practices : Recommend coding best practices and modern language features that could be used.",
    "Refactoring : Provide a refactored version that addresses the identified issues.",
  ];
  
  // Analyze each code sample
  for ( sample_name, code ) in code_samples
  {
    println!( "\n🔍 Analyzing : {sample_name}" );
    let separator = "═".repeat( 50 );
    println!( "{separator}" );
    println!( "Code:" );
    println!( "{code}" );
    println!();
    
    for ( i, task ) in analysis_tasks.iter().enumerate()
    {
      // Add delay between requests to prevent connection exhaustion
      if i > 0
      {
        tokio ::time::sleep( tokio::time::Duration::from_millis( 500 ) ).await;
      }

      let task_num = i + 1;
      let task_title = task.split( ':' ).next().unwrap_or( "Analysis" );
      println!( "📋 Task {task_num}: {task_title}" );
      println!( "─────────────────────" );

      let messages = vec![
        ChatMessage
        {
          role : MessageRole::System,
          content : "You are an expert software engineer and security analyst. \
                     Provide detailed, actionable feedback on code quality, security, \
                     and performance. Be specific and include code examples when suggesting improvements.".to_string(),
          images : None,
          #[ cfg( feature = "tool_calling" ) ]
          tool_calls : None,
        },
        ChatMessage
        {
          role : MessageRole::User,
          content : format!( "{task}\n\nCode to analyze:\n{code}" ),
          images : None,
          #[ cfg( feature = "tool_calling" ) ]
          tool_calls : None,
        }
      ];

      let request = ChatRequest
      {
        model : model_name.to_string(),
        messages,
        stream : Some( false ),
        options : None,
        #[ cfg( feature = "tool_calling" ) ]
        tools : None,
        #[ cfg( feature = "tool_calling" ) ]
        tool_messages : None,
      };

      // Retry logic with exponential backoff
      let mut attempts : u32 = 0;
      let max_attempts : u32 = 3;
      let mut success = false;

      while attempts < max_attempts && !success
      {
        match client.chat( request.clone() ).await
        {
          Ok( response ) =>
          {
            let content = &response.message.content;
            println!( "{content}" );
            println!();
            success = true;
          }
          Err( e ) =>
          {
            attempts += 1;
            if attempts < max_attempts
            {
              let delay = 1000 * ( 2_u64.pow( attempts ) ); // Exponential backoff
              eprintln!( "⚠️  Attempt {attempts} failed for {task_title}: {e}" );
              eprintln!( "   Retrying in {delay}ms..." );
              tokio ::time::sleep( tokio::time::Duration::from_millis( delay ) ).await;
            }
            else
            {
              eprintln!( "❌ Error in analysis for {task_title} after {max_attempts} attempts : {e}" );
            }
          }
        }
      }
    }
  }
  
  Ok( () )
}

async fn demonstrate_code_explanation(
  client : &mut OllamaClient,
  model_name : &str
) -> Result< (), Box< dyn core::error::Error > >
{
  // Add delay before this request
  tokio ::time::sleep( tokio::time::Duration::from_millis( 500 ) ).await;

  println!( "\n🎓 Code Explanation Example" );
  println!( "===========================" );

  let explanation_prompt = format!(
    "Please provide a detailed, educational explanation of this Rust code. \
     Explain how the algorithm works step by step, what each function does, \
     and highlight any interesting language features being used. Make it \
     accessible for someone learning Rust.\n\nCode:\n{EXAMPLE_MERGE_SORT_CODE}"
  );
  
  let explanation_request = ChatRequest
  {
    model : model_name.to_string(),
    messages : vec![ ChatMessage
    {
      role : MessageRole::User,
      content : explanation_prompt,
      images : None,
      #[ cfg( feature = "tool_calling" ) ]
      tool_calls : None,
    } ],
    stream : Some( false ),
    options : None,
    #[ cfg( feature = "tool_calling" ) ]
    tools : None,
    #[ cfg( feature = "tool_calling" ) ]
    tool_messages : None,
  };
  
  // Retry logic with exponential backoff
  let mut attempts : u32 = 0;
  let max_attempts : u32 = 3;
  let mut success = false;

  while attempts < max_attempts && !success
  {
    match client.chat( explanation_request.clone() ).await
    {
      Ok( response ) =>
      {
        let content = &response.message.content;
        println!( "{content}" );
        success = true;
      }
      Err( e ) =>
      {
        attempts += 1;
        if attempts < max_attempts
        {
          let delay = 1000 * ( 2_u64.pow( attempts ) ); // Exponential backoff
          eprintln!( "⚠️  Attempt {attempts} failed for code explanation : {e}" );
          eprintln!( "   Retrying in {delay}ms..." );
          tokio ::time::sleep( tokio::time::Duration::from_millis( delay ) ).await;
        }
        else
        {
          eprintln!( "❌ Error in code explanation after {max_attempts} attempts : {e}" );
        }
      }
    }
  }

  Ok( () )
}

#[ tokio::main ]
async fn main() -> Result< (), Box< dyn core::error::Error > >
{
  println!( "💻 Code Review and Programming Assistant" );
  println!( "=======================================" );
  
  let ( mut client, model_name ) = setup_client_and_model().await?;
  
  // Sample code snippets for analysis
  let code_samples = vec![
    (
      "Rust Function with Potential Issues",
      r"
fn process_user_data( data : Vec< String > ) -> Vec< i32 >
{
    let mut results = Vec::new();
    for item in data
    {
        let num = item.parse::< i32 >().unwrap();
        if num > 0
        {
            results.push(num * 2);
        }
    }
    results
}
      ",
    ),
    (
      "JavaScript Function Needing Optimization", 
      r"
function findDuplicates(arr) {
    let duplicates = [];
    for (let i = 0; i < arr.length; i++)
    {
        for (let j = i + 1; j < arr.length; j++)
        {
            if (arr[i] === arr[j] && !duplicates.includes(arr[i]))
            {
                duplicates.push(arr[i]);
            }
        }
    }
    return duplicates;
}
      ",
    ),
    (
      "Python Class with Security Concerns",
      r#"
import os
import subprocess

class FileManager:
    def __init__(self, base_path):
        self.base_path = base_path
    
    def read_file(self, filename):
        filepath = self.base_path + "/" + filename
        with open(filepath, 'r') as f:
            return f.read()
    
    def execute_command(self, cmd):
        return subprocess.run(cmd, shell=True, capture_output=True, text=True)
      "#,
    ),
  ];
  
  // Run code analysis using helper function
  analyze_code_samples( &mut client, &model_name, code_samples ).await?;
  
  // Demonstrate code explanation capability using helper function
  demonstrate_code_explanation( &mut client, &model_name ).await?;
  
  println!( "\n✅ Code analysis complete!" );
  println!( "💡 This demonstrates how local LLMs can:" );
  println!( "   - Review code for bugs and security issues" );
  println!( "   - Suggest performance optimizations" );
  println!( "   - Provide educational explanations" );
  println!( "   - Assist with refactoring and best practices" );
  println!( "   - All while keeping your code private and secure!" );
  
  Ok( () )
}
