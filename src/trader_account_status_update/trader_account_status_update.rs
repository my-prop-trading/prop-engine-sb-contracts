#[derive(Clone, PartialEq, ::prost::Message)]
#[my_service_bus_macros::my_sb_entity_protobuf_model(topic_id = "trader-account-creation")]
pub struct TraderAccountStatusUpdateSbModel {
    #[prost(message, tag = "1")]
    pub event: Option<TraderAccountCreatedBodySbModel>,
}

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TraderAccountStatusUpdateBodySbModel {
    #[prost(string, tag = "1")]
    pub trader_account_id: String,

    #[prost(enumeration="TradingPlatformDto", tag = "2")]
    pub r#type: i32,

    #[prost(enumeration="BrokerDto", tag = "3")]
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