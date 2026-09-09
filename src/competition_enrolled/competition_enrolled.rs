service_sdk::macros::use_my_sb_entity_protobuf_model!();
#[derive(Clone, PartialEq, ::prost::Message)]
#[my_sb_entity_protobuf_model(topic_id = "competition-enrolled")]
pub struct CompetitionEnrolledSbModel {
    #[prost(message, tag = "1")]
    pub event: Option<CompetitionEnrolledBodySbModel>,
}

/// Published by competitions-flows-grpc when a client successfully enrolls into a
/// competition (EnrollFlow). confirmation-email-sender listens and sends the
/// "you're enrolled" notification. Skipped by the publisher when the client
/// enrolls after the competition already started (enrolled_at >= starts_at) — in
/// that case only the "competition started" email is sent.
///
/// Carries what the email path needs: recipient (client_id/brand), the dedup
/// discriminator (competition_id — no order/account exists yet at enroll time),
/// and the placeholders (name + start/end timestamps in unix microseconds, UTC).
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CompetitionEnrolledBodySbModel {
    #[prost(string, tag = "1")]
    pub client_id: String,

    #[prost(string, tag = "2")]
    pub brand: String,

    #[prost(string, tag = "3")]
    pub competition_id: String,

    #[prost(string, tag = "4")]
    pub competition_name: String,

    /// Competition start, unix microseconds (UTC).
    #[prost(int64, tag = "5")]
    pub competition_starts_at: i64,

    /// Competition end, unix microseconds (UTC).
    #[prost(int64, tag = "6")]
    pub competition_ends_at: i64,
}
