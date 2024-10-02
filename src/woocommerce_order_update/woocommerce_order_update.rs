use crate::common::OrderStatus; 
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

    #[prost(int32, tag = "4")]
    pub woo_order_id: i32,

    #[prost(string, tag = "5")]
    pub payment_method_title: String,
}

