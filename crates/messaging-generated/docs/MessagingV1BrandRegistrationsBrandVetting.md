# MessagingV1BrandRegistrationsBrandVetting

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**account_sid** | Option<**String**> | The SID of the [Account](https://www.twilio.com/docs/iam/api/account) that created the vetting record. | [optional]
**brand_sid** | Option<**String**> | The unique string to identify Brand Registration. | [optional]
**brand_vetting_sid** | Option<**String**> | The Twilio SID of the third-party vetting record. | [optional]
**date_updated** | Option<**chrono::DateTime<chrono::FixedOffset>**> | The date and time in GMT when the resource was last updated specified in [ISO 8601](https://en.wikipedia.org/wiki/ISO_8601) format. | [optional]
**date_created** | Option<**chrono::DateTime<chrono::FixedOffset>**> | The date and time in GMT when the resource was created specified in [ISO 8601](https://en.wikipedia.org/wiki/ISO_8601) format. | [optional]
**vetting_id** | Option<**String**> | The unique identifier of the vetting from the third-party provider. | [optional]
**vetting_class** | Option<**String**> | The type of vetting that has been conducted. One of “STANDARD” (Aegis) or “POLITICAL” (Campaign Verify). | [optional]
**vetting_status** | Option<**String**> | The status of the import vetting attempt. One of “PENDING,” “SUCCESS,” or “FAILED”. | [optional]
**vetting_provider** | Option<[**models::BrandVettingEnumVettingProvider**](BrandVettingEnumVettingProvider.md)> |  | [optional]
**url** | Option<**String**> | The absolute URL of the Brand Vetting resource. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


