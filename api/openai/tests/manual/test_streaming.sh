#!/bin/bash

echo "Testing streaming chatbot with controlled input..."
echo -e "hello\nquit\n" | cargo run --example streaming_chatbot