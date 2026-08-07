# \ProjectsApi

All URIs are relative to *https://events26.koudaisai.jp*

| Method                                                  | HTTP request                          | Description        |
| ------------------------------------------------------- | ------------------------------------- | ------------------ |
| [**get_project**](ProjectsApi.md#get_project)           | **GET** /v1/projects/{projectId}      | 企画の取得         |
| [**get_project_icon**](ProjectsApi.md#get_project_icon) | **GET** /v1/projects/{projectId}/icon | 企画アイコンの取得 |
| [**list_projects**](ProjectsApi.md#list_projects)       | **GET** /v1/projects                  | 企画の一覧         |

## get_project

> models::Project get_project(project_id)
> 企画の取得

IDで指定した企画を1件返します。

### Parameters

| Name           | Type       | Description | Required   | Notes |
| -------------- | ---------- | ----------- | ---------- | ----- |
| **project_id** | **String** |             | [required] |

### Return type

[**models::Project**](Project.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## get_project_icon

> std::path::PathBuf get_project_icon(project_id)
> 企画アイコンの取得

IDで指定した企画のアイコン原本を返します。

### Parameters

| Name           | Type       | Description | Required   | Notes |
| -------------- | ---------- | ----------- | ---------- | ----- |
| **project_id** | **String** |             | [required] |

### Return type

[**std::path::PathBuf**](std::path::PathBuf.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: image/png, image/jpeg, image/gif, image/webp, image/heic, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## list_projects

> Vec<models::Project> list_projects()
> 企画の一覧

登録されている企画をすべて返します。

### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<models::Project>**](Project.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)
