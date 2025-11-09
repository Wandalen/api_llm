//! OllamaClient retry methods extension.
//!
//! Explicit retry methods for API operations.

#[ cfg( feature = "retry" ) ]
mod private
{
  use crate::client::OllamaClient;
  use crate::{ OllamaResult, ChatRequest, ChatResponse, GenerateRequest, GenerateResponse, TagsResponse, ModelInfo };

  impl OllamaClient
  {
    #[ cfg( feature = "retry" ) ]
    /// Execute chat request with retry logic (explicit retry method)
    pub async fn chat_with_retries( &mut self, request : ChatRequest ) -> OllamaResult< ChatResponse >
    {
      match &self.retry_client
      {
        Some( retry_client ) =>
        {
          let chat_operation = ||
          {
            let req = request.clone();
            let client_clone = self.clone();
            Box::pin( async move
            {
              let mut client = client_clone;
              client.chat( req ).await
            } ) as std::pin::Pin< Box< dyn std::future::Future< Output = OllamaResult< ChatResponse > > + Send > >
          };

          retry_client.execute( chat_operation ).await
        }
        None =>
        {
          // No retry configured, execute normally
          self.chat( request ).await
        }
      }
    }

    #[ cfg( feature = "retry" ) ]
    /// Execute generate request with retry logic (explicit retry method)
    pub async fn generate_with_retries( &mut self, request : GenerateRequest ) -> OllamaResult< GenerateResponse >
    {
      match &self.retry_client
      {
        Some( retry_client ) =>
        {
          let generate_operation = ||
          {
            let req = request.clone();
            let client_clone = self.clone();
            Box::pin( async move
            {
              let mut client = client_clone;
              client.generate( req ).await
            } ) as std::pin::Pin< Box< dyn std::future::Future< Output = OllamaResult< GenerateResponse > > + Send > >
          };

          retry_client.execute( generate_operation ).await
        }
        None =>
        {
          // No retry configured, execute normally
          self.generate( request ).await
        }
      }
    }

    #[ cfg( feature = "retry" ) ]
    /// Execute list models request with retry logic (explicit retry method)
    pub async fn list_models_with_retries( &mut self ) -> OllamaResult< TagsResponse >
    {
      match &self.retry_client
      {
        Some( retry_client ) =>
        {
          let list_models_operation = ||
          {
            let client_clone = self.clone();
            Box::pin( async move
            {
              let mut client = client_clone;
              client.list_models().await
            } ) as std::pin::Pin< Box< dyn std::future::Future< Output = OllamaResult< TagsResponse > > + Send > >
          };

          retry_client.execute( list_models_operation ).await
        }
        None =>
        {
          // No retry configured, execute normally
          self.list_models().await
        }
      }
    }

    #[ cfg( feature = "retry" ) ]
    /// Execute model info request with retry logic (explicit retry method)
    pub async fn model_info_with_retries( &mut self, model_name : String ) -> OllamaResult< ModelInfo >
    {
      match &self.retry_client
      {
        Some( retry_client ) =>
        {
          let model_info_operation = ||
          {
            let name = model_name.clone();
            let client_clone = self.clone();
            Box::pin( async move
            {
              let mut client = client_clone;
              client.model_info( name ).await
            } ) as std::pin::Pin< Box< dyn std::future::Future< Output = OllamaResult< ModelInfo > > + Send > >
          };

          retry_client.execute( model_info_operation ).await
        }
        None =>
        {
          // No retry configured, execute normally
          self.model_info( model_name ).await
        }
      }
    }
  }
}
