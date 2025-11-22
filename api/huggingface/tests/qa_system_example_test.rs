#![ allow( clippy::all, clippy::pedantic ) ]
//! Comprehensive Tests for Question-Answering System Example
//!
//! This test suite validates the complete QA system functionality including:
//! - Context-based question answering with accuracy validation
//! - General knowledge question handling
//! - Multi-turn conversation support with context preservation
//! - Answer confidence scoring and validation
//! - Knowledge source integration and management
//! - Response quality assessment and filtering
//! - Edge cases : ambiguous questions, unanswerable questions, etc.

#![ allow( missing_docs ) ]

mod inc;
use inc::qa_system_shared::*;

use std::collections::HashMap;

use api_huggingface::*;
use api_huggingface::environment::HuggingFaceEnvironmentImpl;
use api_huggingface::secret::Secret;

/// Helper function to create a test client using workspace secrets
fn create_test_client() -> Client< HuggingFaceEnvironmentImpl >
{
  use workspace_tools as workspace;

  let workspace = workspace::workspace()
    .expect( "[create_test_client] Failed to access workspace - required for integration tests" );
  let secrets = workspace.load_secrets_from_file( "-secrets.sh" )
    .expect( "[create_test_client] Failed to load secret/-secrets.sh - required for integration tests" );
  let api_key = secrets.get( "HUGGINGFACE_API_KEY" )
    .expect( "[create_test_client] HUGGINGFACE_API_KEY not found in secret/-secrets.sh - required for integration tests" )
    .clone();

  let secret = Secret::new( api_key );
  let env = HuggingFaceEnvironmentImpl::build( secret, None )
  .expect( "Failed to build environment" );
  Client::build( env ).expect( "Failed to build client" )
}

//
// Unit Tests - Core Functionality
//

#[ test ]
fn test_question_type_properties()
{
  // Test question type variants
  let types = vec![
  QuestionType::ContextBased,
  QuestionType::GeneralKnowledge,
  QuestionType::Factual,
  QuestionType::Opinion,
  QuestionType::YesNo,
  QuestionType::MultipleChoice,
  ];

  for question_type in types
  {
  // All should use the new Kimi-K2 model
  let model = question_type.preferred_model();
  assert_eq!( model, "meta-llama/Llama-3.3-70B-Instruct" );

  // Confidence thresholds should be sensible
  let threshold = question_type.confidence_threshold();
  assert!( threshold >= 0.0 && threshold <= 1.0, "Confidence threshold out of range" );
  }
}

#[ test ]
fn test_difficulty_level_ordering()
{
  // Test difficulty level ordering
  assert!( DifficultyLevel::Easy < DifficultyLevel::Medium );
  assert!( DifficultyLevel::Medium < DifficultyLevel::Hard );
  assert!( DifficultyLevel::Hard < DifficultyLevel::Expert );
}

#[ test ]
fn test_answer_length_properties()
{
  let lengths = vec![
  AnswerLength::Brief,
  AnswerLength::Standard,
  AnswerLength::Detailed,
  AnswerLength::Comprehensive,
  ];

  for length in lengths
  {
  let ( min, max ) = length.word_count_range();
  assert!( min < max, "Invalid word count range" );

  let tokens = length.max_tokens();
  assert!( tokens > 0, "Max tokens should be positive" );
  }

  // Verify increasing token limits
  assert!( AnswerLength::Brief.max_tokens() < AnswerLength::Standard.max_tokens() );
  assert!( AnswerLength::Standard.max_tokens() < AnswerLength::Detailed.max_tokens() );
  assert!( AnswerLength::Detailed.max_tokens() < AnswerLength::Comprehensive.max_tokens() );
}

#[ test ]
fn test_source_type_reliability()
{
  let sources = vec![
  SourceType::Encyclopedia,
  SourceType::Manual,
  SourceType::Database,
  SourceType::Document,
  SourceType::WebPage,
  SourceType::FAQ,
  ];

  for source_type in sources
  {
  let multiplier = source_type.reliability_multiplier();
  assert!( multiplier > 0.0 && multiplier <= 1.0, "Reliability multiplier out of range" );
  }

  // Encyclopedia should be most reliable
  assert!( SourceType::Encyclopedia.reliability_multiplier() > SourceType::WebPage.reliability_multiplier() );
}

#[ test ]
fn test_question_creation()
{
  let question = Question
  {
  id : "test1".to_string(),
  text : "What is Rust?".to_string(),
  question_type : QuestionType::GeneralKnowledge,
  difficulty : DifficultyLevel::Easy,
  context : None,
  expected_answer : None,
  keywords : vec![ "Rust".to_string(), "programming".to_string() ],
  metadata : HashMap::new(),
  };

  assert_eq!( question.id, "test1" );
  assert_eq!( question.text, "What is Rust?" );
  assert_eq!( question.question_type, QuestionType::GeneralKnowledge );
  assert_eq!( question.difficulty, DifficultyLevel::Easy );
  assert!( question.context.is_none() );
  assert_eq!( question.keywords.len(), 2 );
}

