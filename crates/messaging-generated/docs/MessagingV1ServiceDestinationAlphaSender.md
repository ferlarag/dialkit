# MessagingV1ServiceDestinationAlphaSender

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**sid** | Option<**String**> | The unique string that we created to identify the AlphaSender resource. | [optional]
**account_sid** | Option<**String**> | The SID of the [Account](https://www.twilio.com/docs/iam/api/account) that created the AlphaSender resource. | [optional]
**service_sid** | Option<**String**> | The SID of the [Service](https://www.twilio.com/docs/chat/rest/service-resource) the resource is associated with. | [optional]
**date_created** | Option<**chrono::DateTime<chrono::FixedOffset>**> | The date and time in GMT when the resource was created specified in [ISO 8601](https://en.wikipedia.org/wiki/ISO_8601) format. | [optional]
**date_updated** | Option<**chrono::DateTime<chrono::FixedOffset>**> | The date and time in GMT when the resource was last updated specified in [ISO 8601](https://en.wikipedia.org/wiki/ISO_8601) format. | [optional]
**alpha_sender** | Option<**String**> | The Alphanumeric Sender ID string. | [optional]
**capabilities** | Option<**Vec<String>**> | An array of values that describe whether the number can receive calls or messages. Can be: `SMS`. | [optional]
**url** | Option<**String**> | The absolute URL of the AlphaSender resource. | [optional]
**iso_country_code** | Option<**String**> | The Two Character ISO Country Code the Alphanumeric Sender ID will be used for. For Default Alpha Senders that work across countries, this value will be an empty string | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


