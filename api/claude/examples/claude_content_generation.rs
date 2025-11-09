//! Content generation example using real Anthropic API
//! Run with : cargo run --example `claude_content_generation` --features integration

use api_claude::{ Client, CreateMessageRequest, Message };

#[ tokio::main( flavor = "current_thread" ) ]
async fn main() -> Result< (), Box< dyn core::error::Error > >
{
    println!("🚀 AI Content Generation Example");
    println!("=================================");
    
    // Load API key from workspace secrets or environment
    let client = Client::from_workspace()
        .expect("Must have valid ANTHROPIC_API_KEY in ../../secret/-secrets.sh or environment");
    
    let request = CreateMessageRequest
    {
        model : "claude-sonnet-4-5-20250929".to_string(),
        max_tokens : 1000,
        messages : vec![
            Message::user(
                "Write a technical blog post introduction about why Rust is ideal for building AI applications. Focus on memory safety, performance, and async capabilities. Make it engaging and informative.".to_string()
            )
        ],
        temperature : Some(0.7),
        stream : None,
        tools : None,
        tool_choice : None,
        system : Some( vec![ api_claude::SystemContent::text( "You are a technical writer specializing in systems programming and AI. Write in a clear, engaging style suitable for developers." ) ] ),
    };
    
    println!("📝 Generating technical blog post...");
    let response = client.create_message(request).await?;
    
    println!("✅ Generated Content:");
    println!("{}", "=".repeat(60));
    if let Some(text_content) = response.content.first()
    {
        if let Some(text) = &text_content.text
        {
            println!("{text}");
        }
    }
    println!("{}", "=".repeat(60));
    println!("📊 Token usage : {} input, {} output", 
        response.usage.input_tokens, response.usage.output_tokens);
    println!("⚡ Model : {}", response.model);
    
    Ok(())
}