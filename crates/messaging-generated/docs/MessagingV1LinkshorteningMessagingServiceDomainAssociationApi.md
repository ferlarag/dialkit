# \MessagingV1LinkshorteningMessagingServiceDomainAssociationApi

All URIs are relative to *https://messaging.twilio.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**fetch_linkshortening_messaging_service_domain_association**](MessagingV1LinkshorteningMessagingServiceDomainAssociationApi.md#fetch_linkshortening_messaging_service_domain_association) | **GET** /v1/LinkShortening/MessagingServices/{MessagingServiceSid}/Domain | 



## fetch_linkshortening_messaging_service_domain_association

> models::MessagingV1LinkshorteningMessagingServiceDomainAssociation fetch_linkshortening_messaging_service_domain_association(messaging_service_sid)




### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**messaging_service_sid** | **String** | Unique string used to identify the Messaging service that this domain should be associated with. | [required] |

### Return type

[**models::MessagingV1LinkshorteningMessagingServiceDomainAssociation**](messaging.v1.linkshortening_messaging_service_domain_association.md)

### Authorization

[accountSid_authToken](../README.md#accountSid_authToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

