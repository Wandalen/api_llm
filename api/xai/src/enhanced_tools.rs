mod private
{
  //! Enhanced function calling utilities for parallel and sequential tool execution.
  //!
  //! This module provides higher-level abstractions over the raw function calling
  //! API to simplify common patterns like executing multiple tools in parallel.
  //!
  //! # Design Decisions
  //!
  //! ## Why Two Separate Functions?
  //!
  //! We provide both `execute_tool_calls_parallel()` and `execute_tool_calls_sequential()`
  //! as separate functions instead of a single function with a mode parameter because:
  //!
  //! 1. **Type Safety**: Parallel execution requires `Send + 'static` bounds that
  //!    sequential execution doesn't need. Separate functions allow precise bounds.
  //!
  //! 2. **API Clarity**: The choice between parallel and sequential is fundamental
  //!    to the application's correctness (dependent tools must be sequential).
  //!    Making this explicit in the function name prevents accidental misuse.
  //!
  //! 3. **Performance Contract**: Parallel execution provides better performance but
  //!    requires independent tools. Sequential provides ordering guarantees but is
  //!    slower. The function name makes this tradeoff explicit.
  //!
  //! ## Parallel Execution with `tokio::spawn`
  //!
  //! The parallel executor uses `tokio::spawn` instead of alternatives:
  //!
  //! - **`FuturesUnordered`**: Would require collecting futures first, less flexible
  //! - **`join_all`**: Would await in the caller's task, blocking other operations
  //! - **`tokio::spawn`**: Allows true concurrent execution on tokio runtime, with
  //!   proper error isolation (one tool failure doesn't crash the executor)
  //!
  //! **Tradeoff**: Requires `Send + 'static` bounds, which means tool executors
  //! can't borrow from the calling context. This is acceptable because tool
  //! execution should be self-contained anyway.
  //!
  //! ## Independent Error Handling
  //!
  //! Each tool result is wrapped in `Result< ToolResult, Box< dyn Error > >` rather than
  //! failing fast on the first error. This design choice:
  //!
  //! 1. **Maximizes Useful Work**: With parallel execution, some tools may succeed
  //!    even if others fail. Collecting all results allows the application to use
  //!    partial results instead of throwing everything away.
  //!
  //! 2. **Debugging**: Applications can log all failures at once instead of seeing
  //!    only the first failure and having to re-run to discover subsequent ones.
  //!
  //! 3. **Agent Patterns**: LLM agents often benefit from partial tool results -
  //!    they can reason about failures and adjust their strategy.
  //!
  //! ## Generic Executor Pattern
  //!
  //! Both functions accept a generic `Exec : Fn(ToolCall) -> Future< Output = Result<...> >`
  //! instead of requiring a specific trait. This provides maximum flexibility:
  //!
  //! - **Closures**: Can capture context for tool execution
  //! - **Async blocks**: Natural Rust syntax
  //! - **Function pointers**: For simple stateless tools
  //! - **No trait boilerplate**: No need to define and implement custom traits
  //!
  //! The executor receives ownership of `ToolCall` (not `&ToolCall`) because:
  //! 1. Parallel execution needs `'static` lifetime (can't borrow)
  //! 2. Tool execution often needs to serialize arguments, which requires owned data
  //! 3. `ToolCall` is small enough that cloning is cheap

  use crate::components::ToolCall;
  use serde_json::Value;
  use std::future::Future;

  /// Result of executing a tool call.
  ///
  /// Contains the tool call ID and the result as a JSON value.
  #[ derive( Debug, Clone ) ]
  pub struct ToolResult
  {
    /// Tool call ID (matches the ID from the original `ToolCall`).
    pub tool_call_id : String,

    /// Result of the tool execution (as JSON string).
    pub result : String,
  }

  impl ToolResult
  {
    /// Creates a new tool result.
    ///
    /// # Arguments
    ///
    /// * `tool_call_id` - The ID of the tool call this result corresponds to
    /// * `result` - The result value (will be serialized to JSON string)
    ///
    /// # Examples
    ///
    /// ```
    /// use api_xai::ToolResult;
    /// use serde_json::json;
    ///
    /// let result = ToolResult::new(
    ///   "call_123".to_string(),
    ///   &json!({ "temperature": 72, "conditions": "sunny" })
    /// );
    /// ```
    pub fn new( tool_call_id : String, result : &Value ) -> Self
    {
      Self
      {
        tool_call_id,
        result : result.to_string(),
      }
    }

    /// Creates a new tool result from a string.
    ///
    /// # Arguments
    ///
    /// * `tool_call_id` - The ID of the tool call this result corresponds to
    /// * `result_str` - The result as a JSON string
    pub fn from_string( tool_call_id : String, result_str : String ) -> Self
    {
      Self
      {
        tool_call_id,
        result : result_str,
      }
    }

    /// Creates a new tool result from an error.
    ///
    /// # Arguments
    ///
    /// * `tool_call_id` - The ID of the tool call that failed
    /// * `error` - The error message
    ///
    /// # Examples
    ///
    /// ```
    /// use api_xai::ToolResult;
    ///
    /// let result = ToolResult::from_error(
    ///   "call_123".to_string(),
    ///   "Function execution failed : invalid parameters"
    /// );
    /// ```
    pub fn from_error( tool_call_id : String, error : &str ) -> Self
    {
      let error_json = serde_json::json!({
        "error": error
      });

      Self
      {
        tool_call_id,
        result : error_json.to_string(),
      }
    }
  }

  /// Executes multiple tool calls in parallel.
  ///
  /// Takes a list of tool calls and an executor function, runs them
  /// concurrently using `tokio::spawn`, and returns all results.
  ///
  /// # Arguments
  ///
  /// * `tool_calls` - List of tool calls to execute
  /// * `executor` - Async function that executes a single tool call
  ///
  /// # Type Parameters
  ///
  /// * `F` - Future returned by the executor
  /// * `Exec` - Executor function type
  ///
  /// # Returns
  ///
  /// Vector of tool results in the same order as input tool calls.
  ///
  /// # Examples
  ///
  /// ```no_run
  /// use api_xai::{ ToolCall, ToolResult, execute_tool_calls_parallel };
  /// use serde_json::json;
  ///
  /// # async fn example( tool_calls : Vec< ToolCall > ) -> Result< (), Box< dyn std::error::Error > > {
  /// // Execute all tool calls in parallel
  /// let results = execute_tool_calls_parallel( tool_calls, | call | async move {
  ///   // Your tool execution logic here
  ///   match call.function.name.as_str() {
  ///     "get_weather" => {
  ///       let result = json!({ "temperature": 72, "conditions": "sunny" });
  ///       Ok( ToolResult::new( call.id, &result ) )
  ///     }
  ///     _ => {
  ///       Err( format!( "Unknown function : {name}", name = call.function.name ).into() )
  ///     }
  ///   }
  /// } ).await;
  /// # Ok( () )
  /// # }
  /// ```
  pub async fn execute_tool_calls_parallel< F, Exec >(
    tool_calls : Vec< ToolCall >,
    executor : Exec
  ) -> Vec< Result< ToolResult, Box< dyn std::error::Error + Send + Sync > > >
  where
    F : Future< Output = Result< ToolResult, Box< dyn std::error::Error + Send + Sync > > > + Send + 'static,
    Exec : Fn( ToolCall ) -> F,
  {
    let mut handles = Vec::new();

    // Spawn a task for each tool call
    for call in tool_calls
    {
      let future = executor( call );
      let handle = tokio::spawn( future );
      handles.push( handle );
    }

    // Collect results
    let mut results = Vec::new();
    for handle in handles
    {
      match handle.await
      {
        Ok( result ) => results.push( result ),
        Err( e ) => results.push( Err( format!( "Task join error : {e}" ).into() ) ),
      }
    }

    results
  }

  /// Executes multiple tool calls sequentially.
  ///
  /// Takes a list of tool calls and an executor function, runs them
  /// one by one, and returns all results.
  ///
  /// # Arguments
  ///
  /// * `tool_calls` - List of tool calls to execute
  /// * `executor` - Async function that executes a single tool call
  ///
  /// # Type Parameters
  ///
  /// * `F` - Future returned by the executor
  /// * `Exec` - Executor function type
  ///
  /// # Returns
  ///
  /// Vector of tool results in the same order as input tool calls.
  ///
  /// # Examples
  ///
  /// ```no_run
  /// use api_xai::{ ToolCall, ToolResult, execute_tool_calls_sequential };
  /// use serde_json::json;
  ///
  /// # async fn example( tool_calls : Vec< ToolCall > ) -> Result< (), Box< dyn std::error::Error > > {
  /// // Execute all tool calls sequentially
  /// let results = execute_tool_calls_sequential( tool_calls, | call | async move {
  ///   // Your tool execution logic here
  ///   match call.function.name.as_str() {
  ///     "get_weather" => {
  ///       let result = json!({ "temperature": 72, "conditions": "sunny" });
  ///       Ok( ToolResult::new( call.id, &result ) )
  ///     }
  ///     _ => {
  ///       Err( format!( "Unknown function : {name}", name = call.function.name ).into() )
  ///     }
  ///   }
  /// } ).await;
  /// # Ok( () )
  /// # }
  /// ```
  pub async fn execute_tool_calls_sequential< F, Exec >(
    tool_calls : Vec< ToolCall >,
    executor : Exec
  ) -> Vec< Result< ToolResult, Box< dyn std::error::Error + Send + Sync > > >
  where
    F : Future< Output = Result< ToolResult, Box< dyn std::error::Error + Send + Sync > > > + Send + 'static,
    Exec : Fn( ToolCall ) -> F,
  {
    let mut results = Vec::new();

    for call in tool_calls
    {
      let result = executor( call ).await;
      results.push( result );
    }

    results
  }
}

crate::mod_interface!
{
  exposed use
  {
    ToolResult,
    execute_tool_calls_parallel,
    execute_tool_calls_sequential,
  };
}
