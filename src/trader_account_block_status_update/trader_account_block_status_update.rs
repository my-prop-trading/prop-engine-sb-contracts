use crate::common::AccountType; 

service_sdk::macros::use_my_sb_entity_protobuf_model!();
#[derive(Clone, PartialEq, ::prost::Message)]
#[my_sb_entity_protobuf_model(topic_id = "trader-account-block-status-update")]
pub struct TraderAccountBlockStatusUpdateSbModel {
    #[prost(string, tag = "1")]
    pub trader_account_id: String,

    #[prost(enumeration="BlockType", tag = "2")]
    pub block_type: i32,

    #[prost(string, tag = "3")]
    pub brand: String,

    #[prost(string, tag = "4")]
    pub client_id: String,

    #[prost(enumeration = "AccountType", tag = "5")]
    pub account_type: i32,

    #[prost(int32, tag = "6")]
    pub platform_id: i32,

    #[prost(enumeration = "BlockStatus", tag = "7")]
    pub block_status: i32,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum BlockType {
    Empty = 0,
    BlockTrades = 1,
    BlockTradesAndFuture = 2,
    BlockTradesAndFutureAndCurrent = 4,
}

impl BlockType {
    pub fn as_str_name(&self) -> &'static str {
        match self {
            BlockType::Empty => "Empty",
            BlockType::BlockTrades => "BlockTrades",
            BlockType::BlockTradesAndFuture => "BlockTradesAndFuture",
            BlockType::BlockTradesAndFutureAndCurrent => "BlockTradesAndFutureAndCurrent",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum BlockStatus {
    Blocked = 0,
    Unblocked = 1,
}

impl BlockStatus {
    pub fn as_str_name(&self) -> &'static str {
        match self {
            BlockStatus::Blocked => "Blocked",
            BlockStatus::Unblocked => "Unblocked",
        }
    }
}