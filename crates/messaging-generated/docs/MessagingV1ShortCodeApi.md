# \MessagingV1ShortCodeApi

All URIs are relative to *https://messaging.twilio.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_short_code**](MessagingV1ShortCodeApi.md#create_short_code) | **POST** /v1/Services/{ServiceSid}/ShortCodes | 
[**delete_short_code**](MessagingV1ShortCodeApi.md#delete_short_code) | **DELETE** /v1/Services/{ServiceSid}/ShortCodes/{Sid} | 
[**fetch_short_code**](MessagingV1ShortCodeApi.md#fetch_short_code) | **GET** /v1/Services/{ServiceSid}/ShortCodes/{Sid} | 
[**list_short_code**](MessagingV1ShortCodeApi.md#list_short_code) | **GET** /v1/Services/{ServiceSid}/ShortCodes | 



## create_short_code

> models::MessagingV1ServiceShortCode create_short_code(service_sid, short_code_sid)




### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**service_sid** | **String** | The SID of the [Service](https://www.twilio.com/docs/chat/rest/service-resource) to create the resource under. | [required] |
**short_code_sid** | **String** | The SID of the ShortCode resource being added to the Service. | [required] |

### Return type

[**models::MessagingV1ServiceShortCode**](messaging.v1.service.short_code.md)

### Authorization

[accountSid_authToken](../README.md#accountSid_authToken)

### HTTP request headers

- **Content-Type**: application/x-www-form-urlencoded
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_short_code

> delete_short_code(service_sid, sid)




### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**service_sid** | **String** | The SID of the [Service](https://www.twilio.com/docs/chat/rest/service-resource) to delete the resource from. | [required] |
**sid** | **String** | The SID of the ShortCode resource to delete. | [required] |

### Return type

 (empty response body)

### Authorization

[accountSid_authToken](../README.md#accountSid_authToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## fetch_short_code

> models::MessagingV1ServiceShortCode fetch_short_code(service_sid, sid)




### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**service_sid** | **String** | The SID of the [Service](https://www.twilio.com/docs/chat/rest/service-resource) to fetch the resource from. | [required] |
**sid** | **String** | The SID of the ShortCode resource to fetch. | [required] |

### Return type

[**models::MessagingV1ServiceShortCode**](messaging.v1.service.short_code.md)

### Authorization

[accountSid_authToken](../README.md#accountSid_authToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_short_code

> models::ListShortCodeResponse list_short_code(service_sid, page_size, page, page_token)




### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**service_sid** | **String** | The SID of the [Service](https://www.twilio.com/docs/chat/rest/service-resource) to read the resources from. | [required] |
**page_size** | Option<**u64**> | How many resources to return in each list page. The default is 50, and the maximum is 1000. |  |
**page** | Option<**u32**> | The page index. This value is simply for client state. |  |
**page_token** | Option<**String**> | The page token. This is provided by the API. |  |

### Return type

[**models::ListShortCodeResponse**](ListShortCodeResponse.md)

### Authorization

[accountSid_authToken](../README.md#accountSid_authToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

