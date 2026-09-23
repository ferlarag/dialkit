# \MessagingV1DeactivationsApi

All URIs are relative to *https://messaging.twilio.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**fetch_deactivation**](MessagingV1DeactivationsApi.md#fetch_deactivation) | **GET** /v1/Deactivations | Fetch a list of all United States numbers that have been deactivated on a specific date.



## fetch_deactivation

> fetch_deactivation(date)
Fetch a list of all United States numbers that have been deactivated on a specific date.

Fetch a list of all United States numbers that have been deactivated on a specific date.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**date** | Option<**chrono::NaiveDate**> | The request will return a list of all United States Phone Numbers that were deactivated on the day specified by this parameter. This date should be specified in YYYY-MM-DD format. |  |

### Return type

 (empty response body)

### Authorization

[accountSid_authToken](../README.md#accountSid_authToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

