# \MessagingV1LinkshorteningMessagingServiceApi

All URIs are relative to *https://messaging.twilio.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_linkshortening_messaging_service**](MessagingV1LinkshorteningMessagingServiceApi.md#create_linkshortening_messaging_service) | **POST** /v1/LinkShortening/Domains/{DomainSid}/MessagingServices/{MessagingServiceSid} | 
[**delete_linkshortening_messaging_service**](MessagingV1LinkshorteningMessagingServiceApi.md#delete_linkshortening_messaging_service) | **DELETE** /v1/LinkShortening/Domains/{DomainSid}/MessagingServices/{MessagingServiceSid} | 



## create_linkshortening_messaging_service

> models::MessagingV1LinkshorteningMessagingService create_linkshortening_messaging_service(domain_sid, messaging_service_sid)




### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**domain_sid** | **String** | The domain SID to associate with a messaging service. With URL shortening enabled, links in messages sent with the associated messaging service will be shortened to the provided domain | [required] |
**messaging_service_sid** | **String** | A messaging service SID to associate with a domain. With URL shortening enabled, links in messages sent with the provided messaging service will be shortened to the associated domain | [required] |

### Return type

[**models::MessagingV1LinkshorteningMessagingService**](messaging.v1.linkshortening_messaging_service.md)

### Authorization

[accountSid_authToken](../README.md#accountSid_authToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_linkshortening_messaging_service

> delete_linkshortening_messaging_service(domain_sid, messaging_service_sid)




### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**domain_sid** | **String** | The domain SID to dissociate from a messaging service. With URL shortening enabled, links in messages sent with the associated messaging service will be shortened to the provided domain | [required] |
**messaging_service_sid** | **String** | A messaging service SID to dissociate from a domain. With URL shortening enabled, links in messages sent with the provided messaging service will be shortened to the associated domain | [required] |

### Return type

 (empty response body)

### Authorization

[accountSid_authToken](../README.md#accountSid_authToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

