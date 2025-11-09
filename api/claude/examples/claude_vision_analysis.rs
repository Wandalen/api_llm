//! Vision analysis example using real Anthropic API
//! Run with : cargo run --example `claude_vision_analysis` --features "integration,vision"

#[ cfg( feature = "vision" ) ]
use api_claude::{ Client, CreateMessageRequest, Message, ImageContent, ImageSource };

#[ tokio::main( flavor = "current_thread" ) ]
async fn main() -> Result< (), Box< dyn core::error::Error > >
{
    #[ cfg( not( feature = "vision" ) ) ]
    {
        println!("❌ Vision feature not enabled. Run with --features vision");
        return Ok( () );
    }
    
    #[ cfg( feature = "vision" ) ]
    vision_example().await
}

#[ cfg( feature = "vision" ) ]
async fn vision_example() -> Result< (), Box< dyn core::error::Error > >
{
    println!("👁️ AI Vision Analysis Example");
    println!("==============================");
    
    let client = Client::from_workspace()
        .expect("Must have valid ANTHROPIC_API_KEY in ../../secret/-secrets.sh or environment");
    
    // For demo purposes, we'll use a simple 1x1 red pixel PNG image
    // In real applications, you'd load actual images from files
    let test_image_base64 = "iVBORw0KGgoAAAANSUhEUgAAAAEAAAABCAYAAAAfFcSJAAAADUlEQVR42mNk+M9QDwADhgGAWjR9awAAAABJRU5ErkJggg==";
    
    println!("🖼️ Creating image content from base64 data...");
    let image_content = ImageContent::new(ImageSource::png(test_image_base64));

    let request = CreateMessageRequest {
        model : "claude-sonnet-4-5-20250929".to_string(), // Vision-capable model
        max_tokens : 800,
        messages : vec![
            Message::user_with_image(
                "Please analyze this image in detail. Describe what you see including colors, shapes, patterns, and any other visual elements. Also suggest what this type of image might be used for in applications.".to_string(),
                image_content
            )
        ],
        temperature : Some(0.3),
        stream : None,
        tools : None,
        tool_choice : None,
        system : Some( vec![ api_claude::SystemContent::text( "You are an expert visual analyst and UI/UX specialist. Provide detailed, technical descriptions of images and their potential applications." ) ] ),
    };
    
    println!("🔍 Analyzing image with Claude Vision...");
    let response = client.create_message(request).await?;
    
    println!("🎨 Vision Analysis Results:");
    println!("{}", "=".repeat(50));
    if let Some(text_content) = response.content.first()
    {
        if let Some(text) = &text_content.text
        {
            println!("{text}");
        }
    }
    
    println!("{}", "=".repeat(50));
    println!("📊 Analysis Summary:");
    println!("   • Input tokens : {}", response.usage.input_tokens);
    println!("   • Output tokens : {}", response.usage.output_tokens);
    println!("   • Model used : {}", response.model);
    println!("   • Response ID: {}", response.id);
    
    // Example 2: Analyzing a more complex scenario
    println!("\n🔄 Running second analysis with different prompt...");

    let technical_request = CreateMessageRequest {
        model : "claude-sonnet-4-5-20250929".to_string(),
        max_tokens : 600,
        messages : vec![
            Message::user_with_image(
                "From a technical perspective, what can you tell me about this image's properties? Consider aspects like resolution, color depth, compression, and potential use cases in web development or mobile apps.".to_string(),
                ImageContent::new(ImageSource::png(test_image_base64))
            )
        ],
        temperature : Some(0.2),
        stream : None,
        tools : None,
        tool_choice : None,
        system : Some( vec![ api_claude::SystemContent::text( "You are a technical image processing expert. Focus on technical aspects and practical applications." ) ] ),
    };
    
    let tech_response = client.create_message(technical_request).await?;
    
    println!("🔧 Technical Analysis:");
    println!("{}", "-".repeat(40));
    if let Some(text_content) = tech_response.content.first()
    {
        if let Some(text) = &text_content.text
        {
            println!("{text}");
        }
    }
    println!("{}", "-".repeat(40));
    
    println!("✅ Vision analysis complete!");
    println!("💡 Tip : In real applications, load images with:");
    println!("   let image_data = std::fs::read(\"path/to/image.png\")?;");
    println!("   let base64_image = base64::encode(image_data);");
    
    Ok(())
}