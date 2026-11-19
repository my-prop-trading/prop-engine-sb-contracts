service_sdk::macros::use_my_sb_entity_protobuf_model!();

#[derive(Clone, PartialEq, ::prost::Message)]
#[my_sb_entity_protobuf_model(topic_id = "certificate-created")]
pub struct CertificateCreatedSbModel {
    #[prost(message, tag = "1")]
    pub event: Option<CertificateCreatedBodySbModel>,
}

/// PROP25-2873: published by certificate-flows-grpc when it issues a certificate to a trader
/// (phase passed, funded account, payout). Marketing certificates created by hand are not
/// published. The same achievement may be issued more than once (one record per status event), so
/// consumers deduplicate by type + trading account, or by `tx_id` for a payout.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CertificateCreatedBodySbModel {
    #[prost(string, tag = "1")]
    pub certificate_id: String,

    #[prost(string, tag = "2")]
    pub client_id: String,

    /// Empty when the certificate is not about one account.
    #[prost(string, tag = "3")]
    pub trader_account_id: String,

    /// The certificate type as named in CertificatesGrpcService.proto `CertificateType`:
    /// "EvaluationPhase1", "EvaluationPhase2", "EvaluationPhase3", "EvaluationLiveAccount",
    /// "PayoutByTxID", "MaxAllocation", "OverallPayout", "PayoutByMonth".
    #[prost(string, tag = "4")]
    pub certificate_type: String,

    /// The payout id of a payout certificate.
    #[prost(string, optional, tag = "5")]
    pub tx_id: Option<String>,

    /// When the certificate was issued, unix microseconds.
    #[prost(int64, tag = "6")]
    pub event_time: i64,
}
