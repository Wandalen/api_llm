//! Example of appending audio to the input buffer using the OpenAI API.
//!
//! Run with:
//! `cargo run --example realtime_input_audio_buffer_append`
//!
//! Make sure you have set the `OPENAI_API_KEY` environment variable
//! or have a `secret/-secret.sh` file with the key.
//!
//! **NOTE:** This event does not have a direct confirmation server event like
//! `input_audio_buffer.appended`. Confirmation is implicit or through subsequent
//! events like VAD detection or transcription results. This example sends the
//! data and waits briefly for any follow-up events.

use api_openai::ClientApiAccessors;
use api_openai::
{
  client ::Client,
  error ::OpenAIError,
  api ::realtime::{ RealtimeClient, ws::WsSession },
  components ::realtime_shared::
  {
    RealtimeSessionCreateRequest,
    RealtimeClientEventInputAudioBufferAppend,
  },

};


use tracing_subscriber::{ EnvFilter, fmt }; // Added for logging
use base64::{ engine::general_purpose::STANDARD as base64_engine, Engine as _ }; // For base64 encoding

#[ tokio::main( flavor = "current_thread" ) ]
async fn main() -> Result< (), OpenAIError >
{
  // Setup tracing for logging
  fmt()
  .with_env_filter( EnvFilter::from_default_env().add_directive( "api_openai=trace".parse().unwrap() ) )
  .init();

  // Load environment variables
  dotenv ::from_filename( "./secret/-secret.sh" ).ok();

  // 1. Create a new OpenAI client.
  tracing ::info!( "Initializing client..." );
  let client = Client::new();

  // 2. Create the request payload to initiate the session, configuring audio input.
  tracing ::info!( "Building realtime session request..." );
  let request = RealtimeSessionCreateRequest::former()
  .model( "gpt-4o-realtime-preview".to_string() )
  .input_audio_format( "pcm16" ) // Specify the format of the audio we'll send
  // Optional : Configure transcription if you want to see results
  // .input_audio_transcription( RealtimeSessionInputAudioTranscription::former().model( "whisper-1" ).form() )
  // Optional : Configure VAD
  // .turn_detection( RealtimeSessionTurnDetection::former().r#type( "server_vad" ).form() )
  .temperature( 0.7 )
  .form();

  tracing ::info!( "Sending request to OpenAI API to create session..." );
  // 3. Call the API endpoint to get session details.
  let session = client.realtime().create( request ).await?;

  tracing ::info!( "Creating Realtime WebSocket Session Client..." );
  let token = session.client_secret.value;
  // 4. Establish the WebSocket connection using the session token.
  let session_client  = WsSession::connect( client.environment().clone(), Some( &token ) ).await?;

  // --- Prepare Dummy Audio Data ---
  let dummy_audio_bytes = include_bytes!("data/example.wav");
  let audio_base64 = base64_engine.encode( &dummy_audio_bytes );

  // 5. Prepare the client event to append the audio data.
  let iaba_append = RealtimeClientEventInputAudioBufferAppend::former()
  .audio( audio_base64 ) // Provide the base64 encoded audio
  .form();

  tracing ::info!( "Sending input_audio_buffer.append event..." );
  // 6. Send the append event over the WebSocket.
  session_client.input_audio_buffer_append( iaba_append ).await?;
  tracing ::info!( "Audio append event sent." );

  // 7. Wait briefly and read any subsequent events (like VAD or transcription).
  //    There's no direct 'appended' confirmation.
  tracing ::info!( "Waiting briefly for any subsequent events (no direct confirmation expected)..." );
  let wait_duration = tokio::time::Duration::from_secs( 2 ); // Wait for 2 seconds
  let start_time = tokio::time::Instant::now();
  loop
  {
    // Check timeout first
if start_time.elapsed() > wait_duration
{
      println!( "\nWait duration elapsed. No specific confirmation event for append." );
      break;
    }

    // Try reading with a small timeout to avoid blocking forever if nothing comes
    let read_timeout = tokio::time::Duration::from_millis( 100 );
    match tokio::time::timeout( read_timeout, session_client.read_event() ).await
    {
      Ok( response_result ) => match response_result
      {
        Ok(Some(event)) =>
        {
          println!( "\n--- Received Subsequent Event ---" );
          println!( "{event:?}" );
          // Depending on session config, you might see:
          // - InputAudioBufferSpeechStarted/Stopped
          // - ConversationItemInputAudioTranscriptionDelta/Completed
          // etc.
        }
        Ok( None ) =>
        {
          println!( "\nWebSocket connection closed by server unexpectedly." );
          return Err( OpenAIError::WsConnectionClosed );
        }
        Err( e ) =>
        {
          eprintln!("\nError reading from WebSocket : {:?}", e);
          return Err(e);
        }
      },
      Err( _ ) =>
      {
        // Timeout elapsed for this read attempt, continue checking overall wait duration
        continue;
      }
    }
  }

  // Since there's no direct confirmation, we usually consider the send successful if no error occurred.
  println!( "Successfully sent input_audio_buffer.append event." );
  Ok( () )
}
