service_sdk::macros::use_my_sb_entity_protobuf_model!();
#[allow(clippy::derive_partial_eq_without_eq)]
#[my_sb_entity_protobuf_model(topic_id = "woocommerce-order-update")]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct WoocommerceOrderUpdateSbModel {
    #[prost(string, tag = "1")]
    pub order_id: String,

    #[prost(enumeration="OrderStatus", tag = "2")]
    pub order_status: i32,

    #[prost(string, tag = "3")]
    pub payment_url: String,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum OrderStatus {
    Pending = 0,
    Processing = 1,
    OnHold = 2,
    Completed = 3,
    Cancelled= 4,
    Refunded = 5,
    Failed = 6,
    Trash = 7,
}