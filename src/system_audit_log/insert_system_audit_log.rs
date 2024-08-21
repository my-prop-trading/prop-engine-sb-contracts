use service_sdk::my_service_bus::macros::my_sb_entity_protobuf_model;

#[derive(Clone, PartialEq, ::prost::Message)]
#[my_sb_entity_protobuf_model(topic_id = "insert-system-audit-log-command")]
pub struct InsertSystemAuditLogSbModel {
    #[prost(string, tag = "1")]
    pub label: String,
    #[prost(string, tag = "2")]
    pub author: String,
    #[prost(string, tag = "3")]
    pub description: String,
    #[prost(string, tag = "4")]
    pub timestamp: i64,
    #[prost(string, tag = "5")]
    pub ref_id: String,
    #[prost(string, tag = "6")]
    pub snapshot_before_update: String,
    #[prost(string, tag = "7")]
    pub snapshot_after_update: Option<String>,
}
