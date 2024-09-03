use crate::common::AccountType; 

service_sdk::macros::use_my_sb_entity_protobuf_model!();
#[derive(Clone, PartialEq, ::prost::Message)]
#[my_sb_entity_protobuf_model(topic_id = "trader-account-reactivated")]
pub struct TraderAccountReactivatedSbModel {
    #[prost(message, tag = "1")]
    pub event: Option<TraderAccountReactivatedBodySbModel>,
}

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TraderAccountReactivatedBodySbModel {
    #[prost(string, tag = "1")]
    pub trader_account_id: String,

    #[prost(string, tag = "2")]
    pub client_id: String,
}