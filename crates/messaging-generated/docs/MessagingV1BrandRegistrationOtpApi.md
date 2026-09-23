# \MessagingV1BrandRegistrationOtpApi

All URIs are relative to *https://messaging.twilio.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_brand_registration_otp**](MessagingV1BrandRegistrationOtpApi.md#create_brand_registration_otp) | **POST** /v1/a2p/BrandRegistrations/{BrandRegistrationSid}/SmsOtp | 



## create_brand_registration_otp

> models::MessagingV1BrandRegistrationsBrandRegistrationOtp create_brand_registration_otp(brand_registration_sid)




### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**brand_registration_sid** | **String** | Brand Registration Sid of Sole Proprietor Brand. | [required] |

### Return type

[**models::MessagingV1BrandRegistrationsBrandRegistrationOtp**](messaging.v1.brand_registrations.brand_registration_otp.md)

### Authorization

[accountSid_authToken](../README.md#accountSid_authToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

