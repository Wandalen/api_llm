#!/bin/bash

echo "Testing streaming chatbot debug version..."
echo -e "hello\nx=13\nwhat is 2+2\nquit\n" | cargo run --example streaming_chatbot