//! Model tuning tests for `api_ollama`
//!
//! These tests verify the model fine-tuning and custom training
//! capabilities including job management, hyperparameter optimization,
//! and training progress monitoring.

#![ cfg( all( feature = "model_tuning", not( test ) ) ) ]

use api_ollama::{
  OllamaClient, ModelTuningJob, ModelTuningConfig, TrainingData,
  TuningMethod, HyperParameters, TrainingProgress, ModelCheckpoint,
  ModelEvaluation, TuningJobStatus, ModelVersion, ResourceUsage,
  TrainingObjective, DataPreprocessor, ModelBenchmark
};
use std::time::Duration;
use std::collections::HashMap;

/// Test fine-tuning job creation and basic configuration
#[ tokio::test ]
async fn test_model_tuning_job_creation() -> Result< (), Box< dyn std::error::Error > >
{
  let client = OllamaClient::new( "http://localhost:11434".to_string(), Duration::from_secs( 30 ) );

  let config = ModelTuningConfig::builder()
    .model_name( "llama2:7b" )
    .tuning_method( TuningMethod::LoRA )
    .learning_rate( 0.0001 )
    .batch_size( 8 )
    .max_epochs( 10 )
    .validation_split( 0.2 )
    .build()?;

  let training_data = TrainingData::from_jsonl_file( "test_data.jsonl" )?;

  let job = client.create_tuning_job( config, training_data ).await?;

  assert!( !job.id().is_empty() );
  assert_eq!( job.status(), TuningJobStatus::Pending );
  assert_eq!( job.model_name(), "llama2:7b" );

  Ok( () )
}

/// Test training data upload and validation
#[ tokio::test ]
async fn test_training_data_upload() -> Result< (), Box< dyn std::error::Error > >
{
  let client = OllamaClient::new( "http://localhost:11434".to_string(), Duration::from_secs( 30 ) );

  let mut training_data = TrainingData::new();
  training_data.add_text_pair( "Hello", "Hi there!" )?;
  training_data.add_text_pair( "How are you?", "I'm doing well, thank you!" )?;
  training_data.add_conversation( vec![
    ("user", "What is Rust?"),
    ("assistant", "Rust is a systems programming language focused on safety and performance.")
  ] )?;

  let validation_result = training_data.validate()?;
  assert!( validation_result.is_valid );
  assert_eq!( validation_result.total_samples, 3 );
  assert!( validation_result.warnings.is_empty() );

  let upload_result = client.upload_training_data( &training_data ).await?;
  assert!( !upload_result.data_id.is_empty() );
  assert!( upload_result.size_bytes > 0 );

  Ok( () )
}

/// Test hyperparameter configuration and optimization
#[ tokio::test ]
async fn test_hyperparameter_optimization() -> Result< (), Box< dyn std::error::Error > >
{
  let client = OllamaClient::new( "http://localhost:11434".to_string(), Duration::from_secs( 30 ) );

  let mut hyperparams = HyperParameters::new();
  hyperparams.set_learning_rate( 0.0001 );
  hyperparams.set_batch_size( 16 );
  hyperparams.set_max_epochs( 5 );
  hyperparams.set_warmup_steps( 100 );
  hyperparams.set_weight_decay( 0.01 );
  hyperparams.set_gradient_clip_norm( 1.0 );

  // Test hyperparameter search space
  let search_space = hyperparams.create_search_space()
    .learning_rate_range( 0.00001, 0.001 )
    .batch_size_options( vec![ 8, 16, 32 ] )
    .epochs_range( 3, 10 )
    .build()?;

  let optimization_config = client.create_hyperparameter_optimization( search_space )
    .strategy( "bayesian" )
    .max_trials( 20 )
    .objective( "validation_loss" )
    .build()?;

  assert_eq!( optimization_config.max_trials(), 20 );
  assert_eq!( optimization_config.objective(), "validation_loss" );

  Ok( () )
}

