service_sdk::macros::use_my_sb_entity_protobuf_model!();
#[derive(Clone, PartialEq, ::prost::Message)]
#[my_sb_entity_protobuf_model(topic_id = "account-email-changed")]
pub struct AccountEmailChangedSbModel {
    #[prost(message, tag = "1")]
    pub event: Option<AccountEmailChangedSbModel>,
}

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AccountEmailChangedSbModel {
    #[prost(string, tag = "1")]
    pub trader_id: String,

    #[prost(string, tag = "2")]
    pub brand_id: String,

    #[prost(string, tag = "3")]
    pub new_email: String,
}
