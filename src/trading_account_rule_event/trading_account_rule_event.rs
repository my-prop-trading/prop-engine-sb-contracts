service_sdk::macros::use_my_sb_entity_protobuf_model!();

#[derive(Clone, PartialEq, ::prost::Message)]
#[my_sb_entity_protobuf_model(topic_id = "trading-account-rule-event")]
pub struct TradingAccountRuleEventSbModel {
    #[prost(message, tag = "1")]
    pub event: Option<TradingAccountRuleEventBodySbModel>,
}

/// PROP25-2870: published by trading-control-flows-grpc when a trading rule moves on an account:
/// a new soft breach, the Consistency (Best Day) rule got worse, the profit reached half of the
/// phase target. Only on the change, not on every evaluation. Exactly one of the parts is set.
///
/// Delivery is best effort and a restart of the publisher may repeat a change, so consumers
/// deduplicate (by the counter of a soft breach, by account and status and day of a
/// Consistency change, by account of the halfway point).
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TradingAccountRuleEventBodySbModel {
    #[prost(string, tag = "1")]
    pub client_id: String,

    #[prost(string, tag = "2")]
    pub trader_account_id: String,

    /// Unix microseconds of the evaluation that saw the change. The trading day is the UTC day of
    /// this time (the same boundary as the daily loss).
    #[prost(int64, tag = "3")]
    pub event_time: i64,

    #[prost(message, optional, tag = "4")]
    pub soft_breach: Option<SoftBreachSbModel>,

    #[prost(message, optional, tag = "5")]
    pub consistency: Option<ConsistencyChangedSbModel>,

    #[prost(message, optional, tag = "6")]
    pub profit_target_halfway: Option<ProfitTargetHalfwaySbModel>,
}

/// A new soft breach. Not published when the same evaluation closes the account (the breach over
/// the allowed count is a hard breach).
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SoftBreachSbModel {
    #[prost(enumeration = "SoftBreachRule", tag = "1")]
    pub rule: i32,

    /// The account counter of this rule after the breach. It never resets.
    #[prost(int32, tag = "2")]
    pub breach_n: i32,

    /// Soft breaches the package allows for this rule; 0 = no limit.
    #[prost(int32, tag = "3")]
    pub max: i32,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum SoftBreachRule {
    Unspecified = 0,
    ExposureLoss = 1,
    MarginRule = 2,
    MarginRulePerIdea = 3,
}

/// The Best Day rule status got worse: GOOD to WARNING, or GOOD / WARNING to BREACHED. Not
/// published while the ratio cannot be computed (no profit yet, right after an approved payout).
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ConsistencyChangedSbModel {
    #[prost(enumeration = "ConsistencyStatus", tag = "1")]
    pub prev_status: i32,

    #[prost(enumeration = "ConsistencyStatus", tag = "2")]
    pub status: i32,

    /// Best day share of the profit, percent (`42.5`).
    #[prost(double, tag = "3")]
    pub ratio: f64,

    /// The limit, percent (`50`).
    #[prost(double, tag = "4")]
    pub cap: f64,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum ConsistencyStatus {
    Good = 0,
    Warning = 1,
    Breached = 2,
}

/// The profit reached half of the phase profit target (the Profit Target gauge of the account
/// page, equity based). Evaluation phases only.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ProfitTargetHalfwaySbModel {
    /// Share of the target reached, percent (`53.2`).
    #[prost(double, tag = "1")]
    pub profit_percent: f64,

    /// The phase profit target in the account currency (`4000`).
    #[prost(double, tag = "2")]
    pub target: f64,
}