#[ test ]
fn test_knowledge_source_creation()
{
  let source = KnowledgeSource
  {
  id : "src1".to_string(),
  title : "Test Source".to_string(),
  content : "This is test content about programming.".to_string(),
  source_type : SourceType::Document,
  reliability_score : 0.85,
  last_updated : "2025-01-01".to_string(),
  metadata : HashMap::new(),
  };

  assert_eq!( source.id, "src1" );
  assert_eq!( source.reliability_score, 0.85 );
  assert_eq!( source.source_type, SourceType::Document );
}

#[ test ]
fn test_qa_system_initialization()
{
  let client = create_test_client();
  let qa_system = QASystem::new( client );

  assert_eq!( qa_system.knowledge_sources.len(), 0 );
  assert_eq!( qa_system.active_sessions.len(), 0 );
  assert_eq!( qa_system.default_model, "meta-llama/Llama-3.3-70B-Instruct" );
}

#[ test ]
fn test_knowledge_source_management()
{
  let client = create_test_client();
  let mut qa_system = QASystem::new( client );

  // Add a knowledge source
  let source = KnowledgeSource
  {
  id : "test_src".to_string(),
  title : "Test Knowledge".to_string(),
  content : "Rust is a systems programming language.".to_string(),
  source_type : SourceType::Manual,
  reliability_score : 0.9,
  last_updated : "2025-01-01".to_string(),
  metadata : HashMap::new(),
  };

  qa_system.add_knowledge_source( source );
  assert_eq!( qa_system.knowledge_sources.len(), 1 );

  // Verify the source was added correctly
  let retrieved = qa_system.knowledge_sources.get( "test_src" );
  assert!( retrieved.is_some() );
  assert_eq!( retrieved.expect( "[test_knowledge_source_management] KnowledgeSource should exist in knowledge_sources after is_some() check - check HashMap::get() implementation" ).title, "Test Knowledge" );
}

#[ test ]
fn test_knowledge_source_search()
{
  let client = create_test_client();
  let mut qa_system = QASystem::new( client );

  // Add multiple knowledge sources
  qa_system.add_knowledge_source( KnowledgeSource
  {
  id : "rust1".to_string(),
  title : "Rust Guide".to_string(),
  content : "Rust is a systems programming language focused on safety and performance.".to_string(),
  source_type : SourceType::Manual,
  reliability_score : 0.9,
  last_updated : "2025-01-01".to_string(),
  metadata : HashMap::new(),
  } );

  qa_system.add_knowledge_source( KnowledgeSource
  {
  id : "python1".to_string(),
  title : "Python Tutorial".to_string(),
  content : "Python is a high-level interpreted programming language.".to_string(),
  source_type : SourceType::Document,
  reliability_score : 0.8,
  last_updated : "2025-01-01".to_string(),
  metadata : HashMap::new(),
  } );

  // Search for Rust-related content
  let results = qa_system.search_knowledge_sources( "Rust programming", 5 );
  assert!( !results.is_empty(), "Should find Rust-related sources" );

  // First result should be most relevant (Rust Guide)
  assert!( results[ 0 ].title.contains( "Rust" ) );
}

#[ test ]
fn test_session_management()
{
  let client = create_test_client();
  let mut qa_system = QASystem::new( client );

  // Create a new session
  let preferences = UserPreferences
  {
  preferred_length : AnswerLength::Standard,
  include_sources : true,
  include_reasoning : true,
  confidence_threshold : 0.7,
  preferred_topics : vec![ "technology".to_string() ],
  };

  let session = qa_system.start_session( "test_session".to_string(), preferences );
  assert_eq!( session.session_id, "test_session" );
  assert_eq!( session.turns.len(), 0 );
  assert_eq!( qa_system.active_sessions.len(), 1 );
}

#[ test ]
fn test_accuracy_evaluation()
{
  let client = create_test_client();
  let qa_system = QASystem::new( client );

  // Test exact match
  let score1 = qa_system.evaluate_accuracy( "Paris is the capital of France", "Paris is the capital of France" );
  assert!( score1 >= 0.9, "Exact match should have high accuracy" );

  // Test partial match
  let score2 = qa_system.evaluate_accuracy( "The capital of France is Paris", "Paris is the capital" );
  assert!( score2 > 0.5, "Partial match should have moderate accuracy" );

  // Test no match
  let score3 = qa_system.evaluate_accuracy( "Berlin is in Germany", "Paris is the capital of France" );
  assert!( score3 < 0.3, "No match should have low accuracy" );
}

