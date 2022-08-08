use super::*;

#[allow(dead_code)]
#[derive(Default, Debug, Display, PartialEq, Clone, Copy, TryFromPrimitive, IntoPrimitive)]
#[repr(u8)]
pub enum OnOff {
    #[default]
    Off = 0,
    On = 1,
}
