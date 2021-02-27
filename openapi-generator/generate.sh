#!/bin/bash

# Calls openapi generator for multiple clients.

set -euo pipefail

mkdir -p target

readonly apis=('resources')

if [ $# -lt 1 ]; then
    readonly clients=('java' 'go' 'python' 'rust-esc' 'rust')
else
    readonly clients="${@}"
fi
echo "${clients[@]}"
configs=()
for client in "${clients[@]}"; do 
    config_file="configs/${client}.yaml"
    if [ ! -f "${config_file}" ]; then 
        echo "Invalid client: ${client}"
        echo "No config file was found at ${config_file}"
        exit 1
    fi
    configs+=("${config_file}")
done
echo "${configs[@]}"

# The generated clients have to live one directory above because ESC so Cargo
# will let us actually build the Rust stuff without modifying our workspace
# file
# readonly clients_dir="./target/clients"
readonly clients_dir="../../clients"

for api in "${apis[@]}"; do    
    echo "generating ${api} ..."
    echo "inputSpec: ../specs/${api}.yaml" > target/input.yaml
    for client in "${clients[@]}"; do 
        echo "outputDir: ${clients_dir}/${client}/${api}" > "target/${client}-output.yaml"
        if [[ "${client}" == "rust-esc" ]]; then 
            mkdir -p "${clients_dir}/${client}/${api}"
            cp -r templates/rust-esc/static/api/* "${clients_dir}/${client}/${api}"
        fi
    done

    openapi-generator batch --includes-base-dir configs "${configs[@]}" 
done

for client in "${clients[@]}"; do 
    if [[ "${client}" == "java" ]]; then
        pushd "${clients_dir}/java/resources"
        mvn compile
        popd
    elif [[ "${client}" == "rust-esc" ]]; then
        pushd "${clients_dir}/rust-esc/resources"
        cargo build
        popd
    fi
done
