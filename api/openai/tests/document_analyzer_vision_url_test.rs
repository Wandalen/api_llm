//! Test for `document_analyzer_vision` example URL issue
//!
//! This test reproduces the issue where the document analyzer fails
//! when trying to access certain image URLs that return HTTP 400 errors.

use api_openai::ClientApiAccessors;
use api_openai::
{
  Client,
  components::
  {
    responses::{ CreateResponseRequest, ResponseInput },
    input::{ InputItem, InputMessage, InputContentPart, InputText, InputImage },
  },
};

#[ tokio::test ]
async fn test_document_analyzer_vision_new_url_works()
{
  // Load secret using the comprehensive fallback system  
  let secret = api_openai::secret::Secret::load_with_fallbacks("OPENAI_API_KEY")
    .expect("OPENAI_API_KEY should be available in workspace secrets");
  
  let env = api_openai::environment::OpenaiEnvironmentImpl::build(secret, None, None, api_openai::environment::OpenAIRecommended::base_url().to_string(), api_openai::environment::OpenAIRecommended::realtime_base_url().to_string())
    .expect("Failed to create environment");
  let client = Client::build(env).expect("Failed to create client");

  // This is a proven working JPEG URL that should work
  let new_working_url = "https://upload.wikimedia.org/wikipedia/commons/thumb/d/dd/Gfp-wisconsin-madison-the-nature-boardwalk.jpg/320px-Gfp-wisconsin-madison-the-nature-boardwalk.jpg";
  
  println!("🔍 Testing new working image URL: {new_working_url}");

  let request = CreateResponseRequest::former()
    .model("gpt-4o".to_string())
    .input(
      ResponseInput::Items(
        vec![
          InputItem::Message(
            InputMessage::former()
              .role("user")
              .content(
                vec![
                  InputContentPart::Text(
                    InputText::former()
                      .text("Please analyze this image.".to_string())
                      .form()
                  ),
                  InputContentPart::Image(
                    InputImage::former()
                      .image_url(new_working_url.to_string())
                      .detail("high")
                      .form()
                  ),
                ]
              )
              .form()
          ),
        ]
      )
    )
    .form();

  let result = client.responses().create(request).await;
  
  match result
  {
    Ok(_) =>
    {
      println!("✅ Image URL works correctly");
    },
    Err(e) =>
    {
      let error_msg = format!("{e:?}");
      panic!("New URL failed unexpectedly: {error_msg}");
    }
  }
}

#[ tokio::test ]
async fn test_document_analyzer_vision_working_url()
{
  // Load secret using the comprehensive fallback system  
  let secret = api_openai::secret::Secret::load_with_fallbacks("OPENAI_API_KEY")
    .expect("OPENAI_API_KEY should be available in workspace secrets");
  
  let env = api_openai::environment::OpenaiEnvironmentImpl::build(secret, None, None, api_openai::environment::OpenAIRecommended::base_url().to_string(), api_openai::environment::OpenAIRecommended::realtime_base_url().to_string())
    .expect("Failed to create environment");
  let client = Client::build(env).expect("Failed to create client");

  // This is a working URL that should succeed
  let working_image_url = "https://upload.wikimedia.org/wikipedia/commons/thumb/1/15/Cat_August_2010-4.jpg/256px-Cat_August_2010-4.jpg";
  
  println!("🔍 Testing working image URL: {working_image_url}");

  let request = CreateResponseRequest::former()
    .model("gpt-4o".to_string())
    .input(
      ResponseInput::Items(
        vec![
          InputItem::Message(
            InputMessage::former()
              .role("user")
              .content(
                vec![
                  InputContentPart::Text(
                    InputText::former()
                      .text("Please analyze this image.".to_string())
                      .form()
                  ),
                  InputContentPart::Image(
                    InputImage::former()
                      .image_url(working_image_url.to_string())
                      .detail("high")
                      .form()
                  ),
                ]
              )
              .form()
          ),
        ]
      )
    )
    .form();

  let result = client.responses().create(request).await;
  
  match result
  {
    Ok(response) =>
    {
      println!("✅ Working image URL processed successfully");
      
      // Verify we got some analysis content
      assert!(!response.output.is_empty(), "Should have received analysis output");
      println!("✅ Analysis content received");
    },
    Err(e) =>
    {
      panic!("❌ Working URL should not fail: {e:?}");
    }
  }
}