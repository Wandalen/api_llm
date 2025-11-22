//! Secret management for `HuggingFace` API keys and sensitive data.

mod private
{
use crate::error::{ Result, HuggingFaceError };
use secrecy::ExposeSecret;
use serde::{ Deserialize, Serialize };
use std::fmt;

/// Secure wrapper for API keys and other sensitive data
#[ derive( Clone, Serialize, Deserialize ) ]
pub struct Secret
{
  #[ serde( with = "secret_serde" ) ]
  inner : secrecy::SecretString,
}

impl Secret
{
  /// Create a new secret from a string
  #[ inline ]
  #[ must_use ]
  pub fn new( value : impl Into< String > ) -> Self
  {
  Self
  {
      inner : secrecy::SecretString::new( value.into().into() ),
  }
  }
  
  /// Load secret from environment variable
  ///
  /// # Arguments
  /// - `var_name`: Name of the environment variable to read
  ///
  /// # Errors
  /// Returns error if the environment variable is not set or is empty
  #[ inline ]
  pub fn load_from_env( var_name : &str ) -> Result< Self >
  {
  let value = std::env::var( var_name )
      .map_err( | e | HuggingFaceError::Authentication( 
  format!( "Environment variable '{var_name}' not found : {e}" ) 
      ) )?;
  
  if value.trim().is_empty()
  {
      return Err( HuggingFaceError::Authentication( 
  format!( "Environment variable '{var_name}' is empty" ) 
      ) );
  }
  
  Ok( Self::new( value ) )
  }
  
  /// Expose the secret value (use with caution)
  #[ inline ]
  #[ must_use ]
  pub fn expose_secret( &self ) -> &str
  {
  self.inner.expose_secret()
  }
  
  /// Check if the secret appears to be valid (basic validation)
  #[ inline ]
  #[ must_use ]
  pub fn is_valid( &self ) -> bool
  {
  let secret = self.inner.expose_secret();
  !secret.trim().is_empty() && secret.len() >= 8
  }
  
  /// Get the length of the secret (for validation without exposure)
  #[ inline ]
  #[ must_use ]
  pub fn len( &self ) -> usize
  {
  self.inner.expose_secret().len()
  }
  
  /// Check if the secret is empty
  #[ inline ]
  #[ must_use ]
  pub fn is_empty( &self ) -> bool
  {
  self.inner.expose_secret().is_empty()
  }
}

impl fmt::Debug for Secret
{
  #[ inline ]
  fn fmt( &self, f : &mut fmt::Formatter< '_ > ) -> fmt::Result
  {
  f.debug_struct( "Secret" )
      .field( "inner", &"[REDACTED]" )
      .finish()
  }
}

impl fmt::Display for Secret
{
  #[ inline ]
  fn fmt( &self, f : &mut fmt::Formatter< '_ > ) -> fmt::Result
  {
  write!( f, "[REDACTED]" )
  }
}

impl From< String > for Secret
{
  #[ inline ]
  fn from( value : String ) -> Self
  {
  Self::new( value )
  }
}

impl From< &str > for Secret
{
  #[ inline ]
  fn from( value : &str ) -> Self
  {
  Self::new( value.to_string() )
  }
}

/// Custom serde module for secret serialization/deserialization
mod secret_serde
{
  // use secrecy::ExposeSecret;
  use serde::{ Deserializer, Serializer, Deserialize };

  #[ inline ]
  pub fn serialize< S >( _secret : &secrecy::SecretString, serializer : S ) -> Result< S::Ok, S::Error >
  where
  S : Serializer,
  {
  serializer.serialize_str( "[REDACTED]" )
  }

  #[ inline ]
  pub fn deserialize< 'de, D >( deserializer : D ) -> Result< secrecy::SecretString, D::Error >
  where
  D : Deserializer< 'de >,
  {
  let s = String::deserialize( deserializer )?;
  Ok( secrecy::SecretString::new( s.into() ) )
  }
}

/// Load `HuggingFace` API key from standard environment variable
#[ inline ]
/// # Errors
/// Returns error if environment variable is not set or invalid
pub fn load_huggingface_api_key() -> Result< Secret >
{
  Secret::load_from_env( "HUGGINGFACE_API_KEY" )
}

} // end mod private

crate::mod_interface!
{
  exposed use
  {
  private::Secret,
  private::load_huggingface_api_key,
  };
}