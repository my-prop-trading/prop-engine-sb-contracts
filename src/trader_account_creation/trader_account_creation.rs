#[derive(Clone, PartialEq, ::prost::Message)]
#[my_service_bus_macros::my_sb_entity_protobuf_model(topic_id = "trader-account-creation")]
pub struct TraderAccountCreatedSbModel {
    #[prost(message, tag = "1")]
    pub event: Option<TraderAccountCreatedBodySbModel>,
}

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct  TraderAccountCreatedBodySbModel {
    #[prost(string, tag = "1")]
    pub trader_account_id: String,
}