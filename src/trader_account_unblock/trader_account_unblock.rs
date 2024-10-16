use crate::common::AccountType; 

service_sdk::macros::use_my_sb_entity_protobuf_model!();
#[derive(Clone, PartialEq, ::prost::Message)]
#[my_sb_entity_protobuf_model(topic_id = "trader-account-unblock")]
pub struct TraderAccountUnBlockSbModel {
    #[prost(string, tag = "1")]
    pub trader_account_id: String,

    #[prost(enumeration="UnBlockType", tag = "2")]
    pub block_type: i32,

    #[prost(string, tag = "3")]
    pub brand: String,

    #[prost(string, tag = "4")]
    pub client_id: String,

    #[prost(enumeration = "AccountType", tag = "5")]
    pub account_type: i32,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum UnBlockType {
    UnBlockTrades = 1,
}

impl UnBlockType {
    pub fn as_str_name(&self) -> &'static str {
        match self {
            UnBlockType::UnBlockTrades => "UnBlockTrades",
        }
    }
}