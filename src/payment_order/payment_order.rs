#[derive(Clone, PartialEq, ::prost::Message)]
#[my_service_bus_macros::my_sb_entity_protobuf_model(topic_id = "payment-order")]
pub struct PaymentOrderSbModel {
    #[prost(message, tag = "1")]
    pub event: Option<PaymentOrderBodySbModel>,
}

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PaymentOrderBodySbModel {
    #[prost(string, tag = "1")]
    pub order_id: String,
}