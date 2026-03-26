service_sdk::macros::use_my_sb_entity_protobuf_model!();

#[derive(Clone, PartialEq, ::prost::Message)]
#[my_sb_entity_protobuf_model(topic_id = "external-payment-received")]
pub struct ExternalPaymentReceivedSbModel {
    #[prost(string, optional, tag = "1")]
    pub email: Option<String>,
    #[prost(string, tag = "2")]
    pub merchant: String,
    #[prost(double, tag = "3")]
    pub amount: f64,
    #[prost(string, tag = "4")]
    pub currency: String,
    #[prost(string, tag = "5")]
    pub id: String,
    #[prost(int32, tag = "6")]
    pub status: i32,
}
