service_sdk::macros::use_my_sb_entity_protobuf_model!();
#[derive(Clone, PartialEq, ::prost::Message)]
#[my_sb_entity_protobuf_model(topic_id = "rise-status-updated")]
pub struct RiseStatusUpdatedSbModel {
    #[prost(message, tag = "1")]
    pub event: Option<RiseStatusUpdatedEventSbModel>,
}

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RiseStatusUpdatedEventSbModel {
    #[prost(string, tag = "1")]
    pub rise_id: String,

    #[prost(int32, tag = "2")]
    pub status: i32,

    #[prost(string, tag = "3")]
    pub email: String,
}