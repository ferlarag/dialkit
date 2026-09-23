# \MessagingV1DomainConfigApi

All URIs are relative to *https://messaging.twilio.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**fetch_domain_config**](MessagingV1DomainConfigApi.md#fetch_domain_config) | **GET** /v1/LinkShortening/Domains/{DomainSid}/Config | 
[**update_domain_config**](MessagingV1DomainConfigApi.md#update_domain_config) | **POST** /v1/LinkShortening/Domains/{DomainSid}/Config | 



## fetch_domain_config

> models::MessagingV1DomainConfig fetch_domain_config(domain_sid)




### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**domain_sid** | **String** | Unique string used to identify the domain that this config should be associated with. | [required] |

### Return type

[**models::MessagingV1DomainConfig**](messaging.v1.domain_config.md)

### Authorization

[accountSid_authToken](../README.md#accountSid_authToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_domain_config

> models::MessagingV1DomainConfig update_domain_config(domain_sid, fallback_url, callback_url, continue_on_failure, disable_https)




### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**domain_sid** | **String** | Unique string used to identify the domain that this config should be associated with. | [required] |
**fallback_url** | Option<**String**> | Any requests we receive to this domain that do not match an existing shortened message will be redirected to the fallback url. These will likely be either expired messages, random misdirected traffic, or intentional scraping. |  |
**callback_url** | Option<**String**> | URL to receive click events to your webhook whenever the recipients click on the shortened links |  |
**continue_on_failure** | Option<**bool**> | Boolean field to set customer delivery preference when there is a failure in linkShortening service |  |
**disable_https** | Option<**bool**> | Customer's choice to send links with/without \\\"https://\\\" attached to shortened url. If true, messages will not be sent with https:// at the beginning of the url. If false, messages will be sent with https:// at the beginning of the url. False is the default behavior if it is not specified. |  |

### Return type

[**models::MessagingV1DomainConfig**](messaging.v1.domain_config.md)

### Authorization

[accountSid_authToken](../README.md#accountSid_authToken)

### HTTP request headers

- **Content-Type**: application/x-www-form-urlencoded
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

