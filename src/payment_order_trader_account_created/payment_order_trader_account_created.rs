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
}