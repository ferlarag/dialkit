# MessagingV1ServiceGenericSender

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**sid** | Option<**String**> | The SID to identify the number or channel sender resource. | [optional]
**account_sid** | Option<**String**> | The SID of the [Account](https://www.twilio.com/docs/iam/api/account) that created the number or channel sender resource. | [optional]
**service_sid** | Option<**String**> | The SID of the [Service](https://www.twilio.com/docs/chat/rest/service-resource) the resource is associated with. | [optional]
**date_created** | Option<**chrono::DateTime<chrono::FixedOffset>**> | The date and time in GMT when the resource was created specified in [ISO 8601](https://en.wikipedia.org/wiki/ISO_8601) format. | [optional]
**date_updated** | Option<**chrono::DateTime<chrono::FixedOffset>**> | The date and time in GMT when the resource was last updated specified in [ISO 8601](https://en.wikipedia.org/wiki/ISO_8601) format. | [optional]
**sender** | Option<**String**> | The unique string that identifies the number in [E.164](https://www.twilio.com/docs/glossary/what-e164) format or the channel sender e.g whatsapp:+123456XXXX. | [optional]
**sender_type** | Option<**String**> | A string value that identifies the number or channel sender type e.g AlphaSenderId, LongCode, ShortCode, Whatsapp, RCS. | [optional]
**country_code** | Option<**String**> | The 2-character [ISO Country Code](https://www.iso.org/iso-3166-country-codes.html) of the number or channel sender. | [optional]
**capabilities** | Option<**Vec<String>**> | The capabilities of the number or channel sender (e.g. SMS, MMS, Voice). Populated for phone-number senders; empty for channel senders which have a standardized capability. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


