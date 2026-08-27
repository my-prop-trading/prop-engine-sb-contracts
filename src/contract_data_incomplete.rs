service_sdk::macros::use_my_sb_entity_protobuf_model!();

/// Raised by kyc-flows-grpc when a contract could not be auto-sent because the fields it must print
/// did not resolve from the KYC data. Consumed by admin-notifications, which alerts ops (PRD §9).
///
/// Carries no PII: the event ends up in a Slack channel, so it names the unresolved fields and
/// identifies the client by id only.
#[allow(clippy::derive_partial_eq_without_eq)]
#[my_sb_entity_protobuf_model(topic_id = "contract-data-incomplete")]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ContractDataIncompleteSbModel {
    #[prost(string, tag = "1")]
    pub client_id: String,

    #[prost(string, tag = "2")]
    pub trader_account_id: String,

    #[prost(int64, tag = "3")]
    pub contract_id: i64,

    /// Names of the contract fields that did not resolve, e.g. `address`, `document_id`.
    #[prost(string, repeated, tag = "4")]
    pub missing_fields: Vec<String>,
}
