#!/bin/bash
# rm -r ../../client/
mkdir -p ../../client/rust

# Note: The Rust template was extracted with:
# openapi-generator author template -g rust -o ../../client/rusttemp


openapi-generator generate generate -i resources.yaml -c config.json -g rust -o ../../client/rust    --import-mappings=chrono::DateTime=chrono::DateTime  '--type-mappings=DateTime=chrono::DateTime<chrono::Utc>' -t rust-template

# openapi-generator generate -i resources.yaml -g rust -o client/rust --additional-properties=--packageName=escg
# openapi-generator generate -i resources.yaml -g go -o ../../client/go
# openapi-generator generate -i resources.yaml -g typescript -o client/typescript
