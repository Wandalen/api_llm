#!/bin/bash

echo "Testing streaming chatbot with manual inputs..."
echo -e "write a poem about rust programming\nquit\n" | cargo run --example streaming_chatbot