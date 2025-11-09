mod private
{
  use serde::{ Serialize, Deserialize };

  /// Token usage information for API requests.
  ///
  /// Tracks the number of tokens consumed in prompts, completions, and total.
  /// Used for billing and quota management.
  ///
  /// # Examples
  ///
  /// ```
  /// use api_xai::Usage;
  ///
  /// let usage = Usage {
  ///   prompt_tokens : 10,
  ///   completion_tokens : 20,
  ///   total_tokens : 30,
  /// };
  ///
  /// assert_eq!( usage.total_tokens, usage.prompt_tokens + usage.completion_tokens );
  /// ```
  #[ derive( Debug, Serialize, Deserialize, Clone, PartialEq, Eq ) ]
  pub struct Usage
  {
    /// Number of tokens in the prompt (input).
    pub prompt_tokens : u32,

    /// Number of tokens in the completion (output).
    pub completion_tokens : u32,

    /// Total number of tokens (prompt + completion).
    pub total_tokens : u32,
  }

  /// Message role in a conversation.
  ///
  /// XAI API supports flexible message role ordering (unlike `OpenAI` which
  /// requires strict alternation). Messages can be in any order.
  ///
  /// # Roles
  ///
  /// - `System`: System instructions and context
  /// - `User`: User messages and queries
  /// - `Assistant`: AI assistant responses
  /// - `Tool`: Tool execution results (for function calling)
  ///
  /// # Examples
  ///
  /// ```
  /// use api_xai::Role;
  ///
  /// let system_role = Role::System;
  /// let user_role = Role::User;
  /// let assistant_role = Role::Assistant;
  /// ```
  #[ derive( Debug, Serialize, Deserialize, Clone, PartialEq, Eq ) ]
  #[ serde( rename_all = "lowercase" ) ]
  pub enum Role
  {
    /// System message providing instructions or context.
    System,

    /// User message containing queries or prompts.
    User,

    /// Assistant message containing AI responses.
    Assistant,

    /// Tool message containing function execution results.
    Tool,
  }
}

crate::mod_interface!
{
  exposed use
  {
    Usage,
    Role,
  };
}
