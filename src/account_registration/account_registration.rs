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

