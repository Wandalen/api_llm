//! Example of clearing the input audio buffer using the OpenAI API.
//!
//! Run with:
//! `cargo run --example realtime_input_audio_buffer_clear`
//!
//! Make sure you have set the `OPENAI_API_KEY` environment variable
//! or have a `secret/-secret.sh` file with the key.

use api_openai::ClientApiAccessors;
#[ allow( unused_imports ) ]
use api_openai::
{
  client ::Client,
  error ::OpenAIError,
  api ::realtime::{ RealtimeClient, ws::WsSession },
  components ::realtime_shared::
  {
    RealtimeSessionCreateRequest,
    RealtimeClientEventInputAudioBufferAppend,
    RealtimeClientEventInputAudioBufferClear,
    RealtimeServerEvent,
  },
  components ::common::ModelIds,
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
  .input_audio_format( "pcm16" ) // Necessary for appending audio first
  .temperature( 0.7 )
  .form();

  tracing ::info!( "Sending request to OpenAI API to create session..." );
  // 3. Call the API endpoint to get session details.
  let session = client.realtime().create( request ).await?;

  tracing ::info!( "Creating Realtime WebSocket Session Client..." );
  let token = session.client_secret.value;
  // 4. Establish the WebSocket connection using the session token.
  let session_client  = WsSession::connect( client.environment().clone(), Some( &token ) ).await?;

  let dummy_audio_bytes = include_bytes!("data/example.wav");
  let audio_base64 = base64_engine.encode( &dummy_audio_bytes );

  let iaba_append = RealtimeClientEventInputAudioBufferAppend::former()
  .audio( audio_base64 )
  .form();
  tracing ::info!( "Sending a preliminary input_audio_buffer.append event..." );
  session_client.input_audio_buffer_append( iaba_append ).await?;
  // Give a tiny moment for processing, though not strictly necessary
  tokio ::time::sleep( tokio::time::Duration::from_millis( 50 ) ).await;
  tracing ::info!( "Preliminary audio append sent." );

  // 5. Prepare the client event to clear the audio buffer.
  // We can add a client-side event_id for tracking, though it's optional.
  let client_event_id = "clear-example-id";
  let iabc_clear = RealtimeClientEventInputAudioBufferClear::former()
  .event_id( client_event_id ) // Optional : Client-generated event ID
  .form();

  tracing ::info!( event_id = %client_event_id, "Sending input_audio_buffer.clear event..." );
  // 6. Send the clear event over the WebSocket.
  session_client.input_audio_buffer_clear( iabc_clear ).await?;

  // 7. Loop to read responses, specifically looking for the clear confirmation.
  tracing ::info!( "Waiting for input_audio_buffer.cleared confirmation..." );
  let mut confirmation_received = false;
  loop
  {
    let response = session_client.read_event().await;
    match response
    {
      Ok( Some( event ) ) =>
      {
        match event
        {
          RealtimeServerEvent::InputAudioBufferCleared( cleared_event ) =>
          {
            println!( "\n--- Clear Confirmation Received ---" );
            println!( "{cleared_event:?}" );
            // Optionally check if server event_id relates to our client_event_id if needed,
            // though the server event_id is server-generated and unique.
            // Just receiving the event type is usually sufficient confirmation.
            println!( "Successfully received input_audio_buffer.cleared confirmation." );
            confirmation_received = true;
            break; // Break after receiving confirmation
          }
          // Handle other events
          _ => { println!( "\n--- Received Other Event (while waiting for clear confirmation) --- \n{event:?}" ); }
        }
      }
      Ok( None ) =>
      {
        println!( "\nWebSocket connection closed by server." );
        break; // Exit loop if connection closed
      }
      Err( e ) =>
      {
        eprintln!( "\nError reading from WebSocket : {:?}", e );
        return Err( e ); // Propagate the error
      }
    }
  }

  if !confirmation_received
  {
    eprintln!("Loop finished without receiving input_audio_buffer.cleared confirmation.");
    return Err( OpenAIError::WsInvalidMessage( "Did not receive expected clear confirmation".to_string() ) );
  }

  Ok( () )
}
