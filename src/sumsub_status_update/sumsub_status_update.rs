#[derive(Clone, PartialEq, ::prost::Message)]
#[my_service_bus_macros::my_sb_entity_protobuf_model(topic_id = "sumsub-status-update")]
pub struct SumsubUpdateSbModel {
    #[prost(message, tag = "1")]
    pub event: Option<SumsubUpdateBodySbModel>,
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

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum SumsubProofType {
    ProofOfAddress = 0,
    ProofOfIdentity = 1,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum SumsubReviewAnswerStatus {
    Green = 0,
    Red = 1,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum SumsubReviewRejectStatus {
    Final = 0,
    Retry = 1,
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

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SumsubUpdateBodyCompletedSbModel {
    #[prost(enumeration = "SumsubReviewAnswerStatus", tag = "1")]
    pub answer_status: i32,

    #[prost(optional, enumeration = "SumsubReviewRejectStatus", tag = "2")]
    pub reject_status: Option<i32>,
}

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SumsubUpdateBodySbModel {
    #[prost(string, tag = "1")]
    pub client_id: String,
    #[prost(string, tag = "2")]
    pub verification_id: String,
    #[prost(string, tag = "3")]
    pub applicant_id: String,
    #[prost(int64, tag = "4")]
    pub created_at: i64,
    #[prost(enumeration = "SumsubNotificationType", tag = "5")]
    pub notification_type: i32,
    #[prost(enumeration = "SumsubProofType", tag = "6")]
    pub proof_type: i32,
    #[prost(enumeration = "SumsubReviewStatus", tag = "7")]
    pub review_status: i32,
    #[prost(message, tag = "8")]
    pub completed_answer_status: Option<SumsubUpdateBodyCompletedSbModel>,
}

pub fn i32_to_sumsub_review_status(value: i32) -> SumsubReviewStatus {
    match value {
        0 => SumsubReviewStatus::Init,
        1 => SumsubReviewStatus::Pending,
        2 => SumsubReviewStatus::Prechecked,
        3 => SumsubReviewStatus::Queued,
        4 => SumsubReviewStatus::Completed,
        5 => SumsubReviewStatus::OnHold,
        _ => panic!("Unknown SumsubReviewStatus: {}", value),
    }
}

pub fn i32_to_sumsub_notification_type(value: i32) -> SumsubNotificationType {
    match value {
        0 => SumsubNotificationType::Reviewed,
        1 => SumsubNotificationType::Pending,
        2 => SumsubNotificationType::Created,
        3 => SumsubNotificationType::OnHold,
        4 => SumsubNotificationType::PersonalInfoChanged,
        5 => SumsubNotificationType::Prechecked,
        6 => SumsubNotificationType::Deleted,
        7 => SumsubNotificationType::LevelChanged,
        8 => SumsubNotificationType::VideoIdentStatusChanged,
        9 => SumsubNotificationType::Reset,
        10 => SumsubNotificationType::ActionPending,
        11 => SumsubNotificationType::ActionReviewed,
        12 => SumsubNotificationType::ActionOnHold,
        13 => SumsubNotificationType::TravelRuleStatusChanged,
        14 => SumsubNotificationType::WorkflowCompleted,
        _ => panic!("Unknown SumsubNotificationType: {}", value),
    }
}

pub fn i32_to_sumsub_proof_type(value: i32) -> SumsubProofType {
    match value {
        0 => SumsubProofType::ProofOfAddress,
        1 => SumsubProofType::ProofOfIdentity,
        _ => panic!("Unknown SumsubProofType: {}", value),
    }
}


pub fn i32_to_sumsub_review_answer_status(value: i32) -> SumsubReviewAnswerStatus {
    match value {
        0 => SumsubReviewAnswerStatus::Green,
        1 => SumsubReviewAnswerStatus::Red,
        _ => panic!("Unknown SumsubReviewAnswerStatus: {}", value),
    }
}


pub fn i32_to_sumsub_review_reject_status(value: i32) -> SumsubReviewRejectStatus {
    match value {
        0 => SumsubReviewRejectStatus::Final,
        1 => SumsubReviewRejectStatus::Retry,
        _ => panic!("Unknown SumsubReviewRejectStatus: {}", value),
    }
}
