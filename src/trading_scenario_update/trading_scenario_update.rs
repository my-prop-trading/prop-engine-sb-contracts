#[derive(Clone, PartialEq, ::prost::Message)]
#[my_service_bus_macros::my_sb_entity_protobuf_model(topic_id = "trading-scenario-update")]
pub struct TradingScenarioUpdateSbModel {
    #[prost(message, tag = "1")]
    pub event: Option<TradingScenarioBodySbModel>,
}

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct  TradingScenarioBodySbModel {
    #[prost(string, tag = "1")]
    pub trader_account_id: String,

    #[prost(enumeration="TradingScenarioResolution", tag = "2")]
    pub r#type: i32,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum TradingScenarioResolution {
    Pass = 0,    
    Fail = 1,
}