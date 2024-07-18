service_sdk::macros::use_my_sb_entity_protobuf_model!();
#[derive(Clone, PartialEq, ::prost::Message)]
#[my_sb_entity_protobuf_model(topic_id = "payment-order")]
pub struct PaymentOrderSbModel {
    #[prost(message, tag = "1")]
    pub event: Option<PaymentOrderBodySbModel>,
}

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PaymentOrderBodySbModel {
    #[prost(string, tag = "1")]
    pub order_id: String,

    #[prost(enumeration="PaymentOrderStatus", tag = "2")]
    pub status: i32,

    #[prost(string, tag = "3")]
    pub client_id: String,

    #[prost(double, tag = "4")]
    pub amount: f64,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum PaymentOrderStatus {
    Success = 0,
    Failed = 1,
    Canceled = 2,
}