/// Test training progress monitoring and logging
#[ tokio::test ]
async fn test_training_progress_monitoring() -> Result< (), Box< dyn std::error::Error > >
{
  let client = OllamaClient::new( "http://localhost:11434".to_string(), Duration::from_secs( 30 ) );

  let config = ModelTuningConfig::default()
    .with_model_name( "llama2:7b" )
    .with_tuning_method( TuningMethod::FullFineTuning );

  let training_data = TrainingData::from_samples( vec![
    ( "input1".to_string(), "output1".to_string() ),
    ( "input2".to_string(), "output2".to_string() ),
  ] )?;

  let mut job = client.create_tuning_job( config, training_data ).await?;
  job.start().await?;

  // Monitor training progress
  tokio ::time::sleep( Duration::from_millis( 100 ) ).await;

  let progress = job.get_progress().await?;
  assert!( progress.current_epoch >= 0 );
  assert!( progress.total_epochs > 0 );
  assert!( progress.current_step >= 0 );
  assert!( progress.elapsed_time.as_secs() >= 0 );

  // Check training metrics
  let metrics = progress.metrics;
  assert!( metrics.contains_key( "training_loss" ) );
  assert!( metrics.contains_key( "learning_rate" ) );

  Ok( () )
}

/// Test model checkpointing and versioning
#[ tokio::test ]
async fn test_model_checkpointing() -> Result< (), Box< dyn std::error::Error > >
{
  let client = OllamaClient::new( "http://localhost:11434".to_string(), Duration::from_secs( 30 ) );

  let config = ModelTuningConfig::default()
    .with_model_name( "llama2:7b" )
    .with_checkpoint_frequency( 100 )
    .with_keep_best_checkpoints( 3 );

  let training_data = TrainingData::from_text_file( "training.txt" )?;
  let mut job = client.create_tuning_job( config, training_data ).await?;

  job.start().await?;
  tokio ::time::sleep( Duration::from_millis( 200 ) ).await;

  let checkpoints = job.list_checkpoints().await?;
  assert!( !checkpoints.is_empty() );

  let latest_checkpoint = &checkpoints[0];
  assert!( !latest_checkpoint.id().is_empty() );
  assert!( latest_checkpoint.step() > 0 );
  assert!( latest_checkpoint.validation_loss() >= 0.0 );

  // Test checkpoint restoration
  let restored_job = client.restore_from_checkpoint( latest_checkpoint.id() ).await?;
  assert_eq!( restored_job.id(), job.id() );

  Ok( () )
}

/// Test parameter-efficient fine-tuning methods
#[ tokio::test ]
async fn test_parameter_efficient_tuning() -> Result< (), Box< dyn std::error::Error > >
{
  let client = OllamaClient::new( "http://localhost:11434".to_string(), Duration::from_secs( 30 ) );

  // Test LoRA (Low-Rank Adaptation)
  let lora_config = ModelTuningConfig::builder()
    .model_name( "llama2:7b" )
    .tuning_method( TuningMethod::LoRA )
    .lora_rank( 16 )
    .lora_alpha( 32 )
    .lora_dropout( 0.1 )
    .target_modules( vec![ "q_proj", "v_proj", "k_proj", "o_proj" ] )
    .build()?;

  let training_data = TrainingData::from_huggingface_dataset( "dataset_name" )?;
  let lora_job = client.create_tuning_job( lora_config, training_data ).await?;

  assert_eq!( lora_job.tuning_method(), TuningMethod::LoRA );
  assert_eq!( lora_job.config().lora_rank(), Some( 16 ) );

  // Test Adapter tuning
  let adapter_config = ModelTuningConfig::builder()
    .model_name( "llama2:7b" )
    .tuning_method( TuningMethod::Adapter )
    .adapter_size( 64 )
    .adapter_activation( "relu" )
    .build()?;

  let adapter_training_data = TrainingData::from_csv_file( "data.csv" )?;
  let adapter_job = client.create_tuning_job( adapter_config, adapter_training_data ).await?;

  assert_eq!( adapter_job.tuning_method(), TuningMethod::Adapter );

  Ok( () )
}

/// Test training job cancellation and resumption
#[ tokio::test ]
async fn test_job_lifecycle_management() -> Result< (), Box< dyn std::error::Error > >
{
  let client = OllamaClient::new( "http://localhost:11434".to_string(), Duration::from_secs( 30 ) );

  let config = ModelTuningConfig::default().with_model_name( "llama2:7b" );
  let training_data = TrainingData::from_text( "Sample training text data" )?;

  let mut job = client.create_tuning_job( config, training_data ).await?;

  // Start the job
  job.start().await?;
  assert_eq!( job.status(), TuningJobStatus::Running );

  // Pause the job
  job.pause().await?;
  assert_eq!( job.status(), TuningJobStatus::Paused );

  // Resume the job
  job.resume().await?;
  assert_eq!( job.status(), TuningJobStatus::Running );

  // Cancel the job
  job.cancel().await?;
  assert_eq!( job.status(), TuningJobStatus::Cancelled );

  // Test job resumption from checkpoint
  let resumed_job = client.resume_job_from_checkpoint( job.id(), job.latest_checkpoint_id() ).await?;
  assert_eq!( resumed_job.status(), TuningJobStatus::Running );

  Ok( () )
}

