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

    #[prost(int32, tag = "8")]
    pub decline_reason_type: i32,

    /// PROP25-2868: the raw decline reason from the payment provider. `decline_reason_type` is the
    /// group of the decline emails; the bell also needs the raw text to skip payments the trader
    /// cancelled or abandoned ("Cardholder selected 'Cancel'", "Transaction expired"), which the
    /// groups do not tell apart. None when the order has no decline reason.
    #[prost(string, optional, tag = "9")]
    pub decline_reason: Option<String>,

    /// PROP25-2868: the trading package title of the order, for the bell texts.
    #[prost(string, tag = "10")]
    pub trading_package_title: String,

    /// PROP25-2868: when the order got this status, unix microseconds. A repeated provider
    /// callback with the same result is not published again, so a newer time means a new payment
    /// attempt. 0 from old publishers.
    #[prost(int64, tag = "11")]
    pub event_time: i64,
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
