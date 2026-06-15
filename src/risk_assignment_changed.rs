service_sdk::macros::use_my_sb_entity_protobuf_model!();

/// Published by risk-groups-flows-grpc whenever a client's risk group
/// assignment changes (assigned, reassigned, or removed). trader-accounts
/// subscribes to invalidate its override cache and republish the account
/// status update so trading-control re-reads effective rule settings.
#[derive(Clone, PartialEq, ::prost::Message)]
#[my_sb_entity_protobuf_model(topic_id = "risk-assignment-changed")]
pub struct RiskAssignmentChangedSbModel {
    #[prost(string, tag = "1")]
    pub client_id: String,
}
