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
}
