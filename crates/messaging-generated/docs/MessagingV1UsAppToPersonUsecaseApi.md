# \MessagingV1UsAppToPersonUsecaseApi

All URIs are relative to *https://messaging.twilio.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**fetch_us_app_to_person_usecase**](MessagingV1UsAppToPersonUsecaseApi.md#fetch_us_app_to_person_usecase) | **GET** /v1/Services/{MessagingServiceSid}/Compliance/Usa2p/Usecases | 



## fetch_us_app_to_person_usecase

> models::MessagingV1ServiceUsAppToPersonUsecase fetch_us_app_to_person_usecase(messaging_service_sid, brand_registration_sid)




### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**messaging_service_sid** | **String** | The SID of the [Messaging Service](https://www.twilio.com/docs/messaging/api/service-resource) to fetch the resource from. | [required] |
**brand_registration_sid** | Option<**String**> | The unique string to identify the A2P brand. |  |

### Return type

[**models::MessagingV1ServiceUsAppToPersonUsecase**](messaging.v1.service.us_app_to_person_usecase.md)

### Authorization

[accountSid_authToken](../README.md#accountSid_authToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

