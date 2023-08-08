#!/bin/bash

# --------------------------------------
# Codegen 2.0
# docker pull swaggerapi/swagger-codegen-cli
docker run --rm -v {$PWD}:/local swaggerapi/swagger-codegen-cli generate \
    -i local/swagger.yaml \
    -l rust \
    -o /local

# MacOS
docker run --platform linux/amd64 --rm -v ${PWD}:/local swaggerapi/swagger-codegen-cli generate \
    -i local/swagger.yaml \
    -l rust \
    -o /local 
# --------------------------------------


# --------------------------------------
# Codegen 3.0
# docker pull swaggerapi/swagger-codegen-cli-v3
docker run --rm -v {$PWD}:/local swaggerapi/swagger-codegen-cli-v3 generate \
    -i local/swagger.yaml \
    -l go \
    -o /local/out/go
# --------------------------------------
