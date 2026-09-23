# \MessagingV1UsAppToPersonApi

All URIs are relative to *https://messaging.twilio.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_us_app_to_person**](MessagingV1UsAppToPersonApi.md#create_us_app_to_person) | **POST** /v1/Services/{MessagingServiceSid}/Compliance/Usa2p | 
[**delete_us_app_to_person**](MessagingV1UsAppToPersonApi.md#delete_us_app_to_person) | **DELETE** /v1/Services/{MessagingServiceSid}/Compliance/Usa2p/{Sid} | 
[**fetch_us_app_to_person**](MessagingV1UsAppToPersonApi.md#fetch_us_app_to_person) | **GET** /v1/Services/{MessagingServiceSid}/Compliance/Usa2p/{Sid} | 
[**list_us_app_to_person**](MessagingV1UsAppToPersonApi.md#list_us_app_to_person) | **GET** /v1/Services/{MessagingServiceSid}/Compliance/Usa2p | 
[**update_us_app_to_person**](MessagingV1UsAppToPersonApi.md#update_us_app_to_person) | **POST** /v1/Services/{MessagingServiceSid}/Compliance/Usa2p/{Sid} | 



## create_us_app_to_person

> models::MessagingV1ServiceUsAppToPersonResponse create_us_app_to_person(messaging_service_sid, brand_registration_sid, description, message_flow, message_samples, us_app_to_person_usecase, has_embedded_links, has_embedded_phone, x_twilio_api_version, opt_in_message, opt_out_message, help_message, opt_in_keywords, opt_out_keywords, help_keywords, subscriber_opt_in, age_gated, direct_lending, privacy_policy_url, terms_and_conditions_url)




### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**messaging_service_sid** | **String** | The SID of the [Messaging Service](https://www.twilio.com/docs/messaging/api/service-resource) to create the resources from. | [required] |
**brand_registration_sid** | **String** | A2P Brand Registration SID | [required] |
**description** | **String** | A short description of what this SMS campaign does. Min length: 40 characters. Max length: 4096 characters. | [required] |
**message_flow** | **String** | Required for all Campaigns. Details around how a consumer opts-in to their campaign, therefore giving consent to receive their messages. If multiple opt-in methods can be used for the same campaign, they must all be listed. 40 character minimum. 2048 character maximum. | [required] |
**message_samples** | [**Vec<String>**](String.md) | An array of sample message strings, min two and max five. Min length for each sample: 20 chars. Max length for each sample: 1024 chars. | [required] |
**us_app_to_person_usecase** | **String** | A2P Campaign Use Case. Examples: [ 2FA, EMERGENCY, MARKETING..] | [required] |
**has_embedded_links** | **bool** | Indicates that this SMS campaign will send messages that contain links. | [required] |
**has_embedded_phone** | **bool** | Indicates that this SMS campaign will send messages that contain phone numbers. | [required] |
**x_twilio_api_version** | Option<**String**> | The version of the Messaging API to use for this request |  |
**opt_in_message** | Option<**String**> | If end users can text in a keyword to start receiving messages from this campaign, the auto-reply messages sent to the end users must be provided. The opt-in response should include the Brand name, confirmation of opt-in enrollment to a recurring message campaign, how to get help, and clear description of how to opt-out. This field is required if end users can text in a keyword to start receiving messages from this campaign. 20 character minimum. 320 character maximum. |  |
**opt_out_message** | Option<**String**> | Upon receiving the opt-out keywords from the end users, Twilio customers are expected to send back an auto-generated response, which must provide acknowledgment of the opt-out request and confirmation that no further messages will be sent. It is also recommended that these opt-out messages include the brand name. This field is required if managing opt out keywords yourself (i.e. not using Twilio's Default or Advanced Opt Out features). 20 character minimum. 320 character maximum. |  |
**help_message** | Option<**String**> | When customers receive the help keywords from their end users, Twilio customers are expected to send back an auto-generated response; this may include the brand name and additional support contact information. This field is required if managing help keywords yourself (i.e. not using Twilio's Default or Advanced Opt Out features). 20 character minimum. 320 character maximum. |  |
**opt_in_keywords** | Option<[**Vec<String>**](String.md)> | If end users can text in a keyword to start receiving messages from this campaign, those keywords must be provided. This field is required if end users can text in a keyword to start receiving messages from this campaign. Values must be alphanumeric. 255 character maximum. |  |
**opt_out_keywords** | Option<[**Vec<String>**](String.md)> | End users should be able to text in a keyword to stop receiving messages from this campaign. Those keywords must be provided. This field is required if managing opt out keywords yourself (i.e. not using Twilio's Default or Advanced Opt Out features). Values must be alphanumeric. 255 character maximum. |  |
**help_keywords** | Option<[**Vec<String>**](String.md)> | End users should be able to text in a keyword to receive help. Those keywords must be provided as part of the campaign registration request. This field is required if managing help keywords yourself (i.e. not using Twilio's Default or Advanced Opt Out features). Values must be alphanumeric. 255 character maximum. |  |
**subscriber_opt_in** | Option<**bool**> | A boolean that specifies whether campaign has Subscriber Optin or not. |  |
**age_gated** | Option<**bool**> | A boolean that specifies whether campaign is age gated or not. |  |
**direct_lending** | Option<**bool**> | A boolean that specifies whether campaign allows direct lending or not. |  |
**privacy_policy_url** | Option<**String**> | The URL of the privacy policy for the campaign. |  |
**terms_and_conditions_url** | Option<**String**> | The URL of the terms and conditions for the campaign. |  |

### Return type

[**models::MessagingV1ServiceUsAppToPersonResponse**](messaging.v1.service.us_app_to_person_response.md)

### Authorization

[accountSid_authToken](../README.md#accountSid_authToken)

### HTTP request headers

- **Content-Type**: application/x-www-form-urlencoded
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_us_app_to_person

> delete_us_app_to_person(messaging_service_sid, sid)




### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**messaging_service_sid** | **String** | The SID of the [Messaging Service](https://www.twilio.com/docs/messaging/api/service-resource) to delete the resource from. | [required] |
**sid** | **String** | The SID of the US A2P Compliance resource to delete `QE2c6890da8086d771620e9b13fadeba0b`. | [required] |

### Return type

 (empty response body)

### Authorization

[accountSid_authToken](../README.md#accountSid_authToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## fetch_us_app_to_person

> models::MessagingV1ServiceUsAppToPersonResponse fetch_us_app_to_person(messaging_service_sid, sid, x_twilio_api_version)




### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**messaging_service_sid** | **String** | The SID of the [Messaging Service](https://www.twilio.com/docs/messaging/api/service-resource) to fetch the resource from. | [required] |
**sid** | **String** | The SID of the US A2P Compliance resource to fetch `QE2c6890da8086d771620e9b13fadeba0b`. | [required] |
**x_twilio_api_version** | Option<**String**> | The version of the Messaging API to use for this request |  |

### Return type

[**models::MessagingV1ServiceUsAppToPersonResponse**](messaging.v1.service.us_app_to_person_response.md)

### Authorization

[accountSid_authToken](../README.md#accountSid_authToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_us_app_to_person

> models::ListUsAppToPersonResponse list_us_app_to_person(messaging_service_sid, page_size, page, page_token, x_twilio_api_version)




### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**messaging_service_sid** | **String** | The SID of the [Messaging Service](https://www.twilio.com/docs/messaging/api/service-resource) to fetch the resource from. | [required] |
**page_size** | Option<**u64**> | How many resources to return in each list page. The default is 50, and the maximum is 1000. |  |
**page** | Option<**u32**> | The page index. This value is simply for client state. |  |
**page_token** | Option<**String**> | The page token. This is provided by the API. |  |
**x_twilio_api_version** | Option<**String**> | The version of the Messaging API to use for this request |  |

### Return type

[**models::ListUsAppToPersonResponse**](ListUsAppToPersonResponse.md)

### Authorization

[accountSid_authToken](../README.md#accountSid_authToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_us_app_to_person

> models::MessagingV1ServiceUsAppToPersonResponse update_us_app_to_person(messaging_service_sid, sid, has_embedded_links, has_embedded_phone, message_samples, message_flow, description, age_gated, direct_lending, x_twilio_api_version, privacy_policy_url, terms_and_conditions_url)




### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**messaging_service_sid** | **String** | The SID of the [Messaging Service](https://www.twilio.com/docs/messaging/services/api) to update the resource from. | [required] |
**sid** | **String** | The SID of the US A2P Compliance resource to update `QE2c6890da8086d771620e9b13fadeba0b`. | [required] |
**has_embedded_links** | **bool** | Indicates that this SMS campaign will send messages that contain links. | [required] |
**has_embedded_phone** | **bool** | Indicates that this SMS campaign will send messages that contain phone numbers. | [required] |
**message_samples** | [**Vec<String>**](String.md) | An array of sample message strings, min two and max five. Min length for each sample: 20 chars. Max length for each sample: 1024 chars. | [required] |
**message_flow** | **String** | Required for all Campaigns. Details around how a consumer opts-in to their campaign, therefore giving consent to receive their messages. If multiple opt-in methods can be used for the same campaign, they must all be listed. 40 character minimum. 2048 character maximum. | [required] |
**description** | **String** | A short description of what this SMS campaign does. Min length: 40 characters. Max length: 4096 characters. | [required] |
**age_gated** | **bool** | A boolean that specifies whether campaign requires age gate for federally legal content. | [required] |
**direct_lending** | **bool** | A boolean that specifies whether campaign allows direct lending or not. | [required] |
**x_twilio_api_version** | Option<**String**> | The version of the Messaging API to use for this request |  |
**privacy_policy_url** | Option<**String**> | The URL of the privacy policy for the campaign. |  |
**terms_and_conditions_url** | Option<**String**> | The URL of the terms and conditions for the campaign. |  |

### Return type

[**models::MessagingV1ServiceUsAppToPersonResponse**](messaging.v1.service.us_app_to_person_response.md)

### Authorization

[accountSid_authToken](../README.md#accountSid_authToken)

### HTTP request headers

- **Content-Type**: application/x-www-form-urlencoded
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

