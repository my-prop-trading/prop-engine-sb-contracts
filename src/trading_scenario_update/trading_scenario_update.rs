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

    #[prost(double, tag = "3")]
    pub current_equity: f64,

    #[prost(double, tag = "4")]
    pub current_balance: f64,

    #[prost(double, tag = "5")]
    pub current_profit: f64,

    #[prost(int32, tag = "6")]
    pub days_traded: i32,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum TradingScenarioResolution {
    Pass = 0,    
    Fail = 1,
}