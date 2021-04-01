use std::fmt::{self};

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct RawEventHandlerId(u128);

impl RawEventHandlerId {
    pub const fn from_u128(value: u128) -> RawEventHandlerId {
        return RawEventHandlerId(value);
    }
}

impl fmt::Debug for RawEventHandlerId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self)
    }
}

impl fmt::Display for RawEventHandlerId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let uuid = format!("{:032x}", self.0);
        write!(f, "{}-{}-{}-{}-{}", &uuid[0..8], &uuid[8..12], &uuid[12..16], &uuid[16..20], &uuid[20..32])
    }
}