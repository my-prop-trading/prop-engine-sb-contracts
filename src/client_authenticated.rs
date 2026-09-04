service_sdk::macros::use_my_sb_entity_protobuf_model!();

#[derive(Clone, PartialEq, ::prost::Message)]
#[my_sb_entity_protobuf_model(topic_id = "client-authenticated")]
pub struct ClientAuthenticatedSbModel {
    #[prost(string, tag = "1")]
    pub session_id: String,

    #[prost(string, tag = "2")]
    pub client_id: String,

    /// Client IP at authentication (from X-Forwarded-For). PROP25-2360: consumed by the contractor
    /// auto-approve unique-IP check. Empty when the source could not resolve it.
    #[prost(string, tag = "3")]
    pub ip: String,

    /// ISO-3166 alpha-2 country of the login IP, from the Cloudflare `CF-IPCountry` header
    /// (PROP25-2360, Iana 2026-09-04): the unique-IP check now counts distinct countries, so a
    /// client's dynamic same-country IPs no longer trip it. Empty when the header is absent or
    /// Cloudflare could not resolve the country (`XX`/`T1` are normalised to empty upstream).
    #[prost(string, tag = "4")]
    pub country: String,
}