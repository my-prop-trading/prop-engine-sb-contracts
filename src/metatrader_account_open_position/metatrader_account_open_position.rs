#[derive(Clone, PartialEq, ::prost::Message)]
#[my_service_bus_macros::my_sb_entity_protobuf_model(topic_id = "metatrader-account-open-position")]
pub struct MetatraderAccountOpenPositionSbModel {
    #[prost(message, tag = "1")]
    pub event: Option<MetatraderAccountOpenPositionBodySbModel>,
}

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MetatraderAccountOpenPositionBodySbModel {
    #[prost(string, tag = "1")]
    pub trader_account_id: String,

    #[prost(int64, tag = "2")]
    pub metatrader_login: i64,

    #[prost(int64, tag = "3")]
    pub position_id: i64,

    #[prost(enumeration = "TradingPlatformDto", tag = "4")]
    pub trading_platform_type: i32,

    #[prost(enumeration = "BrokerDto", tag = "5")]
    pub broker_type: i32,

    #[prost(int64, tag = "6")]
    pub open_time: i64,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum TradingPlatformDto {
    MetaTrader4 = 0,
    MetaTrader5 = 1,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum BrokerDto {
    Welltrade = 0,
}
