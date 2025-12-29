service_sdk::macros::use_my_sb_entity_protobuf_model!();
#[allow(clippy::derive_partial_eq_without_eq)]
#[my_sb_entity_protobuf_model(topic_id = "contract-created")]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ContractCreatedSbModel {
    #[prost(string, tag = "1")]
    pub id: String,
    #[prost(string, tag = "2")]
    pub trading_package_id: String,
    #[prost(int64, tag = "3")]
    pub contract_id: i64,
    #[prost(string, tag = "4")]
    pub client_id: String,
    #[prost(string, tag = "5")]
    pub trader_account_aggregated_id: String,
    #[prost(enumeration="ContractSignStatusDto", tag = "6")]
    pub sign_status: i32, 
    #[prost(enumeration="ContractLiveAccountStatusDto", tag = "7")]
    pub live_account_status: i32,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum ContractSignStatusDto {
    NotInit = 0,
    Pending = 1,
    Success = 2,
    Failed = 3,
    FailedAndBlock = 4,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum ContractLiveAccountStatusDto {
    NotInit = 0,
    GrantedLiveAccount = 1,
    Rejected = 2,
    RejectedAndBlocked = 3,
}