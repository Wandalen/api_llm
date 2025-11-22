//! Core configuration setters for ClientBuilder.

use core::time::Duration;
use super::ClientBuilder;

impl ClientBuilder
{
    /// Sets the API key for authenticating with the Gemini API.
  #[ must_use ]
  #[ inline ]
  pub fn api_key( mut self, api_key : String ) -> Self
  {
      self.api_key = Some( api_key );
      self
  }

    /// Sets a custom base URL for the API endpoint.
  #[ must_use ]
  #[ inline ]
  pub fn base_url( mut self, base_url : String ) -> Self
  {
      self.base_url = base_url;
      self
  }

    /// Sets the timeout duration for HTTP requests.
  #[ must_use ]
  #[ inline ]
  pub fn timeout( mut self, timeout : Duration ) -> Self
  {
      self.timeout = timeout;
      self
  }
}
