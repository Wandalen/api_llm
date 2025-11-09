use api_xai::{ Client, Secret, XaiEnvironmentImpl };

/// Creates a test client with credentials from environment.
///
/// This helper loads the XAI API key from environment variables or workspace secrets
/// and creates a fully configured client for integration testing.
///
/// # Panics
///
/// Panics with a descriptive message if credentials cannot be loaded. This is intentional
/// to ensure tests fail loudly when credentials are unavailable (NO SILENT FALLBACKS).
///
/// # Examples
///
/// ```no_run
/// # #[ cfg( feature = "integration" ) ]
/// # {
/// use test_helpers::create_test_client;
///
/// #[ tokio::test ]
/// async fn test_something() {
///   let client = create_test_client();
///   // ... use client for testing
/// }
/// # }
/// ```
pub fn create_test_client() -> Client< XaiEnvironmentImpl >
{
  let secret = Secret::load_with_fallbacks( "XAI_API_KEY" )
    .expect(
      "XAI_API_KEY is required for integration tests. \
       Please set the environment variable or add to workspace secrets. \
       Integration tests MUST fail if credentials are unavailable."
    );

  let env = XaiEnvironmentImpl::new( secret )
    .expect( "Failed to create environment" );

  Client::build( env )
    .expect( "Failed to build client" )
}

/// Tries to create a test client, returning None if credentials are unavailable.
///
/// This is useful for conditional test skipping, but should be used sparingly.
/// Prefer `create_test_client()` which fails loudly.
///
/// # Examples
///
/// ```no_run
/// use test_helpers::try_create_test_client;
///
/// #[ tokio::test ]
/// async fn test_optional() {
///   let Some( client ) = try_create_test_client() else {
///     println!( "Skipping test : credentials not available" );
///     return;
///   };
///
///   // ... use client
/// }
/// ```
#[ allow( dead_code ) ]  // Helper for integration tests, may not be used in all test runs
pub fn try_create_test_client() -> Option< Client< XaiEnvironmentImpl > >
{
  let secret = Secret::load_with_fallbacks( "XAI_API_KEY" ).ok()?;
  let env = XaiEnvironmentImpl::new( secret ).ok()?;
  Client::build( env ).ok()
}
