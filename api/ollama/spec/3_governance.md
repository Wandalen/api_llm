### Part III: Project & Process Governance

*The following sections establish the procedures, principles, and deliverables that govern the development and maintenance of the `api_ollama` library.*

### 18. Open Questions

The following questions require resolution before or during implementation to ensure project success:

### Technical Architecture Questions

**Q1: HTTP Client Configuration Strategy**
- **Question**: Should the library provide granular HTTP client configuration options (proxy settings, custom headers, certificate validation) or maintain simplicity with basic timeout/URL configuration?
- **Impact**: Medium - Affects API surface complexity and maintenance burden
- **Decision Needed By**: Implementation start
- **Options**: 
  - A) Basic configuration only (timeout, base URL)
  - B) Advanced configuration with proxy/header/TLS options
  - C) Hybrid approach with optional advanced configuration feature flag

**Q2: Streaming Buffer Management**
- **Question**: What is the optimal streaming buffer size and overflow strategy for various deployment scenarios (memory-constrained vs. high-throughput)?
- **Impact**: High - Directly affects memory usage NFRs and streaming reliability
- **Decision Needed By**: Streaming feature implementation
- **Context**: Current recommendation is 64KB fixed buffers, but real-world usage patterns unknown

**Q3: Error Granularity vs. Simplicity**
- **Question**: Should API errors (4xx/5xx responses) be parsed for specific Ollama error details, or treated generically with status codes?
- **Impact**: Medium - Affects debugging experience and error handling complexity
- **Considerations**: Ollama error format may evolve, adding parsing complexity

### Integration and Compatibility Questions

**Q4: Rust MSRV (Minimum Supported Rust Version)**
- **Question**: Should the MSRV be 1.70.0 (conservative) or latest stable (access to newest features)?
- **Impact**: Medium - Affects ecosystem compatibility and feature availability
- **Dependencies**: Other wTools crates' MSRV decisions
- **Trade-offs**: Stability vs. modern language features

**Q5: Tokio Runtime Assumptions**
- **Question**: How strictly should the library assume tokio runtime vs. remaining executor-agnostic?
- **Impact**: Medium - Affects compatibility with async-std and other runtimes
- **Context**: Most Rust async ecosystem gravitates toward tokio, but some users prefer alternatives

### Development Process Questions

**Q6: Integration Test Strategy**
- **Question**: Should integration tests require manual Ollama server setup or use containerized automation (Docker/testcontainers)?
- **Impact**: Medium - Affects CI/CD complexity and test reliability
- **Options**:
  - A) Manual setup with clear documentation
  - B) Docker Compose automation
  - C) Testcontainers integration
  - D) Hybrid approach with multiple options

**Q7: Documentation Examples Scope**
- **Question**: Should examples cover basic usage only, or include advanced patterns (error handling, streaming, concurrent usage)?
- **Impact**: Low - Affects documentation maintenance burden and user onboarding
- **Trade-offs**: Comprehensive examples vs. focused simplicity

### Future Evolution Questions

**Q8: API Stability Commitment**
- **Question**: What level of API stability should be committed to for 1.0 release?
- **Impact**: High - Affects breaking change policy and ecosystem trust
- **Considerations**: Ollama API may evolve, requiring client API changes

**Q9: Performance Benchmarking**
- **Question**: Should the library include built-in performance benchmarking capabilities for request overhead measurement?
- **Impact**: Low - Nice-to-have for validating NFR compliance
- **Alternatives**: External benchmarking tools vs. built-in measurement

### Resolution Timeline

- **High Impact Questions (Q2, Q8)**: Must be resolved before specification finalization
- **Medium Impact Questions (Q1, Q3, Q4, Q5, Q6)**: Should be resolved during early implementation
- **Low Impact Questions (Q7, Q9)**: Can be resolved during development or deferred to future releases

### Decision-Making Process

For each question:
1. **Research Phase**: Investigate ecosystem patterns and user requirements
2. **Proposal Phase**: Document recommended approach with rationale
3. **Review Phase**: Team review and stakeholder input
4. **Decision Phase**: Final decision with documentation in specification updates
5. **Implementation Phase**: Execute decided approach with validation

These questions are tracked to ensure systematic resolution and prevent implementation blockers.

### 19. Core Principles of Development

This section declares the fundamental rules governing the `api_ollama` library's development process, change management, and collaboration to ensure clarity and consistency throughout the project lifecycle.

### 1. Single Source of Truth

