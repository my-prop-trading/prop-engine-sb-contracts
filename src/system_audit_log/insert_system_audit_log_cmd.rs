service_sdk::macros::use_my_sb_entity_protobuf_model!();

#[derive(Clone, PartialEq, ::prost::Message)]
#[my_sb_entity_protobuf_model(topic_id = "insert-system-audit-log-cmd")]
pub struct InsertSystemAuditLogCmdSbModel {
    #[prost(string, tag = "1")]
    pub label: String,
    #[prost(string, tag = "2")]
    pub author: String,
    #[prost(string, tag = "3")]
    pub description: String,
    #[prost(int64, tag = "4")]
    pub timestamp: i64,
    #[prost(string, tag = "5")]
    pub ref_id: String,
    #[prost(string, optional, tag = "6")]
    pub snapshot_before: Option<String>,
    #[prost(string, optional, tag = "7")]
    pub snapshot_after: Option<String>,
}
