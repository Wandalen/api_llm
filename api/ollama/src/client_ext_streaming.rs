//! OllamaClient streaming methods extension.
//!
//! Provides streaming capabilities for chat and generate endpoints.

#[ cfg( feature = "streaming" ) ]
mod private
{
  use core::pin::Pin;
  use futures_util::Stream;
  use futures_util::StreamExt;
  use crate::client::OllamaClient;
  use crate::{ OllamaResult, ChatRequest, ChatResponse, GenerateRequest, GenerateResponse };
  use error_tools::format_err;

  impl OllamaClient
  {
    /// Send streaming chat request
    ///
    /// # Errors
    ///
    /// Returns an error if the request fails or the response stream is invalid
    #[ inline ]
    pub async fn chat_stream( &mut self, mut request : ChatRequest ) -> OllamaResult< Pin< Box< dyn Stream< Item = OllamaResult< ChatResponse > > + Send > > >
    {
      request.stream = Some( true );
      let url = format!( "{}/api/chat", self.base_url );

      let request_builder = self.client
        .post( &url )
        .header( "Content-Type", "application/json" )
        .json( &request )
        .timeout( self.timeout );
      #[ cfg( feature = "secret_management" ) ]
      let request_builder = self.apply_authentication( request_builder );
      #[ cfg( not( feature = "secret_management" ) ) ]
      let request_builder = request_builder;

      let response = request_builder
        .send()
        .await
        .map_err( | e | format_err!( "Network error : {}", e ) )?;

      if !response.status().is_success()
      {
        return Err( format_err!( "API error {}: Streaming chat request failed : {}", response.status().as_u16(), response.status() ) );
      }

      let byte_stream = response.bytes_stream();

      let event_stream = byte_stream
        .map( | chunk_result |
        {
          match chunk_result
          {
            Ok( bytes ) =>
            {
              let chunk_str = core::str::from_utf8( bytes.as_ref() )
                .map_err( | e | format_err!( "Stream error : UTF-8 decode error : {}", e ) )?;

              let chunk = chunk_str.trim();
              if chunk.is_empty()
              {
                return Err( format_err!( "Stream error : Empty chunk" ) );
              }

              let response : ChatResponse = serde_json::from_str( chunk ).map_err( | e | format_err!( "Parse error : {}", e ) )?;
              Ok( response )
            },
            Err( e ) => Err( format_err!( "Stream error : Stream chunk error : {}", e ) ),
          }
        })
        .filter_map( | result |
        {
          async move
          {
            match result
            {
              Ok( response ) => Some( Ok( response ) ),
              Err( e ) => Some( Err( e ) ),
            }
          }
        });

      Ok( Box::pin( event_stream ) )
    }

    /// Send streaming generation request
    ///
    /// # Errors
    ///
    /// Returns an error if the request fails or the response stream is invalid
    #[ inline ]
    pub async fn generate_stream( &mut self, mut request : GenerateRequest ) -> OllamaResult< Pin< Box< dyn Stream< Item = OllamaResult< GenerateResponse > > + Send > > >
    {
      request.stream = Some( true );
      let url = format!( "{}/api/generate", self.base_url );

      let request_builder = self.client
        .post( &url )
        .header( "Content-Type", "application/json" )
        .json( &request )
        .timeout( self.timeout );
      #[ cfg( feature = "secret_management" ) ]
      let request_builder = self.apply_authentication( request_builder );
      #[ cfg( not( feature = "secret_management" ) ) ]
      let request_builder = request_builder;

      let response = request_builder
        .send()
        .await
        .map_err( | e | format_err!( "Network error : {}", e ) )?;

      if !response.status().is_success()
      {
        return Err( format_err!( "API error {}: Streaming generate request failed : {}", response.status().as_u16(), response.status() ) );
      }

      let byte_stream = response.bytes_stream();

      let event_stream = byte_stream
        .map( | chunk_result |
        {
          match chunk_result
          {
            Ok( bytes ) =>
            {
              let chunk_str = core::str::from_utf8( bytes.as_ref() )
                .map_err( | e | format_err!( "Stream error : UTF-8 decode error : {}", e ) )?;

              let chunk = chunk_str.trim();
              if chunk.is_empty()
              {
                return Err( format_err!( "Stream error : Empty chunk" ) );
              }

              let response : GenerateResponse = serde_json::from_str( chunk ).map_err( | e | format_err!( "Parse error : {}", e ) )?;
              Ok( response )
            },
            Err( e ) => Err( format_err!( "Stream error : Stream chunk error : {}", e ) ),
          }
        })
        .filter_map( | result |
        {
          async move
          {
            match result
            {
              Ok( response ) => Some( Ok( response ) ),
              Err( e ) => Some( Err( e ) ),
            }
          }
        });

      Ok( Box::pin( event_stream ) )
    }
  }
}
