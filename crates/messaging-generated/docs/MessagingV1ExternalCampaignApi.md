# \MessagingV1ExternalCampaignApi

All URIs are relative to *https://messaging.twilio.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_external_campaign**](MessagingV1ExternalCampaignApi.md#create_external_campaign) | **POST** /v1/Services/PreregisteredUsa2p | 



## create_external_campaign

> models::MessagingV1ExternalCampaign create_external_campaign(campaign_id, messaging_service_sid, cnp_migration)




### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**campaign_id** | **String** | ID of the preregistered campaign. | [required] |
**messaging_service_sid** | **String** | The SID of the [Messaging Service](https://www.twilio.com/docs/messaging/api/service-resource) that the resource is associated with. | [required] |
**cnp_migration** | Option<**bool**> | Customers should use this flag during the ERC registration process to indicate to Twilio that the campaign being registered is undergoing CNP migration. It is important for the user to first trigger the CNP migration process for said campaign in their CSP portal and have Twilio accept the sharing request, before making this api call. |  |

### Return type

[**models::MessagingV1ExternalCampaign**](messaging.v1.external_campaign.md)

### Authorization

[accountSid_authToken](../README.md#accountSid_authToken)

### HTTP request headers

- **Content-Type**: application/x-www-form-urlencoded
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

