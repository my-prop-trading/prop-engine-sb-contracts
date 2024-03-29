use crate::common::TradingPlatform; 
use crate::common::Broker;

service_sdk::macros::use_my_sb_entity_protobuf_model!();
#[derive(Clone, PartialEq, ::prost::Message)]
#[my_sb_entity_protobuf_model(topic_id = "payout-status-update")]
pub struct PayoutStatusUpdateSbModel {
    #[prost(message, tag = "1")]
    pub event: Option<PayoutStatusUpdateBodySbModel>,
}

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PayoutStatusUpdateBodySbModel {
    #[prost(string, tag = "1")]
    pub payout_id: String,

    #[prost(enumeration="TradingPlatform", tag = "2")]
    pub trading_platform_type: i32,

    #[prost(enumeration="Broker", tag = "3")]
    pub broker_type: i32,

    #[prost(enumeration="PayoutStatus", tag = "4")]
    pub status: i32 ,

    #[prost(string, tag = "5")]
    pub brand: String,

    #[prost(string, tag = "6")]
    pub client_id: String,

    #[prost(string, tag = "7")]
    pub trader_account_id: String,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum PayoutStatus {
    Pending = 0,
    Success = 1,
    Fail = 2,
}
