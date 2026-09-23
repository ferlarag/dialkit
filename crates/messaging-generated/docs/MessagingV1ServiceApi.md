# \MessagingV1ServiceApi

All URIs are relative to *https://messaging.twilio.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_service**](MessagingV1ServiceApi.md#create_service) | **POST** /v1/Services | 
[**delete_service**](MessagingV1ServiceApi.md#delete_service) | **DELETE** /v1/Services/{Sid} | 
[**fetch_service**](MessagingV1ServiceApi.md#fetch_service) | **GET** /v1/Services/{Sid} | 
[**list_service**](MessagingV1ServiceApi.md#list_service) | **GET** /v1/Services | 
[**update_service**](MessagingV1ServiceApi.md#update_service) | **POST** /v1/Services/{Sid} | 



## create_service

> models::MessagingV1Service create_service(friendly_name, inbound_request_url, inbound_method, fallback_url, fallback_method, status_callback, sticky_sender, mms_converter, smart_encoding, scan_message_content, fallback_to_long_code, area_code_geomatch, validity_period, synchronous_validation, usecase, use_inbound_webhook_on_number)




### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**friendly_name** | **String** | A descriptive string that you create to describe the resource. It can be up to 64 characters long. | [required] |
**inbound_request_url** | Option<**String**> | The URL we call using `inbound_method` when a message is received by any phone number or short code in the Service. When this property is `null`, receiving inbound messages is disabled. All messages sent to the Twilio phone number or short code will not be logged and received on the Account. If the `use_inbound_webhook_on_number` field is enabled then the webhook url defined on the phone number will override the `inbound_request_url` defined for the Messaging Service. |  |
**inbound_method** | Option<**String**> | The HTTP method we should use to call `inbound_request_url`. Can be `GET` or `POST` and the default is `POST`. |  |
**fallback_url** | Option<**String**> | The URL that we call using `fallback_method` if an error occurs while retrieving or executing the TwiML from the Inbound Request URL. If the `use_inbound_webhook_on_number` field is enabled then the webhook url defined on the phone number will override the `fallback_url` defined for the Messaging Service. |  |
**fallback_method** | Option<**String**> | The HTTP method we should use to call `fallback_url`. Can be: `GET` or `POST`. |  |
**status_callback** | Option<**String**> | The URL we should call to [pass status updates](https://www.twilio.com/docs/sms/api/message-resource#message-status-values) about message delivery. |  |
**sticky_sender** | Option<**bool**> | Whether to enable [Sticky Sender](https://www.twilio.com/docs/messaging/services#sticky-sender) on the Service instance. |  |
**mms_converter** | Option<**bool**> | Whether to enable the [MMS Converter](https://www.twilio.com/docs/messaging/services#mms-converter) for messages sent through the Service instance. |  |
**smart_encoding** | Option<**bool**> | Whether to enable [Smart Encoding](https://www.twilio.com/docs/messaging/services#smart-encoding) for messages sent through the Service instance. |  |
**scan_message_content** | Option<[**models::ServiceEnumScanMessageContent**](ServiceEnumScanMessageContent.md)> |  |  |
**fallback_to_long_code** | Option<**bool**> | [OBSOLETE] Former feature used to fallback to long code sender after certain short code message failures. |  |
**area_code_geomatch** | Option<**bool**> | Whether to enable [Area Code Geomatch](https://www.twilio.com/docs/messaging/services#area-code-geomatch) on the Service Instance. |  |
**validity_period** | Option<**i32**> | How long, in seconds, messages sent from the Service are valid. Can be an integer from `1` to `36,000`. Default value is `36,000`. |  |
**synchronous_validation** | Option<**bool**> | Reserved. |  |
**usecase** | Option<**String**> | A string that describes the scenario in which the Messaging Service will be used. Possible values are `notifications`, `marketing`, `verification`, `discussion`, `poll`, `undeclared`. |  |
**use_inbound_webhook_on_number** | Option<**bool**> | A boolean value that indicates either the webhook url configured on the phone number will be used or `inbound_request_url`/`fallback_url` url will be called when a message is received from the phone number. If this field is enabled then the webhook url defined on the phone number will override the `inbound_request_url`/`fallback_url` defined for the Messaging Service. |  |

### Return type

[**models::MessagingV1Service**](messaging.v1.service.md)

### Authorization

[accountSid_authToken](../README.md#accountSid_authToken)

### HTTP request headers

- **Content-Type**: application/x-www-form-urlencoded
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_service

> delete_service(sid)




### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**sid** | **String** | The SID of the Service resource to delete. | [required] |

### Return type

 (empty response body)

### Authorization

[accountSid_authToken](../README.md#accountSid_authToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## fetch_service

> models::MessagingV1Service fetch_service(sid)




### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**sid** | **String** | The SID of the Service resource to fetch. | [required] |

### Return type

[**models::MessagingV1Service**](messaging.v1.service.md)

### Authorization

[accountSid_authToken](../README.md#accountSid_authToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_service

> models::ListServiceResponse list_service(page_size, page, page_token)




### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**page_size** | Option<**u64**> | How many resources to return in each list page. The default is 50, and the maximum is 1000. |  |
**page** | Option<**u32**> | The page index. This value is simply for client state. |  |
**page_token** | Option<**String**> | The page token. This is provided by the API. |  |

### Return type

[**models::ListServiceResponse**](ListServiceResponse.md)

### Authorization

[accountSid_authToken](../README.md#accountSid_authToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_service

> models::MessagingV1Service update_service(sid, friendly_name, inbound_request_url, inbound_method, fallback_url, fallback_method, status_callback, sticky_sender, mms_converter, smart_encoding, scan_message_content, fallback_to_long_code, area_code_geomatch, validity_period, synchronous_validation, usecase, use_inbound_webhook_on_number)




### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**sid** | **String** | The SID of the Service resource to update. | [required] |
**friendly_name** | Option<**String**> | A descriptive string that you create to describe the resource. It can be up to 64 characters long. |  |
**inbound_request_url** | Option<**String**> | The URL we call using `inbound_method` when a message is received by any phone number or short code in the Service. When this property is `null`, receiving inbound messages is disabled. All messages sent to the Twilio phone number or short code will not be logged and received on the Account. If the `use_inbound_webhook_on_number` field is enabled then the webhook url defined on the phone number will override the `inbound_request_url` defined for the Messaging Service. |  |
**inbound_method** | Option<**String**> | The HTTP method we should use to call `inbound_request_url`. Can be `GET` or `POST` and the default is `POST`. |  |
**fallback_url** | Option<**String**> | The URL that we call using `fallback_method` if an error occurs while retrieving or executing the TwiML from the Inbound Request URL. If the `use_inbound_webhook_on_number` field is enabled then the webhook url defined on the phone number will override the `fallback_url` defined for the Messaging Service. |  |
**fallback_method** | Option<**String**> | The HTTP method we should use to call `fallback_url`. Can be: `GET` or `POST`. |  |
**status_callback** | Option<**String**> | The URL we should call to [pass status updates](https://www.twilio.com/docs/sms/api/message-resource#message-status-values) about message delivery. |  |
**sticky_sender** | Option<**bool**> | Whether to enable [Sticky Sender](https://www.twilio.com/docs/messaging/services#sticky-sender) on the Service instance. |  |
**mms_converter** | Option<**bool**> | Whether to enable the [MMS Converter](https://www.twilio.com/docs/messaging/services#mms-converter) for messages sent through the Service instance. |  |
**smart_encoding** | Option<**bool**> | Whether to enable [Smart Encoding](https://www.twilio.com/docs/messaging/services#smart-encoding) for messages sent through the Service instance. |  |
**scan_message_content** | Option<[**models::ServiceEnumScanMessageContent**](ServiceEnumScanMessageContent.md)> |  |  |
**fallback_to_long_code** | Option<**bool**> | [OBSOLETE] Former feature used to fallback to long code sender after certain short code message failures. |  |
**area_code_geomatch** | Option<**bool**> | Whether to enable [Area Code Geomatch](https://www.twilio.com/docs/messaging/services#area-code-geomatch) on the Service Instance. |  |
**validity_period** | Option<**i32**> | How long, in seconds, messages sent from the Service are valid. Can be an integer from `1` to `36,000`. Default value is `36,000`. |  |
**synchronous_validation** | Option<**bool**> | Reserved. |  |
**usecase** | Option<**String**> | A string that describes the scenario in which the Messaging Service will be used. Possible values are `notifications`, `marketing`, `verification`, `discussion`, `poll`, `undeclared`. |  |
**use_inbound_webhook_on_number** | Option<**bool**> | A boolean value that indicates either the webhook url configured on the phone number will be used or `inbound_request_url`/`fallback_url` url will be called when a message is received from the phone number. If this field is enabled then the webhook url defined on the phone number will override the `inbound_request_url`/`fallback_url` defined for the Messaging Service. |  |

### Return type

[**models::MessagingV1Service**](messaging.v1.service.md)

### Authorization

[accountSid_authToken](../README.md#accountSid_authToken)

### HTTP request headers

- **Content-Type**: application/x-www-form-urlencoded
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

