service_sdk::macros::use_my_sb_entity_protobuf_model!();
#[allow(clippy::derive_partial_eq_without_eq)]
#[my_sb_entity_protobuf_model(topic_id = "sumsub-status-update")]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SumsubUpdateSbModel {
    #[prost(enumeration = "SumsubProofType", tag = "1")]
    pub proof_type: i32,
    #[prost(oneof = "sumsub_update_sb_model::Response", tags = "2, 3, 4")]
    pub response: ::core::option::Option<sumsub_update_sb_model::Response>,

}
/// Nested message and enum types in `SumsubUpdateSbModel`.
pub mod sumsub_update_sb_model {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Response {
        #[prost(message, tag = "1")]
        Updated(super::UpdatedSbModel),
        #[prost(message, tag = "2")]
        Successful(super::CompletedSuccessfulSbModel),
        #[prost(message, tag = "3")]
        Rejected(super::CompletedRejectedSbModel),
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CompletedRejectedSbModel {
    #[prost(message, optional, tag = "1")]
    pub body: ::core::option::Option<SumsubUpdateBodySbModel>,
    #[prost(enumeration = "SumsubReviewRejectStatus", tag = "2")]
    pub reject_status: i32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CompletedSuccessfulSbModel {
    #[prost(message, optional, tag = "1")]
    pub body: ::core::option::Option<SumsubUpdateBodySbModel>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdatedSbModel {
    #[prost(message, optional, tag = "1")]
    pub body: ::core::option::Option<SumsubUpdateBodySbModel>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SumsubUpdateBodySbModel {
    #[prost(string, tag = "1")]
    pub client_id: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub verification_id: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub applicant_id: ::prost::alloc::string::String,
    #[prost(int64, tag = "4")]
    pub created_at: i64,
    #[prost(enumeration = "SumsubNotificationType", tag = "5")]
    pub notification_type: i32,
    #[prost(enumeration = "SumsubReviewStatus", tag = "6")]
    pub review_status: i32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CompletedStatusSbModel {
    #[prost(oneof = "completed_status_sb_model::Response", tags = "1, 2")]
    pub response: ::core::option::Option<completed_status_sb_model::Response>,
}
/// Nested message and enum types in `CompletedStatusSbModel`.
pub mod completed_status_sb_model {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Response {
        #[prost(message, tag = "1")]
        Successful(super::CompletedSuccessfulSbModel),
        #[prost(message, tag = "2")]
        Rejected(super::CompletedRejectedSbModel),
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum SumsubNotificationType {
    Reviewed = 0,
    Pending = 1,
    Created = 2,
    OnHold = 3,
    PersonalInfoChanged = 4,
    Prechecked = 5,
    Deleted = 6,
    LevelChanged = 7,
    VideoIdentStatusChanged = 8,
    Reset = 9,
    ActionPending = 10,
    ActionReviewed = 11,
    ActionOnHold = 12,
    TravelRuleStatusChanged = 13,
    WorkflowCompleted = 14,
}
impl SumsubNotificationType {
    pub fn as_str_name(&self) -> &'static str {
        match self {
            SumsubNotificationType::Reviewed => "Reviewed",
            SumsubNotificationType::Pending => "Pending",
            SumsubNotificationType::Created => "Created",
            SumsubNotificationType::OnHold => "OnHold",
            SumsubNotificationType::PersonalInfoChanged => "PersonalInfoChanged",
            SumsubNotificationType::Prechecked => "Prechecked",
            SumsubNotificationType::Deleted => "Deleted",
            SumsubNotificationType::LevelChanged => "LevelChanged",
            SumsubNotificationType::VideoIdentStatusChanged => "VideoIdentStatusChanged",
            SumsubNotificationType::Reset => "Reset",
            SumsubNotificationType::ActionPending => "ActionPending",
            SumsubNotificationType::ActionReviewed => "ActionReviewed",
            SumsubNotificationType::ActionOnHold => "ActionOnHold",
            SumsubNotificationType::TravelRuleStatusChanged => "TravelRuleStatusChanged",
            SumsubNotificationType::WorkflowCompleted => "WorkflowCompleted",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "Reviewed" => Some(Self::Reviewed),
            "Pending" => Some(Self::Pending),
            "Created" => Some(Self::Created),
            "OnHold" => Some(Self::OnHold),
            "PersonalInfoChanged" => Some(Self::PersonalInfoChanged),
            "Prechecked" => Some(Self::Prechecked),
            "Deleted" => Some(Self::Deleted),
            "LevelChanged" => Some(Self::LevelChanged),
            "VideoIdentStatusChanged" => Some(Self::VideoIdentStatusChanged),
            "Reset" => Some(Self::Reset),
            "ActionPending" => Some(Self::ActionPending),
            "ActionReviewed" => Some(Self::ActionReviewed),
            "ActionOnHold" => Some(Self::ActionOnHold),
            "TravelRuleStatusChanged" => Some(Self::TravelRuleStatusChanged),
            "WorkflowCompleted" => Some(Self::WorkflowCompleted),
            _ => None,
        }
    }    
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum SumsubReviewRejectStatus {
    Final = 0,
    Retry = 1,
}
impl SumsubReviewRejectStatus {
    pub fn as_str_name(&self) -> &'static str {
        match self {
            SumsubReviewRejectStatus::Final => "Final",
            SumsubReviewRejectStatus::Retry => "Retry",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "Final" => Some(Self::Final),
            "Retry" => Some(Self::Retry),
            _ => None,
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum SumsubProofType {
    ProofOfAddress = 0,
    ProofOfIdentity = 1,
}
impl SumsubProofType {
    pub fn as_str_name(&self) -> &'static str {
        match self {
            SumsubProofType::ProofOfAddress => "ProofOfAddress",
            SumsubProofType::ProofOfIdentity => "ProofOfIdentity",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "ProofOfAddress" => Some(Self::ProofOfAddress),
            "ProofOfIdentity" => Some(Self::ProofOfIdentity),
            _ => None,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum SumsubReviewStatus {
    Init = 0,
    Pending = 1,
    Prechecked = 2,
    Queued = 3,
    Completed = 4,
    OnHold = 5,
}
impl SumsubReviewStatus {
    pub fn as_str_name(&self) -> &'static str {
        match self {
            SumsubReviewStatus::Init => "Init",
            SumsubReviewStatus::Pending => "Pending",
            SumsubReviewStatus::Prechecked => "Prechecked",
            SumsubReviewStatus::Queued => "Queued",
            SumsubReviewStatus::Completed => "Completed",
            SumsubReviewStatus::OnHold => "OnHold",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "Init" => Some(Self::Init),
            "Pending" => Some(Self::Pending),
            "Prechecked" => Some(Self::Prechecked),
            "Queued" => Some(Self::Queued),
            "Completed" => Some(Self::Completed),
            "OnHold" => Some(Self::OnHold),
            _ => None,
        }
    }
}