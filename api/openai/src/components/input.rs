//! Structures related to input content parts and messages.

/// Define a private namespace for all its items.
mod private
{
  use serde::{ Deserialize, Serialize };
  use former::Former;

  /// Represents a text input part within a message's content.
  ///
  /// # Used By
  /// - `InputContentPart`
  /// - `ChatCompletionRequestMessageContentPartText` (within `chat_shared.rs`)
  /// - `ChatCompletionRequestSystemMessageContentPart` (within `chat_shared.rs`)
  /// - `ChatCompletionRequestToolMessageContentPart` (within `chat_shared.rs`)
  /// - `ChatCompletionRequestUserMessageContentPart` (within `chat_shared.rs`)
  /// - `MessageRequestContentTextObject` (within `assistants_shared.rs`)
  #[ derive( Debug, Serialize, Deserialize, Clone, PartialEq, Former ) ]
  pub struct InputText
  {
    /// The text content.
    pub text : String,
  }
  impl Default for InputText
  {
    /// Creates a default `InputText` with an empty string.
    #[ inline ]
    fn default() -> Self
    {
      Self { text : String::new() }
    }
  }

  /// Represents an image input part, specified either by URL or File ID.
  /// Learn about [image inputs](/docs/guides/vision).
  ///
  /// # Used By
  /// - `InputContentPart`
  /// - `ChatCompletionRequestMessageContentPartImage` (within `chat_shared.rs`)
  #[ derive( Debug, Serialize, Deserialize, Clone, PartialEq, Former, Default ) ]
  pub struct InputImage
  {
    /// The URL of the image or a base64 encoded data URL.
    #[ serde( skip_serializing_if = "Option::is_none" ) ]
    pub image_url : Option< String >,
    /// The ID of an uploaded file to use as input.
    #[ serde( skip_serializing_if = "Option::is_none" ) ]
    pub file_id : Option< String >,
    /// Specifies the detail level of the image (`low`, `high`, or `auto`). Defaults to `auto`.
    #[ serde( skip_serializing_if = "Option::is_none" ) ]
    pub detail : Option< String >,
  }

  /// Represents a file input part, specified by File ID or embedded data.
  /// Learn about [file inputs](/docs/guides/text) for text generation.
  ///
  /// # Used By
  /// - `InputContentPart`
  /// - `ChatCompletionRequestMessageContentPartFile` (within `chat_shared.rs`)
  #[ derive( Debug, Serialize, Deserialize, Clone, PartialEq, Former, Default ) ]
  pub struct InputFile
  {
    /// The ID of an uploaded file to use as input.
    #[ serde( skip_serializing_if = "Option::is_none" ) ]
    pub file_id : Option< String >,
    /// The name of the file, used when passing the file to the model as a string.
    #[ serde( skip_serializing_if = "Option::is_none" ) ]
    pub filename : Option< String >,
    /// The base64 encoded file data, used when passing the file to the model as a string.
    #[ serde( skip_serializing_if = "Option::is_none" ) ]
    pub file_data : Option< String >,
  }

  /// Represents different types of input content parts within a message for request creation.
  ///
  /// # Used By
  /// - `InputMessage`
  #[ derive( Debug, Serialize, Deserialize, Clone, PartialEq ) ]
  #[ serde( tag = "type" ) ]
  pub enum InputContentPart
  {
    /// Text content.
    #[ serde( rename = "input_text" ) ]
    Text( InputText ),
    /// Image content (URL or file ID).
    #[ serde( rename = "input_image" ) ]
    Image( InputImage ),
    /// File content (file ID or data).
    #[ serde( rename = "input_file" ) ]
    File( InputFile ),
  }

  /// Represents a message input item for request creation, including role and content.
  ///
  /// # Used By
  /// - `InputItem`
  #[ derive( Debug, Serialize, Deserialize, Clone, PartialEq, Former ) ]
  pub struct InputMessage
  {
    /// The type of the item, always "message".
    #[ former( default = "message".to_string() ) ]
    pub r#type : String,
    /// The role of the message author (e.g., "user", "system", "developer").
    pub role : String,
    /// The content parts of the message (text, image, file).
    pub content : Vec< InputContentPart >,
    /// The status of the item (`in_progress`, `completed`, `incomplete`). Populated when returned via API.
    #[ serde( skip_serializing_if = "Option::is_none" ) ]
    pub status : Option< String >,
    /// The unique ID of the message input. Populated when returned via API.
    #[ serde( skip_serializing_if = "Option::is_none" ) ]
    pub id : Option< String >,
  }
  impl Default for InputMessage
  {
    /// Creates a default `InputMessage` with type "message" and role "user".
    #[ inline ]
    fn default() -> Self
    {
      Self
      {
        r#type : "message".to_string(),
        role : "user".to_string(),
        content : Vec::new(),
        status : None,
        id : None,
      }
    }
  }

  /// Represents an input item within a request, currently only supporting messages.
  ///
  /// # Used By
  /// - `ResponseInput` (within `responses.rs`)
  #[ derive( Debug, Serialize, Deserialize, Clone, PartialEq ) ]
  #[ serde( untagged ) ]
  pub enum InputItem
  {
    /// An input message.
    Message( InputMessage ),
    // Potentially other item types like ItemReference could be added here.
  }

  /// Represents a content part within a listed input item (used in file search results).
  ///
  /// # Used By
  /// - `ListedInputItem`
  /// - `FileSearchResultItem` (within `output.rs`)
  #[ derive( Debug, Serialize, Deserialize, Clone, PartialEq, Default ) ] // Added Serialize
  pub struct ListedInputContentPart
  {
    /// The type of the content part (e.g., "`input_text`").
    pub r#type : String,
    /// The text content.
    pub text : String,
  }

  /// Represents an input item as returned by the list operation (e.g., listing response inputs).
  ///
  /// # Used By
  /// - `ResponseItemList` (within `responses.rs`)
  #[ derive( Debug, Deserialize, Clone, PartialEq ) ]
  pub struct ListedInputItem
  {
    /// The unique ID of the input item (message).
    pub id : String,
    /// The role of the message author (e.g., "user").
    pub role : String,
    /// The content parts of the message.
    #[ serde( default ) ]
    pub content : Vec< ListedInputContentPart >,
    /// Optional name for the participant.
    #[ serde( skip_serializing_if = "Option::is_none" ) ]
    pub name : Option< String >,
    /// The status of the item (`completed`, etc.).
    #[ serde( skip_serializing_if = "Option::is_none" ) ]
    pub status : Option< String >,
    /// The type of the item (e.g., "message").
    #[ serde( skip_serializing_if = "Option::is_none" ) ]
    pub r#type : Option< String >,
  }
} // end mod private

crate ::mod_interface!
{
  exposed use
  {
    InputText,
    InputImage,
    InputFile,
    InputContentPart,
    InputMessage,
    InputItem,
    ListedInputContentPart,
    ListedInputItem,
  };
}