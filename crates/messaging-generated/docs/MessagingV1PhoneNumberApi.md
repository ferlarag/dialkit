# \MessagingV1PhoneNumberApi

All URIs are relative to *https://messaging.twilio.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_phone_number**](MessagingV1PhoneNumberApi.md#create_phone_number) | **POST** /v1/Services/{ServiceSid}/PhoneNumbers | 
[**delete_phone_number**](MessagingV1PhoneNumberApi.md#delete_phone_number) | **DELETE** /v1/Services/{ServiceSid}/PhoneNumbers/{Sid} | 
[**fetch_phone_number**](MessagingV1PhoneNumberApi.md#fetch_phone_number) | **GET** /v1/Services/{ServiceSid}/PhoneNumbers/{Sid} | 
[**list_phone_number**](MessagingV1PhoneNumberApi.md#list_phone_number) | **GET** /v1/Services/{ServiceSid}/PhoneNumbers | 



## create_phone_number

> models::MessagingV1ServicePhoneNumber create_phone_number(service_sid, phone_number_sid)




### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**service_sid** | **String** | The SID of the [Service](https://www.twilio.com/docs/chat/rest/service-resource) to create the resource under. | [required] |
**phone_number_sid** | **String** | The SID of the Phone Number being added to the Service. | [required] |

### Return type

[**models::MessagingV1ServicePhoneNumber**](messaging.v1.service.phone_number.md)

### Authorization

[accountSid_authToken](../README.md#accountSid_authToken)

### HTTP request headers

- **Content-Type**: application/x-www-form-urlencoded
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_phone_number

> delete_phone_number(service_sid, sid)




### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**service_sid** | **String** | The SID of the [Service](https://www.twilio.com/docs/chat/rest/service-resource) to delete the resource from. | [required] |
**sid** | **String** | The SID of the PhoneNumber resource to delete. | [required] |

### Return type

 (empty response body)

### Authorization

[accountSid_authToken](../README.md#accountSid_authToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## fetch_phone_number

> models::MessagingV1ServicePhoneNumber fetch_phone_number(service_sid, sid)




### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**service_sid** | **String** | The SID of the [Service](https://www.twilio.com/docs/chat/rest/service-resource) to fetch the resource from. | [required] |
**sid** | **String** | The SID of the PhoneNumber resource to fetch. | [required] |

### Return type

[**models::MessagingV1ServicePhoneNumber**](messaging.v1.service.phone_number.md)

### Authorization

[accountSid_authToken](../README.md#accountSid_authToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_phone_number

> models::ListPhoneNumberResponse list_phone_number(service_sid, page_size, page, page_token)




### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**service_sid** | **String** | The SID of the [Service](https://www.twilio.com/docs/chat/rest/service-resource) to read the resources from. | [required] |
**page_size** | Option<**u64**> | How many resources to return in each list page. The default is 50, and the maximum is 1000. |  |
**page** | Option<**u32**> | The page index. This value is simply for client state. |  |
**page_token** | Option<**String**> | The page token. This is provided by the API. |  |

### Return type

[**models::ListPhoneNumberResponse**](ListPhoneNumberResponse.md)

### Authorization

[accountSid_authToken](../README.md#accountSid_authToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

