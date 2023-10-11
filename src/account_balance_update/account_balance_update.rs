service_sdk::macros::use_my_sb_entity_protobuf_model!();
#[derive(Clone, PartialEq, ::prost::Message)]
#[my_sb_entity_protobuf_model(topic_id = "account-balance-update")]
pub struct AccountBalanceUpdateSbModel {
    #[prost(message, tag = "1")]
    pub event: Option<AccountBalanceUpdateBodySbModel>,
}

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AccountBalanceUpdateBodySbModel {
    #[prost(string, tag = "1")]
    pub trader_account_id: String,
    #[prost(int64, tag = "2")]
    pub updated_at: i64,
    #[prost(double, tag = "4")]
    pub balance: f64,
    #[prost(double, tag = "5")]
    pub equity: f64,
    #[prost(double, tag = "6")]
    pub margin: f64,
    #[prost(double, tag = "7")]
    pub margin_free: f64,
    #[prost(double, tag = "8")]
    pub margin_level: f64,
}