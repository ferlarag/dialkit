# \MessagingV1BrandVettingApi

All URIs are relative to *https://messaging.twilio.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_brand_vetting**](MessagingV1BrandVettingApi.md#create_brand_vetting) | **POST** /v1/a2p/BrandRegistrations/{BrandSid}/Vettings | 
[**fetch_brand_vetting**](MessagingV1BrandVettingApi.md#fetch_brand_vetting) | **GET** /v1/a2p/BrandRegistrations/{BrandSid}/Vettings/{BrandVettingSid} | 
[**list_brand_vetting**](MessagingV1BrandVettingApi.md#list_brand_vetting) | **GET** /v1/a2p/BrandRegistrations/{BrandSid}/Vettings | 



## create_brand_vetting

> models::MessagingV1BrandRegistrationsBrandVetting create_brand_vetting(brand_sid, vetting_provider, vetting_id)




### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**brand_sid** | **String** | The SID of the Brand Registration resource of the vettings to create . | [required] |
**vetting_provider** | [**models::BrandVettingEnumVettingProvider**](BrandVettingEnumVettingProvider.md) |  | [required] |
**vetting_id** | Option<**String**> | The unique ID of the vetting |  |

### Return type

[**models::MessagingV1BrandRegistrationsBrandVetting**](messaging.v1.brand_registrations.brand_vetting.md)

### Authorization

[accountSid_authToken](../README.md#accountSid_authToken)

### HTTP request headers

- **Content-Type**: application/x-www-form-urlencoded
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## fetch_brand_vetting

> models::MessagingV1BrandRegistrationsBrandVetting fetch_brand_vetting(brand_sid, brand_vetting_sid)




### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**brand_sid** | **String** | The SID of the Brand Registration resource of the vettings to read . | [required] |
**brand_vetting_sid** | **String** | The Twilio SID of the third-party vetting record. | [required] |

### Return type

[**models::MessagingV1BrandRegistrationsBrandVetting**](messaging.v1.brand_registrations.brand_vetting.md)

### Authorization

[accountSid_authToken](../README.md#accountSid_authToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_brand_vetting

> models::ListBrandVettingResponse list_brand_vetting(brand_sid, vetting_provider)




### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**brand_sid** | **String** | The SID of the Brand Registration resource of the vettings to read . | [required] |
**vetting_provider** | Option<[**BrandVettingEnumVettingProvider**](BrandVettingEnumVettingProvider.md)> | The third-party provider of the vettings to read |  |

### Return type

[**models::ListBrandVettingResponse**](ListBrandVettingResponse.md)

### Authorization

[accountSid_authToken](../README.md#accountSid_authToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

