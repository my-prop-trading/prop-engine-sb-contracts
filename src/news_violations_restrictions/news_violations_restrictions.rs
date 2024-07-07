use crate::common::AccountType; 

service_sdk::macros::use_my_sb_entity_protobuf_model!();

#[derive(Clone, PartialEq, ::prost::Message)]
#[my_sb_entity_protobuf_model(topic_id = "news-violations-restrictions")]
pub struct NewsViolationsRestrictionsSbModel {
    #[prost(message, tag = "1")]
    pub event: Option<NewsViolationsRestrictionsBodySbModel>,
}

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct  NewsViolationsRestrictionsBodySbModel {
    #[prost(string, tag = "1")]
    pub client_id: String,

    #[prost(string, tag = "2")]
    pub trader_account_id: String,

    #[prost(string, tag = "3")]
    pub platform_account_id: String,

    #[prost(int32, tag = "4")]
    pub platform_id: i32,

    #[prost(enumeration = "AccountType", tag = "5")]
    pub account_type: i32,

    #[prost(string, repeated, tag = "6")]
    pub trades: Vec<String>,

    #[prost(string, tag = "7")]
    pub news_id: String,

    #[prost(string, tag = "8")]
    pub news_event: String,

    #[prost(int64, tag = "9")]
    pub news_event_time: i64,
}