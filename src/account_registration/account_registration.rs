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