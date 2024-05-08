use crate::common::AccountType; 

service_sdk::macros::use_my_sb_entity_protobuf_model!();
#[derive(Clone, PartialEq, ::prost::Message)]
#[my_sb_entity_protobuf_model(topic_id = "metatrader-balance")]
pub struct MetatraderBalanceSbModel {
    #[prost(message, tag = "1")]
    pub event: Option<MetatraderBalanceBodySbModel>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum BalanceEventType {
    OperationAdded = 0,
    OperationUpdated = 1,
    OperationDeleted = 2,
}

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MetatraderBalanceBodySbModel {
    #[prost(string, tag = "1")]
    pub trader_account_id: String,

    #[prost(string, tag = "2")]
    pub platform_account_id: String,

    #[prost(string, tag = "3")]
    pub deal_id: String,

    #[prost(int32, tag = "4")]
    pub platform_id: i32,

    #[prost(enumeration = "AccountType", tag = "5")]
    pub account_type: i32,

    #[prost(enumeration = "BalanceEventType", tag = "8")]
    pub event_type: i32,

    #[prost(int64, tag = "9")]
    pub time: i64,
}