# \PointsApi

All URIs are relative to *http://localhost:3000*

Method | HTTP request | Description
------------- | ------------- | -------------
[**api_v1_owntracks_points_post**](PointsApi.md#api_v1_owntracks_points_post) | **POST** /api/v1/owntracks/points | Creates a point
[**api_v1_points_get**](PointsApi.md#api_v1_points_get) | **GET** /api/v1/points | Retrieves all points
[**api_v1_points_id_delete**](PointsApi.md#api_v1_points_id_delete) | **DELETE** /api/v1/points/{id} | Deletes a point
[**api_v1_points_tracked_months_get**](PointsApi.md#api_v1_points_tracked_months_get) | **GET** /api/v1/points/tracked_months | Returns list of tracked years and months



## api_v1_owntracks_points_post

> api_v1_owntracks_points_post(api_key, api_v1_owntracks_points_post_request)
Creates a point

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**api_key** | **String** | API Key | [required] |
**api_v1_owntracks_points_post_request** | Option<[**ApiV1OwntracksPointsPostRequest**](ApiV1OwntracksPointsPostRequest.md)> |  |  |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_v1_points_get

> Vec<models::ApiV1PointsGet200ResponseInner> api_v1_points_get(api_key, start_at, end_at, page, per_page, order)
Retrieves all points

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**api_key** | **String** | API Key | [required] |
**start_at** | Option<**String**> | Start date (i.e. 2024-02-03T13:00:03Z or 2024-02-03) |  |
**end_at** | Option<**String**> | End date (i.e. 2024-02-03T13:00:03Z or 2024-02-03) |  |
**page** | Option<**i32**> | Page number |  |
**per_page** | Option<**i32**> | Number of points per page |  |
**order** | Option<**String**> | Order of points, valid values are `asc` or `desc` |  |

### Return type

[**Vec<models::ApiV1PointsGet200ResponseInner>**](_api_v1_points_get_200_response_inner.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_v1_points_id_delete

> api_v1_points_id_delete(api_key, id)
Deletes a point

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**api_key** | **String** | API Key | [required] |
**id** | **String** | Point ID | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_v1_points_tracked_months_get

> Vec<models::ApiV1PointsTrackedMonthsGet200ResponseInner> api_v1_points_tracked_months_get(api_key)
Returns list of tracked years and months

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**api_key** | **String** | API Key | [required] |

### Return type

[**Vec<models::ApiV1PointsTrackedMonthsGet200ResponseInner>**](_api_v1_points_tracked_months_get_200_response_inner.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

