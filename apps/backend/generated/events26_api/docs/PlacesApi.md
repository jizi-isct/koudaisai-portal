# \PlacesApi

All URIs are relative to *https://events26.koudaisai.jp*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_place**](PlacesApi.md#get_place) | **GET** /v1/places/{placeId} | 場所の取得
[**list_places**](PlacesApi.md#list_places) | **GET** /v1/places | 場所の検索



## get_place

> models::Place get_place(place_id)
場所の取得

階層IDで指定した場所を1件返します。

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**place_id** | **String** |  | [required] |

### Return type

[**models::Place**](Place.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_places

> Vec<models::ListPlaces200ResponseInner> list_places(r#type, name, display_name)
場所の検索

企画実施場所の一覧を階層IDつきのフラットな形式で返します。type(完全一致)・name(完全一致)・displayName(部分一致)で絞り込みできます。

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**r#type** | Option<**String**> |  |  |
**name** | Option<**String**> |  |  |
**display_name** | Option<**String**> |  |  |

### Return type

[**Vec<models::ListPlaces200ResponseInner>**](listPlaces_200_response_inner.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

