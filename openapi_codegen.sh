#!/bin/bash

# --------------------------------------
# Codegen 3.1
docker pull openapitools/openapi-generator-cli

docker run --rm -v ${PWD}:/local openapitools/openapi-generator-cli generate \
    -i local/openapi.yaml \
    -g rust-server \
    -o /local/
# --------------------------------------


# --------------------------------------
# DEV

# --------------------------------------