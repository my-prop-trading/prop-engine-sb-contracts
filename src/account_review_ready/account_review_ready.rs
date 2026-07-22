service_sdk::macros::use_my_sb_entity_protobuf_model!();
#[derive(Clone, PartialEq, ::prost::Message)]
#[my_sb_entity_protobuf_model(topic_id = "account-review-ready")]
pub struct AccountReviewReadySbModel {
    #[prost(message, tag = "1")]
    pub event: Option<AccountReviewReadyBodySbModel>,
}

/// Published by trading-analysis when an account-review report has been computed
/// and persisted (queue row reached Done). confirmation-email-sender listens and
/// sends the "your review is ready" notification. Carries only what the email
/// path needs — no report details (the report lives on the dashboard).
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AccountReviewReadyBodySbModel {
    #[prost(string, tag = "1")]
    pub trader_account_id: String,

    #[prost(string, tag = "2")]
    pub client_id: String,

    #[prost(string, tag = "3")]
    pub brand: String,
}
