service_sdk::macros::use_my_sb_entity_protobuf_model!();
#[derive(Clone, PartialEq, ::prost::Message)]
#[my_sb_entity_protobuf_model(topic_id = "acqufy-marketing-tracker")]
pub struct AcqufyMarketingTrackerSbModel {
    #[prost(string, tag = "1")]
    pub ref_id: String,

    #[prost(string, tag = "2")]
    pub click_id: String,

    #[prost(string, tag = "3")]
    pub user_agent: String,

    #[prost(string, tag = "4")]
    pub ip: String,

    #[prost(map = "string, string", tag = "5")]
    pub parameters: std::collections::HashMap<String, String>,

    #[prost(string, tag = "6")]
    pub fbclid: String,
}
