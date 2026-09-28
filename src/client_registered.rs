service_sdk::macros::use_my_sb_entity_protobuf_model!();

/// Successful registration with its request context, published by auth-rest-api.
/// Unlike `account-registered` (trader-creds, credentials level) this one carries IP and UA,
/// so abuse rules can count per key and a reviewer can tell a farm from a household.
#[derive(Clone, PartialEq, ::prost::Message)]
#[my_sb_entity_protobuf_model(topic_id = "client-registered")]
pub struct ClientRegisteredSbModel {
    #[prost(string, tag = "1")]
    pub client_id: String,

    #[prost(string, tag = "2")]
    pub brand: String,

    /// Client IP as resolved by the producer. Empty when it could not be resolved.
    #[prost(string, tag = "3")]
    pub ip: String,

    /// Raw `User-Agent` header. Evidence for review only — never part of a counter key,
    /// since rotating it costs an attacker one line of code.
    #[prost(string, tag = "4")]
    pub user_agent: String,

    /// ISO-3166 alpha-2 country of the registration IP, from the Cloudflare `CF-IPCountry`
    /// header. Empty when the header is absent or unresolved (`XX`/`T1` normalised to empty).
    #[prost(string, tag = "5")]
    pub country: String,

    /// Registration entry point: `register_v1`, `register_v2` or `subscribe`.
    #[prost(string, tag = "6")]
    pub channel: String,

    /// Unix microseconds.
    #[prost(int64, tag = "7")]
    pub registered_at: i64,
}
