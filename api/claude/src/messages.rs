//! Message types for Anthropic API

mod private {}

crate::mod_interface!
{
  layer content;
  layer tools_and_messages;
}
