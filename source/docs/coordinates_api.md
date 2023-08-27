# coordinates_api

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
**decodeWGS84**](coordinates_api.md#decodeWGS84) | **POST** /wgs84/decode | WGS84 decode endpoint
**encodeWGS84**](coordinates_api.md#encodeWGS84) | **POST** /wgs84/encode | WGS84 encode endpoint


# **decodeWGS84**
> models::Wgs84Decoded decodeWGS84(optional)
WGS84 decode endpoint

Decodes using the World Geodetic System 1984

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **wgs84_encoded** | [**Wgs84Encoded**](Wgs84Encoded.md)| Decode WGS84 | 

### Return type

[**models::Wgs84Decoded**](WGS84_decoded.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **encodeWGS84**
> models::Wgs84Encoded encodeWGS84(optional)
WGS84 encode endpoint

Encodes using the World Geodetic System 1984

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **wgs84_decoded** | [**Wgs84Decoded**](Wgs84Decoded.md)| Encode WGS84 | 

### Return type

[**models::Wgs84Encoded**](WGS84_encoded.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

