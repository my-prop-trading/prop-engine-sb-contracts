#[derive(Clone, PartialEq, ::prost::Message)]
#[my_service_bus_macros::my_sb_entity_protobuf_model(topic_id = "sumsub-status-update")]
pub struct SumsubStatusUpdateSbModel {
    #[prost(message, tag = "1")]
    pub event: Option<SumsubStatusUpdateBodySbModel>,
}

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SumsubStatusUpdateBodySbModel {
    #[prost(string, tag = "1")]
    pub client_id: String,
}
