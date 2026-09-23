# \MessagingV1AlphaSenderApi

All URIs are relative to *https://messaging.twilio.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_alpha_sender**](MessagingV1AlphaSenderApi.md#create_alpha_sender) | **POST** /v1/Services/{ServiceSid}/AlphaSenders | 
[**delete_alpha_sender**](MessagingV1AlphaSenderApi.md#delete_alpha_sender) | **DELETE** /v1/Services/{ServiceSid}/AlphaSenders/{Sid} | 
[**fetch_alpha_sender**](MessagingV1AlphaSenderApi.md#fetch_alpha_sender) | **GET** /v1/Services/{ServiceSid}/AlphaSenders/{Sid} | 
[**list_alpha_sender**](MessagingV1AlphaSenderApi.md#list_alpha_sender) | **GET** /v1/Services/{ServiceSid}/AlphaSenders | 



## create_alpha_sender

> models::MessagingV1ServiceAlphaSender create_alpha_sender(service_sid, alpha_sender)




### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**service_sid** | **String** | The SID of the [Service](https://www.twilio.com/docs/chat/rest/service-resource) to create the resource under. | [required] |
**alpha_sender** | **String** | The Alphanumeric Sender ID string. Can be up to 11 characters long. Valid characters are A-Z, a-z, 0-9, space, hyphen `-`, plus `+`, underscore `_` and ampersand `&`. This value cannot contain only numbers. | [required] |

### Return type

[**models::MessagingV1ServiceAlphaSender**](messaging.v1.service.alpha_sender.md)

### Authorization

[accountSid_authToken](../README.md#accountSid_authToken)

### HTTP request headers

- **Content-Type**: application/x-www-form-urlencoded
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_alpha_sender

> delete_alpha_sender(service_sid, sid)




### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**service_sid** | **String** | The SID of the [Service](https://www.twilio.com/docs/chat/rest/service-resource) to delete the resource from. | [required] |
**sid** | **String** | The SID of the AlphaSender resource to delete. | [required] |

### Return type

 (empty response body)

### Authorization

[accountSid_authToken](../README.md#accountSid_authToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## fetch_alpha_sender

> models::MessagingV1ServiceAlphaSender fetch_alpha_sender(service_sid, sid)




### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**service_sid** | **String** | The SID of the [Service](https://www.twilio.com/docs/chat/rest/service-resource) to fetch the resource from. | [required] |
**sid** | **String** | The SID of the AlphaSender resource to fetch. | [required] |

### Return type

[**models::MessagingV1ServiceAlphaSender**](messaging.v1.service.alpha_sender.md)

### Authorization

[accountSid_authToken](../README.md#accountSid_authToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_alpha_sender

> models::ListAlphaSenderResponse list_alpha_sender(service_sid, page_size, page, page_token)




### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**service_sid** | **String** | The SID of the [Service](https://www.twilio.com/docs/chat/rest/service-resource) to read the resources from. | [required] |
**page_size** | Option<**u64**> | How many resources to return in each list page. The default is 50, and the maximum is 1000. |  |
**page** | Option<**u32**> | The page index. This value is simply for client state. |  |
**page_token** | Option<**String**> | The page token. This is provided by the API. |  |

### Return type

[**models::ListAlphaSenderResponse**](ListAlphaSenderResponse.md)

### Authorization

[accountSid_authToken](../README.md#accountSid_authToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

