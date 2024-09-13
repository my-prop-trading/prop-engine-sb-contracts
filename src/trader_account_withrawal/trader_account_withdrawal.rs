use crate::common::AccountType;
service_sdk::macros::use_my_sb_entity_protobuf_model!();

#[derive(Clone, PartialEq, ::prost::Message)]
#[my_sb_entity_protobuf_model(topic_id = "trader-account-withdrawal")]
pub struct TraderAccountWithdrawalSbModel {
    #[prost(string, tag = "1")]
    pub trader_account_id: String,
    #[prost(int32, tag = "2")]
    pub platform_id: i32,
    #[prost(enumeration = "AccountType", tag = "3")]
    pub account_type: i32,
    #[prost(string, tag = "4")]
    pub platform_account_id: String,
    #[prost(string, tag = "5")]
    pub id: String,
    #[prost(double, tag = "6")]
    pub amount: f64,
    #[prost(string, tag = "7")]
    pub currency: String,
    #[prost(string, tag = "8")]
    pub platform_operation_id: String,
}
