# Realtime API

## Purpose

Implementation of OpenAI's Realtime API for WebSocket-based real-time communication and streaming.

## Organization Principles

- **WebSocket management**: Connection handling, message routing, and session management
- **Event handling**: Real-time event processing and callbacks
- **Streaming support**: Audio streaming, transcription, and real-time responses
- **mod.rs**: Module organization and public API exports

## Navigation Guide

- For realtime API implementation: `mod.rs`
- For realtime data types and events: `../components/realtime_shared/`
- For WebSocket streaming infrastructure: `../websocket_streaming.rs`
- For session management: Use realtime API methods in `mod.rs`
