# MessagingV1DomainConfigMessagingService

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**domain_sid** | Option<**String**> | The unique string that we created to identify the Domain resource. | [optional]
**config_sid** | Option<**String**> | The unique string that we created to identify the Domain config (prefix ZK). | [optional]
**messaging_service_sid** | Option<**String**> | The unique string that identifies the messaging service | [optional]
**fallback_url** | Option<**String**> | Any requests we receive to this domain that do not match an existing shortened message will be redirected to the fallback url. These will likely be either expired messages, random misdirected traffic, or intentional scraping. | [optional]
**callback_url** | Option<**String**> | URL to receive click events to your webhook whenever the recipients click on the shortened links. | [optional]
**continue_on_failure** | Option<**bool**> | Boolean field to set customer delivery preference when there is a failure in linkShortening service | [optional]
**date_created** | Option<**chrono::DateTime<chrono::FixedOffset>**> | Date this Domain Config was created. | [optional]
**date_updated** | Option<**chrono::DateTime<chrono::FixedOffset>**> | Date that this Domain Config was last updated. | [optional]
**url** | Option<**String**> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


