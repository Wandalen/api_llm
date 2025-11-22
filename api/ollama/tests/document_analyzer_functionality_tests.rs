//! Unit tests for document analyzer functionality

#[ cfg( test ) ]
mod tests
{
  use api_ollama::{ OllamaClient, ChatRequest, ChatMessage, MessageRole };
  use std::fs;
  use std::path::Path;

  #[ tokio::test ]
  async fn test_document_analyzer_can_initialize()
  {
    // Test that document analyzer can set up its basic structures
    let _client = OllamaClient::new( "http://localhost:11434".to_string(), OllamaClient::recommended_timeout_fast() );
    
    // Test sample document creation (from document_analyzer example)
    let sample_document = r"
Artificial Intelligence (AI) has rapidly evolved from a theoretical concept to a transformative 
technology that impacts nearly every aspect of modern life. Machine learning, a subset of AI, 
enables computers to learn and improve from experience without being explicitly programmed.

Deep learning, which uses neural networks with multiple layers, has revolutionized fields such 
as image recognition, natural language processing, and autonomous systems. Companies across 
industries are leveraging AI to automate processes, enhance decision-making, and create new 
products and services.

However, the rapid advancement of AI also presents challenges. Concerns about job displacement, 
algorithmic bias, data privacy, and the concentration of AI capabilities in the hands of a few 
large corporations have sparked important discussions about responsible AI development.

The future of AI holds immense promise, with potential breakthroughs in healthcare, climate 
change mitigation, scientific research, and education. As we continue to develop these 
technologies, it is crucial to ensure they are developed and deployed in ways that benefit 
humanity as a whole while minimizing potential risks and negative consequences.
    ".trim();
    
    assert!( sample_document.len() > 1000 );
    
    // Test analysis tasks structure
    let analysis_tasks = [
      ( "Summary", "Please provide a concise summary of this document in 2-3 sentences." ),
      ( "Key Points", "Extract the 3-5 most important key points from this document as bullet points." ),
      ( "Sentiment", "What is the overall sentiment or tone of this document?" ),
      ( "Categories", "What categories or topics does this document cover?" ),
      ( "Action Items", "Are there any implied action items or recommendations in this document?" ),
    ];
    
    assert_eq!( analysis_tasks.len(), 5 );
    
    // Test that chat request can be constructed without making API call
    let messages = vec![
      ChatMessage
      {
        role : MessageRole::System,
        content : "You are an expert document analyst.".to_string(),
        images : None,
        #[ cfg( feature = "tool_calling" ) ]
        tool_calls : None,
      },
      ChatMessage
      {
        role : MessageRole::User,
        content : format!( "{}\\n\\nDocument:\\n{}", analysis_tasks[ 0 ].1, sample_document ),
        images : None,
        #[ cfg( feature = "tool_calling" ) ]
        tool_calls : None,
      }
    ];
    
    let request = ChatRequest
    {
      model : "test-model".to_string(),
      messages,
      stream : Some( false ),
      options : None,
      #[ cfg( feature = "tool_calling" ) ]
      tools : None,
      #[ cfg( feature = "tool_calling" ) ]
      tool_messages : None,
    };
    
    assert_eq!( request.model, "test-model" );
    assert_eq!( request.messages.len(), 2 );
  }

  #[ tokio::test ]
  async fn test_sample_document_creation()
  {
    // Test that sample document can be created and read
    let test_file = "test_sample_document.txt";
    
    let sample_content = r"Test document for analysis.

This is a test document that contains multiple paragraphs
and should be analyzable by the document analyzer.

It has different topics and concepts that can be summarized.";
    
    // Create sample document
    fs ::write( test_file, sample_content ).expect( "Failed to write test document" );
    
    // Verify it exists
    assert!( Path::new( test_file ).exists() );
    
    // Read it back
    let loaded_content = fs::read_to_string( test_file ).expect( "Failed to read test document" );
    assert_eq!( loaded_content, sample_content );
    
    // Clean up
    fs ::remove_file( test_file ).expect( "Failed to remove test document" );
  }
}
