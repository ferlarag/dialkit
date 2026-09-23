# \MessagingV1BrandRegistrationApi

All URIs are relative to *https://messaging.twilio.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_brand_registrations**](MessagingV1BrandRegistrationApi.md#create_brand_registrations) | **POST** /v1/a2p/BrandRegistrations | 
[**fetch_brand_registrations**](MessagingV1BrandRegistrationApi.md#fetch_brand_registrations) | **GET** /v1/a2p/BrandRegistrations/{Sid} | 
[**list_brand_registrations**](MessagingV1BrandRegistrationApi.md#list_brand_registrations) | **GET** /v1/a2p/BrandRegistrations | 
[**update_brand_registrations**](MessagingV1BrandRegistrationApi.md#update_brand_registrations) | **POST** /v1/a2p/BrandRegistrations/{Sid} | 



## create_brand_registrations

> models::MessagingV1BrandRegistrations create_brand_registrations(customer_profile_bundle_sid, a2_p_profile_bundle_sid, brand_type, mock, skip_automatic_sec_vet)




### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**customer_profile_bundle_sid** | **String** | Customer Profile Bundle Sid. | [required] |
**a2_p_profile_bundle_sid** | **String** | A2P Messaging Profile Bundle Sid. | [required] |
**brand_type** | Option<**String**> | Type of brand being created. One of: \\\"STANDARD\\\", \\\"SOLE_PROPRIETOR\\\". SOLE_PROPRIETOR is for low volume, SOLE_PROPRIETOR use cases. STANDARD is for all other use cases. |  |
**mock** | Option<**bool**> | A boolean that specifies whether brand should be a mock or not. If true, brand will be registered as a mock brand. Defaults to false if no value is provided. |  |
**skip_automatic_sec_vet** | Option<**bool**> | A flag to disable automatic secondary vetting for brands which it would otherwise be done. |  |

### Return type

[**models::MessagingV1BrandRegistrations**](messaging.v1.brand_registrations.md)

### Authorization

[accountSid_authToken](../README.md#accountSid_authToken)

### HTTP request headers

- **Content-Type**: application/x-www-form-urlencoded
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## fetch_brand_registrations

> models::MessagingV1BrandRegistrations fetch_brand_registrations(sid)




### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**sid** | **String** | The SID of the Brand Registration resource to fetch. | [required] |

### Return type

[**models::MessagingV1BrandRegistrations**](messaging.v1.brand_registrations.md)

### Authorization

[accountSid_authToken](../README.md#accountSid_authToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_brand_registrations

> models::ListBrandRegistrationsResponse list_brand_registrations(page_size, page, page_token)




### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**page_size** | Option<**u64**> | How many resources to return in each list page. The default is 50, and the maximum is 1000. |  |
**page** | Option<**u32**> | The page index. This value is simply for client state. |  |
**page_token** | Option<**String**> | The page token. This is provided by the API. |  |

### Return type

[**models::ListBrandRegistrationsResponse**](ListBrandRegistrationsResponse.md)

### Authorization

[accountSid_authToken](../README.md#accountSid_authToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_brand_registrations

> models::MessagingV1BrandRegistrations update_brand_registrations(sid)




### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**sid** | **String** | The SID of the Brand Registration resource to update. | [required] |

### Return type

[**models::MessagingV1BrandRegistrations**](messaging.v1.brand_registrations.md)

### Authorization

[accountSid_authToken](../README.md#accountSid_authToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

