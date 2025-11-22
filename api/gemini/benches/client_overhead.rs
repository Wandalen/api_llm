#![allow(missing_docs)]

use criterion::{ criterion_group, criterion_main, Criterion };
use api_gemini::client::Client;
use api_gemini::models::*;

fn benchmark_request_building( c: &mut Criterion )
{
  c.bench_function( "build_generate_content_request", |b|
  {
    b.iter( ||
    {
      GenerateContentRequest
      {
        contents: vec!
        [
        Content
        {
          parts: vec!
          [
          Part
          {
            text: Some( "Hello, Gemini!".to_string() ),
            ..Default::default()
          }
          ],
          role: "user".to_string(),
        }
        ],
        ..Default::default()
      }
    } );
  } );
}

fn benchmark_client_creation( c: &mut Criterion )
{
  c.bench_function( "create_client_builder", |b|
  {
    b.iter( ||
    {
      Client::builder()
    } );
  } );
}

fn benchmark_response_parsing( c: &mut Criterion )
{
  let json_response = r#"{
    "candidates": [{
      "content": {
      "parts": [{"text": "Hello! How can I help you?"}],
        "role": "model"
      },
      "finishReason": "STOP",
      "index": 0
    }],
    "usageMetadata": {
      "promptTokenCount": 5,
      "candidatesTokenCount": 10,
      "totalTokenCount": 15
    }
  }"#;

  c.bench_function( "parse_generate_content_response", |b|
  {
    b.iter( ||
    {
      serde_json ::from_str::< GenerateContentResponse >( json_response ).unwrap()
    } );
  } );
}

criterion_group!( benches, benchmark_request_building, benchmark_client_creation, benchmark_response_parsing );
criterion_main!( benches );