The project's Git repository **must** be the absolute single source of truth for all project-related information. This includes specifications, documentation, source code, configuration files, and architectural diagrams. Links to all external project assets (e.g., deployed test environments, Ollama server configurations, shared benchmarking data) **must** be stored in a central, version-controlled file (e.g., `project_assets.md`).

**Implementation Requirements**:
- All project decisions must be documented in repository files
- External references must be catalogued with links and access instructions
- No "shadow documentation" or out-of-band decision making
- Repository history serves as complete project audit trail

### 2. Documentation-First Development

All changes to the system's functionality or architecture **must** be documented in the relevant specification files *before* implementation begins. The workflow is:

1. **Propose**: A change is proposed by creating a new branch and modifying the specification
2. **Review**: The change is submitted as a Pull Request (PR) for team review
3. **Implement**: Implementation work starts only after the specification PR is approved and merged

**Rationale**: This ensures that all architectural decisions are vetted and agreed upon before costly coding work begins, preventing knowledge loss and minimizing rework.

**Exception Handling**: Minor bug fixes that don't change the public API may be implemented immediately, but must include specification updates in the same PR.

### 3. Review-Driven Change Control

All modifications to the repository, without exception, **must** go through a formal Pull Request review. Each PR **must** have a clear description of its purpose and be approved by at least one other designated reviewer before being merged.

**Review Requirements**:
- All PRs must include a clear description of changes and rationale
- Code changes must include appropriate tests
- Specification changes must be reviewed for consistency and completeness
- At least one approval required from a designated reviewer
- No self-merging allowed, even for library maintainers

### 4. Radical Transparency and Auditability

The development process **must** be fully transparent and auditable. All significant decisions and discussions **must** be captured in writing within the relevant Pull Request or a linked issue tracker. The repository's history should provide a clear, chronological narrative of the project's evolution.

**Documentation Standards**:
- All design decisions must include written rationale
- Trade-offs and alternatives considered must be documented
- Breaking changes must include migration guides
- Performance decisions must include benchmarking evidence where applicable

### 5. File Naming Conventions

All file names within the project repository **must** use lowercase `snake_case`. This ensures consistency and avoids issues with case-sensitive file systems.

**Examples**:
- ✅ **Good**: `spec.md`, `chat_request.rs`, `integration_test.rs`  
- ❌ **Bad**: `Spec.md`, `ChatRequest.rs`, `integrationTest.rs`

**Extension Standards**:
- Rust source files: `.rs`
- Documentation files: `.md` 
- Configuration files: `.toml`, `.yaml`, `.json`
- Example files: `examples/*.rs`

### 6. Test-Driven Development (TDD)

All new functionality, without exception, **must** be developed following a strict Test-Driven Development (TDD) methodology. The development cycle for any feature is:

1. **Red**: Write a failing automated test that verifies a specific piece of functionality
2. **Green**: Write the minimum amount of production code necessary to make the test pass  
3. **Refactor**: Refactor the code to meet quality standards, ensuring all tests continue to pass

**TDD Requirements for api_ollama**:
- Unit tests for all public API methods
- Integration tests for real Ollama server communication (feature-gated)
- Error condition tests for all failure modes
- Performance tests for NFR validation
- Documentation example tests to ensure examples remain valid

**Test Organization**:
- Unit tests: Co-located with source code or in `tests/unit/`
- Integration tests: `tests/integration/` (behind `integration` feature)
- Performance tests: `tests/performance/`
- Example tests: `tests/examples/`

### 7. Specification-Driven Development

This library follows a **Specification-Driven Development** model where this specification document serves as the project's living digital knowledge base. All non-trivial changes or enhancements **must** be reflected in specification updates before implementation.

**Specification Update Requirements**:
- New features must include functional requirement updates
- API changes must update the public contract sections  
- Performance changes must update non-functional requirements
- Architecture changes must update internal design recommendations

**Specification Maintenance**:
- Specification version must be incremented with significant changes
- Change log must document what changed and why
- Specification must remain aligned with implementation reality

### 8. Feature Flag Discipline

All optional functionality **must** be properly gated behind Cargo feature flags. Feature flag usage **must** follow these principles:

- **Granular Features**: Each logically distinct capability gets its own feature
- **Disable-by-Default**: Core features may be in default set, but optional features require explicit opt-in
- **Clean Compilation**: All feature combinations must compile without warnings
- **Clear Documentation**: Each feature must be documented with its purpose and dependencies

### 9. Error Handling Standards

Error handling **must** follow wTools ecosystem patterns and provide actionable information:

- **Structured Errors**: Use domain-specific `OllamaError` enum integrated with `error_tools` framework
- **Error Context**: Include sufficient context for debugging (operation attempted, parameters, server response)
- **Error Chains**: Preserve underlying error sources while providing clear high-level messages
- **Consistent Categorization**: Map all underlying errors to appropriate domain error variants
- **error_tools Integration**: Leverage `error_tools` crate for ecosystem consistency and automatic derive implementations

### 10. Performance Accountability

All code changes that could impact performance **must** include justification and, where appropriate, benchmarking evidence:

- **NFR Compliance**: Changes must not violate established non-functional requirements
- **Benchmark-Driven**: Performance claims must be supported by reproducible benchmarks
- **Regression Prevention**: Performance tests should be included in CI/CD pipeline
- **Resource Monitoring**: Memory usage and connection pooling behavior must be validated

These principles are non-negotiable and form the foundation of all development work on the `api_ollama` library. They ensure consistency, quality, and maintainability throughout the project's evolution.

### 20. Deliverables

This section defines the final, tangible products, systems, and artifacts that will be handed over upon successful completion of the `api_ollama` library project.

### Primary Code Deliverables

**1. Production-Ready Rust Library Crate**
- Complete `api_ollama` crate implementing all functional requirements
- Published to crates.io with semantic versioning (target: v0.1.0)
- Full compliance with all non-functional requirements and performance metrics
- Zero compilation warnings under `RUSTFLAGS="-D warnings"`
- Zero Clippy warnings at default lint level

**2. Source Code Repository**
- Complete Git repository with full development history
- All source code, tests, examples, and configuration files
- Properly organized module structure following Rust conventions
- MIT license and appropriate copyright notices
- Repository hosted with public access for community contributions

**3. Comprehensive Test Suite**
- Unit tests achieving ≥90% line coverage for all public APIs
- Integration tests for real Ollama server communication (feature-gated)
- Performance tests validating all non-functional requirements
- Example validation tests ensuring documentation examples remain functional
- All tests passing with 100% success rate ✅ **ACHIEVED** (2025-11-20)
  - Test suite: 413 tests across 50 test binaries
  - Verification run: 413/413 passed (100%)
  - Regression testing: 3/4 runs at 100% (1 intermittent failure identified)
  - Known issue: `test_tool_calling_non_tool_model` exhibits intermittent flakiness (passes 75% of runs)
  - Critical fixes applied:
    - Removed nextest retry mechanism (terminate-after) that caused double-spawn errors
    - Enhanced Ollama process cleanup to prevent resource exhaustion
    - Increased model loading timeout from 60s to 180s
  - Test infrastructure: `.config/nextest.toml` with specialized profiles for integration tests

### Documentation Deliverables

**4. API Documentation**
- Complete rustdoc documentation for all public APIs
- Generated documentation published to docs.rs
- Code examples for all major use cases and patterns
- Error handling examples and troubleshooting guides
- Feature flag documentation with usage examples

**5. Developer Guide and Examples**
- Comprehensive `readme.md` with quickstart instructions
- `examples/` directory with working code samples for:
  - Basic client setup and configuration
  - Chat completion requests
  - Text generation requests  
  - Model management operations
  - Streaming response handling
  - Error handling patterns
  - Advanced configuration scenarios

**6. Integration Documentation**
- Clear integration instructions for common deployment scenarios
- Docker Compose configuration for local Ollama server setup
- Environment variable configuration guide
- Troubleshooting guide for common integration issues
- Migration guide for users upgrading between versions

### Quality Assurance Deliverables

**7. Automated CI/CD Pipeline**
- GitHub Actions or equivalent CI/CD configuration
- Automated testing on multiple Rust versions and platforms
- Integration test execution against live Ollama server instances
- Automated documentation generation and publishing
- Security vulnerability scanning and dependency auditing

**8. Performance Benchmark Suite**
- Reproducible benchmarks validating all performance requirements
- Benchmark results documentation with baseline measurements
- Performance regression testing integrated into CI/CD
- Memory usage profiling and optimization validation
- Concurrent request handling validation

### Compliance and Standards Deliverables

**9. wTools Ecosystem Integration**
- Full compliance with wTools design and style rulebooks
- Integration with workspace dependency management
- Consistent error handling using `error_tools` patterns
- Proper feature gating following ecosystem standards
- Alignment with other `api_llm` API client patterns

**10. Security and Reliability Validation**
- Security audit results with vulnerability assessments
- Input validation testing for all API endpoints
- Error handling robustness testing (network failures, malformed responses)
- Memory safety validation (no unsafe code in public APIs)
- Thread safety validation for concurrent usage patterns

### Community and Maintenance Deliverables