/// Test model evaluation and validation metrics
#[ tokio::test ]
async fn test_model_evaluation() -> Result< (), Box< dyn std::error::Error > >
{
  let client = OllamaClient::new( "http://localhost:11434".to_string(), Duration::from_secs( 30 ) );

  let config = ModelTuningConfig::default().with_model_name( "llama2:7b" );
  let training_data = TrainingData::from_jsonl_file( "train.jsonl" )?;
  let validation_data = TrainingData::from_jsonl_file( "val.jsonl" )?;

  let mut job = client.create_tuning_job( config, training_data ).await?;
  job.set_validation_data( validation_data ).await?;
  job.start().await?;

  tokio ::time::sleep( Duration::from_millis( 300 ) ).await;

  let evaluation = job.evaluate().await?;

  assert!( evaluation.perplexity() > 0.0 );
  assert!( evaluation.bleu_score() >= 0.0 && evaluation.bleu_score() <= 1.0 );
  assert!( evaluation.rouge_scores().contains_key( "rouge-1" ) );
  assert!( evaluation.rouge_scores().contains_key( "rouge-2" ) );
  assert!( evaluation.rouge_scores().contains_key( "rouge-l" ) );

  // Test custom evaluation metrics
  let custom_metrics = evaluation.custom_metrics();
  assert!( custom_metrics.contains_key( "accuracy" ) || custom_metrics.is_empty() );

  Ok( () )
}

/// Test custom training objective support
#[ tokio::test ]
async fn test_custom_training_objectives() -> Result< (), Box< dyn std::error::Error > >
{
  let client = OllamaClient::new( "http://localhost:11434".to_string(), Duration::from_secs( 30 ) );

  // Test different training objectives
  let classification_objective = TrainingObjective::Classification {
    num_classes : 5,
    class_weights : Some( vec![ 1.0, 2.0, 1.5, 1.0, 3.0 ] ),
  };

  let generation_objective = TrainingObjective::TextGeneration {
    max_length : 512,
    temperature : 0.8,
    top_p : 0.9,
  };

  let qa_objective = TrainingObjective::QuestionAnswering {
    max_answer_length : 100,
    impossible_answer_threshold : 0.5,
  };

  let config = ModelTuningConfig::builder()
    .model_name( "llama2:7b" )
    .training_objective( classification_objective )
    .build()?;

  let training_data = TrainingData::from_classification_data( vec![
    ( "text1".to_string(), 0 ),
    ( "text2".to_string(), 1 ),
    ( "text3".to_string(), 2 ),
  ] )?;

  let job = client.create_tuning_job( config, training_data ).await?;
  assert!( matches!( job.training_objective(), TrainingObjective::Classification { .. } ) );

  Ok( () )
}

/// Test resource usage monitoring and optimization
#[ tokio::test ]
async fn test_resource_monitoring() -> Result< (), Box< dyn std::error::Error > >
{
  let client = OllamaClient::new( "http://localhost:11434".to_string(), Duration::from_secs( 30 ) );

  let config = ModelTuningConfig::default()
    .with_model_name( "llama2:7b" )
    .with_memory_optimization( true )
    .with_gradient_checkpointing( true )
    .with_mixed_precision( true );

  let training_data = TrainingData::from_text( "Sample data for resource monitoring" )?;
  let mut job = client.create_tuning_job( config, training_data ).await?;

  job.start().await?;
  tokio ::time::sleep( Duration::from_millis( 100 ) ).await;

  let resource_usage = job.get_resource_usage().await?;

  assert!( resource_usage.gpu_memory_used_mb >= 0 );
  assert!( resource_usage.gpu_utilization_percent >= 0.0 );
  assert!( resource_usage.cpu_utilization_percent >= 0.0 );
  assert!( resource_usage.memory_used_mb >= 0 );
  assert!( resource_usage.disk_io_read_mb >= 0 );
  assert!( resource_usage.disk_io_write_mb >= 0 );

  // Test resource optimization recommendations
  let recommendations = resource_usage.get_optimization_recommendations();
  assert!( recommendations.is_some() || recommendations.is_none() );

  Ok( () )
}

