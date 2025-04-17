service_sdk::macros::use_my_sb_entity_protobuf_model!();

#[derive(Clone, PartialEq, ::prost::Message)]
#[my_sb_entity_protobuf_model(topic_id = "client-authenticated")]
pub struct ClientAuthenticatedSbModel {
    #[prost(string, tag = "1")]
    pub session_id: String,

    #[prost(string, tag = "2")]
    pub client_id: String,
}