# Manual Testing

## Purpose

Manual testing procedures for api_ollama functionality requiring human verification or complex setup scenarios.

## Test Procedures

### Prerequisites
- Ollama server running locally (http://localhost:11434)
- Valid models pulled (e.g., `ollama pull llama3.2`)
- Workspace secrets configured if testing authentication

### Manual Test Cases

1. **Workspace Secrets Integration**
   - Verify secret loading from workspace_tools
   - Test authentication with loaded secrets
   - Validate error handling for missing secrets

2. **Interactive Streaming**
   - Test real-time streaming responses
   - Verify streaming control (pause/resume/cancel)
   - Check WebSocket bidirectional communication

3. **Vision Model Integration**
   - Upload images and verify processing
   - Test multimodal chat with vision-capable models
   - Validate image encoding and transmission

4. **Tool Calling**
   - Test function calling with real tools
   - Verify tool response processing
   - Check error handling for invalid tools

5. **Audio Processing**
   - Test speech-to-text with real audio files
   - Verify text-to-speech output quality
   - Check streaming audio processing

## Execution

Manual tests should be executed before major releases or when related functionality changes.

## Checklist

- [ ] Workspace secrets loading
- [ ] Streaming responses display correctly
- [ ] Vision model image processing
- [ ] Tool calling interactions
- [ ] Audio processing quality
- [ ] Error messages are user-friendly
- [ ] WebSocket bidirectional communication
- [ ] Rate limiting and circuit breaker behavior under load
