service_sdk::macros::use_my_sb_entity_protobuf_model!();
#[derive(Clone, PartialEq, ::prost::Message)]
#[my_sb_entity_protobuf_model(topic_id = "personal-data-update")]
pub struct PersonalDataUpdateSbModel {
    #[prost(message, tag = "1")]
    pub event: Option<PersonalDataUpdateBodySbModel>,
}

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PersonalDataUpdateBodySbModel {
    #[prost(string, tag = "1")]
    pub client_id: String,
    #[prost(int64, tag = "2")]
    pub updated_at: i64,
}
