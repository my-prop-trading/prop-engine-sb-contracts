service_sdk::macros::use_my_sb_entity_protobuf_model!();
#[derive(Clone, PartialEq, ::prost::Message)]
#[my_sb_entity_protobuf_model(topic_id = "account-password-changed")]
pub struct AccountPasswordChangedSbModel {
    #[prost(message, tag = "1")]
    pub event: Option<PasswordChangedSbModel>,
}

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PasswordChangedSbModel {
    #[prost(string, tag = "1")]
    pub trader_id: String,

    #[prost(string, tag = "2")]
    pub brand_id: String,

    #[prost(string, tag = "3")]
    pub new_password: String,
}

