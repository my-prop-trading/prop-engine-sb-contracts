use super::subscription::PaymentProvider;

service_sdk::macros::use_my_sb_entity_protobuf_model!();

/// A one-time order of a payment provider changed; published for every order of the channel,
/// so a consumer skips the orders it does not know.
#[derive(Clone, PartialEq, ::prost::Message)]
#[my_sb_entity_protobuf_model(topic_id = "provider-order-webhook")]
pub struct ProviderOrderWebhook {
    #[prost(enumeration = "PaymentProvider", tag = "1")]
    pub provider: i32,
    /// Order id chosen by the merchant.
    #[prost(string, tag = "2")]
    pub order_id: String,
    #[prost(enumeration = "ProviderOrderStatus", tag = "3")]
    pub status: i32,
    /// Smallest currency unit.
    #[prost(int64, optional, tag = "4")]
    pub amount: Option<i64>,
    #[prost(string, optional, tag = "5")]
    pub currency: Option<String>,
    #[prost(string, optional, tag = "6")]
    pub product_id: Option<String>,
    /// Why the order was declined or charged back.
    #[prost(string, optional, tag = "7")]
    pub reason: Option<String>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum ProviderOrderStatus {
    /// A provider status this contract does not know; not final.
    Unknown = 0,
    Processing = 1,
    /// Waits for the customer to confirm the payment with the bank.
    Verify = 2,
    Paid = 3,
    Declined = 4,
    Refunded = 5,
    Chargeback = 6,
}
