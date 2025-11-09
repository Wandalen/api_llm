//! Code review example using real Anthropic API
//! Run with : cargo run --example `claude_code_review` --features integration

use api_claude::{ Client, CreateMessageRequest, Message };

#[ tokio::main( flavor = "current_thread" ) ]
async fn main() -> Result< (), Box< dyn core::error::Error > >
{
    println!("🔍 AI Code Review Example");
    println!("=========================");
    
    let client = Client::from_workspace()
        .expect("Must have valid ANTHROPIC_API_KEY in ../../secret/-secrets.sh or environment");
    
    let rust_code = r"
pub fn process_data(data : Vec< String >) -> Vec< String >
{
    let mut results = Vec::new();
    for item in data
    {
        let processed = item.clone().to_uppercase();
        if processed.len() > 0
        {
            results.push(processed);
        }
    }
    results
}

pub fn calculate_average(numbers : &[i32]) -> f64
{
    let sum : i32 = numbers.iter().sum();
    sum as f64 / numbers.len() as f64
}
";

    let request = CreateMessageRequest {
        model : "claude-sonnet-4-5-20250929".to_string(),
        max_tokens : 1200,
        messages : vec![
            Message::user(format!("Please review these Rust functions:\n```rust{rust_code}\n```\n\nProvide:\n1. Issues found (bugs, performance, non-idiomatic code)\n2. Specific improvement suggestions with code examples\n3. Overall assessment"))
        ],
        system : Some( vec![ api_claude::SystemContent::text( "You are a senior Rust developer and code reviewer. Analyze code for bugs, performance issues, idiomatic patterns, memory safety, and suggest specific improvements with examples." ) ] ),
        temperature : Some(0.2), // Lower temperature for focused analysis
        stream : None,
        tools : None,
        tool_choice : None,
    };
    
    println!("🔬 Analyzing Rust code with Claude...");
    let response = client.create_message(request).await?;
    
    println!("📋 Code Review Results:");
    println!("{}", "=".repeat(50));
    if let Some(text_content) = response.content.first()
    {
        if let Some(text) = &text_content.text
        {
            println!("{text}");
        }
    }
    println!("{}", "=".repeat(50));
    println!("📊 Review completed - Tokens : {} in, {} out", 
        response.usage.input_tokens, response.usage.output_tokens);
    
    Ok(())
}