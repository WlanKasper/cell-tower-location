#!/bin/bash

# --------------------------------------
# Codegen 2.0
# docker pull swaggerapi/swagger-codegen-cli
docker run --rm -v {$PWD}:/local swaggerapi/swagger-codegen-cli generate \
    -i local/swagger.yaml \
    -l rust \
    -o /local/source/client/

docker run --rm -v {$PWD}:/local swaggerapi/swagger-codegen-cli generate \
    -i local/swagger.yaml \
    -l rust-server \
    -o /local/source/server/
# --------------------------------------

# --------------------------------------
# Codegen 3.0
# docker pull swaggerapi/swagger-codegen-cli-v3
docker run --rm -v {$PWD}:/local swaggerapi/swagger-codegen-cli-v3 generate \
    -i local/swagger.yaml \
    -l ruby \
    -o /local/source/
# --------------------------------------
