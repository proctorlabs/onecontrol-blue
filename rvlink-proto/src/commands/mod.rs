use super::*;
use rvlink_common::error::*;

pub trait CommandTrait: Sized {
    type ResponseType: CommandResponseTrait;
    fn min_length(&self) -> usize;
    fn max_length(&self) -> usize;
    fn command_type(&self) -> CommandType;
    fn to_payload(&self) -> Result<Vec<u8>>;
    fn set_command_id(&mut self, cmdid: u16);
}

pub trait CommandResponseTrait: Sized {
    fn min_length(&self) -> usize;
    fn max_length(&self) -> usize;
    fn success(&self) -> bool;
    fn complete(&self) -> bool;
    fn from_payload(bytes: Vec<u8>) -> Result<Self>;
}

impl std::convert::From<CommandType> for u8 {
    fn from(val: CommandType) -> Self {
        val as u8
    }
}

impl CommandResponseTrait for Event {
    fn min_length(&self) -> usize {
        0
    }

    fn max_length(&self) -> usize {
        384
    }

    fn success(&self) -> bool {
        false
    }

    fn complete(&self) -> bool {
        true
    }

    fn from_payload(_: Vec<u8>) -> Result<Self> {
        Err(AppError::Generic(
            "Cannot call CommandResponseTrait(from_payload) on Event!".into(),
        ))
    }
}

