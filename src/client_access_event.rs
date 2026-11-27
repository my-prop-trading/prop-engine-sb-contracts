service_sdk::macros::use_my_sb_entity_protobuf_model!();

/// Key client-area action recorded in the client's device and access log.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum ClientAccessEventType {
    Unknown = 0,
    Registration = 1,
    Login = 2,
    /// A login refused for a known client: wrong password or a restricted client.
    FailedLogin = 3,
    Purchase = 4,
    PayoutRequest = 5,
    KycStarted = 6,
}

/// One key action of a client in the client area, with the browser and network it came from.
/// Delivery is at-least-once: consumers dedupe by `id`.
#[allow(clippy::derive_partial_eq_without_eq)]
#[my_sb_entity_protobuf_model(topic_id = "client-access-event")]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ClientAccessEventSbModel {
    /// Unique per event.
    #[prost(string, tag = "1")]
    pub id: String,

    #[prost(string, tag = "2")]
    pub client_id: String,

    #[prost(enumeration = "ClientAccessEventType", tag = "3")]
    pub event_type: i32,

    /// Unix microseconds.
    #[prost(int64, tag = "4")]
    pub occurred_at: i64,

    /// Persistent ID of the browser the client area runs in; empty when the request carried none.
    #[prost(string, tag = "5")]
    pub browser_id: String,

    #[prost(string, tag = "6")]
    pub ip: String,

    #[prost(string, tag = "7")]
    pub user_agent: String,

    /// ISO-3166 alpha-2 of `ip`; empty when unresolved.
    #[prost(string, tag = "8")]
    pub country: String,

    /// Entry point the event came through, such as the registration form or the login form.
    #[prost(string, tag = "9")]
    pub channel: String,

    #[prost(string, tag = "10")]
    pub brand: String,
}
