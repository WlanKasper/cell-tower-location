#!/bin/bash

# docker pull swaggerapi/swagger-codegen-cli

docker run --rm -v {$PWD}:/local swaggerapi/swagger-codegen-cli generate \
    -i local/swagger.yaml \
    -l rust \
    -o /local