**11. Community Contribution Framework**
- Contributing guidelines (`CONTRIBUTING.md`)
- Code of conduct and community standards
- Issue templates for bug reports and feature requests
- Pull request templates with required information
- Maintainer response time commitments and escalation procedures

**12. Long-term Maintenance Plan**
- Documented maintenance responsibilities and ownership
- Compatibility matrix with supported Ollama server versions
- Deprecation policy and migration assistance procedures
- Release planning and semantic versioning strategy
- Community feedback integration process

### Deployment and Distribution Deliverables

**13. Package Distribution**
- Published crate on crates.io with appropriate metadata
- Release artifacts with checksums and signatures
- Distribution through Cargo ecosystem with proper categorization
- Integration into wTools workspace and dependency management
- Version compatibility documentation with ecosystem dependencies

**14. Deployment Validation**
- Successful deployment validation in multiple environments
- Integration testing with common Rust async runtimes
- Compatibility validation with major operating systems
- Container deployment examples and validation
- Cloud deployment patterns and configuration examples

### Success Validation Deliverables

**15. Acceptance Testing Results**
- Complete validation of all functional requirements (FR-1 through FR-9)
- Non-functional requirement compliance testing results
- Performance benchmark results meeting all specified targets
- Integration test results with real Ollama server instances
- User acceptance testing with representative use cases

**16. Project Completion Report**
- Final project status report with metrics and achievements
- Lessons learned documentation for future library development
- Outstanding issues and technical debt documentation
- Recommendations for future enhancements and roadmap
- Handover documentation for ongoing maintenance and support

### Delivery Timeline and Acceptance Criteria

**Delivery Phases**:
1. **Alpha Release**: Core functionality with basic documentation
2. **Beta Release**: Full feature set with comprehensive testing  
3. **Release Candidate**: Production-ready with performance validation
4. **General Availability**: All deliverables complete and validated

**Acceptance Criteria**:
- All deliverables must be complete and meet specified quality standards
- All success metrics from Section 5 must be achieved
- Community feedback integration must be demonstrated
- Long-term maintenance commitment must be established

These deliverables constitute the complete, production-ready `api_ollama` library ecosystem, ready for community adoption and long-term maintenance within the wTools ecosystem.

### Appendix: Addendum

### Purpose
This document is intended to be completed by the **Developer** during the implementation phase. It is used to capture the final, as-built details of the **Internal Design**, especially where the implementation differs from the initial `Design Recommendations` in this specification.

### Instructions for the Developer
As you build the system, please use this document to log your key implementation decisions, the final data models, environment variables, and other details. This creates a crucial record for future maintenance, debugging, and onboarding.

---

### Conformance Checklist
*This checklist is the definitive list of acceptance criteria for the project. Before final delivery, each item must be verified as complete and marked with `✅`. Use the 'Verification Notes' column to link to evidence (e.g., test results, screen recordings).*

