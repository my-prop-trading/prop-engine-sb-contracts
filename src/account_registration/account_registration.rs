service_sdk::macros::use_my_sb_entity_protobuf_model!();
#[derive(Clone, PartialEq, ::prost::Message)]
#[my_sb_entity_protobuf_model(topic_id = "account-registered")]
pub struct AccountRegisteredSbModel {
    #[prost(message, tag = "1")]
    pub event: Option<RegisteredSbModel>,
}

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RegisteredSbModel {
    #[prost(string, tag = "1")]
    pub trader_id: String,

    #[prost(string, tag = "2")]
    pub brand_id: String,

    #[prost(string, tag = "3")]
    pub code: String,

    #[prost(string, tag = "4")]
    pub tmp_password: String,

    #[prost(string, tag = "5")]
    pub personal_data: PersonalDataSbModel,
}

pub struct PersonalDataSbModel {
    #[prost(message, tag = "1")]
    pub id: String,
    #[prost(string, tag = "2")]
    pub email: Optional<String>,
    #[prost(string, tag = "3")]
    pub first_name: Optional<String>,
    #[prost(string, tag = "4")]
    pub last_name: Optional<String>,
    #[prost(string, tag = "5")]
    pub city: Optional<String>,
    #[prost(string, tag = "6")]
    pub date_of_birth: Optional<String>,
    #[prost(string, tag = "7")]
    pub country_of_residence: Optional<String>,
    #[prost(string, tag = "8")]
    pub country_of_citizenship: Optional<String>,
    #[prost(string, tag = "9")]
    pub zip_code: Optional<String>,
    #[prost(string, tag = "10")]
    pub phone: Optional<String>,
    #[prost(string, tag = "11")]
    pub address: Optional<String>,
    #[prost(string, tag = "12")]
    pub sex: Optional<String>,
    #[prost(string, tag = "13")]
    pub account_type: Optional<String>,
    #[prost(string, tag = "14")]
    pub created_date: Optional<String>,
    #[prost(string, tag = "15")]
    pub region: Optional<String>,
    #[prost(string, tag = "16")]
    pub country_of_registration: Optional<String>,
    #[prost(string, tag = "17")]
    pub country_of_registration_by_ip: Optional<String>,
}

// Sanitized versions for logging
/// Sanitized version of the RegisteredSbModel
#[derive(Debug)]
pub struct SanitizedRegisteredSbModel {
    pub trader_id: String,
    pub brand_id: String,
}

#[derive(Debug)]
pub struct SanitizedAccountRegisteredSbModel {
    pub event: Option<SanitizedRegisteredSbModel>,
}

// Implement methods to create sanitized models
impl RegisteredSbModel {
    pub fn to_sanitized(&self) -> SanitizedRegisteredSbModel {
        SanitizedRegisteredSbModel {
            trader_id: self.trader_id.clone(),
            brand_id: self.brand_id.clone(),
        }
    }
}

impl AccountRegisteredSbModel {
    pub fn to_sanitized(&self) -> SanitizedAccountRegisteredSbModel {
        SanitizedAccountRegisteredSbModel {
            event: self.event.as_ref().map(|e| e.to_sanitized()),
        }
    }
}