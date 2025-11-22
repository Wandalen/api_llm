# Bug Reproducer Test Documentation Template

Use this template when creating bug reproducer tests as required by
test_organization.rulebook.md and code_design.rulebook.md.

## Test File Documentation (PRIMARY - MANDATORY)

Add this documentation to the bug reproducer test in the test file:

```rust
/// Reproduces [brief description of bug] (issue-XXX).
///
/// ## Root Cause
/// [Technical explanation of WHY the bug occurred - what assumption was wrong,
/// what edge case was missed, what design flaw existed]
///
/// Example: Parser assumed all inputs have at least one character. Empty string
/// caused panic at `input[0]` access.
///
/// ## Why Not Caught Initially
/// [Explanation of WHY existing tests didn't catch this - what was the gap
/// in test coverage or test design]
///
/// Example: Original test suite only covered valid inputs. Edge case testing
/// for boundary conditions was incomplete.
///
/// ## Fix Applied
/// [WHAT was changed to fix the bug - specific code changes, not high-level
/// description]
///
/// Example: Added empty string validation before any indexing operations.
/// Returns `Err(ParseError::EmptyInput)` instead of panicking.
///
/// ## Prevention
/// [HOW similar bugs are prevented in the future - systematic changes,
/// new validation patterns, test matrix updates]
///
/// Example: All parser functions now validate input length before access.
/// Added "empty input" to standard test matrix template.
///
/// ## Pitfall to Avoid
/// [General lesson for future developers - what principle to follow,
/// what assumption to avoid]
///
/// Example: Never assume collections/strings are non-empty. Always validate
/// length before accessing elements.
// test_kind: bug_reproducer(issue-XXX)
#[ test ]
fn test_bug_name_issue_xxx()
{
  // Test implementation
}
```

## Source Code Comment (SECONDARY - MANDATORY)

Add this comment at the point of the fix in source code:

```rust
// Fix(issue-XXX): [Brief description of what was fixed]
// Root cause: [Why bug occurred - one sentence]
// Pitfall: [General lesson - one sentence]
fn function_that_was_fixed()
{
  // Fixed code here
}
```

Example:
```rust
// Fix(issue-123): Validate input length before indexing
// Root cause: Assumed non-empty input without validation
// Pitfall: Never assume collection is non-empty without explicit check
fn parse( input: &str ) -> Result< Data >
{
  if input.is_empty()
  {
    return Err( ParseError::EmptyInput );
  }
  // ... rest of implementation
}
```

## Module Documentation (TERTIARY - CONDITIONAL)

If bug revealed a design flaw affecting the entire module (not just one function),
add to module-level docs:

```rust
//! ## Known Pitfalls
//!
//! ### [Pitfall Category Name]
//!
//! [Description of pitfall and why it's dangerous]
//!
//! Root cause (issue-XXX): [Technical explanation]
//!
//! Prevention: [How to avoid this pitfall]
//!
//! ```rust
//! // ✅ CORRECT: [Good example]
//! code_example();
//!
//! // ❌ FORBIDDEN: [Bad example that triggers pitfall]
//! bad_code_example();
//! ```
```

## Quality Standards (STATC)

All bug documentation MUST be:

- **S**pecific: Not generic ("fixed bug" ❌, "validated empty input before indexing" ✅)
- **T**echnical: Include implementation details, not just symptoms
- **A**ctionable: Provide concrete prevention steps, not vague warnings
- **T**raceable: Include issue number (issue-XXX) for tracking
- **C**oncise: Focus on key information, avoid unnecessary verbosity

## Documentation Checklist

For every bug fix, ensure:

- [ ] Test file has 5-section documentation
- [ ] Test marked with `// test_kind: bug_reproducer(issue-XXX)`
- [ ] Source code has 3-field fix comment
- [ ] Issue number referenced in both places
- [ ] Documentation is STATC-compliant
- [ ] Module docs updated if systemic issue
- [ ] Test passes after fix
- [ ] Full test suite passes (w3 .test level::3 or ctest3)

## Example Issue Numbers

If you don't have formal issue tracking, use:

- Sequential numbers: issue-001, issue-002, etc.
- Date-based: issue-20251107-1, issue-20251107-2
- Component-based: issue-parser-001, issue-validation-001

## References

- Rulebook: test_organization.rulebook.md (Bug Reproducer Documentation)
- Rulebook: code_design.rulebook.md (Bug-Fixing Workflow)
- Rulebook: codebase_hygiene.rulebook.md (Quality Standards)
