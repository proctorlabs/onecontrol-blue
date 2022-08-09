use crate::error::*;
use crate::messages::Encodable;
use fixed::{types::extra::U8, FixedU16};
use num_enum::IntoPrimitive;

pub trait EventTrait: Sized {
    fn min_length(&self) -> usize;
    fn max_length(&self) -> usize;
    fn event_type(&self) -> EventType;
    fn from_payload(bytes: Vec<u8>) -> Result<Self>;
    fn into_data(self) -> Vec<u8>;
}

macro_rules! events {
    ($(
        $msgname:ident ($command_type:literal ; $min:literal .. $max:literal) {
            $( $name:ident : $type:ty [ $index:literal ] , )
        *}
    )*) => {
        #[allow(dead_code)]
        #[derive(Debug, Default, Display, IntoPrimitive, PartialEq, Clone, Copy)]
        #[repr(u8)]
        pub enum EventType {
            #[default]
            $( $msgname = $command_type, )*
        }

        impl std::convert::TryFrom<u8> for EventType {
            type Error = AppError;

            fn try_from(val: u8) -> Result<Self> {
                match val {
                    $( $command_type => Ok(EventType::$msgname), )*
                    v => Err(AppError::InvalidCommand(v)),
                }
            }
        }

        #[allow(dead_code)]
        #[derive(Debug)]
        pub enum Event {$(
            $msgname($msgname),
        )*}

        impl EventTrait for Event {
            fn min_length(&self) -> usize {
                match &self {
                    $( Event::$msgname(inner) => inner.min_length(), )*
                }
            }

            fn max_length(&self) -> usize {
                match &self {
                    $( Event::$msgname(inner) => inner.max_length(), )*
                }
            }

            fn event_type(&self) -> EventType {
                match &self {
                    $( Event::$msgname(inner) => inner.event_type(), )*
                }
            }

            fn from_payload(bytes: Vec<u8>) -> Result<Self> {
                if bytes.len() < 3 {
                    return Err(AppError::InvalidPayload);
                }
                match bytes[0].try_into()? {
                    $( EventType::$msgname => Ok(Event::$msgname($msgname::from_payload(bytes)?)), )*
                }
            }

            fn into_data(self) -> Vec<u8> {
                match self {
                    $( Event::$msgname(inner) => inner.into_data(), )*
                }
            }
        }

        $(
            #[derive(Debug)]
            #[allow(dead_code)]
            pub struct $msgname {
                data: Vec<u8>,
                $( pub $name: $type, )*
            }

            #[allow(dead_code)]
            impl EventTrait for $msgname {
                fn min_length(&self) -> usize { $min }
                fn max_length(&self) -> usize { $max }
                fn event_type(&self) -> EventType { EventType::$msgname }

                fn from_payload(data: Vec<u8>) -> Result<Self> {
                    if data.len() > $max || data.len() < $min || data[0] != $command_type {
                        Err(AppError::InvalidPayload)
                    } else {
                        $( let $name = <$type>::from_data(data[$index..$index + std::mem::size_of::<$type>()].try_into()?)?; )*
                        Ok(Self { data, $( $name, )* })
                    }
                }

                fn into_data(self) -> Vec<u8> {
                    self.data
                }
            }

            impl std::convert::From<$msgname> for Event {
                fn from(val: $msgname) -> Self { Event::$msgname(val) }
            }
        )*
    };
}

// Generate enums and structures for all our command messages
events! {
    GatewayInformation (1; 13..13) {
        protocol_version: u8 [1],
        options: u8 [2],
        device_count: u8 [3],
        device_table_id: u8 [4],
        device_table_crc: u32 [5],
        device_metadata_crc: u32 [9],
    }
    CommandResponse (2; 4..384) { // Not sure what the actual max is, varies by command
        client_command_id: u16 [1],
        status: u8 [3],
    }
    DeviceOnlineStatus (3; 3..200) { // Length depends on number of devices
        device_table_id: u8 [1],
        device_count: u8 [2],
        // device status starts at byte[3], 1 bit per device indicating online/offline
    }
    DeviceLockStatus (4; 8..100) {
        system_lockout_level: u8 [1],
        chassis_info: u8 [2],
        towable_info: u8 [3],
        towable_battery_voltage: u8 [4],
        towable_break_voltage: u8 [5],
        device_table_id: u8 [6],
        device_count: u8 [7],
        // device status starts at byte[8], 1 bit per device indicating lockout
    }
    RelayBasicLatchingStatusType1 (5; 1..100) {
        // TODO
    }
    RelayBasicLatchingStatusType2 (6; 9..9) {
        device_table_id: u8 [1],
        device_index: u8 [2],
        status: u8 [3],
        start_position: u8 [4],
        amp_draw: u16 [5],
        dtc: u16 [7],
    }
    RvStatus (7; 6..6) {
        battery_voltage: FixedU16<U8> [1],
        external_temperature: FixedU16<U8> [3],
        feature_index: u8 [5],
    }
    DimmableLightStatus (8; 1..100) {}
    RgbLightStatus (9; 1..100) {}
    GeneratorGenieStatus (10; 1..100) {}
    HvacStatus (11; 1..100) {}
    TankSensorStatus (12; 2..200) {
        device_table_id: u8 [1],
        // remaining length is repeating: 1 byte device ID, 1 byte percentage
    }
    RelayHBridgeMomentaryStatusType1 (13; 1..100) {}
    RelayHBridgeMomentaryStatusType2 (14; 1..100) {}
    HourMeterStatus (15; 1..100) {
        // TODO
    }
    Leveler4DeviceStatus (16; 1..100) {}
    LevelerConsoleText (17; 1..100) {
        device_table_id: u8 [1],
        device_count: u8 [2],
        // Console text starts at [3]
    }
    Leveler1DeviceStatus (18; 1..100) {}
    Leveler3DeviceStatus (19; 1..100) {}
    DeviceSessionStatus (26; 3..100) {
        device_table_id: u8 [1],
        device_count: u8 [2],
        // session open status at byte[3], 1 bit per device
    }
    RealTimeClock (32; 9..9) {
        seconds_from_epoch: u32 [1],
        time_since_start: u16 [5],
        flags: u8 [8],
    }
    CloudGatewayStatus (33; 1..100) {}
    TemperatureSensorStatus (34; 1..100) {}
    JaycoTbbStatus (35; 1..100) {}
    MonitorPanelStatus (43; 1..100) {}
    AccessoryGatewayStatus (44; 1..100) {}
    AwningSensorStatus (47; 1..100) {}
    BrakingSystemStatus (48; 1..100) {}
    BatteryMonitorStatus (49; 1..100) {}
    DoorLockStatus (51; 1..100) {}
    HostDebug (102; 1..100) {}
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    /// Validates that the unlock process works as expected
    fn parse_payload() -> Result<()> {
        let payload = vec![1u8, 5, 0, 16, 1, 102, 63, 39, 130, 5, 20, 33, 131];
        let event = Event::from_payload(payload)?;
        println!("Event: {:?}", event);
        Ok(())
    }
}
