# MessagingV1ServiceChannelSender

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**account_sid** | Option<**String**> | The SID of the [Account](https://www.twilio.com/docs/iam/api/account) that created the ChannelSender resource. | [optional]
**messaging_service_sid** | Option<**String**> | The SID of the [Service](https://www.twilio.com/docs/messaging/services) the resource is associated with. | [optional]
**sid** | Option<**String**> | The unique string that we created to identify the ChannelSender resource. | [optional]
**sender** | Option<**String**> | The unique string that identifies the sender e.g whatsapp:+123456XXXX. | [optional]
**sender_type** | Option<**String**> | A string value that identifies the sender type e.g WhatsApp, Messenger. | [optional]
**country_code** | Option<**String**> | The 2-character [ISO Country Code](https://www.iso.org/iso-3166-country-codes.html) of the number. | [optional]
**date_created** | Option<**chrono::DateTime<chrono::FixedOffset>**> | The date and time in GMT when the resource was created specified in [ISO 8601](https://en.wikipedia.org/wiki/ISO_8601) format. | [optional]
**date_updated** | Option<**chrono::DateTime<chrono::FixedOffset>**> | The date and time in GMT when the resource was last updated specified in [ISO 8601](https://en.wikipedia.org/wiki/ISO_8601) format. | [optional]
**url** | Option<**String**> | The absolute URL of the ChannelSender resource. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


