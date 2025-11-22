//! Structures related to content moderation results.

/// Define a private namespace for all its items.
mod private
{
  // Serde imports
  use serde::{ Serialize, Deserialize }; // Added Serialize

  /// Represents the boolean flags for each moderation category.
  ///
  /// # Used By
  /// - `ModerationResult`
  #[ derive( Debug, Serialize, Deserialize, Clone, PartialEq ) ] // Added Serialize
  #[ allow( clippy::struct_excessive_bools ) ]
pub struct ModerationCategories
  {
    /// Content that expresses, incites, or promotes hate based on protected characteristics.
    pub hate : bool,
    /// Hateful content that also includes violence or serious harm towards the targeted group.
    #[ serde( rename = "hate/threatening" ) ]
    pub hate_threatening : bool,
    /// Content that expresses, incites, or promotes harassing language towards any target.
    pub harassment : bool,
    /// Harassment content that also includes violence or serious harm towards any target.
    #[ serde( rename = "harassment/threatening" ) ]
    pub harassment_threatening : bool,
    /// Content that includes instructions or advice that facilitate the planning or execution of wrongdoing.
    pub illicit : Option< bool >,
    /// Content that includes instructions or advice for wrongdoing that also includes violence or weapon procurement.
    #[ serde( rename = "illicit/violent" ) ]
    pub illicit_violent : Option< bool >,
    /// Content that promotes, encourages, or depicts acts of self-harm.
    #[ serde( rename = "self-harm" ) ]
    pub self_harm : bool,
    /// Content where the speaker expresses intent to engage in acts of self-harm.
    #[ serde( rename = "self-harm/intent" ) ]
    pub self_harm_intent : bool,
    /// Content that encourages performing acts of self-harm or gives instructions on how to do so.
    #[ serde( rename = "self-harm/instructions" ) ]
    pub self_harm_instructions : bool,
    /// Content meant to arouse sexual excitement or promote sexual services.
    pub sexual : bool,
    /// Sexual content that includes an individual who is under 18 years old.
    #[ serde( rename = "sexual/minors" ) ]
    pub sexual_minors : bool,
    /// Content that depicts death, violence, or physical injury.
    pub violence : bool,
    /// Content that depicts death, violence, or physical injury in graphic detail.
    #[ serde( rename = "violence/graphic" ) ]
    pub violence_graphic : bool,
  }

  /// Represents the raw scores for each moderation category, as predicted by the model.
  ///
  /// # Used By
  /// - `ModerationResult`
  #[ derive( Debug, Serialize, Deserialize, Clone, PartialEq ) ] // Added Serialize
  pub struct ModerationCategoryScores
  {
    /// The score for the 'hate' category.
    pub hate : f64,
    /// The score for the 'hate/threatening' category.
    #[ serde( rename = "hate/threatening" ) ]
    pub hate_threatening : f64,
    /// The score for the 'harassment' category.
    pub harassment : f64,
    /// The score for the 'harassment/threatening' category.
    #[ serde( rename = "harassment/threatening" ) ]
    pub harassment_threatening : f64,
    /// The score for the 'illicit' category.
    pub illicit : Option< f64 >,
    /// The score for the 'illicit/violent' category.
    #[ serde( rename = "illicit/violent" ) ]
    pub illicit_violent : Option< f64 >,
    /// The score for the 'self-harm' category.
    #[ serde( rename = "self-harm" ) ]
    pub self_harm : f64,
    /// The score for the 'self-harm/intent' category.
    #[ serde( rename = "self-harm/intent" ) ]
    pub self_harm_intent : f64,
    /// The score for the 'self-harm/instructions' category.
    #[ serde( rename = "self-harm/instructions" ) ]
    pub self_harm_instructions : f64,
    /// The score for the 'sexual' category.
    pub sexual : f64,
    /// The score for the 'sexual/minors' category.
    #[ serde( rename = "sexual/minors" ) ]
    pub sexual_minors : f64,
    /// The score for the 'violence' category.
    pub violence : f64,
    /// The score for the 'violence/graphic' category.
    #[ serde( rename = "violence/graphic" ) ]
    pub violence_graphic : f64,
  }

