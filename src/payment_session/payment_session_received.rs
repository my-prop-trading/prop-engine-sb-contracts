service_sdk::macros::use_my_sb_entity_protobuf_model!();
#[derive(Clone, PartialEq, ::prost::Message)]
#[my_sb_entity_protobuf_model(topic_id = "payment-session-received")]
pub struct PaymentOrderReceivedSbModel {
    #[prost(string, tag = "1")]
    pub order_id: String,
    #[prost(int32, tag = "2")]
    pub status: i32,
}

pub enum PaymentSessionSbStatus {
    Init = 0,
    Closed = 1,
}