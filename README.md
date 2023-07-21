# Rust API client for swagger

This is a cell tower geolocation converter on the OpenAPI 3.0 specification.

## Overview
This API client was generated by the [swagger-codegen](https://github.com/swagger-api/swagger-codegen) project.  By using the [swagger-spec](https://github.com/swagger-api/swagger-spec) from a remote server, you can easily generate an API client.

- API version: 0.1.0
- Package version: 1.0.0
- Build package: io.swagger.codegen.languages.RustClientCodegen

## Installation
Put the package under your project folder and add the following in import:
```
    "./swagger"
```

## Documentation for API Endpoints

All URIs are relative to *https://localhost:33999*

Class | Method | HTTP request | Description
------------ | ------------- | ------------- | -------------
*CoordinatesApi* | [**decode_wgs84**](docs/CoordinatesApi.md#decode_wgs84) | **Post** /wgs84/decode | WGS84 decode endpoint
*CoordinatesApi* | [**encode_wgs84**](docs/CoordinatesApi.md#encode_wgs84) | **Post** /wgs84/encode | WGS84 encode endpoint


## Documentation For Models

 - [Wgs84Decoded](docs/Wgs84Decoded.md)
 - [Wgs84Encoded](docs/Wgs84Encoded.md)


## Documentation For Authorization
 Endpoints do not require authorization.


## Author



