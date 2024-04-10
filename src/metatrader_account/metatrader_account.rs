use crate::common::AccountType; 

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum AccountEventType {
    AccountCreated = 0,
    AccountUpdated = 1,
    AccountDeleted = 2,
}

service_sdk::macros::use_my_sb_entity_protobuf_model!();
#[derive(Clone, PartialEq, ::prost::Message)]
#[my_sb_entity_protobuf_model(topic_id = "metatrader-account")]
pub struct MetatraderAccountSbModel {
    #[prost(message, tag = "1")]
    pub event: Option<MetatraderAccountBodySbModel>,
}

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MetatraderAccountBodySbModel {
    #[prost(string, tag = "1")]
    pub trader_account_id: String,

    #[prost(int64, tag = "2")]
    pub metatrader_login: i64,

    #[prost(int32, tag = "3")]
    pub platform_id: i32,

    #[prost(enumeration = "AccountType", tag = "4")]
    pub account_type: i32,

    #[prost(enumeration = "AccountEventType", tag = "7")]
    pub event_type: i32,

    #[prost(bool, tag = "8")]
    pub disabled: bool,

    #[prost(bool, tag = "9")]
    pub trade_disabled: bool,
}