# MessagingV1RequestManagedCert

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**domain_sid** | Option<**String**> | The unique string that we created to identify the Domain resource. | [optional]
**date_updated** | Option<**chrono::DateTime<chrono::FixedOffset>**> | Date that this Domain was last updated. | [optional]
**date_created** | Option<**chrono::DateTime<chrono::FixedOffset>**> | Date that this Domain was registered to the Twilio platform to create a new Domain object. | [optional]
**date_expires** | Option<**chrono::DateTime<chrono::FixedOffset>**> | Date that the private certificate associated with this domain expires. This is the expiration date of your existing cert. | [optional]
**domain_name** | Option<**String**> | Full url path for this domain. | [optional]
**certificate_sid** | Option<**String**> | The unique string that we created to identify this Certificate resource. | [optional]
**url** | Option<**String**> |  | [optional]
**managed** | Option<**bool**> | A boolean flag indicating if the certificate is managed by Twilio. | [optional]
**requesting** | Option<**bool**> | A boolean flag indicating if a managed certificate needs to be fulfilled by Twilio. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


