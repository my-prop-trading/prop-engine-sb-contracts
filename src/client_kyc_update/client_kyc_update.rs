#[derive(Clone, PartialEq, ::prost::Message)]
#[my_service_bus_macros::my_sb_entity_protobuf_model(topic_id = "client-kyc-update")]
pub struct ClientKycUpdateSbModel {
    #[prost(message, tag = "1")]
    pub event: Option<ClientKycUpdateBodySbModel>,
}

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ClientKycUpdateBodySbModel {
    #[prost(string, tag = "1")]
    pub client_id: String,
    #[prost(int64, tag = "2")]
    pub updated_at: i64,
    #[prost(enumeration="ClientKycUpdateStatus", tag = "3")]
    pub status: i32,
    #[prost(string, tag = "4")]
    pub brand: String,
}


#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum ClientKycUpdateStatus{
    ProofOfIdentitySuccess = 0,
    ProofOfIdentityRetry = 1,
    ProofOfIdentityFailed = 2,
    ProofOfAddressSuccess = 3,
    ProofOfAddressRetry = 4,
    ProofOfAddressFailed = 5,
    KycRequiredStatusSet = 6,
}