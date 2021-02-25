#!/bin/bash
set -euo pipefail

# rm -r ../../client/
mkdir -p ../../client/rust

# Note: The Rust template was extracted with:
# openapi-generator author template -g rust -o ../../client/rusttemp

export IMPORT_MAPPINGS=

# echo 'Creating OpenAPI Generator style client...'
# openapi-generator generate -i resources.yaml -g rust -o ../../client/rust    --import-mappings=chrono::DateTime=chrono::DateTime  '--type-mappings=DateTime=chrono::DateTime<chrono::Utc>'

mkdir -p target

yq 

# readonly apis=('resources' 'mesdb')
readonly apis=('resources')
readonly clients=('rust-esc' 'rust' 'go')

for api in "${apis[@]}"; do
    for client in "${clients[@]}"; do
        echo "generating ${api} :: ${client} ..."
        echo "inputSpec: ../specs/${api}.yaml" > target/input.yaml
        echo "outputDir: ../../client/${client}/${api}" > target/output.yaml
        # echo "outputDir: ../../client/${client}/${api}" > target/output.yaml
        # echo "../specs/${api}.yaml"
        if [[ "${client}" == "rust-esc" ]]; then 
            mkdir -p "../../client/${client}/${api}"
            cp -r templates/rust-esc/static/api/* "../../client/${client}/${api}"
        fi
        openapi-generator batch --includes-base-dir configs "configs/${client}.yaml"
    done
done

pushd ../../client/rust-esc/resources
cargo build

# mkdir -p ../../client/rust-esc/src
# echo 'Copying over static files...'
# cp -r templates/rust-esc/static/* ../../client/rust-esc/
# echo 'Running openapi-generator...'

# openapi-generator batch --includes-base-dir configs configs/rust-esc.yaml
# echo 'Building...'
# pushd ../../client/rust-esc/api
# RUSTFLAGS="-Z macro-backtrace" cargo build

# openapi-generator generate -i resources.yaml -g rust -o client/rust --additional-properties=--packageName=escg
# openapi-generator generate -i resources.yaml -g go -o ../../client/go
# openapi-generator generate -i resources.yaml -g typescript -o client/typescript
