# \PhotosApi

All URIs are relative to *http://localhost:3000*

Method | HTTP request | Description
------------- | ------------- | -------------
[**api_v1_photos_get**](PhotosApi.md#api_v1_photos_get) | **GET** /api/v1/photos | Lists photos
[**api_v1_photos_id_thumbnail_get**](PhotosApi.md#api_v1_photos_id_thumbnail_get) | **GET** /api/v1/photos/{id}/thumbnail | Retrieves a photo



## api_v1_photos_get

> Vec<models::ApiV1PhotosGet200ResponseInner> api_v1_photos_get(api_key, start_date, end_date)
Lists photos

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**api_key** | **String** |  | [required] |
**start_date** | **String** | Start date in ISO8601 format, e.g. 2024-01-01 | [required] |
**end_date** | **String** | End date in ISO8601 format, e.g. 2024-01-02 | [required] |

### Return type

[**Vec<models::ApiV1PhotosGet200ResponseInner>**](_api_v1_photos_get_200_response_inner.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_v1_photos_id_thumbnail_get

> models::ApiV1PhotosIdThumbnailGet200Response api_v1_photos_id_thumbnail_get(id, api_key, source)
Retrieves a photo

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |
**api_key** | **String** |  | [required] |
**source** | **String** |  | [required] |

### Return type

[**models::ApiV1PhotosIdThumbnailGet200Response**](_api_v1_photos__id__thumbnail_get_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

