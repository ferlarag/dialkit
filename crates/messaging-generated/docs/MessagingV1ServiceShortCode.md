# MessagingV1ServiceShortCode

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**sid** | Option<**String**> | The unique string that we created to identify the ShortCode resource. | [optional]
**account_sid** | Option<**String**> | The SID of the [Account](https://www.twilio.com/docs/iam/api/account) that created the ShortCode resource. | [optional]
**service_sid** | Option<**String**> | The SID of the [Service](https://www.twilio.com/docs/chat/rest/service-resource) the resource is associated with. | [optional]
**date_created** | Option<**chrono::DateTime<chrono::FixedOffset>**> | The date and time in GMT when the resource was created specified in [ISO 8601](https://en.wikipedia.org/wiki/ISO_8601) format. | [optional]
**date_updated** | Option<**chrono::DateTime<chrono::FixedOffset>**> | The date and time in GMT when the resource was last updated specified in [ISO 8601](https://en.wikipedia.org/wiki/ISO_8601) format. | [optional]
**short_code** | Option<**String**> | The [E.164](https://www.twilio.com/docs/glossary/what-e164) format of the short code. | [optional]
**country_code** | Option<**String**> | The 2-character [ISO Country Code](https://www.iso.org/iso-3166-country-codes.html) of the number. | [optional]
**capabilities** | Option<**Vec<String>**> | An array of values that describe whether the number can receive calls or messages. Can be: `SMS` and `MMS`. | [optional]
**url** | Option<**String**> | The absolute URL of the ShortCode resource. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


