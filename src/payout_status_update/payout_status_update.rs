use crate::common::AccountType; 

service_sdk::macros::use_my_sb_entity_protobuf_model!();
#[derive(Clone, PartialEq, ::prost::Message)]
#[my_sb_entity_protobuf_model(topic_id = "payout-status-update")]
pub struct PayoutStatusUpdateSbModel {
    #[prost(message, tag = "1")]
    pub event: Option<PayoutStatusUpdateBodySbModel>,
}

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PayoutStatusUpdateBodySbModel {
    #[prost(string, tag = "1")]
    pub payout_id: String,

    #[prost(int32, tag = "2")]
    pub platform_id: i32,

    #[prost(enumeration = "AccountType", tag = "3")]
    pub account_type: i32,

    #[prost(enumeration="PayoutStatus", tag = "4")]
    pub status: i32 ,

    #[prost(string, tag = "5")]
    pub brand: String,

    #[prost(string, tag = "6")]
    pub client_id: String,

    #[prost(string, tag = "7")]
    pub trader_account_id: String,

    #[prost(double, tag = "8")]
    pub withdrawal_amount: f64,
    
    #[prost(string, tag = "9")]
    pub withdrawal_currency: String,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum PayoutStatus {
    Pending = 0,
    Success = 1,
    Fail = 2,
}
