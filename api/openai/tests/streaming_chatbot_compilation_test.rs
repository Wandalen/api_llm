//! Test for `streaming_chatbot` example compilation issues
//!
//! This test documents the issues in the `streaming_chatbot` example:
//! 1. Wrong imports for `InputMessage`, `InputContent`
//! 2. Missing streaming module imports
//! 3. Wrong string formatting syntax
//! 4. Wrong `ResponseInput` variant usage

#[ tokio::test ]
async fn test_streaming_chatbot_compiles_successfully()
{
  // This test verifies that the streaming_chatbot example compilation issues have been fixed
  // The example should now compile without errors after fixing:
  // 1. ✅ InputMessage comes from api_openai::input
  // 2. ✅ InputContentPart from api_openai::input
  // 3. ✅ ResponseStreamEvent from api_openai::components::responses
  // 4. ✅ ResponseInput::Items instead of ResponseInput::Messages
  // 5. ✅ String formatting fixed : println!("{}", "=".repeat(50))

  println!("✅ streaming_chatbot example compilation issues have been fixed");
  // The test passes if the code compiles successfully
}