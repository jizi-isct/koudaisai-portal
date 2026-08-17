# \AdminApi

All URIs are relative to *https://events26.koudaisai.jp*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_project**](AdminApi.md#create_project) | **POST** /admin/v1/projects | 企画の登録
[**create_projects**](AdminApi.md#create_projects) | **POST** /admin/v1/projects/bulk | 企画の一括登録
[**delete_project**](AdminApi.md#delete_project) | **DELETE** /admin/v1/projects/{projectId} | 企画の削除
[**delete_project_additional_info**](AdminApi.md#delete_project_additional_info) | **DELETE** /admin/v1/projects/{projectId}/details/additionalInfo | 企画追加情報の削除
[**delete_project_icon**](AdminApi.md#delete_project_icon) | **DELETE** /admin/v1/projects/{projectId}/icon | 企画アイコンの削除
[**delete_project_menu**](AdminApi.md#delete_project_menu) | **DELETE** /admin/v1/projects/{projectId}/details/menu | 企画メニューの削除
[**update_project**](AdminApi.md#update_project) | **PUT** /admin/v1/projects/{projectId} | 企画の更新
[**update_project_additional_info**](AdminApi.md#update_project_additional_info) | **PUT** /admin/v1/projects/{projectId}/details/additionalInfo | 企画追加情報の登録・更新
[**update_project_description**](AdminApi.md#update_project_description) | **PATCH** /admin/v1/projects/{projectId}/description | 企画説明の更新
[**update_project_icon**](AdminApi.md#update_project_icon) | **PUT** /admin/v1/projects/{projectId}/icon | 企画アイコンの更新
[**update_project_menu**](AdminApi.md#update_project_menu) | **PUT** /admin/v1/projects/{projectId}/details/menu | 企画メニューの登録・更新



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


## create_projects

> Vec<models::Project> create_projects(project)
企画の一括登録

企画をまとめて新規登録します。ID は呼び出し側が指定します。一件でも登録できなければ、一件も登録されません。一度に登録できるのは 100 件までです。

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project** | [**Vec<models::Project>**](Project.md) |  | [required] |

### Return type

[**Vec<models::Project>**](Project.md)

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


## delete_project_additional_info

> delete_project_additional_info(project_id)
企画追加情報の削除

指定した企画の追加情報を削除します。メニューは変更しません。

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


## delete_project_icon

> delete_project_icon(project_id)
企画アイコンの削除

企画アイコンの原本を削除します。未登録の場合も成功します。

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


## delete_project_menu

> delete_project_menu(project_id)
企画メニューの削除

指定した企画のメニューを削除します。追加情報は変更しません。

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


## update_project_additional_info

> update_project_additional_info(project_id, body)
企画追加情報の登録・更新

指定した企画の追加情報を保存します。メニューは変更しません。

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_id** | **String** |  | [required] |
**body** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_project_description

> models::Project update_project_description(project_id, project_description)
企画説明の更新

企画の説明だけを書き換えます。他の項目・タグ・開催予定は変わりません。

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_id** | **String** |  | [required] |
**project_description** | [**ProjectDescription**](ProjectDescription.md) |  | [required] |

### Return type

[**models::Project**](Project.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_project_icon

> update_project_icon(project_id, body)
企画アイコンの更新

Cloudflare Imagesで扱える正方形の画像を、企画アイコンの原本として保存します。

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_id** | **String** |  | [required] |
**body** | **std::path::PathBuf** |  | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: image/png, image/jpeg, image/gif, image/webp, image/heic
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_project_menu

> update_project_menu(project_id, get_project_details200_response_menu)
企画メニューの登録・更新

指定した企画のメニューを保存します。追加情報は変更しません。

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_id** | **String** |  | [required] |
**get_project_details200_response_menu** | [**GetProjectDetails200ResponseMenu**](GetProjectDetails200ResponseMenu.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

