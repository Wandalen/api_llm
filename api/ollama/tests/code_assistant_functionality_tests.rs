//! Unit tests for code assistant functionality

#[ cfg( test ) ]
mod tests
{
  use api_ollama::{ OllamaClient, ChatRequest, ChatMessage, MessageRole };

  #[ tokio::test ]
  async fn test_code_assistant_can_initialize()
  {
    // Test that code assistant can set up its basic structures
    let _client = OllamaClient::new( "http://localhost:11434".to_string(), OllamaClient::recommended_timeout_fast() );
    
    // Test sample code snippets (from code_assistant example)
    let code_samples = [
      (
        "Rust Function with Potential Issues",
        r"
fn process_user_data( data : Vec< String > ) -> Vec< i32 >
{
    let mut results = Vec::new();
    for item in data
    {
        let num = item.parse::<i32>().unwrap();
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
    ];
    
    assert_eq!( code_samples.len(), 2 );
    
    // Test analysis tasks structure
    let analysis_tasks = [
      "Code Review : Identify potential bugs, error handling issues, and code quality problems.",
      "Security Analysis : Look for security vulnerabilities, unsafe practices, or potential attack vectors.",
      "Performance Optimization : Suggest performance improvements and more efficient approaches.",
      "Best Practices : Recommend coding best practices and modern language features that could be used.",
      "Refactoring : Provide a refactored version that addresses the identified issues.",
    ];
    
    assert_eq!( analysis_tasks.len(), 5 );
    
    // Test that chat request can be constructed for code analysis
    let ( _sample_name, code ) = &code_samples[ 0 ];
    
    let messages = vec![
      ChatMessage
      {
        role : MessageRole::System,
        content : "You are an expert software engineer and security analyst.".to_string(),
        images : None,
        #[ cfg( feature = "tool_calling" ) ]
        tool_calls : None,
      },
      ChatMessage
      {
        role : MessageRole::User,
        content : format!( "{}\\n\\nCode to analyze:\\n{}", analysis_tasks[ 0 ], code ),
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
    assert!( request.messages[ 1 ].content.contains( "process_user_data" ) );
  }

  #[ tokio::test ]
  async fn test_code_explanation_request()
  {
    // Test the code explanation functionality
    let complex_code = r"
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
    ";
    
    let explanation_request = ChatRequest
    {
      model : "test-model".to_string(),
      messages : vec![ ChatMessage
      {
        role : MessageRole::User,
        content : format!( 
          "Please explain this merge sort implementation in Rust. \\
           Make it educational for someone learning algorithms.\\n\\nCode:\\n{complex_code}" 
        ),
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
    
    assert!( explanation_request.messages[ 0 ].content.contains( "merge_sort" ) );
    assert!( explanation_request.messages[ 0 ].content.contains( "educational" ) );
  }
}
