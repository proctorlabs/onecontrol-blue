use super::*;

#[allow(dead_code)]
#[derive(Default, Debug, Display, PartialEq, Clone, Copy, TryFromPrimitive, IntoPrimitive)]
#[repr(u8)]
pub enum OnOff {
    #[default]
    Off = 0,
    On = 1,
}

define_encodable_struct! {
    TankStatus [2] {
        device_id: u8 [0],
        percentage: u8 [1],
    }
    RelayStateType2 [7] {
        device_id: u8 [0],
        status: u8 [1],
        start_position: u8 [2],
        amp_draw: u16 [3],
        dtc: u16 [5],
    }
}

#[allow(dead_code)]
impl RelayStateType2 {
    pub fn is_on(&self) -> bool {
        (self.status & 0x01) == 0x01
    }

    pub fn is_forward_allowed(&self) -> bool {
        (self.status & 0x80) == 0x80
    }

    pub fn is_forward_active(&self) -> bool {
        ((self.status & 0x02) == 0x02) && !self.is_reverse_active()
    }

    pub fn is_reverse_allowed(&self) -> bool {
        (self.status & 0x40) == 0x40
    }

    pub fn is_reverse_active(&self) -> bool {
        (self.status & 0x03) == 0x03
    }

    pub fn is_stopped(&self) -> bool {
        !self.is_forward_active() && !self.is_reverse_active()
    }

    pub fn on_off(&self) -> OnOff {
        if self.is_on() {
            OnOff::On
        } else {
            OnOff::Off
        }
    }
}
