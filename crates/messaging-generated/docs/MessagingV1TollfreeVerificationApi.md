# \MessagingV1TollfreeVerificationApi

All URIs are relative to *https://messaging.twilio.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_tollfree_verification**](MessagingV1TollfreeVerificationApi.md#create_tollfree_verification) | **POST** /v1/Tollfree/Verifications | Create a tollfree verification
[**delete_tollfree_verification**](MessagingV1TollfreeVerificationApi.md#delete_tollfree_verification) | **DELETE** /v1/Tollfree/Verifications/{Sid} | Delete a tollfree verification
[**fetch_tollfree_verification**](MessagingV1TollfreeVerificationApi.md#fetch_tollfree_verification) | **GET** /v1/Tollfree/Verifications/{Sid} | Retrieve a tollfree verification
[**list_tollfree_verification**](MessagingV1TollfreeVerificationApi.md#list_tollfree_verification) | **GET** /v1/Tollfree/Verifications | List tollfree verifications
[**update_tollfree_verification**](MessagingV1TollfreeVerificationApi.md#update_tollfree_verification) | **POST** /v1/Tollfree/Verifications/{Sid} | Edit a tollfree verification



## create_tollfree_verification

> models::MessagingV1TollfreeVerification create_tollfree_verification(business_name, business_website, notification_email, use_case_categories, use_case_summary, production_message_sample, opt_in_image_urls, opt_in_type, message_volume, tollfree_phone_number_sid, customer_profile_sid, business_street_address, business_street_address2, business_city, business_state_province_region, business_postal_code, business_country, additional_information, business_contact_first_name, business_contact_last_name, business_contact_email, business_contact_phone, external_reference_id, business_registration_number, business_registration_authority, business_registration_country, business_type, business_registration_phone_number, doing_business_as, opt_in_confirmation_message, help_message_sample, privacy_policy_url, terms_and_conditions_url, age_gated_content, opt_in_keywords, vetting_provider, vetting_id)
Create a tollfree verification

Create a tollfree verification

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**business_name** | **String** | The name of the business or organization using the Tollfree number. | [required] |
**business_website** | **String** | The website of the business or organization using the Tollfree number. | [required] |
**notification_email** | **String** | The email address to receive the notification about the verification result. . | [required] |
**use_case_categories** | Option<[**Vec<String>**](String.md)> | The category of the use case for the Tollfree Number. List as many as are applicable. | [required] |
**use_case_summary** | **String** | Use this to further explain how messaging is used by the business or organization. | [required] |
**production_message_sample** | **String** | An example of message content, i.e. a sample message. | [required] |
**opt_in_image_urls** | [**Vec<String>**](String.md) | Link to an image that shows the opt-in workflow. Multiple images allowed and must be a publicly hosted URL. | [required] |
**opt_in_type** | [**models::TollfreeVerificationEnumOptInType**](TollfreeVerificationEnumOptInType.md) |  | [required] |
**message_volume** | **String** | Estimate monthly volume of messages from the Tollfree Number. | [required] |
**tollfree_phone_number_sid** | **String** | The SID of the Phone Number associated with the Tollfree Verification. | [required] |
**customer_profile_sid** | Option<**String**> | Customer's Profile Bundle BundleSid. |  |
**business_street_address** | Option<**String**> | The address of the business or organization using the Tollfree number. |  |
**business_street_address2** | Option<**String**> | The address of the business or organization using the Tollfree number. |  |
**business_city** | Option<**String**> | The city of the business or organization using the Tollfree number. |  |
**business_state_province_region** | Option<**String**> | The state/province/region of the business or organization using the Tollfree number. |  |
**business_postal_code** | Option<**String**> | The postal code of the business or organization using the Tollfree number. |  |
**business_country** | Option<**String**> | The country of the business or organization using the Tollfree number. |  |
**additional_information** | Option<**String**> | Additional information to be provided for verification. |  |
**business_contact_first_name** | Option<**String**> | The first name of the contact for the business or organization using the Tollfree number. |  |
**business_contact_last_name** | Option<**String**> | The last name of the contact for the business or organization using the Tollfree number. |  |
**business_contact_email** | Option<**String**> | The email address of the contact for the business or organization using the Tollfree number. |  |
**business_contact_phone** | Option<**String**> | The E.164 formatted phone number of the contact for the business or organization using the Tollfree number. |  |
**external_reference_id** | Option<**String**> | An optional external reference ID supplied by customer and echoed back on status retrieval. |  |
**business_registration_number** | Option<**String**> | A legally recognized business registration number. Required for all business types except SOLE_PROPRIETOR. |  |
**business_registration_authority** | Option<[**models::TollfreeVerificationEnumBusinessRegistrationAuthority**](TollfreeVerificationEnumBusinessRegistrationAuthority.md)> |  |  |
**business_registration_country** | Option<**String**> | The country where the business is registered. Required for all business types except SOLE_PROPRIETOR. |  |
**business_type** | Option<[**models::TollfreeVerificationEnumBusinessType**](TollfreeVerificationEnumBusinessType.md)> |  |  |
**business_registration_phone_number** | Option<**String**> | The E.164 formatted number associated with the business. |  |
**doing_business_as** | Option<**String**> | Trade name, sub entity, or downstream business name of business being submitted for verification |  |
**opt_in_confirmation_message** | Option<**String**> | The confirmation message sent to users when they opt in to receive messages. |  |
**help_message_sample** | Option<**String**> | A sample help message provided to users. |  |
**privacy_policy_url** | Option<**String**> | The URL to the privacy policy for the business or organization. |  |
**terms_and_conditions_url** | Option<**String**> | The URL to the terms and conditions for the business or organization. |  |
**age_gated_content** | Option<**bool**> | Indicates if the content is age gated. |  |
**opt_in_keywords** | Option<[**Vec<String>**](String.md)> | List of keywords that users can text in to opt in to receive messages. |  |
**vetting_provider** | Option<[**models::TollfreeVerificationEnumVettingProvider**](TollfreeVerificationEnumVettingProvider.md)> |  |  |
**vetting_id** | Option<**String**> | The unique ID of the vetting |  |

### Return type

[**models::MessagingV1TollfreeVerification**](messaging.v1.tollfree_verification.md)

### Authorization

[accountSid_authToken](../README.md#accountSid_authToken)

### HTTP request headers

- **Content-Type**: application/x-www-form-urlencoded
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_tollfree_verification

> delete_tollfree_verification(sid)
Delete a tollfree verification

Delete a tollfree verification

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**sid** | **String** | The unique string to identify Tollfree Verification. | [required] |

### Return type

 (empty response body)

### Authorization

[accountSid_authToken](../README.md#accountSid_authToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## fetch_tollfree_verification

> models::MessagingV1TollfreeVerification fetch_tollfree_verification(sid)
Retrieve a tollfree verification

Retrieve a tollfree verification

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**sid** | **String** | A unique string identifying a Tollfree Verification. | [required] |

### Return type

[**models::MessagingV1TollfreeVerification**](messaging.v1.tollfree_verification.md)

### Authorization

[accountSid_authToken](../README.md#accountSid_authToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_tollfree_verification

> models::ListTollfreeVerificationResponse list_tollfree_verification(tollfree_phone_number_sid, status, external_reference_id, include_sub_accounts, page_size, page, page_token, trust_product_sid)
List tollfree verifications

List tollfree verifications

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tollfree_phone_number_sid** | Option<**String**> | The SID of the Phone Number associated with the Tollfree Verification. |  |
**status** | Option<[**TollfreeVerificationEnumStatus**](TollfreeVerificationEnumStatus.md)> | The compliance status of the Tollfree Verification record. |  |
**external_reference_id** | Option<**String**> | Customer supplied reference id for the Tollfree Verification record. |  |
**include_sub_accounts** | Option<**bool**> | Whether to include Tollfree Verifications from sub accounts in list response. |  |
**page_size** | Option<**u64**> | How many resources to return in each list page. The default is 50, and the maximum is 1000. |  |
**page** | Option<**u32**> | The page index. This value is simply for client state. |  |
**page_token** | Option<**String**> | The page token. This is provided by the API. |  |
**trust_product_sid** | Option<[**Vec<String>**](String.md)> | The trust product sids / tollfree bundle sids of tollfree verifications |  |

### Return type

[**models::ListTollfreeVerificationResponse**](ListTollfreeVerificationResponse.md)

### Authorization

[accountSid_authToken](../README.md#accountSid_authToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_tollfree_verification

> models::MessagingV1TollfreeVerification update_tollfree_verification(sid, business_name, business_website, notification_email, use_case_categories, use_case_summary, production_message_sample, opt_in_image_urls, opt_in_type, message_volume, business_street_address, business_street_address2, business_city, business_state_province_region, business_postal_code, business_country, additional_information, business_contact_first_name, business_contact_last_name, business_contact_email, business_contact_phone, edit_reason, business_registration_number, business_registration_authority, business_registration_country, business_type, business_registration_phone_number, doing_business_as, opt_in_confirmation_message, help_message_sample, privacy_policy_url, terms_and_conditions_url, age_gated_content, opt_in_keywords, vetting_provider, vetting_id)
Edit a tollfree verification

Edit a tollfree verification

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**sid** | **String** | The unique string to identify Tollfree Verification. | [required] |
**business_name** | Option<**String**> | The name of the business or organization using the Tollfree number. |  |
**business_website** | Option<**String**> | The website of the business or organization using the Tollfree number. |  |
**notification_email** | Option<**String**> | The email address to receive the notification about the verification result. . |  |
**use_case_categories** | Option<[**Vec<String>**](String.md)> | The category of the use case for the Tollfree Number. List as many as are applicable. |  |
**use_case_summary** | Option<**String**> | Use this to further explain how messaging is used by the business or organization. |  |
**production_message_sample** | Option<**String**> | An example of message content, i.e. a sample message. |  |
**opt_in_image_urls** | Option<[**Vec<String>**](String.md)> | Link to an image that shows the opt-in workflow. Multiple images allowed and must be a publicly hosted URL. |  |
**opt_in_type** | Option<[**models::TollfreeVerificationEnumOptInType**](TollfreeVerificationEnumOptInType.md)> |  |  |
**message_volume** | Option<**String**> | Estimate monthly volume of messages from the Tollfree Number. |  |
**business_street_address** | Option<**String**> | The address of the business or organization using the Tollfree number. |  |
**business_street_address2** | Option<**String**> | The address of the business or organization using the Tollfree number. |  |
**business_city** | Option<**String**> | The city of the business or organization using the Tollfree number. |  |
**business_state_province_region** | Option<**String**> | The state/province/region of the business or organization using the Tollfree number. |  |
**business_postal_code** | Option<**String**> | The postal code of the business or organization using the Tollfree number. |  |
**business_country** | Option<**String**> | The country of the business or organization using the Tollfree number. |  |
**additional_information** | Option<**String**> | Additional information to be provided for verification. |  |
**business_contact_first_name** | Option<**String**> | The first name of the contact for the business or organization using the Tollfree number. |  |
**business_contact_last_name** | Option<**String**> | The last name of the contact for the business or organization using the Tollfree number. |  |
**business_contact_email** | Option<**String**> | The email address of the contact for the business or organization using the Tollfree number. |  |
**business_contact_phone** | Option<**String**> | The E.164 formatted phone number of the contact for the business or organization using the Tollfree number. |  |
**edit_reason** | Option<**String**> | Describe why the verification is being edited. If the verification was rejected because of a technical issue, such as the website being down, and the issue has been resolved this parameter should be set to something similar to 'Website fixed'. |  |
**business_registration_number** | Option<**String**> | A legally recognized business registration number |  |
**business_registration_authority** | Option<[**models::TollfreeVerificationEnumBusinessRegistrationAuthority**](TollfreeVerificationEnumBusinessRegistrationAuthority.md)> |  |  |
**business_registration_country** | Option<**String**> | Country business is registered in |  |
**business_type** | Option<[**models::TollfreeVerificationEnumBusinessType**](TollfreeVerificationEnumBusinessType.md)> |  |  |
**business_registration_phone_number** | Option<**String**> | The E.164 formatted number associated with the business. |  |
**doing_business_as** | Option<**String**> | Trade name, sub entity, or downstream business name of business being submitted for verification |  |
**opt_in_confirmation_message** | Option<**String**> | The confirmation message sent to users when they opt in to receive messages. |  |
**help_message_sample** | Option<**String**> | A sample help message provided to users. |  |
**privacy_policy_url** | Option<**String**> | The URL to the privacy policy for the business or organization. |  |
**terms_and_conditions_url** | Option<**String**> | The URL to the terms and conditions for the business or organization. |  |
**age_gated_content** | Option<**bool**> | Indicates if the content is age gated. |  |
**opt_in_keywords** | Option<[**Vec<String>**](String.md)> | List of keywords that users can text in to opt in to receive messages. |  |
**vetting_provider** | Option<[**models::TollfreeVerificationEnumVettingProvider**](TollfreeVerificationEnumVettingProvider.md)> |  |  |
**vetting_id** | Option<**String**> | The unique ID of the vetting |  |

### Return type

[**models::MessagingV1TollfreeVerification**](messaging.v1.tollfree_verification.md)

### Authorization

[accountSid_authToken](../README.md#accountSid_authToken)

### HTTP request headers

- **Content-Type**: application/x-www-form-urlencoded
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