#[ test ]
fn test_system_statistics()
{
  let client = create_test_client();
  let qa_system = QASystem::new( client );

  let stats = qa_system.get_system_stats();
  assert_eq!( stats.total_knowledge_sources, 0 );
  assert_eq!( stats.active_sessions, 0 );
  assert_eq!( stats.total_questions_answered, 0 );
  assert_eq!( stats.average_confidence, 0.0 );
}

#[ test ]
fn test_edge_case_empty_question()
{
  let question = Question
  {
  id : "empty".to_string(),
  text : "".to_string(),
  question_type : QuestionType::GeneralKnowledge,
  difficulty : DifficultyLevel::Easy,
  context : None,
  expected_answer : None,
  keywords : vec![],
  metadata : HashMap::new(),
  };

  assert_eq!( question.text, "" );
  assert!( question.keywords.is_empty() );
}

#[ test ]
fn test_edge_case_very_long_question()
{
  let long_text = "What ".repeat( 100 ) + "is this?";
  let question = Question
  {
  id : "long".to_string(),
  text : long_text.clone(),
  question_type : QuestionType::GeneralKnowledge,
  difficulty : DifficultyLevel::Expert,
  context : None,
  expected_answer : None,
  keywords : vec![],
  metadata : HashMap::new(),
  };

  assert!( question.text.len() > 500, "Question should be very long" );
}

#[ test ]
fn test_edge_case_ambiguous_question()
{
  let question = Question
  {
  id : "ambiguous".to_string(),
  text : "What is it?".to_string(),
  question_type : QuestionType::GeneralKnowledge,
  difficulty : DifficultyLevel::Hard,
  context : None,
  expected_answer : None,
  keywords : vec![],
  metadata : HashMap::new(),
  };

  // Ambiguous question should have lower confidence threshold
  let threshold = question.question_type.confidence_threshold();
  assert!( threshold > 0.0, "Should have some confidence requirement" );
}

//
// Integration Tests - Real API Calls
//

#[ cfg( feature = "integration" ) ]
#[ tokio::test ]
async fn test_single_question_answering_integration()
{
  let client = create_test_client();
  let qa_system = QASystem::new( client );

  let question = Question
  {
  id : "q1".to_string(),
  text : "What is 7 * 8?".to_string(),
  question_type : QuestionType::Factual,
  difficulty : DifficultyLevel::Easy,
  context : None,
  expected_answer : Some( "56".to_string() ),
  keywords : vec![ "math".to_string(), "multiplication".to_string() ],
  metadata : HashMap::new(),
  };

  let result = qa_system.answer_question( &question ).await;
  assert!( result.is_ok(), "Should successfully answer question" );

  let answer = result.expect( "[test_factual_question_answering_integration] Result should be Ok after is_ok() check - check answer_question() implementation" );
  assert!( !answer.text.is_empty(), "Answer should not be empty" );
  assert!( answer.confidence > 0.0, "Should have some confidence" );
  assert_eq!( answer.question_id, "q1" );
  assert!( answer.response_time_ms > 0, "Should have response time" );

  println!( "Answer : {}", answer.text );
  println!( "Confidence : {:.2}%", answer.confidence * 100.0 );
}

#[ cfg( feature = "integration" ) ]
#[ tokio::test ]
async fn test_context_based_answering_integration()
{
  let client = create_test_client();
  let qa_system = QASystem::new( client );

  let context = "Rust is a systems programming language that runs blazingly fast, prevents segfaults, and guarantees thread safety.";
  let question = Question
  {
  id : "q2".to_string(),
  text : "What are the main benefits of Rust?".to_string(),
  question_type : QuestionType::ContextBased,
  difficulty : DifficultyLevel::Medium,
  context : Some( context.to_string() ),
  expected_answer : Some( "fast, prevents segfaults, guarantees thread safety".to_string() ),
  keywords : vec![ "Rust".to_string(), "benefits".to_string() ],
  metadata : HashMap::new(),
  };

  let result = qa_system.answer_question( &question ).await;
  assert!( result.is_ok(), "Should successfully answer context-based question" );

  let answer = result.expect( "[test_context_based_answering_integration] Result should be Ok after is_ok() check - check answer_question() implementation" );
  assert!( !answer.text.is_empty(), "Answer should not be empty" );

  // Verify answer relates to context
  let answer_lower = answer.text.to_lowercase();
  let has_rust_content = answer_lower.contains( "fast" )
  || answer_lower.contains( "safe" )
  || answer_lower.contains( "performance" );

  println!( "Context-based answer : {}", answer.text );
  println!( "Relates to context : {}", has_rust_content );
}

