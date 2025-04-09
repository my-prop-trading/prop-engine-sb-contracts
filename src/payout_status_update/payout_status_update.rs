use crate::common::AccountType; 

service_sdk::macros::use_my_sb_entity_protobuf_model!();
#[derive(Clone, PartialEq, ::prost::Message)]
#[my_sb_entity_protobuf_model(topic_id = "payout-status-update")]
pub struct PayoutStatusUpdateSbModel {
    #[prost(message, tag = "1")]
    pub event: Option<PayoutStatusUpdateBodySbModel>,
}

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PayoutStatusUpdateBodySbModel {
    #[prost(string, tag = "1")]
    pub payout_id: String,

    #[prost(int32, tag = "2")]
    pub platform_id: i32,

    #[prost(enumeration = "AccountType", tag = "3")]
    pub account_type: i32,

    #[prost(enumeration="PayoutStatus", tag = "4")]
    pub status: i32 ,

    #[prost(string, tag = "5")]
    pub brand: String,

    #[prost(string, tag = "6")]
    pub client_id: String,

    #[prost(string, tag = "7")]
    pub trader_account_id: String,

    #[prost(double, tag = "8")]
    pub withdrawal_amount: f64,
    
    #[prost(string, tag = "9")]
    pub withdrawal_currency: String,

    #[prost(int64, tag = "10")]
    pub created_at: i64,

    #[prost(double, tag = "11")]
    pub revenue_share: f64,

    #[prost(enumeration="PayoutType", tag = "12")]
    pub payout_type: i32,

    #[prost(double, tag = "13")]
    pub aggregated_withdrawal_amount: f64,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum PayoutStatus {
    Pending = 0,
    Success = 1,
    Fail = 2,
    Approved = 3,
}

impl PayoutStatus {
    pub fn as_str_name(&self) -> &'static str {
        match self {
            PayoutStatus::Pending => "Pending",
            PayoutStatus::Success => "Success",
            PayoutStatus::Fail => "Fail",
            PayoutStatus::Approved => "Approved",
        }
    }

    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "Pending" => Some(Self::Pending),
            "Success" => Some(Self::Success),
            "Fail" => Some(Self::Fail),
            "Approved" => Some(Self::Approved),
            _ => None,
        }
    }    
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum PayoutType {
    RevenueShare = 0,
    Withdrawal = 1,
    ExcessiveProfit = 2,
}

impl PayoutType {
    pub fn as_str_name(&self) -> &'static str {
        match self {
            PayoutType::RevenueShare => "RevenueShare",
            PayoutType::Withdrawal => "Withdrawal",
            PayoutType::ExcessiveProfit => "ExcessiveProfit",
        }
    }

    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "RevenueShare" => Some(Self::RevenueShare),
            "Withdrawal" => Some(Self::Withdrawal),
            "ExcessiveProfit" => Some(Self::ExcessiveProfit),
            _ => None,
        }
    }    
}