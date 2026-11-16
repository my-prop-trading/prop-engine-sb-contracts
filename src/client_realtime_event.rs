service_sdk::macros::use_my_sb_entity_protobuf_model!();

/// An event for the open browser connections of one client (PROP25-2846).
///
/// Published by any service that wants to push a signal to the client area. realtime-api routes it
/// by `client_id` to all open WebSocket connections of the client and passes `topic` and `data` to
/// the browser as is, without decoding them.
///
/// Delivery is best effort: realtime-api reads the topic with a temporary queue, so events
/// published while it is down, or for a client with no open connection, are dropped. The browser
/// loads the current state over REST after every (re)connect. Publish after the change is stored,
/// log a publish error and do not retry it.
///
/// `topic` and `data` are defined in proto-files `RealtimeApi.proto`: `topic` is a `RealtimeTopic`
/// value and `data` is the encoded payload message listed for that topic.
#[allow(clippy::derive_partial_eq_without_eq)]
#[my_sb_entity_protobuf_model(topic_id = "client-realtime-event")]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ClientRealtimeEventSbModel {
    /// Recipient. All open connections of this client get the event.
    #[prost(string, tag = "1")]
    pub client_id: String,

    /// A `RealtimeTopic` value from `RealtimeApi.proto`. int32 rather than an enum here, so that a
    /// new topic needs no release of this crate or of realtime-api.
    #[prost(int32, tag = "2")]
    pub topic: i32,

    /// The encoded payload message of the topic. Empty = no payload. Keep it small: realtime-api
    /// drops events above its size limit (4 KiB by default).
    #[prost(bytes = "vec", tag = "3")]
    pub data: Vec<u8>,

    /// When the event happened, unix microseconds.
    #[prost(int64, tag = "4")]
    pub event_time: i64,
}

impl ClientRealtimeEventSbModel {
    /// `topic`: `RealtimeTopic::X as i32` and `data`: the topic payload message, both from the
    /// caller's code generated from `RealtimeApi.proto`.
    pub fn new(
        client_id: String,
        topic: i32,
        data: &impl ::prost::Message,
        event_time: i64,
    ) -> Self {
        Self {
            client_id,
            topic,
            data: data.encode_to_vec(),
            event_time,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ::prost::Message;

    #[derive(Clone, PartialEq, ::prost::Message)]
    struct TestPayload {
        #[prost(int32, tag = "1")]
        value: i32,
    }

    #[test]
    fn new_encodes_payload_that_decodes_back() {
        let payload = TestPayload { value: 42 };

        let event = ClientRealtimeEventSbModel::new("client".to_string(), 1, &payload, 1_000);

        assert_eq!(event.client_id, "client");
        assert_eq!(event.topic, 1);
        assert_eq!(event.event_time, 1_000);
        assert_eq!(TestPayload::decode(event.data.as_slice()).unwrap(), payload);
    }

    #[test]
    fn model_round_trips_through_protobuf() {
        let event = ClientRealtimeEventSbModel::new(
            "client".to_string(),
            1,
            &TestPayload { value: 7 },
            1_000,
        );

        let decoded = ClientRealtimeEventSbModel::decode(event.encode_to_vec().as_slice()).unwrap();

        assert_eq!(decoded, event);
    }
}
