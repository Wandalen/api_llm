//! Function calling example using real Anthropic API
//! Run with : cargo run --example `claude_function_calling` --features integration

use api_claude::{ Client, CreateMessageRequest, Message, ToolChoice, ToolDefinition };
use serde_json::json;

#[ tokio::main( flavor = "current_thread" ) ]
#[ allow( clippy::too_many_lines ) ]
async fn main() -> Result< (), Box< dyn core::error::Error > >
{
    println!("🛠️ AI Function Calling Example");
    println!("===============================");
    
    let client = Client::from_workspace()
        .expect("Must have valid ANTHROPIC_API_KEY in ../../secret/-secrets.sh or environment");
    
    // Define tools that Claude can use
    let calculator_tool = ToolDefinition
    {
        name : "calculator".to_string(),
        description : "Perform mathematical calculations and return the result".to_string(),
        input_schema : json!
        ({
            "type": "object",
            "properties": {
                "expression": {
                    "type": "string",
                    "description": "Mathematical expression to evaluate (e.g., '15 * 100 / 7', '(2847 * 0.15)', 'sqrt(144)')"
                }
            },
            "required": ["expression"]
        })
    };

    let text_analyzer_tool = ToolDefinition
    {
        name : "analyze_text".to_string(),
        description : "Analyze text for various metrics including word count, reading time, and complexity".to_string(),
        input_schema : json!
        ({
            "type": "object",
            "properties": {
                "text": {
                    "type": "string",
                    "description": "Text to analyze"
                },
                "analysis_type": {
                    "type": "string",
                    "enum": ["word_count", "sentiment", "complexity", "reading_time"],
                    "description": "Type of analysis to perform"
                }
            },
            "required": ["text", "analysis_type"]
        })
    };
    
    let weather_tool = ToolDefinition {
        name : "get_weather".to_string(),
        description : "Get current weather information for a specified location".to_string(),
        input_schema : json!({
            "type": "object",
            "properties": {
                "location": {
                    "type": "string",
                    "description": "City name or location (e.g., 'San Francisco', 'London, UK')"
                },
                "units": {
                    "type": "string",
                    "enum": ["celsius", "fahrenheit"],
                    "description": "Temperature units",
                    "default": "celsius"
                }
            },
            "required": ["location"]
        })
    };

    let request = CreateMessageRequest {
        model : "claude-sonnet-4-5-20250929".to_string(),
        max_tokens : 800,
        messages : vec![
            Message::user("I need help with several tasks : 1) Calculate 15% of 2,847 for a tip calculation, 2) Analyze the sentiment of this review : 'I absolutely love using Rust for systems programming! The memory safety guarantees make me feel confident, and the performance is outstanding.', and 3) Get the weather for San Francisco".to_string())
        ],
        tools : Some(vec![calculator_tool, text_analyzer_tool, weather_tool]),
        tool_choice : Some(ToolChoice::Auto),
        stream : None,
        system : Some( vec![ api_claude::SystemContent::text( "You are a helpful assistant that can use tools to help users. Always explain what you're doing and provide clear results." ) ] ),
        temperature : Some(0.7),
    };
    
    println!("🤖 Making API call with function calling capabilities...");
    let response = client.create_message(request).await?;
    
    println!("📋 Claude's Response:");
    println!("{}", "=".repeat(40));
    
    for (i, content) in response.content.iter().enumerate()
    {
        match content.r#type.as_str()
        {
            "text" => {
                if let Some(text) = &content.text
                {
                    println!("💬 Text Response {}:", i + 1);
                    println!("{text}");
                    println!();
                }
            }
            "tool_use" => {
                println!("🔧 Tool Call {} detected (detailed parsing not implemented in this example)", i + 1);
            }
            _ => { let content_type = &content.r#type; println!("📄 Other content type : {content_type}"); }
        }
    }
    
    println!("{}", "=".repeat(40));
    println!("📊 Token usage : {} input, {} output", 
        response.usage.input_tokens, response.usage.output_tokens);
    let model = &response.model; println!("⚡ Model : {model}");
    let response_id = &response.id; println!("🎯 Response ID: {response_id}");
    
    Ok(())
}