| Status | Requirement | Verification Notes |
| :--- | :--- | :--- |
| ❌ | **FR-1.1:** The library must provide a `new()` constructor that creates a client with default settings | [Link to evidence...] |
| ❌ | **FR-1.2:** The library must provide a builder pattern for custom client configuration | [Link to evidence...] |
| ❌ | **FR-1.3:** The library must allow injection of a custom `reqwest::Client` for advanced HTTP configuration | [Link to evidence...] |
| ❌ | **FR-1.4:** The library must validate base URLs at client construction time and reject invalid URLs | [Link to evidence...] |
| ❌ | **FR-2.1:** The library must provide an `is_available()` method that returns a boolean indicating server availability | [Link to evidence...] |
| ❌ | **FR-2.2:** The availability check must complete within configured timeout and return `false` for failures | [Link to evidence...] |
| ❌ | **FR-2.3:** The availability check must not throw exceptions for network failures, timeouts, or server errors | [Link to evidence...] |
| ❌ | **FR-3.1:** The library must provide a `list_models()` method that returns available models from `/api/tags` | [Link to evidence...] |
| ❌ | **FR-3.2:** The library must provide a `model_info()` method that retrieves detailed model information | [Link to evidence...] |
| ❌ | **FR-3.3:** The library must deserialize model information including name, size, digest, and timestamp | [Link to evidence...] |
| ❌ | **FR-3.4:** Model operations must return structured errors for invalid model names or communication failures | [Link to evidence...] |
| ❌ | **FR-4.1:** The library must provide a `chat()` method that accepts `ChatRequest` and returns `ChatResponse` | [Link to evidence...] |
| ❌ | **FR-4.2:** Chat requests must support conversation history via vector of `Message` structs | [Link to evidence...] |
| ❌ | **FR-4.3:** Chat responses must include generated message, completion status, and performance metrics | [Link to evidence...] |
| ❌ | **FR-4.4:** The library must serialize requests and deserialize responses using serde with camelCase | [Link to evidence...] |
| ❌ | **FR-5.1:** The library must provide a `generate()` method accepting `GenerateRequest` returning `GenerateResponse` | [Link to evidence...] |
| ❌ | **FR-5.2:** Generation requests must support text prompts and optional model parameters | [Link to evidence...] |
| ❌ | **FR-5.3:** Generation responses must include generated text, completion status, and performance metrics | [Link to evidence...] |
| ❌ | **FR-5.4:** The library must handle empty or malformed prompts with appropriate validation errors | [Link to evidence...] |
| ❌ | **FR-6.1:** When streaming feature enabled, library must provide `chat_stream()` and `generate_stream()` methods | [Link to evidence...] |
| ❌ | **FR-6.2:** Streaming methods must return async streams yielding incremental response objects | [Link to evidence...] |
| ❌ | **FR-6.3:** Stream items must include partial content and completion indicator (`done` field) | [Link to evidence...] |
| ❌ | **FR-6.4:** Streams must handle network interruptions and incomplete responses with appropriate errors | [Link to evidence...] |
| ❌ | **FR-6.5:** Streaming must use constant memory regardless of response length | [Link to evidence...] |
| ❌ | **FR-7.1:** The library must define comprehensive `OllamaError` enum covering all failure modes | [Link to evidence...] |
| ❌ | **FR-7.2:** Network errors must be distinguished from API errors and parsing errors | [Link to evidence...] |
| ❌ | **FR-7.3:** Error messages must include sufficient context for debugging | [Link to evidence...] |
| ❌ | **FR-7.4:** All errors must implement standard `std::error::Error` trait with meaningful `Display` | [Link to evidence...] |
| ❌ | **FR-7.5:** The library must convert underlying library errors to domain-specific error variants | [Link to evidence...] |
| ❌ | **FR-8.1:** All HTTP operations must respect the configured client timeout | [Link to evidence...] |
| ❌ | **FR-8.2:** Timeout violations must result in specific `NetworkError` variant with clear messaging | [Link to evidence...] |
| ❌ | **FR-8.3:** The library must properly clean up resources when operations are canceled or timeout | [Link to evidence...] |
| ❌ | **FR-8.4:** Long-running streaming operations must be interruptible without resource leaks | [Link to evidence...] |
| ❌ | **FR-9.1:** The library must implement an `enabled` feature that controls core functionality compilation | [Link to evidence...] |
| ❌ | **FR-9.2:** The library must provide a `streaming` feature that gates streaming dependencies and methods | [Link to evidence...] |
| ❌ | **FR-9.3:** The library must provide an `integration` feature for real API testing | [Link to evidence...] |
| ❌ | **FR-9.4:** With all features disabled, library must compile to minimal no-op implementation | [Link to evidence...] |

### Finalized Internal Design Decisions
*A space for the developer to document key implementation choices for the system's internal design, especially where they differ from the initial recommendations in this specification.*

-   [Decision 1: Reason...]
-   [Decision 2: Reason...]

### Finalized Internal Data Models
*The definitive, as-built schema for all databases, data structures, and objects used internally by the system.*

-   [Model 1: Schema and notes...]
-   [Model 2: Schema and notes...]

### Environment Variables
*List all environment variables required to run the application. Include the variable name, a brief description of its purpose, and an example value (use placeholders for secrets).*

| Variable | Description | Example |
| :--- | :--- | :--- |
| `OLLAMA_BASE_URL` | Base URL for Ollama server connection | `http://localhost:11434` |
| `OLLAMA_TIMEOUT` | Request timeout in seconds | `120` |

### Finalized Library & Tool Versions
*List the critical libraries, frameworks, or tools used and their exact locked versions (e.g., from `Cargo.toml`).*

-   `reqwest`: `0.11.x`
-   `serde`: `1.0.x`
-   `tokio`: `1.0.x`
-   `futures-core`: `0.3.x`

### Deployment Checklist
*A step-by-step guide for deploying the application from scratch. Include steps for setting up the environment, running migrations, and starting the services.*

1.  Add dependency to `Cargo.toml`: `api_ollama = "0.1.0"`
2.  Import library: `use api_ollama::OllamaClient;`
3.  Create client: `let client = OllamaClient::new()?;`
4.  Ensure Ollama server is running: `ollama serve`
5.  Verify connectivity: `client.is_available().await`