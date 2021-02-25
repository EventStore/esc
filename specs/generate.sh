#!/bin/bash
set -euxo pipefail

# rm -r ../../client/
mkdir -p ../../client/rust

# Note: The Rust template was extracted with:
# openapi-generator author template -g rust -o ../../client/rusttemp

export IMPORT_MAPPINGS=

echo 'Creating OpenAPI Generator style client...'
openapi-generator generate -i resources.yaml -g rust -o ../../client/rust    --import-mappings=chrono::DateTime=chrono::DateTime  '--type-mappings=DateTime=chrono::DateTime<chrono::Utc>'

echo 'Creating ESC style client...'
mkdir -p ../../client/rust-esc/src
echo 'Copying over static files...'
cp -r templates/rust-esc/static/* ../../client/rust-esc/
echo 'Running openapi-generator...'
# openapi-generator generate -i resources.yaml -g rust --package-name esc-api -o ../../client/rust-esc/api    '--import-mappings=chrono::DateTime=chrono::DateTime,OrgId=crate::OrgId'  '--type-mappings=DateTime=chrono::DateTime<chrono::Utc>,orgid=OrgId' -t templates/rust-esc/mustache
openapi-generator batch generate.yaml
echo 'Building...'
pushd ../../client/rust-esc/api
RUSTFLAGS="-Z macro-backtrace" cargo build

# openapi-generator generate -i resources.yaml -g rust -o client/rust --additional-properties=--packageName=escg
# openapi-generator generate -i resources.yaml -g go -o ../../client/go
# openapi-generator generate -i resources.yaml -g typescript -o client/typescript
