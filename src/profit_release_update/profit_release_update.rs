use crate::common::TradingPlatform; 
use crate::common::Broker;

service_sdk::macros::use_my_sb_entity_protobuf_model!();
#[derive(Clone, PartialEq, ::prost::Message)]
#[my_sb_entity_protobuf_model(topic_id = "profit-release-update")]
pub struct ProfitReleaseStatusUpdateSbModel {
    #[prost(message, tag = "1")]
    pub event: Option<ProfitReleaseStatusUpdateBodySbModel>,
}

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ProfitReleaseStatusUpdateBodySbModel {
    #[prost(string, tag = "1")]
    pub profit_release_id: String,

    #[prost(enumeration="TradingPlatform", tag = "2")]
    pub trading_platform_type: i32,

    #[prost(enumeration="Broker", tag = "3")]
    pub broker_type: i32,

    #[prost(bool, tag = "4")]
    pub is_revenue_share_collected: bool,

    #[prost(string, tag = "5")]
    pub brand: String,

    #[prost(string, tag = "6")]
    pub client_id: String,

    #[prost(string, tag = "7")]
    pub trader_account_id: String,

    #[prost(double, tag = "8")]
    pub revenue_share_amount: f64,

    #[prost(double, tag = "9")]
    pub profit_amount: f64,

    #[prost(string, tag = "10")]
    pub profit_currency: String,

}