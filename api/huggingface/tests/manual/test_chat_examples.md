# Manual Testing Plan: Chat Examples

## Test Scenario: Context Retention and Math Reasoning

**Objective**: Verify that chat examples can retain context and perform basic arithmetic reasoning.

### Test Case 1: Basic Math with Context

**Steps**:
1. Start interactive chat example
2. Tell AI: "x=13"
3. Ask: "x*3?"
4. Expected response: 39 (or "39" or "x*3 = 39")

**Success Criteria**:
- AI remembers that x=13 from previous message
- AI correctly calculates 13 * 3 = 39
- Response contains the number 39

**Failure Indicators**:
- AI responds with wrong number
- AI doesn't remember x value
- AI refuses to answer
- AI gives non-mathematical response

### Test Case 2: Multi-Step Math

**Steps**:
1. Tell AI: "I have 5 apples"
2. Say: "I buy 3 more"
3. Ask: "How many do I have now?"
4. Expected: 8

### Test Case 3: Context Window Limit

**Steps**:
1. Have 15+ exchanges
2. Reference something from exchange #1
3. Verify if it's remembered (should be forgotten due to 10-exchange limit)

## Testing Commands

```bash
# Set API key
export HUGGINGFACE_API_KEY="your_key_here"

# Test interactive chat
cargo run --example interactive_chat --features="full"

# Test multi-turn (can't modify, but verify it runs)
cargo run --example multi_turn_conversation --features="full"

# Test basic chat (single turn only)
cargo run --example chat --features="full"
```

## Known Limitations

1. **Model Capability**: Text generation models may struggle with pure math
2. **No Special Math Handling**: Unlike the complex chatbot example, simple examples don't use `math_completion` API
3. **Response Cleaning**: Our `clean_response()` function might strip important content

## Potential Issues to Watch

1. **Context Loss**: Check if conversation history is properly included in prompts
2. **Math Parsing**: Model might give verbose explanation instead of just "39"
3. **Response Format**: Model might say "The answer is 39" instead of just "39"
