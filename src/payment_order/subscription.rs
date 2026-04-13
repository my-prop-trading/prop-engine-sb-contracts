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
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum SubscriptionEvent {
    Renew = 0,
    Suspend = 1,
    Disable = 2,
    Block = 3,
}
