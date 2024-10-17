use crate::common::AccountType; 

service_sdk::macros::use_my_sb_entity_protobuf_model!();
#[derive(Clone, PartialEq, ::prost::Message)]
#[my_sb_entity_protobuf_model(topic_id = "trader-account-block-status-update")]
pub struct TraderAccountBlockStatusUpdateSbModel {
    #[prost(string, tag = "1")]
    pub trader_account_id: String,

    #[prost(enumeration="BlockingType", tag = "2")]
    pub block_type: i32,

    #[prost(string, tag = "3")]
    pub brand: String,

    #[prost(string, tag = "4")]
    pub client_id: String,

    #[prost(enumeration = "AccountType", tag = "5")]
    pub account_type: i32,

    #[prost(int32, tag = "6")]
    pub platform_id: i32,

    #[prost(enumeration = "BlockingStatus", tag = "7")]
    pub block_status: i32,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum BlockingType {
    Empty = 0,
    BlockTrades = 1,
    BlockTradesAndFuture = 2,
    BlockTradesAndFutureAndCurrent = 4,
}

impl BlockingType {
    pub fn as_str_name(&self) -> &'static str {
        match self {
            BlockingType::Empty => "Empty",
            BlockingType::BlockTrades => "BlockTrades",
            BlockingType::BlockTradesAndFuture => "BlockTradesAndFuture",
            BlockingType::BlockTradesAndFutureAndCurrent => "BlockTradesAndFutureAndCurrent",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum BlockingStatus {
    Blocked = 0,
    Unblocked = 1,
}

impl BlockingStatus {
    pub fn as_str_name(&self) -> &'static str {
        match self {
            BlockingStatus::Blocked => "Blocked",
            BlockingStatus::Unblocked => "Unblocked",
        }
    }
}