# ApiV2010AccountOutgoingCallerIdSmsVerification

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**phone_number** | Option<**String**> | The phone number being verified in [E.164](https://www.twilio.com/docs/glossary/what-e164) format. | [optional]
**verification_sid** | Option<**String**> | The SID that uniquely identifies the verification. | [optional]
**send_code_attempts** | Option<[**Vec<models::ApiV2010AccountOutgoingCallerIdSmsVerificationSendCodeAttemptsInner>**](ApiV2010AccountOutgoingCallerIdSmsVerificationSendCodeAttemptsInner.md)> | An array of verification attempt objects containing the channel attempted and the channel-specific transaction SID. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


