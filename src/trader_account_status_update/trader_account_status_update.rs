use crate::common::TradingPlatform; 
use crate::common::Broker;

#[derive(Clone, PartialEq, ::prost::Message)]
#[my_service_bus_macros::my_sb_entity_protobuf_model(topic_id = "trader-account-status-update")]
pub struct TraderAccountStatusUpdateSbModel {
    #[prost(message, tag = "1")]
    pub event: Option<TraderAccountStatusUpdateBodySbModel>,
}

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TraderAccountStatusUpdateBodySbModel {
    #[prost(string, tag = "1")]
    pub trader_account_id: String,

    #[prost(enumeration="TradingPlatform", tag = "2")]
    pub trading_platform_type: i32,

    #[prost(enumeration="Broker", tag = "3")]
    pub broker_type: i32,

    #[prost(enumeration="TraderAccountStatus", tag = "4")]
    pub status: i32 ,

    #[prost(string, tag = "5")]
    pub brand: String,

    #[prost(string, tag = "6")]
    pub client_id: String,

    #[prost(int32, tag = "7")]
    pub phase: i32,

    #[prost(string, tag = "8")]
    pub trader_account_aggregated_id: String,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum TraderAccountStatus {
    New = 0,
    Active = 1,
    Disabled = 2,
    StageCompleted = 3,
}
