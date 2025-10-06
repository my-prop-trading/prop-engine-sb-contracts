service_sdk::macros::use_my_sb_entity_protobuf_model!();
#[derive(Clone, PartialEq, ::prost::Message)]
#[my_sb_entity_protobuf_model(topic_id = "payment-order-received")]
pub struct PaymentOrderReceivedSbModel {
    #[prost(string, tag = "1")]
    pub order_id: String,
    #[prost(int32, tag = "2")]
    pub status: i32,
    #[prost(string, tag = "4")]
    pub psp_order_id: String,
    #[prost(string, tag = "5")]
    pub psp_name: String,
    #[prost(string, optional, tag = "6")]
    pub decline_code: Option<String>,
    #[prost(string, optional, tag = "7")]
    pub decline_reason: Option<String>,
}
