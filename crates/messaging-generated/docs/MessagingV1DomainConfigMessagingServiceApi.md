# \MessagingV1DomainConfigMessagingServiceApi

All URIs are relative to *https://messaging.twilio.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**fetch_domain_config_messaging_service**](MessagingV1DomainConfigMessagingServiceApi.md#fetch_domain_config_messaging_service) | **GET** /v1/LinkShortening/MessagingService/{MessagingServiceSid}/DomainConfig | 



## fetch_domain_config_messaging_service

> models::MessagingV1DomainConfigMessagingService fetch_domain_config_messaging_service(messaging_service_sid)




### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**messaging_service_sid** | **String** | Unique string used to identify the Messaging service that this domain should be associated with. | [required] |

### Return type

[**models::MessagingV1DomainConfigMessagingService**](messaging.v1.domain_config_messaging_service.md)

### Authorization

[accountSid_authToken](../README.md#accountSid_authToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

