# XAI API Endpoints Summary

**Source:** Official OpenAPI 3.0.3 Specification
**Base URL:** `https://api.x.ai`
**Authentication:** Bearer token (Authorization header)

---

## Core Endpoints by Category

### 🔑 API Key Management

| Method | Endpoint | Description | Compatibility |
|--------|----------|-------------|---------------|
| GET | `/v1/api-key` | Get API key info (name, status, permissions) | XAI-specific |

### 💬 Chat & Completions

| Method | Endpoint | Description | Compatibility |
|--------|----------|-------------|---------------|
| POST | `/v1/chat/completions` | Chat completions (primary) | OpenAI ✅ |
| POST | `/v1/completions` | Text completions (legacy) | OpenAI ✅ |
| POST | `/v1/complete` | Complete endpoint | Anthropic ✅ |
| POST | `/v1/messages` | Messages endpoint | Anthropic ✅ |

### 🤖 Models

| Method | Endpoint | Description | Compatibility |
|--------|----------|-------------|---------------|
| GET | `/v1/models` | List all models | OpenAI ✅ |
| GET | `/v1/models/{model_id}` | Get specific model | OpenAI ✅ |
| GET | `/v1/language-models` | List language models (detailed) | XAI-specific |
| GET | `/v1/language-models/{model_id}` | Get language model (detailed) | XAI-specific |

### 🔢 Embeddings

| Method | Endpoint | Description | Compatibility |
|--------|----------|-------------|---------------|
| POST | `/v1/embeddings` | Create embeddings | OpenAI ✅ |
| GET | `/v1/embedding-models` | List embedding models | XAI-specific |
| GET | `/v1/embedding-models/{model_id}` | Get embedding model | XAI-specific |

---

## Implementation Priority

### Phase 1: MVP (Essential)
1. `POST /v1/chat/completions` - Primary chat endpoint
2. `GET /v1/models` - Model listing
3. `GET /v1/models/{model_id}` - Model details

### Phase 2: Enhanced (Recommended)
4. `POST /v1/embeddings` - Embeddings support
5. `GET /v1/language-models` - Detailed model info
6. `POST /v1/completions` - Legacy completions

### Phase 3: Complete (Optional)
7. `GET /v1/api-key` - Key management
8. `GET /v1/embedding-models` - Embedding model listing
9. `POST /v1/complete` - Anthropic compatibility
10. `POST /v1/messages` - Anthropic compatibility

---

## API Compatibility Matrix

| API Standard | Endpoints | Notes |
|-------------|-----------|-------|
| OpenAI | 5 endpoints | `/chat/completions`, `/completions`, `/models`, `/models/{id}`, `/embeddings` |
| Anthropic | 2 endpoints | `/complete`, `/messages` |
| XAI-specific | 5 endpoints | `/api-key`, `/language-models*`, `/embedding-models*` |

---

## Authentication

**All endpoints require:**
```
Authorization: Bearer xai-YOUR_API_KEY
```

**Parameter format:**
```json
{
  "name": "Authorization",
  "in": "header",
  "required": true,
  "schema": { "type": "string" }
}
```

---

## Request/Response Formats

### Chat Completions
- **Request Schema:** `ChatRequest`
- **Response Schema:** `ChatResponse`
- **Supports:** Streaming (SSE), function calling, tool use

### Models
- **List Response:** `ListModelsResponse`
- **Single Response:** `Model`
- **Language Models:** `ListLanguageModelsResponse`, `LanguageModel`

### Embeddings
- **Request Schema:** `EmbeddingRequest`
- **Response Schema:** `EmbeddingResponse`
- **Models:** `ListEmbeddingModelsResponse`, `EmbeddingModel`

---

## Error Responses

**All endpoints return:**
- `200` - Success
- `400` - Bad request
- `404` - Not found (model-specific endpoints)

---

## Notes for Implementation

1. **OpenAI Compatibility Priority** - Focus on OpenAI-compatible endpoints first (5 endpoints)
2. **Streaming Support** - `/chat/completions` supports SSE streaming
3. **Legacy Endpoints** - `/completions` marked as legacy, prefer `/chat/completions`
4. **Anthropic Compatibility** - Optional, lower priority for Phase 1
5. **XAI-specific Features** - Enhanced model metadata via `/language-models` endpoints

---

**Total Endpoints:** 12
**OpenAI-compatible:** 5
**Anthropic-compatible:** 2
**XAI-specific:** 5
