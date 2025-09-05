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

    #[prost(enumeration = "PaymentOrderStatus", tag = "2")]
    pub status: i32,

    #[prost(string, tag = "3")]
    pub client_id: String,

    #[prost(double, tag = "4")]
    pub amount: f64,

    #[prost(string, optional, tag = "5")]
    pub discount_code: Option<String>,

    #[prost(int32, optional, tag = "6")]
    pub prev_order_status: Option<i32>,

    #[prost(int32, tag = "7")]
    pub order_status: i32,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum PaymentOrderStatus {
    Success = 0,
    Failed = 1,
    Canceled = 2,
    Unpaid = 3,
    Pending = 4,
}

impl PaymentOrderStatus {
    pub fn as_str_name(&self) -> &'static str {
        match self {
            PaymentOrderStatus::Success => "Success",
            PaymentOrderStatus::Failed => "Failed",
            PaymentOrderStatus::Canceled => "Canceled",
            PaymentOrderStatus::Unpaid => "Unpaid",
            PaymentOrderStatus::Pending => "Pending",
        }
    }
}
