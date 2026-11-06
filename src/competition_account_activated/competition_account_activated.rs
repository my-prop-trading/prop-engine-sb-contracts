service_sdk::macros::use_my_sb_entity_protobuf_model!();
#[derive(Clone, PartialEq, ::prost::Message)]
#[my_sb_entity_protobuf_model(topic_id = "competition-account-activated")]
pub struct CompetitionAccountActivatedSbModel {
    #[prost(message, tag = "1")]
    pub event: Option<CompetitionAccountActivatedBodySbModel>,
}

/// Published by competitions-flows-grpc once the trading account a participant asked for
/// (Activate) has been created and attached to the enrollment (Pending -> Active).
/// Published once per enrollment; a redelivery of the account-created event does not repeat it.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CompetitionAccountActivatedBodySbModel {
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

    /// When the account was attached to the enrollment, unix microseconds (UTC).
    #[prost(int64, tag = "6")]
    pub activated_at: i64,
}
