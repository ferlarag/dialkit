# ApiV2010AccountOutgoingCallerIdVerificationCheck

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**phone_number** | Option<**String**> | The phone number being verified in [E.164](https://www.twilio.com/docs/glossary/what-e164) format. | [optional]
**verification_sid** | Option<**String**> | The SID that uniquely identifies the verification. | [optional]
**status** | Option<**String**> | The status of the verification. Can be: `pending`, `approved`, or `failed`. | [optional]
**caller_id_sid** | Option<**String**> | The SID of the OutgoingCallerId resource created when verification is approved. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


