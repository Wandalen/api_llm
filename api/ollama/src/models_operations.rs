//! Model operations types for Ollama API.
//!
//! Provides request structures and types for model operations including
//! showing details, pulling, pushing, and deleting models.

#[ cfg( feature = "model_details" ) ]
mod private
{
  use super::super::*;

  /// Request for showing detailed model information
  #[ derive( Debug, Clone ) ]
  pub struct ShowModelRequest
  {
    name : String,
    verbose : bool,
    template_info : bool,
    parameters_info : bool,
    system_info : bool,
  }

  impl ShowModelRequest
  {
    /// Create new show model request
    #[ inline ]
    #[ must_use ]
    pub fn new( name : impl Into< String > ) -> Self
    {
      Self
      {
        name : name.into(),
        verbose : false,
        template_info : false,
        parameters_info : false,
        system_info : false,
      }
    }

    /// Enable verbose output
    #[ inline ]
    #[ must_use ]
    pub fn with_verbose( mut self, verbose : bool ) -> Self
    {
      self.verbose = verbose;
      self
    }

    /// Include template information
    #[ inline ]
    #[ must_use ]
    pub fn with_template_info( mut self, include : bool ) -> Self
    {
      self.template_info = include;
      self
    }

    /// Include parameters information
    #[ inline ]
    #[ must_use ]
    pub fn with_parameters_info( mut self, include : bool ) -> Self
    {
      self.parameters_info = include;
      self
    }

    /// Include system information
    #[ inline ]
    #[ must_use ]
    pub fn with_system_info( mut self, include : bool ) -> Self
    {
      self.system_info = include;
      self
    }

    /// Get model name
    #[ inline ]
    pub fn name( &self ) -> &str
    {
      &self.name
    }
  }

  /// Request for pulling a model
  #[ derive( Debug, Clone ) ]
  pub struct PullModelRequest
  {
    name : String,
    insecure : bool,
    progress_tracking : bool,
  }

  impl PullModelRequest
  {
    /// Create new pull model request
    #[ inline ]
    #[ must_use ]
    pub fn new( name : impl Into< String > ) -> Self
    {
      Self
      {
        name : name.into(),
        insecure : false,
        progress_tracking : false,
      }
    }

    /// Set insecure mode
    #[ inline ]
    #[ must_use ]
    pub fn with_insecure( mut self, insecure : bool ) -> Self
    {
      self.insecure = insecure;
      self
    }

    /// Enable progress tracking
    #[ inline ]
    #[ must_use ]
    pub fn with_progress_tracking( mut self, tracking : bool ) -> Self
    {
      self.progress_tracking = tracking;
      self
    }

    /// Get model name
    #[ inline ]
    pub fn name( &self ) -> &str
    {
      &self.name
    }
  }

  /// Request for pushing a model
  #[ derive( Debug, Clone ) ]
  pub struct PushModelRequest
  {
    name : String,
    insecure : bool,
    progress_tracking : bool,
  }

  impl PushModelRequest
  {
    /// Create new push model request
    #[ inline ]
    #[ must_use ]
    pub fn new( name : impl Into< String > ) -> Self
    {
      Self
      {
        name : name.into(),
        insecure : false,
        progress_tracking : false,
      }
    }

    /// Set insecure mode
    #[ inline ]
    #[ must_use ]
    pub fn with_insecure( mut self, insecure : bool ) -> Self
    {
      self.insecure = insecure;
      self
    }

    /// Enable progress tracking
    #[ inline ]
    #[ must_use ]
    pub fn with_progress_tracking( mut self, tracking : bool ) -> Self
    {
      self.progress_tracking = tracking;
      self
    }

    /// Get model name
    #[ inline ]
    pub fn name( &self ) -> &str
    {
      &self.name
    }
  }

  /// Request for deleting a model
  #[ derive( Debug, Clone ) ]
  pub struct DeleteModelRequest
  {
    name : String,
  }

  impl DeleteModelRequest
  {
    /// Create new delete model request
    #[ inline ]
    #[ must_use ]
    pub fn new( name : impl Into< String > ) -> Self
    {
      Self
      {
        name : name.into(),
      }
    }

    /// Get model name
    #[ inline ]
    pub fn name( &self ) -> &str
    {
      &self.name
    }
  }

  /// Progress update for model operations
  #[ derive( Debug, Clone ) ]
  pub struct ModelProgressUpdate
  {
    /// Number of completed units
    pub completed : u64,
    /// Total number of units
    pub total : u64,
  }

  /// Stream of progress updates
  pub type ModelProgressStream = std::pin::Pin< Box< dyn futures_core::Stream< Item = OllamaResult< ModelProgressUpdate > > + Send > >;
}

#[ cfg( feature = "model_details" ) ]
crate ::mod_interface!
{
  exposed use
  {
    ShowModelRequest,
    PullModelRequest,
    PushModelRequest,
    DeleteModelRequest,
    ModelProgressUpdate,
    ModelProgressStream,
  };
}
