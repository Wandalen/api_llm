//! OllamaClient cached content methods extension.
//!
//! Provides methods for working with cached content feature.

#[ cfg( feature = "cached_content" ) ]
mod private
{
  use crate::client::OllamaClient;
  use crate::{ OllamaResult, CachedContentRequest, CachedContentResponse, CacheInvalidationRequest, CacheInvalidationResponse, CachePerformanceMetrics };
  use error_tools::format_err;

  impl OllamaClient
  {
    /// Cache content for faster retrieval
    ///
    /// # Errors
    ///
    /// Returns an error if caching fails
    #[ inline ]
    pub async fn cache_content( &mut self, request : CachedContentRequest ) -> OllamaResult< CachedContentResponse >
    {
      let url = format!( "{}/api/cache/content", self.base_url );

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
        return Err( format_err!( "API error {}: Cache content request failed : {}", response.status().as_u16(), response.status() ) );
      }

      let cache_response : CachedContentResponse = response.json().await.map_err( | e | format_err!( "Parse error : {}", e ) )?;
      Ok( cache_response )
    }

    /// Invalidate cached content
    ///
    /// # Errors
    ///
    /// Returns an error if invalidation fails
    #[ inline ]
    pub async fn invalidate_cache( &mut self, request : CacheInvalidationRequest ) -> OllamaResult< CacheInvalidationResponse >
    {
      let url = format!( "{}/api/cache/invalidate", self.base_url );

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
        return Err( format_err!( "API error {}: Cache invalidation request failed : {}", response.status().as_u16(), response.status() ) );
      }

      let invalidation_response : CacheInvalidationResponse = response.json().await.map_err( | e | format_err!( "Parse error : {}", e ) )?;
      Ok( invalidation_response )
    }

    /// Get cache performance metrics
    ///
    /// # Errors
    ///
    /// Returns an error if metrics retrieval fails
    #[ inline ]
    pub async fn cache_metrics( &mut self ) -> OllamaResult< CachePerformanceMetrics >
    {
      let url = format!( "{}/api/cache/metrics", self.base_url );

      let request_builder = self.client
        .get( &url )
        .header( "Content-Type", "application/json" )
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
        return Err( format_err!( "API error {}: Cache metrics request failed : {}", response.status().as_u16(), response.status() ) );
      }

      let metrics : CachePerformanceMetrics = response.json().await.map_err( | e | format_err!( "Parse error : {}", e ) )?;
      Ok( metrics )
    }
  }
}
