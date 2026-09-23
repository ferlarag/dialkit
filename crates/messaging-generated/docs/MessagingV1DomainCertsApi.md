# \MessagingV1DomainCertsApi

All URIs are relative to *https://messaging.twilio.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**delete_domain_cert_v4**](MessagingV1DomainCertsApi.md#delete_domain_cert_v4) | **DELETE** /v1/LinkShortening/Domains/{DomainSid}/Certificate | 
[**fetch_domain_cert_v4**](MessagingV1DomainCertsApi.md#fetch_domain_cert_v4) | **GET** /v1/LinkShortening/Domains/{DomainSid}/Certificate | 
[**update_domain_cert_v4**](MessagingV1DomainCertsApi.md#update_domain_cert_v4) | **POST** /v1/LinkShortening/Domains/{DomainSid}/Certificate | 



## delete_domain_cert_v4

> delete_domain_cert_v4(domain_sid)




### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**domain_sid** | **String** | Unique string used to identify the domain that this certificate should be associated with. | [required] |

### Return type

 (empty response body)

### Authorization

[accountSid_authToken](../README.md#accountSid_authToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## fetch_domain_cert_v4

> models::MessagingV1DomainCertV4 fetch_domain_cert_v4(domain_sid)




### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**domain_sid** | **String** | Unique string used to identify the domain that this certificate should be associated with. | [required] |

### Return type

[**models::MessagingV1DomainCertV4**](messaging.v1.domain_cert_v4.md)

### Authorization

[accountSid_authToken](../README.md#accountSid_authToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_domain_cert_v4

> models::MessagingV1DomainCertV4 update_domain_cert_v4(domain_sid, tls_cert)




### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**domain_sid** | **String** | Unique string used to identify the domain that this certificate should be associated with. | [required] |
**tls_cert** | **String** | Contains the full TLS certificate and private for this domain in PEM format: https://en.wikipedia.org/wiki/Privacy-Enhanced_Mail. Twilio uses this information to process HTTPS traffic sent to your domain. | [required] |

### Return type

[**models::MessagingV1DomainCertV4**](messaging.v1.domain_cert_v4.md)

### Authorization

[accountSid_authToken](../README.md#accountSid_authToken)

### HTTP request headers

- **Content-Type**: application/x-www-form-urlencoded
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

