service_sdk::macros::use_my_sb_entity_protobuf_model!();
#[derive(Clone, PartialEq, ::prost::Message)]
#[my_sb_entity_protobuf_model(topic_id = "acqufy-client-click")]
pub struct AcqufyClientClickSbModel {
    #[prost(message, tag = "1")]
    pub event: Option<AcqufyClientClickBodySbModel>,
}

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AcqufyClientClickBodySbModel {
    #[prost(string, tag = "1")]
    pub client_id: String,

    #[prost(string, tag = "2")]
    pub click_id: String,

    #[prost(string, tag = "3")]
    pub affiliate_id: String,
}

