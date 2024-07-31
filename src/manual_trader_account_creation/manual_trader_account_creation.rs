service_sdk::macros::use_my_sb_entity_protobuf_model!();
#[derive(Clone, PartialEq, ::prost::Message)]
#[my_sb_entity_protobuf_model(topic_id = "manual-trading-account-creation")]
pub struct ManualTradingAccountCreationSbModel {
    #[prost(message, tag = "1")]
    pub event: Option<ManualTradingAccountCreationBodySbModel>,
}

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct  ManualTradingAccountCreationBodySbModel {
    #[prost(string, tag = "1")]
    pub trader_account_id: String,

    #[prost(int32, tag = "2")]
    pub remaining_phases: i32,
}
