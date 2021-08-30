/// A Matrix key ID.
///
/// Device identifiers in Matrix are completely opaque character sequences. This type is
/// provided simply for its semantic value.
#[repr(transparent)]
pub struct DeviceId(str);

/// An owned [DeviceId].
pub type DeviceIdBox = Box<DeviceId>;

impl DeviceId {
    #[allow(clippy::transmute_ptr_to_ptr)]
    fn from_borrowed(s: &str) -> &Self {
        unsafe { ::std::mem::transmute(s) }
    }

    pub(super) fn from_owned(s: ::std::boxed::Box<str>) -> ::std::boxed::Box<Self> {
        unsafe { ::std::mem::transmute(s) }
    }

    fn into_owned(self: ::std::boxed::Box<Self>) -> ::std::boxed::Box<str> {
        unsafe { ::std::mem::transmute(self) }
    }

    doc_concat! {
        #[doc = concat!("Creates a string slice from this `", "DeviceId", "`.")]
        pub fn as_str(&self) -> &str {
            &self.0
        }
    }

    doc_concat! {
        #[doc = concat!("Creates a byte slice from this `", "DeviceId", "`.")]
        pub fn as_bytes(&self) -> &[u8] {
            self.0.as_bytes()
        }
    }
}

impl ::std::fmt::Debug for DeviceId {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl Clone for DeviceIdBox {
    fn clone(&self) -> Self {
        (**self).to_owned()
    }
}

impl ToOwned for DeviceId {
    type Owned = DeviceIdBox;

    fn to_owned(&self) -> Self::Owned {
        Self::from_owned(self.0.into())
    }
}

impl From<&DeviceId> for DeviceIdBox {
    fn from(id: &DeviceId) -> Self {
        id.to_owned()
    }
}

impl AsRef<str> for DeviceIdBox {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl<'a> From<&'a str> for &'a DeviceId {
    fn from(s: &'a str) -> Self {
        DeviceId::from_borrowed(s)
    }
}

impl From<&str> for DeviceIdBox {
    fn from(s: &str) -> Self {
        DeviceId::from_owned(s.into())
    }
}

impl From<String> for DeviceIdBox {
    fn from(s: String) -> Self {
        DeviceId::from_owned(s.into())
    }
}

#[cfg(feature = "serde")]
impl<'de> ::serde::Deserialize<'de> for DeviceIdBox {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        ::std::boxed::Box::<str>::deserialize(deserializer).map(DeviceId::from_owned)
    }
}

impl From<DeviceIdBox> for String {
    fn from(id: DeviceIdBox) -> Self {
        id.into_owned().into()
    }
}

as_str_based_impls!(DeviceId);
partial_eq_string!(DeviceId);
partial_eq_string!(DeviceIdBox);

#[cfg(all(test, feature = "rand"))]
mod tests {
    use super::DeviceId;

    #[test]
    fn generate_device_id() {
        assert_eq!(DeviceId::new().as_str().len(), 8);
    }
}
