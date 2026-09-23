# \MessagingV1RequestManagedCertApi

All URIs are relative to *https://messaging.twilio.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**update_request_managed_cert**](MessagingV1RequestManagedCertApi.md#update_request_managed_cert) | **POST** /v1/LinkShortening/Domains/{DomainSid}/RequestManagedCert | 



## update_request_managed_cert

> models::MessagingV1RequestManagedCert update_request_managed_cert(domain_sid)




### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**domain_sid** | **String** | Unique string used to identify the domain that this certificate should be associated with. | [required] |

### Return type

[**models::MessagingV1RequestManagedCert**](messaging.v1.request_managed_cert.md)

### Authorization

[accountSid_authToken](../README.md#accountSid_authToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

