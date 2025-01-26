# \CountriesApi

All URIs are relative to *http://localhost:3000*

Method | HTTP request | Description
------------- | ------------- | -------------
[**api_v1_countries_visited_cities_get**](CountriesApi.md#api_v1_countries_visited_cities_get) | **GET** /api/v1/countries/visited_cities | Get visited cities by date range



## api_v1_countries_visited_cities_get

> models::ApiV1CountriesVisitedCitiesGet200Response api_v1_countries_visited_cities_get(api_key, start_at, end_at)
Get visited cities by date range

Returns a list of visited cities and countries based on tracked points within the specified date range

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**api_key** | **String** |  | [required] |
**start_at** | **String** | Start date in YYYY-MM-DD format | [required] |
**end_at** | **String** | End date in YYYY-MM-DD format | [required] |

### Return type

[**models::ApiV1CountriesVisitedCitiesGet200Response**](_api_v1_countries_visited_cities_get_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

