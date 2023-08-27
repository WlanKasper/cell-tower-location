# Wgs84Decoded

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**latitude** | [***serde_json::Value**](.md) | from -90 to +90 [degrees] | 
**longitude** | [***serde_json::Value**](.md) | from -180 to +180 [degrees] | 
**inner_radiuse** | [***serde_json::Value**](.md) | increment of 5 from 0 to 327 675 [meters] | [optional] [default to None]
**uncertainty_radiuse** | [***serde_json::Value**](.md) | from 0 to 1 800 000 [meters] | [optional] [default to None]
**offset_angle** | [***serde_json::Value**](.md) | increment of 2 from 0 to 359.9...9 [degrees] | [optional] [default to None]
**included_angle** | [***serde_json::Value**](.md) | increment of 2 from 0.0...1 to 360 [degrees] | [optional] [default to None]
**confidence** | [***serde_json::Value**](.md) |  | [optional] [default to None]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


