#[derive(Clone, PartialEq, ::prost::Message)]
#[my_service_bus_macros::my_sb_entity_protobuf_model(topic_id = "account-registered")]
pub struct AccountRegisteredSbModel {
    #[prost(message, tag = "1")]
    pub event: Option<RegisteredSbModel>,
}

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RegisteredSbModel {
    #[prost(string, tag = "1")]
    pub trader_id: String,
}