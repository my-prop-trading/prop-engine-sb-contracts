use crate::common::AccountType;

service_sdk::macros::use_my_sb_entity_protobuf_model!();

/// A change of status of the news trading rule's record of a position: flagged in a news window,
/// settled at the full close, or reversed. Times are unix microseconds, UTC.
#[derive(Clone, PartialEq, ::prost::Message)]
#[my_sb_entity_protobuf_model(topic_id = "news-trade-adjustment")]
pub struct NewsTradeAdjustmentSbModel {
    #[prost(string, tag = "1")]
    pub trader_account_id: String,

    #[prost(string, tag = "2")]
    pub client_id: String,

    #[prost(string, tag = "3")]
    pub position_id: String,

    #[prost(enumeration = "AccountType", tag = "4")]
    pub account_type: i32,

    #[prost(int32, tag = "5")]
    pub platform_id: i32,

    #[prost(string, tag = "6")]
    pub symbol: String,

    #[prost(string, tag = "7")]
    pub event_title: String,

    #[prost(int64, tag = "8")]
    pub release_time: i64,

    /// trading_control.NewsCaseGrpc value.
    #[prost(int32, tag = "9")]
    pub news_case: i32,

    /// trading_control.NewsAdjustmentStatusGrpc value.
    #[prost(int32, tag = "10")]
    pub status: i32,

    /// Share of the counted profit withheld, 0..100.
    #[prost(double, tag = "11")]
    pub percent: f64,

    #[prost(double, tag = "12")]
    pub original_profit: f64,

    #[prost(double, tag = "13")]
    pub removed: f64,

    #[prost(string, optional, tag = "14")]
    pub reverse_comment: Option<String>,

    /// When the status changed.
    #[prost(int64, tag = "15")]
    pub event_time: i64,
}
