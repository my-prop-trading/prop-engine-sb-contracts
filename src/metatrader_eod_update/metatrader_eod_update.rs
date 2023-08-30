use crate::common::TradingPlatform; 
use crate::common::Broker;


#[derive(Clone, PartialEq, ::prost::Message)]
#[my_service_bus_macros::my_sb_entity_protobuf_model(topic_id = "metatrader-eod-update")]
pub struct MetatraderEndOfDayUpdateSbModel {
    #[prost(message, tag = "1")]
    pub event: Option<MetatraderEndOfDayUpdateBodySbModel>,
}

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MetatraderEndOfDayUpdateBodySbModel {
    #[prost(string, tag = "1")]
    pub trader_account_id: String,

    #[prost(enumiration="EventType", tag = "2")]
    pub event_type: i32,

    #[prost(enumeration = "TradingPlatform", tag = "3")]
    pub trading_platform_type: i32,

    #[prost(enumeration = "Broker", tag = "4")]
    pub broker_type: i32,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum EventType {
    Started = 0,
    TradeDisabled = 1,
    PositionsClosed = 2,
    TradeEnabled =3,
    Finished = 4,
}