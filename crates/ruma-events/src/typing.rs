//! Types for the `m.typing` event.

use ruma_events_macros::EventContent;
use ruma_identifiers::UserId;
use serde::{Deserialize, Serialize};

/// The content of an `m.typing` event.
///
/// Informs the client who is currently typing in a given room.
#[derive(Clone, Debug, Default, Deserialize, Serialize, EventContent)]
#[cfg_attr(not(feature = "unstable-exhaustive-types"), non_exhaustive)]
#[ruma_event(type = "m.typing", kind = EphemeralRoom)]
pub struct TypingEventContent {
    /// The list of user IDs typing in this room, if any.
    pub user_ids: Vec<Box<UserId>>,
}

impl TypingEventContent {
    /// Creates a new `TypingEventContent` with the given user IDs.
    pub fn new(user_ids: Vec<Box<UserId>>) -> Self {
        Self { user_ids }
    }
}