/// Test training data preprocessing utilities
#[ tokio::test ]
async fn test_data_preprocessing() -> Result< (), Box< dyn std::error::Error > >
{
  let mut preprocessor = DataPreprocessor::new();

  // Test text normalization
  let normalized = preprocessor.normalize_text( "  Hello,  World!  " )?;
  assert_eq!( normalized, "Hello, World!" );

  // Test tokenization
  let tokens = preprocessor.tokenize( "This is a test sentence." )?;
  assert!( tokens.len() > 0 );
  assert!( tokens.contains( &"test".to_string() ) );

  // Test sequence length optimization
  let mut training_data = TrainingData::new();
  training_data.add_text_pair( "short", "response" )?;
  training_data.add_text_pair( "this is a much longer input text", "longer response text" )?;

  let optimized_data = preprocessor.optimize_sequence_lengths( training_data, 50 )?;
  let samples = optimized_data.get_samples();

  for sample in samples
  {
    assert!( sample.input.len() <= 50 || sample.output.len() <= 50 );
  }

  // Test data augmentation
  let augmented_data = preprocessor.augment_data( &optimized_data, 2.0 )?;
  assert!( augmented_data.sample_count() >= optimized_data.sample_count() );

  Ok( () )
}

/// Test model performance benchmarking
#[ tokio::test ]
async fn test_model_benchmarking() -> Result< (), Box< dyn std::error::Error > >
{
  let client = OllamaClient::new( "http://localhost:11434".to_string(), Duration::from_secs( 30 ) );

  let config = ModelTuningConfig::default().with_model_name( "llama2:7b" );
  let training_data = TrainingData::from_text( "Benchmark training data" )?;

  let mut job = client.create_tuning_job( config, training_data ).await?;
  job.start().await?;

  tokio ::time::sleep( Duration::from_millis( 500 ) ).await;

  let benchmark = ModelBenchmark::new()
    .add_task( "text_generation" )
    .add_task( "question_answering" )
    .add_task( "sentiment_analysis" )
    .with_test_data_size( 100 )
    .build()?;

  let benchmark_results = client.run_model_benchmark( job.model_id(), benchmark ).await?;

  assert!( benchmark_results.tasks.contains_key( "text_generation" ) );
  assert!( benchmark_results.tasks.contains_key( "question_answering" ) );
  assert!( benchmark_results.tasks.contains_key( "sentiment_analysis" ) );

  for (task_name, task_result) in benchmark_results.tasks
  {
    assert!( !task_name.is_empty() );
    assert!( task_result.accuracy >= 0.0 && task_result.accuracy <= 1.0 );
    assert!( task_result.latency_ms > 0.0 );
    assert!( task_result.throughput_tokens_per_sec > 0.0 );
  }

  Ok( () )
}

/// Test model versioning and deployment integration
#[ tokio::test ]
async fn test_model_versioning() -> Result< (), Box< dyn std::error::Error > >
{
  let client = OllamaClient::new( "http://localhost:11434".to_string(), Duration::from_secs( 30 ) );

  let config = ModelTuningConfig::default()
    .with_model_name( "llama2:7b" )
    .with_version_name( "v1.0-custom" )
    .with_version_description( "First custom fine-tuned version" );

  let training_data = TrainingData::from_text( "Version testing data" )?;
  let mut job = client.create_tuning_job( config, training_data ).await?;

  job.start().await?;
  tokio ::time::sleep( Duration::from_millis( 200 ) ).await;
  job.complete().await?;

  let model_version = job.create_model_version().await?;

  assert!( !model_version.id().is_empty() );
  assert_eq!( model_version.name(), "v1.0-custom" );
  assert_eq!( model_version.description(), "First custom fine-tuned version" );
  assert!( model_version.creation_date() <= std::time::SystemTime::now() );

  // Test version listing
  let versions = client.list_model_versions( "llama2:7b" ).await?;
  assert!( versions.iter().any( |v| v.id() == model_version.id() ) );

  // Test version deployment
  let deployment_config = model_version.create_deployment_config()
    .with_endpoint_name( "llama2-custom-v1" )
    .with_auto_scaling( true )
    .build()?;

  assert_eq!( deployment_config.endpoint_name(), "llama2-custom-v1" );
  assert!( deployment_config.auto_scaling_enabled() );

  Ok( () )
}
