//! Types for the `m.key.verification.done` event.

use ruma_events_macros::EventContent;
use serde::{Deserialize, Serialize};

use super::Relation;

/// The content of a to-device `m.m.key.verification.done` event.
///
/// Event signaling that the interactive key verification has successfully concluded.
#[derive(Clone, Debug, Deserialize, Serialize, EventContent)]
#[cfg_attr(not(feature = "unstable-exhaustive-types"), non_exhaustive)]
#[ruma_event(type = "m.key.verification.done", kind = ToDevice)]
pub struct ToDeviceKeyVerificationDoneEventContent {
    /// An opaque identifier for the verification process.
    ///
    /// Must be the same as the one used for the `m.key.verification.start` message.
    pub transaction_id: String,
}

impl ToDeviceKeyVerificationDoneEventContent {
    /// Creates a new `ToDeviceKeyVerificationDoneEventContent` with the given transaction ID.
    pub fn new(transaction_id: String) -> Self {
        Self { transaction_id }
    }
}

/// The payload for a in-room `m.key.verification.done` event.
///
/// Event signaling that the interactive key verification has successfully concluded.
#[derive(Clone, Debug, Deserialize, Serialize, EventContent)]
#[cfg_attr(not(feature = "unstable-exhaustive-types"), non_exhaustive)]
#[ruma_event(type = "m.key.verification.done", kind = Message)]
pub struct KeyVerificationDoneEventContent {
    /// Relation signaling which verification request this event is responding to.
    #[serde(rename = "m.relates_to")]
    pub relates_to: Relation,
}

impl KeyVerificationDoneEventContent {
    /// Creates a new `KeyVerificationDoneEventContent` with the given relation.
    pub fn new(relates_to: Relation) -> Self {
        Self { relates_to }
    }
}

#[cfg(test)]
mod tests {
    use matches::assert_matches;
    use ruma_identifiers::event_id;
    use serde_json::{from_value as from_json_value, json, to_value as to_json_value};

    use super::KeyVerificationDoneEventContent;
    use crate::key::verification::Relation;

    #[test]
    fn serialization() {
        let event_id = event_id!("$1598361704261elfgc:localhost").to_owned();

        let json_data = json!({
            "m.relates_to": {
                "rel_type": "m.reference",
                "event_id": event_id,
            }
        });

        let content = KeyVerificationDoneEventContent { relates_to: Relation { event_id } };

        assert_eq!(to_json_value(&content).unwrap(), json_data);
    }

    #[test]
    fn deserialization() {
        let id = event_id!("$1598361704261elfgc:localhost");

        let json_data = json!({
            "m.relates_to": {
                "rel_type": "m.reference",
                "event_id": id,
            }
        });

        assert_matches!(
            from_json_value::<KeyVerificationDoneEventContent>(json_data).unwrap(),
            KeyVerificationDoneEventContent {
                relates_to: Relation {
                    event_id
                },
            } if *event_id == *id
        );
    }
}
