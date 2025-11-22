mod private
{
}

#[ cfg( feature = "enabled" ) ]
crate::mod_interface!
{
  /// Common types shared across API components.
  layer common;

  /// Chat completion types (requests, responses, messages).
  layer chat;

  /// Model information types (model details, listings).
  layer models;
}
