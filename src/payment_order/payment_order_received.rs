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
    #[prost(string, optional, tag = "8")]
    pub credit_card_token: Option<String>,
    #[prost(string, optional, tag = "9")]
    pub card_masked_number: Option<String>,
    #[prost(int32, optional, tag = "10")]
    pub provider_id: Option<i32>,
    /// Amount the payer actually paid, in `paid_currency`; absent when the provider does not report it.
    #[prost(double, optional, tag = "11")]
    pub paid_amount: Option<f64>,
    #[prost(string, optional, tag = "12")]
    pub paid_currency: Option<String>,
    /// Complaint reference; present only on a chargeback.
    #[prost(string, optional, tag = "13")]
    pub chargeback_claim_id: Option<String>,
}
