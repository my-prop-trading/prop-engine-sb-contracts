use crate::common::AccountType; 

service_sdk::macros::use_my_sb_entity_protobuf_model!();
#[derive(Clone, PartialEq, ::prost::Message)]
#[my_sb_entity_protobuf_model(topic_id = "contract-created")]
pub struct ContractCreatedSbModel { // contract = contractor
    #[prost(string, tag = "1")]
    pub id: String,
    #[prost(string, tag = "2")]
    pub trading_package_id: String,
    #[prost(i64, tag = "3")]
    pub contract_id: i64,
    #[prost(string, tag = "4")]
    pub client_id: String,
    #[prost(string, tag = "5")]
    pub trader_account_aggregated_id: String,
    #[prost(enumeration="ClientLiveAccountContractStatus", tag = "6")]
    pub sign_status: Option<ContractSignStatusDto>,
    #[prost(enumeration="ContractLiveAccountStatusDto", tag = "7")]
    pub live_account_status: ContractLiveAccountStatusDto,
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