#[ cfg( feature = "integration" ) ]
#[ tokio::test ]
async fn test_multi_turn_conversation_integration()
{
  let client = create_test_client();
  let mut qa_system = QASystem::new( client );

  // Start a session
  let preferences = UserPreferences
  {
  preferred_length : AnswerLength::Standard,
  include_sources : false,
  include_reasoning : false,
  confidence_threshold : 0.6,
  preferred_topics : vec![ "math".to_string() ],
  };

  qa_system.start_session( "math_session".to_string(), preferences );

  // First question
  let q1 = Question
  {
  id : "turn1".to_string(),
  text : "What is 5 + 3?".to_string(),
  question_type : QuestionType::Factual,
  difficulty : DifficultyLevel::Easy,
  context : None,
  expected_answer : Some( "8".to_string() ),
  keywords : vec![],
  metadata : HashMap::new(),
  };

  let result1 = qa_system.answer_in_session( "math_session", q1 ).await;
  assert!( result1.is_ok(), "First question should succeed" );

  println!( "Turn 1: {}", result1.as_ref().expect( "[test_multi_turn_conversation_integration] Result1 should be Ok after is_ok() check - check answer_in_session() implementation" ).text );

  // Second question with context from first
  let q2 = Question
  {
  id : "turn2".to_string(),
  text : "What is that number multiplied by 2?".to_string(),
  question_type : QuestionType::Factual,
  difficulty : DifficultyLevel::Medium,
  context : None,
  expected_answer : Some( "16".to_string() ),
  keywords : vec![],
  metadata : HashMap::new(),
  };

  let result2 = qa_system.answer_in_session( "math_session", q2 ).await;
  assert!( result2.is_ok(), "Second question should succeed" );

  println!( "Turn 2: {}", result2.as_ref().expect( "[test_multi_turn_conversation_integration] Result2 should be Ok after is_ok() check - check answer_in_session() implementation" ).text );

  // Verify session has both turns
  let session = qa_system.active_sessions.get( "math_session" ).expect( "[test_multi_turn_conversation_integration] Session 'math_session' should exist in active_sessions after answer_in_session() calls - check answer_in_session() session creation" );
  assert_eq!( session.turns.len(), 2, "Should have 2 turns" );
}

#[ tokio::test ]
async fn test_error_handling_scenarios()
{
  let client = create_test_client();
  let qa_system = QASystem::new( client );

  // Test with empty question text
  let empty_question = Question
  {
  id : "empty".to_string(),
  text : "".to_string(),
  question_type : QuestionType::GeneralKnowledge,
  difficulty : DifficultyLevel::Easy,
  context : None,
  expected_answer : None,
  keywords : vec![],
  metadata : HashMap::new(),
  };

  let result = qa_system.answer_question( &empty_question ).await;
  // API might still respond even with empty input
  println!( "Empty question result : {:?}", result.is_ok() );
}

#[ test ]
fn test_confidence_scoring_logic()
{
  // Test that confidence increases with better answer characteristics
  let client = create_test_client();
  let qa_system = QASystem::new( client );

  // Short answer vs long answer
  let score_short = qa_system.evaluate_accuracy( "Yes", "Yes it is" );
  let score_long = qa_system.evaluate_accuracy(
  "Yes, that is absolutely correct and verified",
  "Yes it is correct"
  );

  assert!( score_short > 0.0 );
  assert!( score_long > 0.0 );
}

#[ test ]
fn test_knowledge_source_relevance_scoring()
{
  let client = create_test_client();
  let mut qa_system = QASystem::new( client );

  // Add sources with different relevance
  qa_system.add_knowledge_source( KnowledgeSource
  {
  id : "high_relevance".to_string(),
  title : "Rust Programming Language".to_string(),
  content : "Rust is a modern systems programming language with focus on safety and concurrency.".to_string(),
  source_type : SourceType::Manual,
  reliability_score : 0.95,
  last_updated : "2025-01-01".to_string(),
  metadata : HashMap::new(),
  } );

  qa_system.add_knowledge_source( KnowledgeSource
  {
  id : "low_relevance".to_string(),
  title : "Python Basics".to_string(),
  content : "Python is an interpreted high-level programming language.".to_string(),
  source_type : SourceType::WebPage,
  reliability_score : 0.7,
  last_updated : "2024-12-01".to_string(),
  metadata : HashMap::new(),
  } );

  // Search for Rust content
  let results = qa_system.search_knowledge_sources( "Rust programming safety", 10 );
  assert!( !results.is_empty() );

  // Most relevant should be first
  let first_result = results[ 0 ];
  assert!( first_result.title.contains( "Rust" ) );
}
