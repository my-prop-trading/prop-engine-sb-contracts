service_sdk::macros::use_my_sb_entity_protobuf_model!();
#[allow(clippy::derive_partial_eq_without_eq)]
#[my_sb_entity_protobuf_model(topic_id = "kyc-webhook-update")]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct KycWebhookUpdateSbModel {
    #[prost(enumeration = "KycWebhookProofType", tag = "1")]
    pub proof_type: i32,
    #[prost(oneof = "kyc_webhook_sb_model::Response", tags = "2, 3, 4")]
    pub response: ::core::option::Option<kyc_webhook_sb_model::Response>,
}

pub mod kyc_webhook_sb_model {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Response {
        #[prost(message, tag = "1")]
        Created(super::KycWebhookCreatedSbModel),
        #[prost(message, tag = "2")]
        Updated(super::KycWebhookUpdatedSbModel),
        #[prost(message, tag = "3")]
        Successful(super::KycWebhookSuccessfulSbModel),
        #[prost(message, tag = "4")]
        Rejected(super::KycWebhookRejectedSbModel),
    }
}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct KycWebhookCreatedSbModel {
    #[prost(message, optional, tag = "1")]
    pub body: ::core::option::Option<KycWebhookCreatedBodySbModel>,
}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct KycWebhookRejectedSbModel {
    #[prost(message, optional, tag = "1")]
    pub body: ::core::option::Option<KycWebhookUpdateBodySbModel>,
}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct KycWebhookSuccessfulSbModel {
    #[prost(message, optional, tag = "1")]
    pub body: ::core::option::Option<KycWebhookUpdateBodySbModel>,
}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct KycWebhookUpdatedSbModel {
    #[prost(message, optional, tag = "1")]
    pub body: ::core::option::Option<KycWebhookUpdateBodySbModel>,
}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct KycWebhookCreatedBodySbModel {
    #[prost(string, tag = "1")]
    pub client_id: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub verification_id: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub applicant_id: ::prost::alloc::string::String,
    #[prost(int64, tag = "4")]
    pub created_at: i64,
}


#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct KycWebhookUpdateBodySbModel {
    #[prost(string, tag = "1")]
    pub client_id: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub verification_id: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub applicant_id: ::prost::alloc::string::String,
    #[prost(int64, tag = "4")]
    pub created_at: i64,
    #[prost(enumeration = "KycProofStatus", tag = "5")]
    pub proof_status: i32,
    #[prost(enumeration = "KycProofResult", tag = "6")]
    pub proof_result: i32,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum KycProofStatus {
    Approved = 0,
    Resubmission = 1,
    Rejected = 2,
    Pending = 3,
}

impl KycProofStatus {
    pub fn as_str_name(&self) -> &'static str {
        match self {
            KycProofStatus::Approved => "Approved",
            KycProofStatus::Resubmission => "Resubmission",
            KycProofStatus::Rejected => "Rejected",
            KycProofStatus::Pending => "Pending",
        }
    }

    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "Approved" => Some(Self::Approved),
            "Resubmission" => Some(Self::Resubmission),
            "Rejected" => Some(Self::Rejected),
            "Pending" => Some(Self::Pending),
            _ => None,
        }
    }    
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum KycProofResult {
    Final = 0,
    Retry = 1,
    Approved = 2,
}

impl KycProofResult {
    pub fn as_str_name(&self) -> &'static str {
        match self {
            KycProofResult::Final => "Final",
            KycProofResult::Retry => "Retry",
            KycProofResult::Approved => "Approved",
        }
    }

    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "Final" => Some(Self::Final),
            "Retry" => Some(Self::Retry),
            "Approved" => Some(Self::Approved),
            _ => None,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum KycWebhookProofType {
    ProofOfIdentity = 0,
    ProofOfAddress = 1,
}

impl KycWebhookProofType {
    pub fn as_str_name(&self) -> &'static str {
        match self {
            KycWebhookProofType::ProofOfIdentity => "ProofOfIdentity",
            KycWebhookProofType::ProofOfAddress => "ProofOfAddress",
        }
    }

    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "ProofOfIdentity" => Some(Self::ProofOfIdentity),
            "ProofOfAddress" => Some(Self::ProofOfAddress),
            _ => None,
        }
    }
}
