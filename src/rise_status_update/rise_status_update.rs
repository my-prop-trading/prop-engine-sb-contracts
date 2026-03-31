service_sdk::macros::use_my_sb_entity_protobuf_model!();
#[derive(Clone, PartialEq, ::prost::Message)]
#[my_sb_entity_protobuf_model(topic_id = "rise-status-updated")]
pub struct RiseStatusUpdatedSbModel {
    #[prost(message, tag = "1")]
    pub event: Option<AccountEmailChangedSbModel>,
}

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ARiseStatusUpdatedEventSbModel {
    #[prost(string, tag = "1")]
    pub rise_id: String,

    #[prost(string, tag = "2")]
    pub status: int32,

    #[prost(string, tag = "3")]
    pub email: String,
}