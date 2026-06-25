service_sdk::macros::use_my_sb_entity_protobuf_model!();
#[derive(Clone, PartialEq, ::prost::Message)]
#[my_sb_entity_protobuf_model(topic_id = "trading-scenario-update")]
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

    #[prost(string, tag = "7")]
    pub client_id: String,

    // Fair Shot (PROP25-2225): the concrete fail reason so the grant side can
    // tell the 5 auto-breaches apart from TargetDate/TrialEnded/Legacy, plus the
    // peak realized-profit % used to evaluate the >=50% threshold.
    #[prost(enumeration = "TradingScenarioFailReason", tag = "8")]
    pub fail_reason: i32,

    #[prost(double, tag = "9")]
    pub max_profit_pct: f64,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum TradingScenarioResolution {
    Pass = 0,
    Fail = 1,
}

/// Mirrors trading-control's TradingObjectiveFailReason, shifted by +1 so that
/// 0 = Unspecified (proto3 default, i.e. field not set) rather than a real reason.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum TradingScenarioFailReason {
    Unspecified = 0,
    DailyLossLevel = 1,
    OverallLossLevel = 2,
    TargetDate = 3,
    Legacy = 4,
    Inactivity = 5,
    TrialEnded = 6,
    ExposureLossBreach = 7,
    MarginRule = 8,
    MarginRulePerIdea = 9,
}