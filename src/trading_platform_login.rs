service_sdk::macros::use_my_sb_entity_protobuf_model!();

/// Terminal a platform login came from, normalised across platforms.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum TradingPlatformDeviceType {
    Unknown = 0,
    Desktop = 1,
    Web = 2,
    Android = 3,
    Ios = 4,
}

/// One login attempt to a trading platform account, successful or refused.
/// Delivery is at-least-once and a record may be republished: consumers dedupe by `id`.
/// Platform-specific fields the platform does not report stay empty.
#[allow(clippy::derive_partial_eq_without_eq)]
#[my_sb_entity_protobuf_model(topic_id = "trading-platform-login")]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TradingPlatformLoginSbModel {
    /// Stable per platform record: the same attempt always yields the same id.
    #[prost(string, tag = "1")]
    pub id: String,

    #[prost(int32, tag = "2")]
    pub platform_id: i32,

    #[prost(string, tag = "3")]
    pub platform_account_id: String,

    /// Unix microseconds.
    #[prost(int64, tag = "4")]
    pub occurred_at: i64,

    #[prost(bool, tag = "5")]
    pub success: bool,

    /// Platform wording of the refusal. Empty on success.
    #[prost(string, tag = "6")]
    pub failure_reason: String,

    #[prost(string, tag = "7")]
    pub ip: String,

    /// Platform installation id of the terminal: a reinstall or another device gives a new one.
    #[prost(string, tag = "8")]
    pub device_id: String,

    #[prost(enumeration = "TradingPlatformDeviceType", tag = "9")]
    pub device_type: i32,

    #[prost(string, tag = "10")]
    pub app_build: String,

    /// ISO-3166 alpha-2.
    #[prost(string, tag = "11")]
    pub country: String,

    #[prost(string, tag = "12")]
    pub city: String,

    /// Network provider (ISP) of the IP.
    #[prost(string, tag = "13")]
    pub provider: String,

    /// Network classes the platform assigned to the IP, in the platform's wording.
    #[prost(string, repeated, tag = "14")]
    pub network_flags: Vec<String>,

    #[prost(double, optional, tag = "15")]
    pub ping_ms: Option<f64>,

    /// The platform record as received, kept for review and re-parsing.
    #[prost(string, tag = "16")]
    pub source_text: String,
}
