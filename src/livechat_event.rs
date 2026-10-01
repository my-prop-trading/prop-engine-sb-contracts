service_sdk::macros::use_my_sb_entity_protobuf_model!();

// Emitted by livechat-webhook on each subscribed LiveChat webhook event
// (thread_tagged / thread_untagged / chat_deactivated). It is only a trigger: the payload
// carries the chat id and the action, and livechat-bridge-flows-grpc re-reads the full thread
// from the LiveChat API, resolves the client, and persists it. PROP25-1705.
#[derive(Clone, PartialEq, ::prost::Message)]
#[my_sb_entity_protobuf_model(topic_id = "livechat-event")]
pub struct LiveChatEventSbModel {
    #[prost(string, tag = "1")]
    pub chat_id: String,
    /// The LiveChat webhook action, e.g. "thread_tagged" / "chat_deactivated".
    #[prost(string, tag = "2")]
    pub action: String,
}
