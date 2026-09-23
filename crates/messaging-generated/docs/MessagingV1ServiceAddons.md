# MessagingV1ServiceAddons

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**sid** | Option<**String**> | The unique string that we created to identify the add on resource. | [optional]
**account_sid** | Option<**String**> | The SID of the [Account](https://www.twilio.com/docs/iam/api/account) that created the add on resource. | [optional]
**service_sid** | Option<**String**> | The SID of the [Service](https://www.twilio.com/docs/chat/rest/service-resource) the resource is associated with. | [optional]
**date_created** | Option<**chrono::DateTime<chrono::FixedOffset>**> | The date and time in GMT when the resource was created specified in [ISO 8601](https://en.wikipedia.org/wiki/ISO_8601) format. | [optional]
**date_updated** | Option<**chrono::DateTime<chrono::FixedOffset>**> | The date and time in GMT when the resource was last updated specified in [ISO 8601](https://en.wikipedia.org/wiki/ISO_8601) format. | [optional]
**add_on_type_sid** | Option<**String**> | The SID that identifies the add on type. | [optional]
**add_on_config** | Option<**String**> | The config of the add on in JSON string format. | [optional]
**url** | Option<**String**> | The absolute URL of the add on resource. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


