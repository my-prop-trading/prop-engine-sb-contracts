use crate::common::AccountType; 

service_sdk::macros::use_my_sb_entity_protobuf_model!();
#[derive(Clone, PartialEq, ::prost::Message)]
#[my_sb_entity_protobuf_model(topic_id = "trader-account-status-update")]
pub struct TraderAccountStatusUpdateSbModel {
    #[prost(message, tag = "1")]
    pub event: Option<TraderAccountStatusUpdateBodySbModel>,
}

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TraderAccountStatusUpdateBodySbModel {
    #[prost(string, tag = "1")]
    pub trader_account_id: String,

    #[prost(int32, tag = "2")]
    pub platform_id: i32,

    #[prost(enumeration = "AccountType", tag = "3")]
    pub account_type: i32,

    #[prost(enumeration="TraderAccountStatus", tag = "4")]
    pub status: i32 ,

    #[prost(string, tag = "5")]
    pub brand: String,

    #[prost(string, tag = "6")]
    pub client_id: String,

    #[prost(int32, tag = "7")]
    pub phase: i32,

    #[prost(string, tag = "8")]
    pub trader_account_aggregated_id: String,

    #[prost(string, tag = "9")]
    pub trading_package_id: String,

    #[prost(int64, tag = "10")]
    pub e_tag: i64,

    #[prost(enumeration="TraderPackagePhaseType", tag = "12")]
    pub phase_type: i32,

    #[prost(string, tag = "13")]
    pub order_id: String,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum TraderAccountStatus {
    New = 0,
    Active = 1,
    Disabled = 2,
    StageCompleted = 3,
    Blocked = 4,
}

impl TraderAccountStatus {
    pub fn as_str_name(&self) -> &'static str {
        match self {
            TraderAccountStatus::New => "New",
            TraderAccountStatus::Active => "Active",
            TraderAccountStatus::Disabled => "Disabled",
            TraderAccountStatus::StageCompleted => "StageCompleted",
            TraderAccountStatus::Blocked => "Blocked",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum TraderPackagePhaseType {
    Phase1 = 0,
    Phase2 = 1,
    Phase3 = 2,
    InstantFunding = 3,
}

impl TraderPackagePhaseType {
    pub fn as_str_name(&self) -> &'static str {
        match self {
            TraderPackagePhaseType::Phase1 => "Phase1",
            TraderPackagePhaseType::Phase2 => "Phase2",
            TraderPackagePhaseType::Phase3 => "Phase3",
            TraderPackagePhaseType::InstantFunding => "InstantFunding",
        }
    }
}