# \SettingsApi

All URIs are relative to *http://localhost:3000*

Method | HTTP request | Description
------------- | ------------- | -------------
[**api_v1_settings_get**](SettingsApi.md#api_v1_settings_get) | **GET** /api/v1/settings | Retrieves user settings
[**api_v1_settings_patch**](SettingsApi.md#api_v1_settings_patch) | **PATCH** /api/v1/settings | Updates user settings



## api_v1_settings_get

> models::ApiV1SettingsGet200Response api_v1_settings_get(api_key)
Retrieves user settings

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**api_key** | **String** | API Key | [required] |

### Return type

[**models::ApiV1SettingsGet200Response**](_api_v1_settings_get_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_v1_settings_patch

> api_v1_settings_patch(api_key, api_v1_settings_patch_request)
Updates user settings

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**api_key** | **String** | API Key | [required] |
**api_v1_settings_patch_request** | Option<[**ApiV1SettingsPatchRequest**](ApiV1SettingsPatchRequest.md)> |  |  |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

