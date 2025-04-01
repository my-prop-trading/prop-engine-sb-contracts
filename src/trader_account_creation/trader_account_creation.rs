use crate::common::{AccountType, TraderAccountPhaseType};
service_sdk::macros::use_my_sb_entity_protobuf_model!();
#[derive(Clone, PartialEq, ::prost::Message)]
#[my_sb_entity_protobuf_model(topic_id = "trader-account-creation")]
pub struct TraderAccountCreatedSbModel {
    #[prost(message, tag = "1")]
    pub event: Option<TraderAccountCreatedBodySbModel>,
}

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct  TraderAccountCreatedBodySbModel {
    #[prost(string, tag = "1")]
    pub trader_account_id: String,

    #[prost(int32, tag = "2")]
    pub platform_id: i32,

    #[prost(enumeration = "AccountType", tag = "3")]
    pub account_type: i32,

    #[prost(string, tag = "4")]
    pub brand: String,

    #[prost(string, tag = "5")]
    pub client_id: String,

    #[prost(int32, tag = "6")]
    pub phase: i32,

    #[prost(string, tag = "7")]
    pub trader_account_aggregated_id: String,

    #[prost(string, tag = "8")]
    pub trading_package_id: String,

    #[prost(int64, tag = "9")]
    pub e_tag: i64,

    #[prost(enumeration="TraderAccountPhaseType", tag = "10")]
    pub phase_type: i32,

    #[prost(string, tag = "11")]
    pub order_id: String,
}
