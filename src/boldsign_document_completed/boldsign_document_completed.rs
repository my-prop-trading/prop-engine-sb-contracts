service_sdk::macros::use_my_sb_entity_protobuf_model!();
#[derive(Clone, PartialEq, ::prost::Message)]
#[my_sb_entity_protobuf_model(topic_id = "boldsign-document-completed")]
pub struct BoldSignDocumentCompletedSbModel {
    #[prost(message, tag = "1")]
    pub event: Option<BoldSignDocumentCompletedBodySbModel>,
}

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BoldSignDocumentCompletedBodySbModel {
    /// BoldSign event id — dedup key for retried webhook deliveries.
    #[prost(string, tag = "1")]
    pub event_id: String,
    /// BoldSign document id of the completed contract. The consumer (kyc-flows-grpc) maps this
    /// to the contract/account it stored when the contract was sent.
    #[prost(string, tag = "2")]
    pub document_id: String,
    /// BoldSign event type, e.g. "Completed".
    #[prost(string, tag = "3")]
    pub event_type: String,
    /// When the webhook was received (unix microseconds).
    #[prost(int64, tag = "4")]
    pub received_at: i64,
}
