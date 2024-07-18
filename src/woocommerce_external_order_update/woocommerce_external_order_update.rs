use crate::common::OrderStatus; 
service_sdk::macros::use_my_sb_entity_protobuf_model!();
#[allow(clippy::derive_partial_eq_without_eq)]
#[my_sb_entity_protobuf_model(topic_id = "woocommerce-external-order-update")]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct WoocommerceExternalOrderUpdateSbModel {
    #[prost(int32, tag = "1")]
    pub woo_order_id: i32,

    #[prost(enumeration="OrderStatus", tag = "2")]
    pub order_status: i32,

    #[prost(string, tag = "3")]
    pub payment_url: String,

    #[prost(string, tag = "5")]
    pub first_name: String,

    #[prost(string, tag = "6")]
    pub last_name: String,

    #[prost(string, tag = "7")]
    pub address_1: String,

    #[prost(string, tag = "8")]
    pub address_2: String,

    #[prost(string, tag = "9")]
    pub city: String,

    #[prost(string, tag = "10")]
    pub state: String,

    #[prost(string, tag = "11")]
    pub postcode: String,

    #[prost(string, tag = "12")]
    pub country: String,

    #[prost(string, tag = "13")]
    pub email: String,

    #[prost(string, tag = "14")]
    pub phone: String,

    #[prost(int64, tag = "15")]
    pub date_created: i64,

    #[prost(double, tag = "16")]
    pub discount_total: f64,

    #[prost(int64, optional, tag = "17")]
    pub date_paid: Option<i64>,

    #[prost(double, tag = "18")]
    pub total: f64,

    #[prost(int32, tag = "19")]
    pub product_id: i32,

    #[prost(string, tag = "20")]
    pub trader_package_id: String,

    #[prost(string, tag = "21")]
    pub discount_code: String,

    #[prost(int64, tag = "22")]
    pub date_of_birth: i64,

    #[prost(int32, tag = "23")]
    pub platform_id: i32,

    #[prost(enumeration="DiscountType", tag = "24")]
    pub discount_type: i32,

    #[prost(double, tag = "25")]
    pub discount_nominal_amount: f64,

    #[prost(string, optional, tag = "26")]
    pub click_id: Option<String>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum DiscountType {
    Fixed = 0,    
    Percent = 1,
}