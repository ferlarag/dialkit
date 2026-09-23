//! Stable, task-oriented Voice resource handles.

mod calls;
mod conferences;
mod configuration;
mod queues;
mod recordings;
mod sip;

pub use calls::*;
pub use conferences::*;
pub use configuration::*;
pub use queues::*;
pub use recordings::*;
pub use sip::*;

macro_rules! sid_type {
    ($name:ident, $prefix:literal, $description:literal) => {
        #[derive(Clone, Debug, Eq, Hash, PartialEq)]
        pub struct $name(String);

        impl $name {
            pub fn new(value: impl Into<String>) -> Result<Self, crate::Error> {
                let value = value.into();
                if value.len() != 34
                    || !value.starts_with($prefix)
                    || !value[2..].bytes().all(|byte| byte.is_ascii_hexdigit())
                {
                    return Err(crate::Error::Validation(
                        concat!($description, " must use its documented SID prefix").into(),
                    ));
                }
                Ok(Self(value))
            }

            #[must_use]
            pub fn as_str(&self) -> &str {
                &self.0
            }
        }
    };
}

pub(crate) use sid_type;
