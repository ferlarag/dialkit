# MessagingV1ServiceUsAppToPerson

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**sid** | Option<**String**> | The unique string that identifies a US A2P Compliance resource `QE2c6890da8086d771620e9b13fadeba0b`. | [optional]
**account_sid** | Option<**String**> | The SID of the [Account](https://www.twilio.com/docs/iam/api/account) that the Campaign belongs to. | [optional]
**brand_registration_sid** | Option<**String**> | The unique string to identify the A2P brand. | [optional]
**messaging_service_sid** | Option<**String**> | The SID of the [Messaging Service](https://www.twilio.com/docs/messaging/api/service-resource) that the resource is associated with. | [optional]
**description** | Option<**String**> | A short description of what this SMS campaign does. Min length: 40 characters. Max length: 4096 characters. | [optional]
**message_samples** | Option<**Vec<String>**> | An array of sample message strings, min two and max five. Min length for each sample: 20 chars. Max length for each sample: 1024 chars. | [optional]
**us_app_to_person_usecase** | Option<**String**> | A2P Campaign Use Case. Examples: [ 2FA, EMERGENCY, MARKETING, SOLE_PROPRIETOR...]. SOLE_PROPRIETOR campaign use cases can only be created by SOLE_PROPRIETOR Brands, and there can only be one SOLE_PROPRIETOR campaign created per SOLE_PROPRIETOR Brand. | [optional]
**has_embedded_links** | Option<**bool**> | Indicate that this SMS campaign will send messages that contain links. | [optional]
**has_embedded_phone** | Option<**bool**> | Indicates that this SMS campaign will send messages that contain phone numbers. | [optional]
**subscriber_opt_in** | Option<**bool**> | A boolean that specifies whether campaign has Subscriber Optin or not. | [optional]
**age_gated** | Option<**bool**> | A boolean that specifies whether campaign is age gated or not. | [optional]
**direct_lending** | Option<**bool**> | A boolean that specifies whether campaign allows direct lending or not. | [optional]
**campaign_status** | Option<**String**> | Campaign status. Examples: IN_PROGRESS, VERIFIED, FAILED. | [optional]
**campaign_id** | Option<**String**> | The Campaign Registry (TCR) Campaign ID. | [optional]
**is_externally_registered** | Option<**bool**> | Indicates whether the campaign was registered externally or not. | [optional]
**rate_limits** | Option<**serde_json::Value**> | Rate limit and/or classification set by each carrier, Ex. AT&T or T-Mobile. | [optional]
**message_flow** | Option<**String**> | Details around how a consumer opts-in to their campaign, therefore giving consent to receive their messages. If multiple opt-in methods can be used for the same campaign, they must all be listed. 40 character minimum. 2048 character maximum. | [optional]
**opt_in_message** | Option<**String**> | If end users can text in a keyword to start receiving messages from this campaign, the auto-reply messages sent to the end users must be provided. The opt-in response should include the Brand name, confirmation of opt-in enrollment to a recurring message campaign, how to get help, and clear description of how to opt-out. This field is required if end users can text in a keyword to start receiving messages from this campaign. 20 character minimum. 320 character maximum. | [optional]
**opt_out_message** | Option<**String**> | Upon receiving the opt-out keywords from the end users, Twilio customers are expected to send back an auto-generated response, which must provide acknowledgment of the opt-out request and confirmation that no further messages will be sent. It is also recommended that these opt-out messages include the brand name. This field is required if managing opt out keywords yourself (i.e. not using Twilio's Default or Advanced Opt Out features). 20 character minimum. 320 character maximum. | [optional]
**help_message** | Option<**String**> | When customers receive the help keywords from their end users, Twilio customers are expected to send back an auto-generated response; this may include the brand name and additional support contact information. This field is required if managing help keywords yourself (i.e. not using Twilio's Default or Advanced Opt Out features). 20 character minimum. 320 character maximum. | [optional]
**opt_in_keywords** | Option<**Vec<String>**> | If end users can text in a keyword to start receiving messages from this campaign, those keywords must be provided. This field is required if end users can text in a keyword to start receiving messages from this campaign. Values must be alphanumeric. 255 character maximum. | [optional]
**opt_out_keywords** | Option<**Vec<String>**> | End users should be able to text in a keyword to stop receiving messages from this campaign. Those keywords must be provided. This field is required if managing opt out keywords yourself (i.e. not using Twilio's Default or Advanced Opt Out features). Values must be alphanumeric. 255 character maximum. | [optional]
**help_keywords** | Option<**Vec<String>**> | End users should be able to text in a keyword to receive help. Those keywords must be provided as part of the campaign registration request. This field is required if managing help keywords yourself (i.e. not using Twilio's Default or Advanced Opt Out features). Values must be alphanumeric. 255 character maximum. | [optional]
**date_created** | Option<**chrono::DateTime<chrono::FixedOffset>**> | The date and time in GMT when the resource was created specified in [ISO 8601](https://en.wikipedia.org/wiki/ISO_8601) format. | [optional]
**date_updated** | Option<**chrono::DateTime<chrono::FixedOffset>**> | The date and time in GMT when the resource was last updated specified in [ISO 8601](https://en.wikipedia.org/wiki/ISO_8601) format. | [optional]
**url** | Option<**String**> | The absolute URL of the US App to Person resource. | [optional]
**mock** | Option<**bool**> | A boolean that specifies whether campaign is a mock or not. Mock campaigns will be automatically created if using a mock brand. Mock campaigns should only be used for testing purposes. | [optional]
**errors** | Option<**Vec<serde_json::Value>**> | Details indicating why a campaign registration failed. These errors can indicate one or more fields that were incorrect or did not meet review requirements. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


