#[derive(Clone, PartialEq, ::prost::Message)]
#[my_service_bus_macros::my_sb_entity_protobuf_model(topic_id = "trader-account-open-position")]
pub struct TraderAccountOpenPositionSbModel {
    #[prost(message, tag = "1")]
    pub event: Option<TraderAccountOpenPositionBodySbModel>,
}

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TraderAccountOpenPositionBodySbModel {
    #[prost(string, tag = "1")]
    pub trader_account_id: String,

    #[prost(int, tag = "2")]
    pub metatrader_login: String,

    #[prost(string, tag = "3")]
    pub position_id: String,

    #[prost(enumeration = "TradingPlatformDto", tag = "4")]
    pub r#type: i32,

    #[prost(enumeration = "BrokerDto", tag = "5")]
    pub r#type: i32,
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
