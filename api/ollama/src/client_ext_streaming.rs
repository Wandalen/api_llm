//! OllamaClient streaming methods extension.
//!
//! Provides streaming capabilities for chat and generate endpoints.

#[ cfg( feature = "streaming" ) ]
mod private
{
  use core::pin::Pin;
  use futures_util::Stream;
  use crate::client::OllamaClient;
  use crate::{ OllamaResult, ChatRequest, ChatResponse, GenerateRequest, GenerateResponse };
  use error_tools::format_err;
  use core::task::{ Context, Poll };
  use futures_util::stream::Stream as FuturesStream;

  /// Helper stream wrapper that buffers incomplete lines for newline-delimited JSON parsing
  struct LineBufferedJsonStream< S, T, B, E >
  where
    S : Stream< Item = Result< B, E > > + Unpin,
    B : AsRef< [u8] >,
    E : core::fmt::Display,
    T : serde::de::DeserializeOwned,
  {
    inner : S,
    buffer : String,
    _phantom : core::marker::PhantomData< ( T, B, E ) >,
  }

  // Safe to implement Unpin because all fields are Unpin
  impl< S, T, B, E > Unpin for LineBufferedJsonStream< S, T, B, E >
  where
    S : Stream< Item = Result< B, E > > + Unpin,
    B : AsRef< [u8] >,
    E : core::fmt::Display,
    T : serde::de::DeserializeOwned,
  {
  }

  impl< S, T, B, E > LineBufferedJsonStream< S, T, B, E >
  where
    S : Stream< Item = Result< B, E > > + Unpin,
    B : AsRef< [u8] >,
    E : core::fmt::Display,
    T : serde::de::DeserializeOwned,
  {
    fn new( inner : S ) -> Self
    {
      Self
      {
        inner,
        buffer : String::new(),
        _phantom : core::marker::PhantomData,
      }
    }
  }

  impl< S, T, B, E > FuturesStream for LineBufferedJsonStream< S, T, B, E >
  where
    S : Stream< Item = Result< B, E > > + Unpin,
    B : AsRef< [u8] >,
    E : core::fmt::Display,
    T : serde::de::DeserializeOwned,
  {
    type Item = OllamaResult< T >;

    fn poll_next( mut self : Pin< &mut Self >, cx : &mut Context< '_ > ) -> Poll< Option< Self::Item > >
    {
      // Safe to use get_mut because we implemented Unpin
      let this = self.as_mut().get_mut();

      loop
      {
        // Check if we have a complete line in buffer
        if let Some( newline_pos ) = this.buffer.find( '\n' )
        {
          let line = this.buffer[ ..newline_pos ].trim().to_string();
          this.buffer = this.buffer[ newline_pos + 1.. ].to_string();

          if !line.is_empty()
          {
            match serde_json::from_str::< T >( &line )
            {
              Ok( response ) => return Poll::Ready( Some( Ok( response ) ) ),
              Err( e ) => return Poll::Ready( Some( Err( format_err!( "Parse error : {}", e ) ) ) ),
            }
          }
          continue;
        }

        // Need more data, poll inner stream
        match Pin::new( &mut this.inner ).poll_next( cx )
        {
          Poll::Ready( Some( Ok( bytes ) ) ) =>
          {
            match core::str::from_utf8( bytes.as_ref() )
            {
              Ok( chunk_str ) => this.buffer.push_str( chunk_str ),
              Err( e ) => return Poll::Ready( Some( Err( format_err!( "Stream error : UTF-8 decode error : {}", e ) ) ) ),
            }
          },
          Poll::Ready( Some( Err( e ) ) ) => return Poll::Ready( Some( Err( format_err!( "Stream error : Stream chunk error : {}", e ) ) ) ),
          Poll::Ready( None ) =>
          {
            // Stream ended, check if buffer has remaining data
            if !this.buffer.trim().is_empty()
            {
              let remaining = this.buffer.trim().to_string();
              this.buffer.clear();
              match serde_json::from_str::< T >( &remaining )
              {
                Ok( response ) => return Poll::Ready( Some( Ok( response ) ) ),
                Err( e ) => return Poll::Ready( Some( Err( format_err!( "Parse error : {}", e ) ) ) ),
              }
            }
            return Poll::Ready( None );
          },
          Poll::Pending => return Poll::Pending,
        }
      }
    }
  }

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
      let event_stream = LineBufferedJsonStream::new( byte_stream );

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
      let event_stream = LineBufferedJsonStream::new( byte_stream );

      Ok( Box::pin( event_stream ) )
    }
  }
}
