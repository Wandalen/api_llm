//! Example of updating the session configuration using the OpenAI API.
//!
//! Run with:
//! `cargo run --example realtime_session_update`
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
    RealtimeClientEventSessionUpdate,
    RealtimeServerEvent,
  },

};

use tracing_subscriber::{ EnvFilter, fmt }; // Added for logging

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

  // 2. Create the request payload to initiate the session with initial settings.
  tracing ::info!( "Building initial realtime session request..." );
  let initial_request = RealtimeSessionCreateRequest::former()
  .model( "gpt-4o-realtime-preview".to_string() )
  .temperature( 0.7 ) // Initial temperature
  .output_audio_format( "pcm16" )
  .form();

  tracing ::info!( "Sending request to OpenAI API to create session..." );
  // 3. Call the API endpoint to get session details.
  let session = client.realtime().create( initial_request ).await?;

  tracing ::info!( "Creating Realtime WebSocket Session Client..." );
  let token = session.client_secret.value;
  // 4. Establish the WebSocket connection using the session token.
  let session_client  = WsSession::connect( client.environment().clone(), Some( &token ) ).await?;

  // --- Prepare the session update ---
  let new_temperature = 0.9;
  let new_output_format = "g711_alaw"; // Change output format

  // 5. Prepare the update payload using RealtimeSessionCreateRequest struct.
  //    Only include the fields you want to update.
  let session_update_payload = RealtimeSessionCreateRequest::former()
  .temperature( new_temperature )
  .output_audio_format( new_output_format )
  // .instructions("Be extremely concise.") // Example : update instructions too
  .form();

  // 6. Prepare the client event to update the session.
  let su_update = RealtimeClientEventSessionUpdate::former()
  .session( session_update_payload ) // Embed the update payload
  .form();

  tracing ::info!( temp = new_temperature, output_format = new_output_format, "Sending session.update event..." );
  // 7. Send the session update event over the WebSocket.
  session_client.session_update( su_update ).await?;

  // 8. Loop to read responses, specifically looking for the SessionUpdated confirmation.
  tracing ::info!( "Waiting for session.updated confirmation..." );
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
          RealtimeServerEvent::SessionUpdated( updated_event ) =>
          {
            println!( "\n--- Session Updated Confirmation Received ---" );
            println!( "{updated_event:?}" );
            // Verify the updated fields
            let updated_session = updated_event.session;
            let temp_matches = updated_session.temperature == Some( new_temperature );
            let format_matches = updated_session.output_audio_format.as_deref() == Some( new_output_format );

            if temp_matches && format_matches
            {
              println!( "Successfully received session.updated confirmation with expected changes." );
              confirmation_received = true;
              break; // Break after receiving confirmation
            }
            else
            {
              eprintln!( "Received session.updated confirmation, but changes did not match request fully (Temp match : {}, Format match : {}).", temp_matches, format_matches);
              // Decide how to handle partial matches, here we break but don't confirm success.
              break;
            }
          }
          // Handle SessionCreated (initial event after connection)
          RealtimeServerEvent::SessionCreated( session_info ) =>
          {
            println!( "\n--- Received Session Info (Initial) ---" );
            println!( "{session_info:?}" );
          }
          _ => { println!( "\n--- Received Other Event (while waiting for session update confirmation) --- \n{event:?}" ); }
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
    eprintln!("Loop finished without receiving session.updated confirmation with expected changes.");
    return Err( OpenAIError::WsInvalidMessage( "Did not receive expected session update confirmation".to_string() ) );
  }

  Ok( () )
}
