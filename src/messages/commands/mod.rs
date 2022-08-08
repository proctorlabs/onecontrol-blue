use super::*;
use crate::error::*;

pub trait CommandTrait: Sized {
    fn min_length(&self) -> usize;
    fn max_length(&self) -> usize;
    fn command_type(&self) -> CommandType;

    fn to_payload(&self) -> Result<Vec<u8>>;
    fn from_payload(bytes: &[u8]) -> Result<Self>;
}

impl std::convert::From<CommandType> for u8 {
    fn from(val: CommandType) -> Self {
        val as u8
    }
}

impl Encodable for CommandType {
    fn from_data(data: &[u8]) -> Result<Self> {
        data[0].try_into()
    }

    fn to_data(&self) -> Vec<u8> {
        vec![*self as u8]
    }

    fn data_size(&self) -> usize {
        1
    }
}

macro_rules! commands {
    ($(
        $msgname:ident ($command_type:literal ; $min:literal .. $max:literal) {
            $( $name:ident : $type:ty [ $index:literal ] , )
        *}
    )*) => {
        #[allow(dead_code)]
        #[derive(Debug, Clone, Copy)]
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

            fn to_payload(&self) -> Result<Vec<u8>> {
                match &self {
                    $( Command::$msgname(inner) => inner.to_payload(), )*
                }
            }

            fn from_payload(bytes: &[u8]) -> Result<Self> {
                if bytes.len() < 3 {
                    return Err(AppError::InvalidPayload);
                }
                match bytes[2].try_into()? {
                    $( CommandType::$msgname => Ok(Command::$msgname($msgname::from_payload(bytes)?)), )*
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

                fn to_payload(&self) -> Result<Vec<u8>> {
                    let mut res = vec![];
                    {
                        res.append(&mut self.client_command_id.to_data());
                        res.append(&mut CommandType::$msgname.to_data());
                    }
                    $({
                        res.append(&mut self.$name.to_data());
                    })*
                    if res.len() > $max || res.len() < $min {
                        Err(AppError::InvalidPayload)
                    } else {
                        Ok(res)
                    }
                }

                fn from_payload(bytes: &[u8]) -> Result<Self> {
                    if bytes.len() > $max || bytes.len() < $min || bytes[2] != $command_type {
                        Err(AppError::InvalidPayload)
                    } else {
                        let mut res = Self::default();
                        res.client_command_id = <u16>::from_data(bytes)?;
                        $({
                            res.$name = <$type>::from_data(bytes[$index..$index + std::mem::size_of::<$type>()].try_into()?)?;
                        })*
                        // todo!();
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
    GetDevices (1; 6..6) {
        device_table_id: u8 [3],
        start_device_id: u8 [4],
        max_device_request_count: u8 [5],
    }
    GetDevicesMetadata (2; 6..6) {
        device_table_id: u8 [3],
        start_device_id: u8 [4],
        max_device_request_count: u8 [5],
    }
    RemoveOfflineDevices (3; 5..5) {
        device_table_id: u8 [3],
        device_options: u8 [4],
    }
    RenameDevice (4; 12..12) {
        device_table_id: u8 [3],
        device_id: u8 [4],
        to_function_name: u16 [5],
        to_function_name_session: u16 [7],
        to_function_instance: u8 [9],
        to_function_instance_session: u16 [10],
    }
    SetRealTimeClock (5; 10..10) {
        month: u8 [3],
        day: u8 [4],
        year: u16 [5],
        hour: u8 [7],
        minutes: u8 [8],
        seconds: u8 [9],
    }
    GetProductDtcValues (16; 10..10) {
        device_table_id: u8 [3],
        device_id: u8 [4],
        option: u8 [5],
        start_dtc: u16 [6],
        end_dtc: u16 [8],
    }
    GetDevicePidList (17; 9..9) {
        device_table_id: u8 [3],
        device_id: u8 [4],
        start_index: u16 [5],
        end_index: u16 [7],
    }
    GetDevicePid (18; 7..7) {
        device_table_id: u8 [3],
        device_id: u8 [4],
        pid: ParameterID [5],
    }
    SetDevicePid (19; 9..15) {
        device_table_id: u8 [3],
        device_id: u8 [4],
        pid: ParameterID [5],
        session_id: u16 [7],
        // Value @ 9, variable size 0-6
    }
    GetDevicePidWithAddress (20; 9..9) {
        device_table_id: u8 [3],
        device_id: u8 [4],
        pid: ParameterID [5],
        pid_address: u16 [7],
    }
    SetDevicePidWithAddress (21; 11..15) {
        device_table_id: u8 [3],
        device_id: u8 [4],
        pid: ParameterID [5],
        session_id: u16 [7],
        pid_address: u16 [9],
        // Value @ 11, variable size 0-4
    }
    SoftwareUpdateAuthorization (35; 5..5) {
        device_table_id: u8 [3],
        device_id: u8 [4],
    }
    GetDeviceBlockList (48; 5..5) {
        device_table_id: u8 [3],
        device_id: u8 [4],
    }
    GetDeviceBlockProperties (49; 8..8) {
        device_table_id: u8 [3],
        device_id: u8 [4],
        block_id_start: u8 [5],
        property: u8 [7],
    }
    StartDeviceBlockTransfer (50; 8..16) {
        device_table_id: u8 [3],
        device_id: u8 [4],
        block_id_start: u16 [5],
        options: u8 [7],
        start_address: u32 [8],
        size: u32 [12],
    }
    DeviceBlockWriteData (51; 12..140) {
        device_table_id: u8 [3],
        device_id: u8 [4],
        block_id_start: u16 [5],
        address_offset: u32 [7],
        size: u8 [11],
        // data @ 12 -> finish
    }
    StopDeviceBlockTransfer (52; 8..8) {
        device_table_id: u8 [3],
        device_id: u8 [4],
        block_id_start: u16 [5],
        options: u8 [7],
    }
    ActionSwitch (64; 5..5) {
        device_table_id: u8 [3], //1
        device_state: OnOff [4],
        first_device_id: u8 [5], //7
    }
    ActionMovement (65; 6..6) {
        device_table_id: u8 [3],
        device_id: u8 [4],
        device_state: u8 [5],
    }
    ActionGeneratorGenie (66; 6..6) {
        device_table_id: u8 [3],
        device_id: u8 [4],
        device_command: u8 [5],
    }
    ActionDimmable (67; 6..12) {
        device_table_id: u8 [3],
        device_id: u8 [4],
        device_command: u8 [5],
    }
    ActionRgb (68; 6..12) {
        device_table_id: u8 [3],
        device_id: u8 [4],
        device_command: u8 [5],
        // Alternate modes to be enumed
    }
    ActionHvac (69; 8..8) {
        device_table_id: u8 [3],
        device_id: u8 [4],
        device_command: u8 [5],
        // ?
    }
    ActionAccessoryGateway (70; 6..6) {
        device_table_id: u8 [3],
        device_id: u8 [4],
        device_command: u8 [5],
    }
    Leveler4ButtonCommand (80; 10..10) {
        device_table_id: u8 [3],
        device_id: u8 [4],
        device_mode: u8 [5],
        ui_mode: u8 [6],
        ui_button_data_1: u8 [7],
        ui_button_data_2: u8 [8],
        ui_button_data_3: u8 [9],
    }
    Leveler1ButtonCommand (82; 7..7) {
        device_table_id: u8 [3],
        device_id: u8 [4],
        button_state_1: u8 [5],
        button_state_2: u8 [6],
    }
    Leveler3ButtonCommand (83; 8..8) {
        device_table_id: u8 [3],
        device_id: u8 [4],
        screen_enum: u8 [5],
        button_state_1: u8 [6],
        button_state_2: u8 [7],
    }
    GetFirmwareInformation (96; 4..4) {
        firmware_information_code: u8 [3],
    }
    Diagnostics (102; 9..9) {
        diagnostic_command_type: u8 [3],
        diagnostic_command_state: u8 [4],
        diagnostic_event_type: u8 [5],
        diagnostic_event_state: u8 [6],
        diagnostic_host_value: u8 [7],
        diagnostic_device_link_id: u8 [8],
    }
}