  /// Indicates which input types (text, image) contributed to the score for each category.
  /// Only available for `omni-moderation` models.
  ///
  /// # Used By
  /// - `ModerationResult`
  #[ derive( Debug, Serialize, Deserialize, Clone, PartialEq ) ] // Added Serialize
  pub struct ModerationCategoryAppliedInputTypes
  {
    /// Input types applied for the 'hate' category.
    pub hate : Vec< String >,
    /// Input types applied for the 'hate/threatening' category.
    #[ serde( rename = "hate/threatening" ) ]
    pub hate_threatening : Vec< String >,
    /// Input types applied for the 'harassment' category.
    pub harassment : Vec< String >,
    /// Input types applied for the 'harassment/threatening' category.
    #[ serde( rename = "harassment/threatening" ) ]
    pub harassment_threatening : Vec< String >,
    /// Input types applied for the 'illicit' category.
    pub illicit : Option< Vec< String > >,
    /// Input types applied for the 'illicit/violent' category.
    #[ serde( rename = "illicit/violent" ) ]
    pub illicit_violent : Option< Vec< String > >,
    /// Input types applied for the 'self-harm' category.
    #[ serde( rename = "self-harm" ) ]
    pub self_harm : Vec< String >,
    /// Input types applied for the 'self-harm/intent' category.
    #[ serde( rename = "self-harm/intent" ) ]
    pub self_harm_intent : Vec< String >,
    /// Input types applied for the 'self-harm/instructions' category.
    #[ serde( rename = "self-harm/instructions" ) ]
    pub self_harm_instructions : Vec< String >,
    /// Input types applied for the 'sexual' category.
    pub sexual : Vec< String >,
    /// Input types applied for the 'sexual/minors' category.
    #[ serde( rename = "sexual/minors" ) ]
    pub sexual_minors : Vec< String >,
    /// Input types applied for the 'violence' category.
    pub violence : Vec< String >,
    /// Input types applied for the 'violence/graphic' category.
    #[ serde( rename = "violence/graphic" ) ]
    pub violence_graphic : Vec< String >,
  }

  /// Contains the moderation analysis results for a single input.
  ///
  /// # Used By
  /// - `CreateModerationResponse`
  #[ derive( Debug, Serialize, Deserialize, Clone, PartialEq ) ] // Added Serialize
  pub struct ModerationResult
  {
    /// Whether the content violates `OpenAI`'s usage policies.
    pub flagged : bool,
    /// A list of the categories, and whether they are flagged or not.
    pub categories : ModerationCategories,
    /// A list of the categories along with their scores as predicted by model.
    pub category_scores : ModerationCategoryScores,
    /// A list of the categories along with the input type(s) that the score applies to.
    #[ serde( skip_serializing_if = "Option::is_none" ) ]
    pub category_applied_input_types : Option< ModerationCategoryAppliedInputTypes >,
  }

  /// Represents the response from a moderation request.
  ///
  /// # Used By
  /// - `/moderations` (POST)
  #[ derive( Debug, Serialize, Deserialize, Clone, PartialEq ) ] // Added Serialize
  pub struct CreateModerationResponse
  {
    /// The unique identifier for the moderation request.
    pub id : String,
    /// The model used to generate the moderation results.
    pub model : String,
    /// A list of moderation objects, one for each input provided in the request.
    pub results : Vec< ModerationResult >,
  }
} // end mod private

crate ::mod_interface!
{
  exposed use
  {
    ModerationCategories,
    ModerationCategoryScores,
    ModerationCategoryAppliedInputTypes,
    ModerationResult,
    CreateModerationResponse
  };
}
