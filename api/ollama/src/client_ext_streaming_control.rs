//! OllamaClient extension for controlled streaming.
//!
//! Provides controlled streaming capabilities with pause/resume/cancel operations.

#[ cfg( all( feature = "streaming", feature = "streaming_control" ) ) ]
mod private
{
  use error_tools::format_err;
  use crate::client::OllamaClient;
  use crate::chat::{ ChatRequest, ChatResponse };
  use crate::stream_control::{ StreamControl, ControlledStream };

  /// Extension to OllamaClient for controlled streaming
  impl OllamaClient
  {
    /// Create a chat stream with control capabilities
    #[ inline ]
    pub async fn chat_stream_controlled(
      &mut self,
      request : ChatRequest
    ) -> crate::OllamaResult< ControlledStream< crate::OllamaResult< ChatResponse > > >
    {
      let regular_stream = self.chat_stream( request ).await?;
      let control = StreamControl::new();
      control.start().await.map_err( | e | format_err!( "Failed to start controlled stream : {}", e ) )?;

      Ok( ControlledStream::new( regular_stream, control ) )
    }
  }
}
