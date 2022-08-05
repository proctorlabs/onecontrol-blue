use super::{EventType, EventWrapper};
use crate::error::*;

#[allow(dead_code)]
#[derive(Debug, PartialEq, Clone, Copy)]
#[repr(u8)]
pub enum CommandResult {
    Success = 1,
    Failure = 2,
}

#[allow(dead_code)]
impl CommandResult {
    pub const MASK: u8 = 0x7F;
}

impl TryFrom<u8> for CommandResult {
    type Error = AppError;

    fn try_from(b: u8) -> Result<Self> {
        let b = b & Self::MASK;
        if b == (Self::Success as u8) {
            Ok(Self::Success)
        } else if b == Self::Failure as u8 {
            Ok(Self::Failure)
        } else {
            Err(AppError::InvalidPayload)
        }
    }
}

#[allow(dead_code)]
#[derive(Debug, PartialEq, Clone, Copy)]
#[repr(u8)]
pub enum CommandStatus {
    Incomplete = 0,
    Complete = 128,
}

#[allow(dead_code)]
impl CommandStatus {
    pub const MASK: u8 = 0x80;
}

impl TryFrom<u8> for CommandStatus {
    type Error = AppError;

    fn try_from(b: u8) -> Result<Self> {
        if (b & Self::MASK) == (Self::Complete as u8) {
            Ok(Self::Complete)
        } else {
            Ok(Self::Incomplete)
        }
    }
}

#[derive(Debug, Deref)]
pub struct CommandEvent(pub Vec<u8>);

impl TryFrom<EventWrapper> for CommandEvent {
    type Error = AppError;

    fn try_from(event: EventWrapper) -> Result<Self> {
        if (event.event_type()? != EventType::DeviceCommand) || (event.len() < 4) {
            Err(AppError::InvalidPayload)
        } else {
            Ok(Self(event.0))
        }
    }
}

#[allow(dead_code)]
impl CommandEvent {
    pub fn client_command_id(&self) -> Result<u16> {
        Ok(u16::from_be_bytes(self[1..2].try_into()?))
    }

    pub fn command_status(&self) -> Result<CommandStatus> {
        CommandStatus::try_from(self[3])
    }

    pub fn command_event_result(&self) -> Result<CommandResult> {
        CommandResult::try_from(self[3])
    }

    pub fn command_data(&self) -> Option<&[u8]> {
        if self.len() > 4 {
            Some(&self[4..])
        } else {
            None
        }
    }
}
