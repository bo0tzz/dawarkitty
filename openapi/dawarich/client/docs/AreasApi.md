# \AreasApi

All URIs are relative to *http://localhost:3000*

Method | HTTP request | Description
------------- | ------------- | -------------
[**api_v1_areas_get**](AreasApi.md#api_v1_areas_get) | **GET** /api/v1/areas | Retrieves all areas
[**api_v1_areas_id_delete**](AreasApi.md#api_v1_areas_id_delete) | **DELETE** /api/v1/areas/{id} | Deletes an area
[**api_v1_areas_post**](AreasApi.md#api_v1_areas_post) | **POST** /api/v1/areas | Creates an area



## api_v1_areas_get

> Vec<models::ApiV1AreasGet200ResponseInner> api_v1_areas_get(api_key)
Retrieves all areas

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**api_key** | **String** | API Key | [required] |

### Return type

[**Vec<models::ApiV1AreasGet200ResponseInner>**](_api_v1_areas_get_200_response_inner.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_v1_areas_id_delete

> api_v1_areas_id_delete(api_key, id)
Deletes an area

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**api_key** | **String** | API Key | [required] |
**id** | **String** | Area ID | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_v1_areas_post

> api_v1_areas_post(api_key, api_v1_areas_post_request)
Creates an area

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**api_key** | **String** | API Key | [required] |
**api_v1_areas_post_request** | Option<[**ApiV1AreasPostRequest**](ApiV1AreasPostRequest.md)> |  |  |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

