use super::*;

#[allow(dead_code)]
#[derive(Default, Debug, Display, PartialEq, Clone, Copy, TryFromPrimitive, IntoPrimitive)]
#[repr(u8)]
pub enum ProtocolType {
    #[default]
    Unknown = 0,
    Host = 1,
    Can = 2,
}
