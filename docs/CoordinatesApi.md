# \CoordinatesApi

All URIs are relative to *https://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**decode_wgs84**](CoordinatesApi.md#decode_wgs84) | **Post** /wgs84/decode | WGS84 decode endpoint
[**encode_wgs84**](CoordinatesApi.md#encode_wgs84) | **Post** /wgs84/encode | WGS84 encode endpoint


# **decode_wgs84**
> decode_wgs84(wgs84)
WGS84 decode endpoint

World Geodetic System 1984

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **wgs84** | [**Value**](.md)|  | 

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **encode_wgs84**
> encode_wgs84(latitude, longitude, optional)
WGS84 encode endpoint

World Geodetic System 1984

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **latitude** | [**Value**](.md)|  | 
  **longitude** | [**Value**](.md)|  | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **latitude** | [**Value**](.md)|  | 
 **longitude** | [**Value**](.md)|  | 
 **inner_radiuse** | [**Value**](.md)|  | 
 **uncertainty_radiuse** | [**Value**](.md)|  | 
 **offset_angle** | [**Value**](.md)|  | 
 **included_angle** | [**Value**](.md)|  | 
 **confidence** | [**Value**](.md)|  | 

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

