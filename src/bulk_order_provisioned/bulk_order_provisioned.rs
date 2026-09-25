service_sdk::macros::use_my_sb_entity_protobuf_model!();
#[derive(Clone, PartialEq, ::prost::Message)]
#[my_sb_entity_protobuf_model(topic_id = "bulk-order-provisioned")]
pub struct BulkOrderProvisionedSbModel {
    #[prost(message, tag = "1")]
    pub event: Option<BulkOrderProvisionedBodySbModel>,
}

/// Published by orders-flows-grpc once ALL trading accounts of a bulk order have been created
/// (PROP25-2491). A bulk order is one payment that produces several accounts; the per-account
/// PaymentOrderTraderAccountCreated events still fire (they drive credential delivery), but the
/// purchase-confirmation email must be sent ONCE for the whole order, not once per account. This
/// order-level event is that single trigger: confirmation-email-sender listens and sends one
/// "Bulk Payment Received" email listing every account (suppressing the per-account emails for
/// bulk orders).
///
/// Carries what the email path needs: recipient (client_id), the dedup discriminator (order_id),
/// and the placeholders — the client's name and the list of created trading account ids (our own
/// trader_account_id, not platform ids). `is_instant_funding` selects between the two variants of
/// the email (Instant Funding "contract details coming soon" vs demo "Phase 1 accounts active").
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BulkOrderProvisionedBodySbModel {
    #[prost(string, tag = "1")]
    pub order_id: String,

    #[prost(string, tag = "2")]
    pub client_id: String,

    #[prost(string, tag = "3")]
    pub client_name: String,

    /// The created trading account ids (our trader_account_id), one per account in the order.
    #[prost(string, repeated, tag = "4")]
    pub account_ids: Vec<String>,

    /// True when the order's package is Instant Funding, selecting the Instant Funding email variant.
    #[prost(bool, tag = "5")]
    pub is_instant_funding: bool,

    /// The trading package title, for the email body (the package all accounts in the order share).
    #[prost(string, tag = "6")]
    pub trading_package_title: String,
}
