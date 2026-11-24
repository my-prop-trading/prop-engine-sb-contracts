use crate::common::AccountType;

service_sdk::macros::use_my_sb_entity_protobuf_model!();

/// PROP25-2924: a trade idea went above its Risk Per Trade Idea limit, once per idea. The idea is
/// keyed by the account and its first position. Percents are of the initial balance.
/// Delivery is best effort and may repeat, so consumers deduplicate by that key.
#[derive(Clone, PartialEq, ::prost::Message)]
#[my_sb_entity_protobuf_model(topic_id = "rpt-violation")]
pub struct RptViolationSbModel {
    #[prost(string, tag = "1")]
    pub trader_account_id: String,

    #[prost(string, tag = "2")]
    pub first_position_id: String,

    #[prost(string, tag = "3")]
    pub client_id: String,

    /// Evaluation the account belongs to.
    #[prost(string, tag = "4")]
    pub aggregated_id: String,

    #[prost(enumeration = "AccountType", tag = "5")]
    pub account_type: i32,

    #[prost(int32, tag = "6")]
    pub platform_id: i32,

    #[prost(string, tag = "7")]
    pub symbol: String,

    /// 0 = Sell, 1 = Buy.
    #[prost(int32, tag = "8")]
    pub direction: i32,

    /// Loss of the idea when it went above the limit.
    #[prost(double, tag = "9")]
    pub loss_percent: f64,

    #[prost(double, tag = "10")]
    pub limit_percent: f64,

    /// trading_control.RptNoticeGrpc value: which notice the trader gets, none on live.
    #[prost(int32, tag = "11")]
    pub notice: i32,

    /// Live: restricted the client. Demo: flagged the evaluation.
    #[prost(bool, tag = "12")]
    pub restricting: bool,

    /// Violations counted towards the restriction, this one included: of the evaluation on demo,
    /// of the account on live.
    #[prost(int32, tag = "13")]
    pub violations_used: i32,

    #[prost(int32, tag = "14")]
    pub violations_allowed: i32,

    /// Live limit of a restricted client.
    #[prost(double, tag = "15")]
    pub restricted_limit_percent: f64,

    /// Unix microseconds, UTC.
    #[prost(int64, tag = "16")]
    pub event_time: i64,
}
