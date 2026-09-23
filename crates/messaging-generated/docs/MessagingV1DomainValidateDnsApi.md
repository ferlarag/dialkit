# \MessagingV1DomainValidateDnsApi

All URIs are relative to *https://messaging.twilio.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**fetch_domain_dns_validation**](MessagingV1DomainValidateDnsApi.md#fetch_domain_dns_validation) | **GET** /v1/LinkShortening/Domains/{DomainSid}/ValidateDns | 



## fetch_domain_dns_validation

> models::MessagingV1DomainDnsValidation fetch_domain_dns_validation(domain_sid)




### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**domain_sid** | **String** | Unique string used to identify the domain. | [required] |

### Return type

[**models::MessagingV1DomainDnsValidation**](messaging.v1.domain_dns_validation.md)

### Authorization

[accountSid_authToken](../README.md#accountSid_authToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

