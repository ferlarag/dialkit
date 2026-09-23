# MessagingV1Service

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**sid** | Option<**String**> | The unique string that we created to identify the Service resource. | [optional]
**account_sid** | Option<**String**> | The SID of the [Account](https://www.twilio.com/docs/iam/api/account) that created the Service resource. | [optional]
**friendly_name** | Option<**String**> | The string that you assigned to describe the resource. | [optional]
**date_created** | Option<**chrono::DateTime<chrono::FixedOffset>**> | The date and time in GMT when the resource was created specified in [ISO 8601](https://en.wikipedia.org/wiki/ISO_8601) format. | [optional]
**date_updated** | Option<**chrono::DateTime<chrono::FixedOffset>**> | The date and time in GMT when the resource was last updated specified in [ISO 8601](https://en.wikipedia.org/wiki/ISO_8601) format. | [optional]
**inbound_request_url** | Option<**String**> | The URL we call using `inbound_method` when a message is received by any phone number or short code in the Service. When this property is `null`, receiving inbound messages is disabled. All messages sent to the Twilio phone number or short code will not be logged and received on the Account. If the `use_inbound_webhook_on_number` field is enabled then the webhook url defined on the phone number will override the `inbound_request_url` defined for the Messaging Service. | [optional]
**inbound_method** | Option<**InboundMethod**> | The HTTP method we use to call `inbound_request_url`. Can be `GET` or `POST`. (enum: GET, POST) | [optional]
**fallback_url** | Option<**String**> | The URL that we call using `fallback_method` if an error occurs while retrieving or executing the TwiML from the Inbound Request URL. If the `use_inbound_webhook_on_number` field is enabled then the webhook url defined on the phone number will override the `fallback_url` defined for the Messaging Service. | [optional]
**fallback_method** | Option<**FallbackMethod**> | The HTTP method we use to call `fallback_url`. Can be: `GET` or `POST`. (enum: GET, POST) | [optional]
**status_callback** | Option<**String**> | The URL we call to [pass status updates](https://www.twilio.com/docs/sms/api/message-resource#message-status-values) about message delivery. | [optional]
**sticky_sender** | Option<**bool**> | Whether to enable [Sticky Sender](https://www.twilio.com/docs/messaging/services#sticky-sender) on the Service instance. | [optional]
**mms_converter** | Option<**bool**> | Whether to enable the [MMS Converter](https://www.twilio.com/docs/messaging/services#mms-converter) for messages sent through the Service instance. | [optional]
**smart_encoding** | Option<**bool**> | Whether to enable [Smart Encoding](https://www.twilio.com/docs/messaging/services#smart-encoding) for messages sent through the Service instance. | [optional]
**scan_message_content** | Option<[**models::ServiceEnumScanMessageContent**](ServiceEnumScanMessageContent.md)> |  | [optional]
**fallback_to_long_code** | Option<**bool**> | [OBSOLETE] Former feature used to fallback to long code sender after certain short code message failures. | [optional]
**area_code_geomatch** | Option<**bool**> | Whether to enable [Area Code Geomatch](https://www.twilio.com/docs/messaging/services#area-code-geomatch) on the Service Instance. | [optional]
**synchronous_validation** | Option<**bool**> | Reserved. | [optional]
**validity_period** | Option<**i32**> | How long, in seconds, messages sent from the Service are valid. Can be an integer from `1` to `36,000`. Default value is `36,000`. | [optional][default to 0]
**url** | Option<**String**> | The absolute URL of the Service resource. | [optional]
**links** | Option<**serde_json::Value**> | The absolute URLs of related resources. | [optional]
**usecase** | Option<**String**> | A string that describes the scenario in which the Messaging Service will be used. Possible values are `notifications`, `marketing`, `verification`, `discussion`, `poll`, `undeclared`. | [optional]
**us_app_to_person_registered** | Option<**bool**> | Whether US A2P campaign is registered for this Service. | [optional]
**use_inbound_webhook_on_number** | Option<**bool**> | A boolean value that indicates either the webhook url configured on the phone number will be used or `inbound_request_url`/`fallback_url` url will be called when a message is received from the phone number. If this field is enabled then the webhook url defined on the phone number will override the `inbound_request_url`/`fallback_url` defined for the Messaging Service. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


