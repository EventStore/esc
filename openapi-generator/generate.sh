#!/bin/bash
set -euo pipefail

# Calls openapi generator for multiple clients.



mkdir -p target

readonly apis=('mesdb' 'resources')

if [ $# -lt 1 ]; then
    readonly clients=('java' 'go' 'python' 'rust-esc' 'rust-esc-1' 'rust')
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
readonly gen_clients_dir="target/clients"

for api in "${apis[@]}"; do        
    echo "generating ${api} ..."
    echo "inputSpec: ../specs/${api}.yaml" > target/input.yaml
    
    output_dir="${gen_clients_dir}/${client}/${api}"

    for client in "${clients[@]}"; do 
        echo "outputDir: ${gen_clients_dir}/${client}/${api}" > "target/${client}-output.yaml"
    done

    openapi-generator batch --includes-base-dir configs "${configs[@]}" 

    for client in "${clients[@]}"; do 
        if [[ "${client}" == "java" ]]; then
            pushd "${gen_clients_dir}/java/${api}"
            mvn compile
            popd
        elif [[ "${client}" == "rust" ]]; then
            pushd "${gen_clients_dir}/rust/${api}"
            # cargo build
            popd
        elif [[ "${client}" == "rust-esc" ]]; then
            pushd "${gen_clients_dir}/rust-esc/${api}"
            cargo build
            popd
        elif [[ "${client}" == "rust-esc-1" ]]; then
            src_dir="${output_dir}/src/"
            dst_dir="../api/src/apis/${api}"
            # mkdir -p "${output_dir}"
            pushd ../generator
            cargo run -- "${api}" "../openapi-generator/${src_dir}" "../openapi-generator/${dst_dir}"
            popd
        fi
    done

done

for client in "${clients[@]}"; do 
    if [[ "${client}" == "rust-esc-1" ]]; then
        src_dir="${output_dir}/src/"
        dst_dir="../api/src/apis/${api}"
        pushd ../api
        cargo build
        popd
    fi
done