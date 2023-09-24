#[derive(Clone, PartialEq, ::prost::Message)]
#[my_service_bus_macros::my_sb_entity_protobuf_model(topic_id = "client-live-account-contract", label = "ClientLiveAccountContract")]
pub struct ClientLiveAccountContractUpdateSbModel{
    #[prost(message, tag = "1")]
    pub event: Option<ClientLiveAccountContractUpdateBodySbModel>,
}

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ClientLiveAccountContractUpdateBodySbModel{
    #[prost(string, tag = "1")]
    pub trader_account_aggregated_id: String,
    #[prost(enumeration="ClientLiveAccountContractStatus", tag = "2")]
    pub status: i32,
    #[prost(string, optional, tag = "3")]
    pub description: Option<String>,
    #[prost(string, tag = "4")]
    pub client_id: String,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum ClientLiveAccountContractStatus{
    Created = 0,
    SentToClient = 1,
    SignedByClientAndReceived = 2,
    GrantedLiveAccount = 3,
    Rejected = 4,
    RejectedAndBlocked = 5,
    Uploaded = 6,
    Downloaded = 7,
}
