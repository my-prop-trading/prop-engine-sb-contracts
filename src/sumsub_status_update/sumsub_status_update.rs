#[derive(Clone, PartialEq, ::prost::Message)]
#[my_service_bus_macros::my_sb_entity_protobuf_model(topic_id = "sumsub-status-update")]
pub struct SumsubUpdateSbModel {
    #[prost(message, tag = "1")]
    pub event: Option<SumsubUpdateBodySbModel>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum ReviewStatus {
    Init = 0,
    Pending = 1,
    Prechecked = 2,
    Queued =3,
    Completed = 4,
    OnHold = 5,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum ProofType {
    ProofOfAddress = 0,
    ProofOfIdentity = 1,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum NotificationType {
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
pub struct SumsubUpdateBodySbModel {
    #[prost(string, tag = "1")]
    pub client_id: String,
    #[prost(string, tag = "2")]
    pub verification_id: String,
    #[prost(string, tag = "3")]
    pub applicant_id: String,
    #[prost(string, tag = "4")]
    pub created_at: i64,
    #[prost(enumeration="NotificationType", tag = "5")]
    pub notification_type: i32,
    #[prost(enumeration = "ProofType", tag = "6")]
    pub proof_type: i32,
    #[prost(enumeration="ReviewStatus", tag = "7")]
    pub review_status: i32
}
