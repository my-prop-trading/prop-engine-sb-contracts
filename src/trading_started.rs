service_sdk::macros::use_my_sb_entity_protobuf_model!();

#[derive(Clone, PartialEq, ::prost::Message)]
#[my_sb_entity_protobuf_model(topic_id = "trading-started")]
pub struct TradingStartedSbModel {
    #[prost(string, tag = "1")]
    pub trader_account_id: String,
    
    #[prost(string, tag = "2")]
    pub client_id: String,

    #[prost(int64, tag = "3")]
    pub start_date: i64,

    #[prost(int64, tag = "4")]
    pub end_date: Option<i64>,
}