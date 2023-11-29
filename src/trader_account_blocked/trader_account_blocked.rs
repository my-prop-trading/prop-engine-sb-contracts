service_sdk::macros::use_my_sb_entity_protobuf_model!();
#[derive(Clone, PartialEq, ::prost::Message)]
#[my_sb_entity_protobuf_model(topic_id = "trader-account-blocked")]
pub struct TraderAccountBlockedSbModel {
    #[prost(message, tag = "1")]
    pub event: Option<TraderAccountBlockedBodySbModel>,
}

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TraderAccountBlockedBodySbModel {
    #[prost(string, tag = "1")]
    pub trader_account_id: String,

    #[prost(enumeration="BlockType", tag = "2")]
    pub block_type: i32,

    #[prost(string, tag = "3")]
    pub brand: String,

    #[prost(string, tag = "4")]
    pub client_id: String,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum BlockType {
    Empty = 0,
    BlockTrades = 1,
    BlockTradesAndFuture = 2,
    BlockTradesAndFutureAndCurrent = 4,
}