macro_rules! commands {
    (*RESPONSE $msgrsp:ident ($rspmin:literal .. $rspmax:literal) $success:literal $complete:literal {
        $( $rspname:ident : $rsptype:ty [ $rspindex:literal ] , )*
        $( << $repname:ident : $reptype:ty [ $repindex:literal ] ,)*
    }) => {
        #[derive(Debug, Default)]
        #[allow(dead_code)]
        pub struct $msgrsp {
            data: Vec<u8>,
            pub client_command_id: u16,
            $( pub $rspname: $rsptype, )*
            $( pub $repname: Vec<$reptype>, )*
        }

        #[allow(dead_code)]
        impl CommandResponseTrait for $msgrsp {
            fn min_length(&self) -> usize { $rspmin }
            fn max_length(&self) -> usize { $rspmax }
            fn success(&self) -> bool { $success }
            fn complete(&self) -> bool { $complete }

            fn from_payload(bytes: Vec<u8>) -> Result<Self> {
                if bytes.len() < $rspmin || bytes.len() > $rspmax {
                    return Err(AppError::InvalidPayload);
                }
                let client_command_id = <u16>::from_data(bytes[1..].try_into()?)?;
                $( let $rspname = <$rsptype>::from_data(bytes[$rspindex..].try_into()?)?; )*
                $( let $repname = <$reptype>::decode_buffer(&bytes[$repindex..])?; )*
                let data = bytes;
                Ok(Self{
                    data,
                    client_command_id,
                    $( $rspname, )*
                    $( $repname, )*
                })
            }
        }
    };
    ($(
        $msgname:ident ($command_type:literal ; $min:literal .. $max:literal) {
            $( $name:ident : $type:ty [ $index:literal ] , )*
        } -> $rsp_name:ident :

        + $rsp_suc_name:ident ($rsp_suc_min:literal .. $rsp_suc_max:literal) { $( $rsp_suc_content:tt )* }
        - $rsp_fail_name:ident ($rsp_fail_min:literal .. $rsp_fail_max:literal) { $( $rsp_fail_content:tt )* }
        &+ $rsp_suc_done_name:ident ($rsp_suc_done_min:literal .. $rsp_suc_done_max:literal) { $( $rsp_suc_done_content:tt )* }
        &- $rsp_fail_done_name:ident ($rsp_fail_done_min:literal .. $rsp_fail_done_max:literal) { $( $rsp_fail_done_content:tt )* }
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
            type ResponseType = Event;

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

            fn set_command_id(&mut self, cmdid: u16) {
                match self {
                    $( Command::$msgname(inner) => inner.set_command_id(cmdid), )*
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
                type ResponseType = $rsp_name;
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

                fn set_command_id(&mut self, cmdid: u16) {
                    self.client_command_id = cmdid;
                }
            }

            impl std::convert::From<$msgname> for Command {
                fn from(val: $msgname) -> Self { Command::$msgname(val) }
            }

            #[allow(dead_code)]
            #[derive(Debug)]
            pub enum $rsp_name {
                Success($rsp_suc_name),
                Failure($rsp_fail_name),
                SuccessComplete($rsp_suc_done_name),
                FailureComplete($rsp_fail_done_name),
            }

            #[allow(dead_code)]
            impl CommandResponseTrait for $rsp_name {
                fn min_length(&self) -> usize {
                    match self {
                        Self::Success(r) => r.min_length(),
                        Self::Failure(r) => r.min_length(),
                        Self::SuccessComplete(r) => r.min_length(),
                        Self::FailureComplete(r) => r.min_length(),
                    }
                }

                fn max_length(&self) -> usize {
                    match self {
                        Self::Success(r) => r.max_length(),
                        Self::Failure(r) => r.max_length(),
                        Self::SuccessComplete(r) => r.max_length(),
                        Self::FailureComplete(r) => r.max_length(),
                    }
                }

                fn success(&self) -> bool {
                    match self {
                        Self::Success(r) => r.success(),
                        Self::Failure(r) => r.success(),
                        Self::SuccessComplete(r) => r.success(),
                        Self::FailureComplete(r) => r.success(),
                    }
                }

                fn complete(&self) -> bool{
                    match self {
                        Self::Success(r) => r.complete(),
                        Self::Failure(r) => r.complete(),
                        Self::SuccessComplete(r) => r.complete(),
                        Self::FailureComplete(r) => r.complete(),
                    }
                }

                fn from_payload(bytes: Vec<u8>) -> Result<Self> {
                    let command_status = <u8>::from_data(&[bytes[3]])?;
                    let completed = (command_status & 128) == 128;
                    let success = (command_status & 1) == 1;
                    Ok(match (success, completed) {
                        (false, false) => $rsp_name::Failure($rsp_fail_name::from_payload(bytes)?),
                        (true, false) => $rsp_name::Success($rsp_suc_name::from_payload(bytes)?),
                        (false, true) => $rsp_name::FailureComplete($rsp_fail_done_name::from_payload(bytes)?),
                        (true, true) => $rsp_name::SuccessComplete($rsp_suc_done_name::from_payload(bytes)?),
                    })
                }
            }

            commands! { *RESPONSE $rsp_suc_name       ($rsp_suc_min .. $rsp_suc_max)             true  false { $( $rsp_suc_content       )* } }
            commands! { *RESPONSE $rsp_fail_name      ($rsp_fail_min .. $rsp_fail_max)           false false { $( $rsp_fail_content      )* } }
            commands! { *RESPONSE $rsp_suc_done_name  ($rsp_suc_done_min .. $rsp_suc_done_max)   true  true  { $( $rsp_suc_done_content  )* } }
            commands! { *RESPONSE $rsp_fail_done_name ($rsp_fail_done_min .. $rsp_fail_done_max) false true  { $( $rsp_fail_done_content )* } }
        )*
    };
}

// Generate enums and structures for all our command messages
commands! {
    GetDevices (1; 6..6) {
        device_table_id: u8 [3],
        start_device_id: u8 [4],
        max_device_request_count: u8 [5],
    } -> GetDevicesResponse:
    + GetDevicesResponseSuccess (7..384) {
        device_table_id: u8 [4],
        start_device_id: u8 [5],
        device_count: u8 [6],
        << devices: Device [7],
    }
    - GetDevicesResponseFailure (4..5) {}
    &+ GetDevicesResponseSuccessCompleted (9..9) {
        device_table_crc: u32 [4],
        device_count: u8 [8],
    }
    &- GetDevicesResponseFailureCompleted (4..4) {}

    GetDevicesMetadata (2; 6..6) {
        device_table_id: u8 [3],
        start_device_id: u8 [4],
        max_device_request_count: u8 [5],
    } -> GetDevicesMetadataResponse:
    + GetDevicesMetadataResponseSuccess (7..384) {
        device_table_id: u8 [4],
        start_device_id: u8 [5],
        device_count: u8 [6],
        << devices: DeviceMetadata [7],
    }
    - GetDevicesMetadataResponseFailure (4..5) {}
    &+ GetDevicesMetadataResponseSuccessCompleted (9..9) {
        device_metadata_table_crc: u32 [4],
        device_count: u8 [8],
    }
    &- GetDevicesMetadataResponseFailureCompleted (4..4) {}

    RemoveOfflineDevices (3; 5..5) {
        device_table_id: u8 [3],
        device_options: u8 [4],
    } -> RemoveOfflineDevicesResponse:
    + RemoveOfflineDevicesResponseSuccess (4..4) {}
    - RemoveOfflineDevicesResponseFailure (4..4) {}
    &+ RemoveOfflineDevicesResponseSuccessCompleted (4..4) {}
    &- RemoveOfflineDevicesResponseFailureCompleted (4..4) {}

    RenameDevice (4; 12..12) {
        device_table_id: u8 [3],
        device_id: u8 [4],
        to_function_name: u16 [5],
        to_function_name_session: u16 [7],
        to_function_instance: u8 [9],
        to_function_instance_session: u16 [10],
    } -> RenameDeviceResponse:
    + RenameDeviceResponseSuccess (4..4) {}
    - RenameDeviceResponseFailure (4..4) {}
    &+ RenameDeviceResponseSuccessCompleted (4..4) {}
    &- RenameDeviceResponseFailureCompleted (4..4) {}

    SetRealTimeClock (5; 10..10) {
        month: u8 [3],
        day: u8 [4],
        year: u16 [5],
        hour: u8 [7],
        minutes: u8 [8],
        seconds: u8 [9],
    } -> SetRealTimeClockResponse:
    + SetRealTimeClockResponseSuccess (4..4) {}
    - SetRealTimeClockResponseFailure (4..4) {}
    &+ SetRealTimeClockResponseSuccessCompleted (4..4) {}
    &- SetRealTimeClockResponseFailureCompleted (4..4) {}

    GetProductDtcValues (16; 10..10) {
        device_table_id: u8 [3],
        device_id: u8 [4],
        option: u8 [5],
        start_dtc: u16 [6],
        end_dtc: u16 [8],
    } -> GetProductDtcValuesResponse:
    + GetProductDtcValuesResponseSuccess (4..4) {}
    - GetProductDtcValuesResponseFailure (4..4) {}
    &+ GetProductDtcValuesResponseSuccessCompleted (4..4) {}
    &- GetProductDtcValuesResponseFailureCompleted (4..4) {}

    GetDevicePidList (17; 9..9) {
        device_table_id: u8 [3],
        device_id: u8 [4],
        start_index: u16 [5],
        end_index: u16 [7],
    } -> GetDevicePidListResponse:
    + GetDevicePidListResponseSuccess (4..4) {}
    - GetDevicePidListResponseFailure (4..4) {}
    &+ GetDevicePidListResponseSuccessCompleted (4..4) {}
    &- GetDevicePidListResponseFailureCompleted (4..4) {}

    GetDevicePid (18; 7..7) {
        device_table_id: u8 [3],
        device_id: u8 [4],
        pid: ParameterID [5],
    } -> GetDevicePidResponse:
    + GetDevicePidResponseSuccess (4..4) {}
    - GetDevicePidResponseFailure (4..4) {}
    &+ GetDevicePidResponseSuccessCompleted (4..4) {}
    &- GetDevicePidResponseFailureCompleted (4..4) {}

    SetDevicePid (19; 9..15) {
        device_table_id: u8 [3],
        device_id: u8 [4],
        pid: ParameterID [5],
        session_id: u16 [7],
        // Value @ 9, variable size 0-6
    } -> SetDevicePidResponse:
    + SetDevicePidResponseSuccess (4..4) {}
    - SetDevicePidResponseFailure (4..4) {}
    &+ SetDevicePidResponseSuccessCompleted (4..4) {}
    &- SetDevicePidResponseFailureCompleted (4..4) {}

    GetDevicePidWithAddress (20; 9..9) {
        device_table_id: u8 [3],
        device_id: u8 [4],
        pid: ParameterID [5],
        pid_address: u16 [7],
    } -> GetDevicePidWithAddressResponse:
    + GetDevicePidWithAddressResponseSuccess (4..4) {}
    - GetDevicePidWithAddressResponseFailure (4..4) {}
    &+ GetDevicePidWithAddressResponseSuccessCompleted (4..4) {}
    &- GetDevicePidWithAddressResponseFailureCompleted (4..4) {}

    SetDevicePidWithAddress (21; 11..15) {
        device_table_id: u8 [3],
        device_id: u8 [4],
        pid: ParameterID [5],
        session_id: u16 [7],
        pid_address: u16 [9],
        // Value @ 11, variable size 0-4
    } -> SetDevicePidWithAddressResponse:
    + SetDevicePidWithAddressResponseSuccess (4..4) {}
    - SetDevicePidWithAddressResponseFailure (4..4) {}
    &+ SetDevicePidWithAddressResponseSuccessCompleted (4..4) {}
    &- SetDevicePidWithAddressResponseFailureCompleted (4..4) {}

    SoftwareUpdateAuthorization (35; 5..5) {
        device_table_id: u8 [3],
        device_id: u8 [4],
    } -> SoftwareUpdateAuthorizationResponse:
    + SoftwareUpdateAuthorizationResponseSuccess (4..4) {}
    - SoftwareUpdateAuthorizationResponseFailure (4..4) {}
    &+ SoftwareUpdateAuthorizationResponseSuccessCompleted (4..4) {}
    &- SoftwareUpdateAuthorizationResponseFailureCompleted (4..4) {}

    GetDeviceBlockList (48; 5..5) {
        device_table_id: u8 [3],
        device_id: u8 [4],
    } -> GetDeviceBlockListResponse:
    + GetDeviceBlockListResponseSuccess (4..4) {}
    - GetDeviceBlockListResponseFailure (4..4) {}
    &+ GetDeviceBlockListResponseSuccessCompleted (4..4) {}
    &- GetDeviceBlockListResponseFailureCompleted (4..4) {}

    GetDeviceBlockProperties (49; 8..8) {
        device_table_id: u8 [3],
        device_id: u8 [4],
        block_id_start: u8 [5],
        property: u8 [7],
    } -> GetDeviceBlockPropertiesResponse:
    + GetDeviceBlockPropertiesResponseSuccess (4..4) {}
    - GetDeviceBlockPropertiesResponseFailure (4..4) {}
    &+ GetDeviceBlockPropertiesResponseSuccessCompleted (4..4) {}
    &- GetDeviceBlockPropertiesResponseFailureCompleted (4..4) {}

    StartDeviceBlockTransfer (50; 8..16) {
        device_table_id: u8 [3],
        device_id: u8 [4],
        block_id_start: u16 [5],
        options: u8 [7],
        start_address: u32 [8],
        size: u32 [12],
    } -> StartDeviceBlockTransferResponse:
    + StartDeviceBlockTransferResponseSuccess (4..4) {}
    - StartDeviceBlockTransferResponseFailure (4..4) {}
    &+ StartDeviceBlockTransferResponseSuccessCompleted (4..4) {}
    &- StartDeviceBlockTransferResponseFailureCompleted (4..4) {}

    DeviceBlockWriteData (51; 12..140) {
        device_table_id: u8 [3],
        device_id: u8 [4],
        block_id_start: u16 [5],
        address_offset: u32 [7],
        size: u8 [11],
        // data @ 12 -> finish
    } -> DeviceBlockWriteDataResponse:
    + DeviceBlockWriteDataResponseSuccess (4..4) {}
    - DeviceBlockWriteDataResponseFailure (4..4) {}
    &+ DeviceBlockWriteDataResponseSuccessCompleted (4..4) {}
    &- DeviceBlockWriteDataResponseFailureCompleted (4..4) {}

    StopDeviceBlockTransfer (52; 8..8) {
        device_table_id: u8 [3],
        device_id: u8 [4],
        block_id_start: u16 [5],
        options: u8 [7],
    } -> StopDeviceBlockTransferResponse:
    + StopDeviceBlockTransferResponseSuccess (4..4) {}
    - StopDeviceBlockTransferResponseFailure (4..4) {}
    &+ StopDeviceBlockTransferResponseSuccessCompleted (4..4) {}
    &- StopDeviceBlockTransferResponseFailureCompleted (4..4) {}

    ActionSwitch (64; 5..5) {
        device_table_id: u8 [3], //1
        device_state: OnOff [4],
        first_device_id: u8 [5], //7
    } -> ActionSwitchResponse:
    + ActionSwitchResponseSuccess (4..4) {}
    - ActionSwitchResponseFailure (4..4) {}
    &+ ActionSwitchResponseSuccessCompleted (4..4) {}
    &- ActionSwitchResponseFailureCompleted (4..4) {}

    ActionMovement (65; 6..6) {
        device_table_id: u8 [3],
        device_id: u8 [4],
        device_state: u8 [5],
    } -> ActionMovementResponse:
    + ActionMovementResponseSuccess (4..4) {}
    - ActionMovementResponseFailure (4..4) {}
    &+ ActionMovementResponseSuccessCompleted (4..4) {}
    &- ActionMovementResponseFailureCompleted (4..4) {}

    ActionGeneratorGenie (66; 6..6) {
        device_table_id: u8 [3],
        device_id: u8 [4],
        device_command: u8 [5],
    } -> ActionGeneratorGenieResponse:
    + ActionGeneratorGenieResponseSuccess (4..4) {}
    - ActionGeneratorGenieResponseFailure (4..4) {}
    &+ ActionGeneratorGenieResponseSuccessCompleted (4..4) {}
    &- ActionGeneratorGenieResponseFailureCompleted (4..4) {}

    ActionDimmable (67; 6..12) {
        device_table_id: u8 [3],
        device_id: u8 [4],
        device_command: u8 [5],
    } -> ActionDimmableResponse:
    + ActionDimmableResponseSuccess (4..4) {}
    - ActionDimmableResponseFailure (4..4) {}
    &+ ActionDimmableResponseSuccessCompleted (4..4) {}
    &- ActionDimmableResponseFailureCompleted (4..4) {}

    ActionRgb (68; 6..12) {
        device_table_id: u8 [3],
        device_id: u8 [4],
        device_command: u8 [5],
        // Alternate modes to be enumed
    } -> ActionRgbResponse:
    + ActionRgbResponseSuccess (4..4) {}
    - ActionRgbResponseFailure (4..4) {}
    &+ ActionRgbResponseSuccessCompleted (4..4) {}
    &- ActionRgbResponseFailureCompleted (4..4) {}

    ActionHvac (69; 8..8) {
        device_table_id: u8 [3],
        device_id: u8 [4],
        device_command: u8 [5],
        // ?
    } -> ActionHvacResponse:
    + ActionHvacResponseSuccess (4..4) {}
    - ActionHvacResponseFailure (4..4) {}
    &+ ActionHvacResponseSuccessCompleted (4..4) {}
    &- ActionHvacResponseFailureCompleted (4..4) {}

    ActionAccessoryGateway (70; 6..6) {
        device_table_id: u8 [3],
        device_id: u8 [4],
        device_command: u8 [5],
    } -> ActionAccessoryGatewayResponse:
    + ActionAccessoryGatewayResponseSuccess (4..4) {}
    - ActionAccessoryGatewayResponseFailure (4..4) {}
    &+ ActionAccessoryGatewayResponseSuccessCompleted (4..4) {}
    &- ActionAccessoryGatewayResponseFailureCompleted (4..4) {}

    Leveler4ButtonCommand (80; 10..10) {
        device_table_id: u8 [3],
        device_id: u8 [4],
        device_mode: u8 [5],
        ui_mode: u8 [6],
        ui_button_data_1: u8 [7],
        ui_button_data_2: u8 [8],
        ui_button_data_3: u8 [9],
    } -> Leveler4ButtonCommandResponse:
    + Leveler4ButtonCommandResponseSuccess (4..4) {}
    - Leveler4ButtonCommandResponseFailure (4..4) {}
    &+ Leveler4ButtonCommandResponseSuccessCompleted (4..4) {}
    &- Leveler4ButtonCommandResponseFailureCompleted (4..4) {}

    Leveler1ButtonCommand (82; 7..7) {
        device_table_id: u8 [3],
        device_id: u8 [4],
        button_state_1: u8 [5],
        button_state_2: u8 [6],
    } -> Leveler1ButtonCommandResponse:
    + Leveler1ButtonCommandResponseSuccess (4..4) {}
    - Leveler1ButtonCommandResponseFailure (4..4) {}
    &+ Leveler1ButtonCommandResponseSuccessCompleted (4..4) {}
    &- Leveler1ButtonCommandResponseFailureCompleted (4..4) {}

    Leveler3ButtonCommand (83; 8..8) {
        device_table_id: u8 [3],
        device_id: u8 [4],
        screen_enum: u8 [5],
        button_state_1: u8 [6],
        button_state_2: u8 [7],
    } -> Leveler3ButtonCommandResponse:
    + Leveler3ButtonCommandResponseSuccess (4..4) {}
    - Leveler3ButtonCommandResponseFailure (4..4) {}
    &+ Leveler3ButtonCommandResponseSuccessCompleted (4..4) {}
    &- Leveler3ButtonCommandResponseFailureCompleted (4..4) {}

    GetFirmwareInformation (96; 4..4) {
        firmware_information_code: u8 [3],
    } -> GetFirmwareInformationResponse:
    + GetFirmwareInformationResponseSuccess (4..4) {}
    - GetFirmwareInformationResponseFailure (4..4) {}
    &+ GetFirmwareInformationResponseSuccessCompleted (4..4) {}
    &- GetFirmwareInformationResponseFailureCompleted (4..4) {}

    Diagnostics (102; 9..9) {
        diagnostic_command_type: u8 [3],
        diagnostic_command_state: u8 [4],
        diagnostic_event_type: u8 [5],
        diagnostic_event_state: u8 [6],
        diagnostic_host_value: u8 [7],
        diagnostic_device_link_id: u8 [8],
    } -> DiagnosticsResponse:
    + DiagnosticsResponseSuccess (4..4) {}
    - DiagnosticsResponseFailure (4..4) {}
    &+ DiagnosticsResponseSuccessCompleted (4..4) {}
    &- DiagnosticsResponseFailureCompleted (4..4) {}
}
