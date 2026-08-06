# \AdminApi

All URIs are relative to *https://events26.koudaisai.jp*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_project**](AdminApi.md#create_project) | **POST** /admin/v1/projects | 企画の登録
[**delete_project**](AdminApi.md#delete_project) | **DELETE** /admin/v1/projects/{projectId} | 企画の削除
[**update_project**](AdminApi.md#update_project) | **PUT** /admin/v1/projects/{projectId} | 企画の更新



## create_project

> models::Project create_project(project)
企画の登録

企画を新規登録します。ID は呼び出し側が指定します。

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project** | [**Project**](Project.md) |  | [required] |

### Return type

[**models::Project**](Project.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_project

> delete_project(project_id)
企画の削除

企画を削除します。タグと開催予定も一緒に消えます。

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_id** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_project

> models::Project update_project(project_id, project)
企画の更新

企画を丸ごと置き換えます。タグと開催予定は差分ではなく総入れ替えになります。

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_id** | **String** |  | [required] |
**project** | [**Project**](Project.md) |  | [required] |

### Return type

[**models::Project**](Project.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

