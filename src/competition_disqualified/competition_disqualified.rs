use crate::TradingScenarioFailReason;

service_sdk::macros::use_my_sb_entity_protobuf_model!();
#[derive(Clone, PartialEq, ::prost::Message)]
#[my_sb_entity_protobuf_model(topic_id = "competition-disqualified")]
pub struct CompetitionDisqualifiedSbModel {
    #[prost(message, tag = "1")]
    pub event: Option<CompetitionDisqualifiedBodySbModel>,
}

/// Published by competitions-flows-grpc when a participant is disqualified: the competition
/// account broke a trading rule while the competition was running. Published once per
/// enrollment, at the moment it is marked disqualified.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CompetitionDisqualifiedBodySbModel {
    #[prost(string, tag = "1")]
    pub client_id: String,

    #[prost(string, tag = "2")]
    pub brand: String,

    #[prost(string, tag = "3")]
    pub competition_id: String,

    #[prost(string, tag = "4")]
    pub competition_name: String,

    #[prost(string, tag = "5")]
    pub trader_account_id: String,

    /// The broken rule, from the trading scenario that failed the account. It is not stored by
    /// competitions-flows-grpc, so this event is the only place it is available.
    #[prost(enumeration = "TradingScenarioFailReason", tag = "6")]
    pub fail_reason: i32,

    /// When the participant was disqualified, unix microseconds (UTC).
    #[prost(int64, tag = "7")]
    pub disqualified_at: i64,
}
