service_sdk::macros::use_my_sb_entity_protobuf_model!();

#[derive(Clone, PartialEq, ::prost::Message)]
#[my_sb_entity_protobuf_model(topic_id = "payment-subscription-webhook")]
pub struct PaymentSubscriptionWebhook {
    #[prost(string, tag = "1")]
    pub subscription_id: String,
    #[prost(enumeration = "SubscriptionEvent", tag = "2")]
    pub subscription_event: i32,
    #[prost(string, tag = "3")]
    pub buyer_email: String,
    #[prost(double, tag = "4")]
    pub product_price: f64,
    #[prost(string, tag = "5")]
    pub product_currency: String,
    #[prost(string, tag = "6")]
    pub product_code: String,
    #[prost(int64, optional, tag = "7")]
    pub started_at: Option<i64>,
    #[prost(int64, optional, tag = "8")]
    pub expires_at: Option<i64>,
    #[prost(string, optional, tag = "9")]
    pub buyer_id: Option<String>,
    // proto3 default = 0 = Unspecified — означает "провайдер не указан".
    // Consumer'ы НЕ должны интерпретировать Unspecified как Billerix или другой конкретный
    // провайдер: это самостоятельный legitimate state. Publisher'ы (billerix-webhook,
    // solidgate-webhook) обязаны проставлять конкретный provider явно.
    #[prost(enumeration = "PaymentProvider", tag = "10")]
    pub provider: i32,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum SubscriptionEvent {
    Renew = 0,
    Suspend = 1,
    Disable = 2,
    Block = 3,
    Create = 4,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum PaymentProvider {
    Unspecified = 0,
    Billerix = 1,
    SolidGate = 2,
}
