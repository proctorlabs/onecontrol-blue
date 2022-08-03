use crate::crc;
use crate::error::*;
use cobs;

pub trait CommandTrait {
    fn min_length(&self) -> usize;
    fn max_length(&self) -> usize;
    fn command_type(&self) -> CommandType;

    fn payload(&self) -> Result<Vec<u8>>;

    fn encode(&self) -> Result<Vec<u8>> {
        let mut payload = self.payload()?;
        let crcval = crc::calc(&payload);
        payload.push(crcval);
        Ok(cobs::encode_vec(&payload))
    }
}

impl std::convert::From<CommandType> for u8 {
    fn from(val: CommandType) -> Self { val as u8 }
}

macro_rules! commands {
    ($(
        $msgname:ident ($command_type:literal ; $min:literal .. $max:literal) {
            $( $name:ident : $type:ty [ $index:literal ] , )
        *}
    )*) => {
        #[allow(dead_code)]
        #[derive(Debug)]
        #[repr(u8)]
        pub enum CommandType {$(
            $msgname = $command_type,
        )*}

        impl std::convert::TryFrom<u8> for CommandType {
            type Error = AppError;

            fn try_from(val: u8) -> Result<Self> {
                match val {
                    $( $command_type => Ok(CommandType::$msgname), )*
                    v => Err(AppError::InvalidCommand(v)),
                }
            }
        }

        #[allow(dead_code)]
        #[derive(Debug)]
        pub enum Command {$(
            $msgname($msgname),
        )*}

        impl CommandTrait for Command {
            fn min_length(&self) -> usize {
                match &self {
                    $( Command::$msgname(inner) => inner.min_length(), )*
                }
            }

            fn max_length(&self) -> usize {
                match &self {
                    $( Command::$msgname(inner) => inner.max_length(), )*
                }
            }

            fn command_type(&self) -> CommandType {
                match &self {
                    $( Command::$msgname(inner) => inner.command_type(), )*
                }
            }

            fn payload(&self) -> Result<Vec<u8>> {
                match &self {
                    $( Command::$msgname(inner) => inner.payload(), )*
                }
            }
        }

        $(
            #[derive(Debug, Default)]
            pub struct $msgname {
                pub client_command_id: u16,
                $( pub $name: $type, )*
            }

            #[allow(dead_code)]
            impl CommandTrait for $msgname {
                fn min_length(&self) -> usize { $min }
                fn max_length(&self) -> usize { $max }
                fn command_type(&self) -> CommandType { CommandType::$msgname }

                fn payload(&self) -> Result<Vec<u8>> {
                    let mut res = vec![0; $max];
                    {
                        let srcbytes = self.client_command_id.to_be_bytes();
                        res[0..2].clone_from_slice(&srcbytes);
                        res[2] = CommandType::$msgname.into();
                    }
                    $({
                        let srcbytes = self.$name.to_be_bytes();
                        res[$index..$index + srcbytes.len()].clone_from_slice(&srcbytes);
                    })*
                    if res.len() > $max || res.len() < $min {
                        Err(AppError::InvalidPayload)
                    } else {
                        Ok(res)
                    }
                }
            }

            impl std::convert::From<$msgname> for Command {
                fn from(val: $msgname) -> Self { Command::$msgname(val) }
            }
        )*
    };
}

// Generate enums and structures for all our command messages
commands! {
    GetDevices (1; 6..6) { //done
        device_table_id: u8 [3],
        start_device_id: u8 [4],
        max_device_request_count: u8 [5],
    }
    GetDevicesMetadata (2; 6..6) { //done
        device_table_id: u8 [3],
        start_device_id: u8 [4],
        max_device_request_count: u8 [5],
    }
    RemoveOfflineDevices (3; 5..5) {}
    RenameDevice (4; 12..12) {}
    SetRealTimeClock (5; 10..10) { //done
        month: u8 [3],
        day: u8 [4],
        year: u16 [5],
        hour: u8 [7],
        minutes: u8 [8],
        seconds: u8 [9],
    }
    GetProductDtcValues (16; 10..10) {}
    GetDevicePidList (17; 9..9) {}
    GetDevicePid (18; 7..7) {}
    SetDevicePid (19; 9..15) {}
    GetDevicePidWithAddress (20; 9..9) {}
    SetDevicePidWithAddress (21; 11..15) {}
    SoftwareUpdateAuthorization (35; 5..5) { //done
        device_table_id: u8 [3],
        device_id: u8 [4],
    }
    GetDeviceBlockList (48; 5..5) {}
    GetDeviceBlockProperties (49; 8..8) {}
    StartDeviceBlockTransfer (50; 8..16) {}
    DeviceBlockWriteData (51; 12..140) {}
    StopDeviceBlockTransfer (52; 8..8) {}
    ActionSwitch (64; 5..5) { //done
        device_table_id: u8 [3],
        device_id: u8 [4],
        first_device_id: u8 [5],
    }
    ActionMovement (65; 6..6) { //done
        device_table_id: u8 [3],
        device_id: u8 [4],
        device_state: u8 [5],
    }
    ActionGeneratorGenie (66; 6..6) { //done
        device_table_id: u8 [3],
        device_id: u8 [4],
        device_command: u8 [5],
    }
    ActionDimmable (67; 6..12) { //done
        device_table_id: u8 [3],
        device_id: u8 [4],
        device_command: u8 [5],
    }
    ActionRgb (68; 6..12) {}
    ActionHvac (69; 8..8) {}
    ActionAccessoryGateway (70; 6..6) { //done
        device_table_id: u8 [3],
        device_id: u8 [4],
        device_command: u8 [5],
    }
    Leveler4ButtonCommand (80; 10..10) {}
    Leveler1ButtonCommand (82; 7..7) {}
    Leveler3ButtonCommand (83; 8..8) {}
    GetFirmwareInformation (96; 4..4) { //done
        firmware_information_code: u8 [3],
    }
    Diagnostics (102; 9..9) {}
}
