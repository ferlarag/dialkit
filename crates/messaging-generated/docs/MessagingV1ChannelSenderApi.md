# \MessagingV1ChannelSenderApi

All URIs are relative to *https://messaging.twilio.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_channel_sender**](MessagingV1ChannelSenderApi.md#create_channel_sender) | **POST** /v1/Services/{MessagingServiceSid}/ChannelSenders | 
[**delete_channel_sender**](MessagingV1ChannelSenderApi.md#delete_channel_sender) | **DELETE** /v1/Services/{MessagingServiceSid}/ChannelSenders/{Sid} | 
[**fetch_channel_sender**](MessagingV1ChannelSenderApi.md#fetch_channel_sender) | **GET** /v1/Services/{MessagingServiceSid}/ChannelSenders/{Sid} | 
[**list_channel_sender**](MessagingV1ChannelSenderApi.md#list_channel_sender) | **GET** /v1/Services/{MessagingServiceSid}/ChannelSenders | 



## create_channel_sender

> models::MessagingV1ServiceChannelSender create_channel_sender(messaging_service_sid, sid)




### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**messaging_service_sid** | **String** | The SID of the [Service](https://www.twilio.com/docs/chat/rest/service-resource) to create the resource under. | [required] |
**sid** | **String** | The SID of the Channel Sender being added to the Service. | [required] |

### Return type

[**models::MessagingV1ServiceChannelSender**](messaging.v1.service.channel_sender.md)

### Authorization

[accountSid_authToken](../README.md#accountSid_authToken)

### HTTP request headers

- **Content-Type**: application/x-www-form-urlencoded
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_channel_sender

> delete_channel_sender(messaging_service_sid, sid)




### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**messaging_service_sid** | **String** | The SID of the [Service](https://www.twilio.com/docs/chat/rest/service-resource) to delete the resource from. | [required] |
**sid** | **String** | The SID of the Channel Sender resource to delete. | [required] |

### Return type

 (empty response body)

### Authorization

[accountSid_authToken](../README.md#accountSid_authToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## fetch_channel_sender

> models::MessagingV1ServiceChannelSender fetch_channel_sender(messaging_service_sid, sid)




### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**messaging_service_sid** | **String** | The SID of the [Service](https://www.twilio.com/docs/chat/rest/service-resource) to fetch the resource from. | [required] |
**sid** | **String** | The SID of the ChannelSender resource to fetch. | [required] |

### Return type

[**models::MessagingV1ServiceChannelSender**](messaging.v1.service.channel_sender.md)

### Authorization

[accountSid_authToken](../README.md#accountSid_authToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_channel_sender

> models::ListChannelSenderResponse list_channel_sender(messaging_service_sid, page_size, page, page_token)




### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**messaging_service_sid** | **String** | The SID of the [Service](https://www.twilio.com/docs/chat/rest/service-resource) to read the resources from. | [required] |
**page_size** | Option<**u64**> | How many resources to return in each list page. The default is 50, and the maximum is 1000. |  |
**page** | Option<**u32**> | The page index. This value is simply for client state. |  |
**page_token** | Option<**String**> | The page token. This is provided by the API. |  |

### Return type

[**models::ListChannelSenderResponse**](ListChannelSenderResponse.md)

### Authorization

[accountSid_authToken](../README.md#accountSid_authToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

