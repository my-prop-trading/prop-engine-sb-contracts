use crate::common::TradingPlatform; 
use crate::common::Broker;

service_sdk::macros::use_my_sb_entity_protobuf_model!();
#[derive(Clone, PartialEq, ::prost::Message)]
#[my_sb_entity_protobuf_model(topic_id = "metatrader-account-open-position")]
pub struct MetatraderAccountClosePositionSbModel {
    #[prost(message, tag = "1")]
    pub event: Option<MetatraderAccountClosePositionBodySbModel>,
}

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MetatraderAccountClosePositionBodySbModel {
    #[prost(string, tag = "1")]
    pub trader_account_id: String,

    #[prost(int64, tag = "2")]
    pub metatrader_login: i64,

    #[prost(int64, tag = "3")]
    pub position_id: i64,

    #[prost(enumeration = "TradingPlatform", tag = "4")]
    pub trading_platform_type: i32,

    #[prost(int32, tag = "5")]
    pub broker_type: i32,

    #[prost(int64, tag = "6")]
    pub close_time: i64,
}