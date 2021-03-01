# How to Generate the Clients

1. Make sure the following tools are available on your path:

    [openapi-generator](api-generator.tech/)        

2. Run `generate.sh`

This will call the openapi-generator to create most clients, plus a small utility written in Rust to create the "rust-esc-1" client which has it's code added into the existing source code for the `api` crate in this repository.

To build only a single client, add the client as an argument to the generate script. Example:

```bash
    generate.sh rust-esc-1
```

The other clients are currently generate to help test the robustness of the OpenAPI spec files located in `../specs`. Ideally all major languages should at least be able to have their clients generated and compiled without hacks or work arounds.