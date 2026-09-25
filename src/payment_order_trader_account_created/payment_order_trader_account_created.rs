service_sdk::macros::use_my_sb_entity_protobuf_model!();
#[derive(Clone, PartialEq, ::prost::Message)]
#[my_sb_entity_protobuf_model(topic_id = "payment-order-trader-account-created")]
pub struct PaymentOrderTraderAccountCreatedSbModel {
    #[prost(message, tag = "1")]
    pub event: Option<PaymentOrderTraderAccountCreatedBodySbModel>,
}

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PaymentOrderTraderAccountCreatedBodySbModel {
    #[prost(string, tag = "1")]
    pub order_id: String,

    #[prost(string, tag = "2")]
    pub aggregated_id: String,

    #[prost(string, tag = "3")]
    pub client_id: String,

    #[prost(int64, tag = "4")]
    pub paid_at: i64,

    #[prost(string, tag = "5")]
    pub currency: String,

    #[prost(double, tag = "6")]
    pub price: f64,

    #[prost(double, tag = "7")]
    pub total_paid: f64,

    #[prost(double, tag = "8")]
    pub total_discount: f64,

    #[prost(string, tag = "9")]
    pub trader_account_id: String,

    /// PROP25-2491: true when this account belongs to a bulk order (quantity > 1). The email path
    /// suppresses the per-account payment-received email for bulk accounts, because a single
    /// order-level "Bulk Payment Received" email is sent from BulkOrderProvisioned instead. Additive:
    /// defaults to false, so single-account orders are unchanged.
    #[prost(bool, tag = "10")]
    pub is_bulk: bool,

    /// PROP25-2491: the number of accounts in the order (1 for a normal single-account order). Lets
    /// consumers such as admin-notifications show "N accounts" for a bulk order. Additive: defaults
    /// to 0 for old publishers; consumers should treat 0 as 1.
    #[prost(uint32, tag = "11")]
    pub quantity: u32,
}
