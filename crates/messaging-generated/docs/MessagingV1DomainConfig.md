# MessagingV1DomainConfig

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**domain_sid** | Option<**String**> | The unique string that we created to identify the Domain resource. | [optional]
**config_sid** | Option<**String**> | The unique string that we created to identify the Domain config (prefix ZK). | [optional]
**fallback_url** | Option<**String**> | Any requests we receive to this domain that do not match an existing shortened message will be redirected to the fallback url. These will likely be either expired messages, random misdirected traffic, or intentional scraping. | [optional]
**callback_url** | Option<**String**> | URL to receive click events to your webhook whenever the recipients click on the shortened links. | [optional]
**continue_on_failure** | Option<**bool**> | Boolean field to set customer delivery preference when there is a failure in linkShortening service | [optional]
**date_created** | Option<**chrono::DateTime<chrono::FixedOffset>**> | Date this Domain Config was created. | [optional]
**date_updated** | Option<**chrono::DateTime<chrono::FixedOffset>**> | Date that this Domain Config was last updated. | [optional]
**url** | Option<**String**> |  | [optional]
**disable_https** | Option<**bool**> | Customer's choice to send links with/without \"https://\" attached to shortened url. If true, messages will not be sent with https:// at the beginning of the url. If false, messages will be sent with https:// at the beginning of the url. False is the default behavior if it is not specified. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


