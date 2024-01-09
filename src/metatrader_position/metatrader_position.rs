use crate::common::TradingPlatform; 

service_sdk::macros::use_my_sb_entity_protobuf_model!();
#[derive(Clone, PartialEq, ::prost::Message)]
#[my_sb_entity_protobuf_model(topic_id = "metatrader-position")]
pub struct MetatraderPositionSbModel {
    #[prost(message, tag = "1")]
    pub event: Option<MetatraderPositionBodySbModel>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum AccountType {
    Demo = 0,
    Live = 1,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum PositionEventType {
    PositionOpened = 0,
    PositionUpdated = 1,
    PositionClosed = 2,
}


#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MetatraderPositionBodySbModel {
    #[prost(string, tag = "1")]
    pub trader_account_id: String,

    #[prost(int64, tag = "2")]
    pub metatrader_login: i64,

    #[prost(int64, tag = "3")]
    pub position_id: i64,

    #[prost(enumeration = "TradingPlatform", tag = "4")]
    pub trading_platform_type: i32,

    #[prost(enumeration = "AccountType", tag = "5")]
    pub account_type: i32,

    #[prost(int32, tag = "6")]
    pub broker_id: i32,

    #[prost(int32, tag = "7")]
    pub trading_platform_id: i32,

    #[prost(enumeration = "PositionEventType", tag = "8")]
    pub event_type: i32,
}