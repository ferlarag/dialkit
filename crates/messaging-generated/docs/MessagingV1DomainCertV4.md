# MessagingV1DomainCertV4

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**domain_sid** | Option<**String**> | The unique string that we created to identify the Domain resource. | [optional]
**date_updated** | Option<**chrono::DateTime<chrono::FixedOffset>**> | Date that this Domain was last updated. | [optional]
**date_expires** | Option<**chrono::DateTime<chrono::FixedOffset>**> | Date that the private certificate associated with this domain expires. You will need to update the certificate before that date to ensure your shortened links will continue to work. | [optional]
**date_created** | Option<**chrono::DateTime<chrono::FixedOffset>**> | Date that this Domain was registered to the Twilio platform to create a new Domain object. | [optional]
**domain_name** | Option<**String**> | Full url path for this domain. | [optional]
**certificate_sid** | Option<**String**> | The unique string that we created to identify this Certificate resource. | [optional]
**url** | Option<**String**> |  | [optional]
**cert_in_validation** | Option<**serde_json::Value**> | Optional JSON field describing the status and upload date of a new certificate in the process of validation | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


