//! Example of explicitly creating a response using the OpenAI API.
//!
//! Run with:
//! `cargo run --example realtime_response_create`
//!
//! Make sure you have set the `OPENAI_API_KEY` environment variable
//! or have a `secret/-secret.sh` file with the key.
//!
//! **NOTE:** This is often not needed if using server-side VAD with
//! `turn_detection.create_response : true` (the default). This example shows
//! how to manually trigger a response, potentially with overrides.

use api_openai::ClientApiAccessors;
use api_openai::
{
  client ::Client,
  error ::OpenAIError,
  api ::realtime::{ RealtimeClient, ws::WsSession },
  components ::realtime_shared::
  {
    RealtimeSessionCreateRequest,
    RealtimeConversationItemContent,
    RealtimeConversationItem,
    RealtimeClientEventConversationItemCreate,
    RealtimeResponseCreateParams,
    RealtimeClientEventResponseCreate,
    RealtimeServerEvent,
  },

};

use tracing_subscriber::{ EnvFilter, fmt }; // Added for logging
use std::sync::{ Arc, Mutex }; // To share the response ID

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

  // 2. Create the request payload to initiate the session.
  //    May disable auto-response creation if explicitly triggering.
  tracing ::info!( "Building realtime session request..." );
  let request = RealtimeSessionCreateRequest::former()
  .model( "gpt-4o-realtime-preview".to_string() )
  // Example : Configure VAD *not* to automatically create responses
  // .turn_detection(RealtimeSessionTurnDetection::former().r#type("server_vad").create_response(false).form())
  .temperature( 0.7 )
  .output_audio_format( "pcm16" ) // Request audio output
  .form();

  tracing ::info!( "Sending request to OpenAI API to create session..." );
  // 3. Call the API endpoint to get session details.
  let session = client.realtime().create( request ).await?;

  tracing ::info!( "Creating Realtime WebSocket Session Client..." );
  let token = session.client_secret.value;
  // 4. Establish the WebSocket connection using the session token.
  let session_client  = WsSession::connect( client.environment().clone(), Some( &token ) ).await?;

  // --- Optional : Create a user message first to provide context ---
  let content = RealtimeConversationItemContent::former()
  .r#type( "input_text" )
  .text( "What's the weather like in San Francisco?" )
  .form();
  let ci_to_create = RealtimeConversationItem::former()
  .r#type( "message" )
  .role( "user" )
  .content( vec![ content ] )
  .form();
  let cic_create = RealtimeClientEventConversationItemCreate::former()
  .item( ci_to_create )
  .form();
  tracing ::info!( "Sending preliminary conversation.item.create event..." );
  session_client.conversation_item_create( cic_create ).await?;
  // Wait briefly for the item to be potentially processed server-side
  tokio ::time::sleep( tokio::time::Duration::from_millis( 100 ) ).await;


  // 5. Prepare the client event to create a response.
  //    Optionally add overrides for this specific response.
  let response_params = RealtimeResponseCreateParams::former()
  .temperature( 0.9 ) // Override temperature for this response
  .modalities( vec![ "text".to_string(), "audio".to_string() ] ) // Specify expected modalities
  // Example : Override context - use 'auto' (default) or 'none' or provide specific item references
  // .conversation("auto")
  // .input(vec![RealtimeConversationItemWithReference::Reference { id : "user_item_id_123".to_string(), r#type : "item_reference".to_string() }])
  .form();

  let rc_create = RealtimeClientEventResponseCreate::former()
  .response( response_params )
  .form();

  tracing ::info!( "Sending response.create event..." );
  // 6. Send the response create event over the WebSocket.
  session_client.response_create( rc_create ).await?;

  // 7. Loop to read responses, specifically looking for the ResponseCreated confirmation.
  tracing ::info!( "Waiting for response.created confirmation..." );
  let mut confirmation_received = false;
  let created_response_id = Arc::new( Mutex::new( None::< String > ) );

  loop
  {
    let response = session_client.read_event().await;
    match response
    {
      Ok( Some( event ) ) =>
      {
        match event
        {
          RealtimeServerEvent::ResponseCreated( created_event ) =>
          {
            println!( "\n--- Response Created Confirmation Received ---" );
            println!( "{created_event:?}" );
            let response_id = created_event.response.id.clone();
            println!( "Successfully received response.created confirmation. Response ID: {}", response_id );
            *created_response_id.lock().unwrap() = Some( response_id );
            confirmation_received = true;
            // Don't break yet, let's also wait for ResponseDone for completeness
            // break;
          }
          RealtimeServerEvent::ResponseDone( done_event ) =>
          {
            println!( "\n--- Response Done Event Received ---" );
            println!( "{done_event:?}" );
            let expected_id = created_response_id.lock().unwrap().clone();
            if let Some(expected) = expected_id
            {
              if done_event.response.id == expected
              {
                println!( "Received response.done for the created response (ID: {}). Status : {}", expected, done_event.response.status );
                // Now we can break, as the response is complete.
                break;
              }
              else
              {
                println!("Received response.done for a different response ID: {}", done_event.response.id);
              }
            }
            else
            {
              println!( "Received response.done before response.created was confirmed." );
              // If confirmation_received is true, we might assume this is the one.
              if confirmation_received { break; }
            }
          }
          // Handle response deltas
          RealtimeServerEvent::ResponseTextDelta( _ ) |
          RealtimeServerEvent::ResponseAudioDelta( _ ) =>
          {
            println!( "\n--- Received Delta --- \n{event:?}" );
          }
          _ => { println!( "\n--- Received Other Event (while waiting for response confirmation) --- \n{event:?}" ); }
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
    eprintln!( "Loop finished without receiving response.created confirmation." );
    // Check if response ID was captured even if the loop broke on ResponseDone before created confirmation was logged/set
if created_response_id.lock().unwrap().is_none()
{
      return Err( OpenAIError::WsInvalidMessage( "Did not receive expected response.created confirmation".to_string() ) );
    }
    else
    {
      println!( "Warning : Response.created confirmation flag not set, but response ID was likely captured before loop exit." );
    }
  }

  Ok( () )
}
