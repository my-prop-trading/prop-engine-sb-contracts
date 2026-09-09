service_sdk::macros::use_my_sb_entity_protobuf_model!();
#[derive(Clone, PartialEq, ::prost::Message)]
#[my_sb_entity_protobuf_model(topic_id = "competition-started")]
pub struct CompetitionStartedSbModel {
    #[prost(message, tag = "1")]
    pub event: Option<CompetitionStartedBodySbModel>,
}

/// Published by competitions-flows-grpc when a competition starts (LifecycleTimer
/// advances RegistrationOpen -> Active at starts_at) — one message per enrollment
/// whose trading account is already created (enrollment Active). Guarantees the
/// ticket's two conditions: competition started AND trading account available.
/// confirmation-email-sender listens and sends the "competition started, trading
/// account available" email.
///
/// Carries what the email path needs: recipient (client_id/brand), the dedup key
/// (order_id + trader_account_id — both exist once the account is created), and
/// the placeholders (name + start/end timestamps in unix microseconds, UTC).
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CompetitionStartedBodySbModel {
    #[prost(string, tag = "1")]
    pub client_id: String,

    #[prost(string, tag = "2")]
    pub brand: String,

    #[prost(string, tag = "3")]
    pub competition_id: String,

    #[prost(string, tag = "4")]
    pub order_id: String,

    #[prost(string, tag = "5")]
    pub trader_account_id: String,

    #[prost(string, tag = "6")]
    pub competition_name: String,

    /// Competition start, unix microseconds (UTC).
    #[prost(int64, tag = "7")]
    pub competition_starts_at: i64,

    /// Competition end, unix microseconds (UTC).
    #[prost(int64, tag = "8")]
    pub competition_ends_at: i64,
}
