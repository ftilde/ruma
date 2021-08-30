//! Matrix device identifiers.

#[cfg(feature = "rand")]
use crate::generate_localpart;

opaque_identifier! {
    /// A Matrix key identifier.
    ///
    /// Key identifiers in Matrix are opaque character sequences of `[a-zA-Z_]`. This type is
    /// provided simply for its semantic value.
    pub type KeyName [ validate ];
}
