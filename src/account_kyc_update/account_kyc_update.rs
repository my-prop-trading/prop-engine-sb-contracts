service_sdk::macros::use_my_sb_entity_protobuf_model!();
#[derive(Clone, PartialEq, ::prost::Message)]
#[my_sb_entity_protobuf_model(topic_id = "account-kyc-update")]
pub struct AccountKycUpdateSbModel {
    #[prost(message, tag = "1")]
    pub event: Option<AccountKycUpdateBodySbModel>,
}

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AccountKycUpdateBodySbModel {
    #[prost(string, tag = "1")]
    pub client_id: String,
    #[prost(int64, tag = "2")]
    pub updated_at: i64,
}
