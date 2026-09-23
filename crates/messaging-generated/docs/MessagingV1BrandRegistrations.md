# MessagingV1BrandRegistrations

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**sid** | Option<**String**> | The unique string to identify Brand Registration. | [optional]
**account_sid** | Option<**String**> | The SID of the [Account](https://www.twilio.com/docs/iam/api/account) that created the Brand Registration resource. | [optional]
**customer_profile_bundle_sid** | Option<**String**> | A2P Messaging Profile Bundle BundleSid. | [optional]
**a2p_profile_bundle_sid** | Option<**String**> | A2P Messaging Profile Bundle BundleSid. | [optional]
**date_created** | Option<**chrono::DateTime<chrono::FixedOffset>**> | The date and time in GMT when the resource was created specified in [ISO 8601](https://en.wikipedia.org/wiki/ISO_8601) format. | [optional]
**date_updated** | Option<**chrono::DateTime<chrono::FixedOffset>**> | The date and time in GMT when the resource was last updated specified in [ISO 8601](https://en.wikipedia.org/wiki/ISO_8601) format. | [optional]
**brand_type** | Option<**String**> | Type of brand. One of: \"STANDARD\", \"SOLE_PROPRIETOR\". SOLE_PROPRIETOR is for the low volume, SOLE_PROPRIETOR campaign use case. There can only be one SOLE_PROPRIETOR campaign created per SOLE_PROPRIETOR brand. STANDARD is for all other campaign use cases. Multiple campaign use cases can be created per STANDARD brand. | [optional]
**status** | Option<[**models::BrandRegistrationsEnumStatus**](BrandRegistrationsEnumStatus.md)> |  | [optional]
**tcr_id** | Option<**String**> | Campaign Registry (TCR) Brand ID. Assigned only after successful brand registration. | [optional]
**failure_reason** | Option<**String**> | DEPRECATED. A reason why brand registration has failed. Only applicable when status is FAILED. | [optional]
**errors** | Option<**Vec<serde_json::Value>**> | A list of errors that occurred during the brand registration process. | [optional]
**url** | Option<**String**> | The absolute URL of the Brand Registration resource. | [optional]
**brand_score** | Option<**i32**> | The secondary vetting score if it was done. Otherwise, it will be the brand score if it's returned from TCR. It may be null if no score is available. | [optional]
**brand_feedback** | Option<[**Vec<models::BrandRegistrationsEnumBrandFeedback>**](BrandRegistrationsEnumBrandFeedback.md)> | DEPRECATED. Feedback on how to improve brand score | [optional]
**identity_status** | Option<[**models::BrandRegistrationsEnumIdentityStatus**](BrandRegistrationsEnumIdentityStatus.md)> |  | [optional]
**russell_3000** | Option<**bool**> | Publicly traded company identified in the Russell 3000 Index | [optional]
**government_entity** | Option<**bool**> | Identified as a government entity | [optional]
**tax_exempt_status** | Option<**String**> | Nonprofit organization tax-exempt status per section 501 of the U.S. tax code. | [optional]
**skip_automatic_sec_vet** | Option<**bool**> | A flag to disable automatic secondary vetting for brands which it would otherwise be done. | [optional]
**mock** | Option<**bool**> | A boolean that specifies whether brand should be a mock or not. If true, brand will be registered as a mock brand. Defaults to false if no value is provided. | [optional]
**links** | Option<**serde_json::Value**> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


