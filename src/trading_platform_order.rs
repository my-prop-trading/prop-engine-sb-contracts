service_sdk::macros::use_my_sb_entity_protobuf_model!();

/// Kind of a trade request sent to a trading platform, normalised across platforms.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum TradingPlatformOrderKind {
    Unknown = 0,
    /// Market order that opens or adds to a position.
    Market = 1,
    /// Market order that closes a position.
    Close = 2,
    /// Pending order placed: limit, stop, stop limit.
    Pending = 3,
    /// New price, stop loss or take profit of a position or a pending order.
    Modify = 4,
    /// Pending order removed.
    Cancel = 5,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum TradingPlatformOrderSide {
    Unknown = 0,
    Buy = 1,
    Sell = 2,
}

/// One trade request a client sent from a terminal to a trading platform account.
/// Requests the platform executes on its own (stop loss, take profit, stop out, a triggered
/// pending order) are not published. Delivery is at-least-once: consumers dedupe by `id`.
#[allow(clippy::derive_partial_eq_without_eq)]
#[my_sb_entity_protobuf_model(topic_id = "trading-platform-order")]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TradingPlatformOrderSbModel {
    /// Stable per platform record: the same request always yields the same id.
    #[prost(string, tag = "1")]
    pub id: String,

    #[prost(int32, tag = "2")]
    pub platform_id: i32,

    #[prost(string, tag = "3")]
    pub platform_account_id: String,

    /// Unix microseconds.
    #[prost(int64, tag = "4")]
    pub occurred_at: i64,

    /// IP the request was sent from.
    #[prost(string, tag = "5")]
    pub ip: String,

    /// Platform installation id of the sending terminal; empty when the platform does not report it.
    #[prost(string, tag = "6")]
    pub device_id: String,

    #[prost(enumeration = "TradingPlatformOrderKind", tag = "7")]
    pub kind: i32,

    #[prost(enumeration = "TradingPlatformOrderSide", tag = "8")]
    pub side: i32,

    #[prost(string, tag = "9")]
    pub symbol: String,

    /// Lots.
    #[prost(double, tag = "10")]
    pub volume: f64,

    /// Platform id of the position or order the request refers to; empty for a new order.
    #[prost(string, tag = "11")]
    pub ticket: String,

    /// The platform record as received, kept for review and re-parsing.
    #[prost(string, tag = "12")]
    pub source_text: String,
}
