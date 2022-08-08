use super::*;
use crate::error::*;

#[derive(Debug, Deref)]
pub struct EventWrapper(pub Vec<u8>);

#[allow(dead_code)]
impl EventWrapper {
    pub fn event_type(&self) -> Result<EventType> {
        Ok(EventType::try_from(self[0])?)
    }
}
