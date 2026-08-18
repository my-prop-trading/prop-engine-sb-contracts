use crate::common::AccountType;
use crate::{TraderAccountStatus, TraderPackagePhaseType};

service_sdk::macros::use_my_sb_entity_protobuf_model!();

/// Live account activation (PROP25-2366): the activation fee of an evaluation-only package has
/// been paid, so the account that passed the final phase may now go on to the contract and the
/// funded account.
///
/// A topic of its own instead of a second `trader-account-status-update` for the same pass: that
/// topic has a dozen subscribers, which would re-send the pass emails, the analytics conversions
/// and the certificate.
#[derive(Clone, PartialEq, ::prost::Message)]
#[my_sb_entity_protobuf_model(topic_id = "trader-account-activation-paid")]
pub struct TraderAccountActivationPaidSbModel {
    #[prost(message, tag = "1")]
    pub event: Option<TraderAccountActivationPaidBodySbModel>,
}

/// Carries the whole account identity, because the consumer resumes the pass handling that the
/// status event started and needs the same values it would have got from there.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TraderAccountActivationPaidBodySbModel {
    #[prost(string, tag = "1")]
    pub trader_account_id: String,

    #[prost(string, tag = "2")]
    pub trader_account_aggregated_id: String,

    #[prost(string, tag = "3")]
    pub client_id: String,

    #[prost(string, tag = "4")]
    pub brand: String,

    #[prost(string, tag = "5")]
    pub trading_package_id: String,

    #[prost(int32, tag = "6")]
    pub platform_id: i32,

    #[prost(int32, tag = "7")]
    pub phase: i32,

    #[prost(enumeration = "AccountType", tag = "8")]
    pub account_type: i32,

    #[prost(enumeration = "TraderPackagePhaseType", tag = "9")]
    pub phase_type: i32,

    #[prost(enumeration = "TraderAccountStatus", tag = "10")]
    pub status: i32,

    #[prost(int64, tag = "11")]
    pub e_tag: i64,

    /// Activation order that was paid
    #[prost(string, tag = "12")]
    pub order_id: String,

    /// Unix microseconds
    #[prost(int64, tag = "13")]
    pub paid_at: i64,
}
