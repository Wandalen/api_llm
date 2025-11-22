//! Server-Sent Events streaming support for Anthropic API

mod private {}

#[ cfg( feature = "streaming" ) ]
crate::mod_interface!
{
  layer types;
  layer client_impl;
}

#[ cfg( not( feature = "streaming" ) ) ]
crate::mod_interface!
{
  // Empty when streaming feature is disabled
}
