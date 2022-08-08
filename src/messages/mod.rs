mod commands;
mod data;
mod events;

use crate::error::*;
pub use commands::*;
pub use data::*;
pub use events::*;

pub trait Encodable: Sized {
    fn from_data(data: &[u8]) -> Result<Self>;
    fn to_data(&self) -> Vec<u8>;
    fn data_size(&self) -> usize;
}

macro_rules! encodable_primitive {
    ($( $type:ty : $size:literal ,)*) => {$(
        impl Encodable for $type {
            fn from_data(data: &[u8]) -> Result<Self> {
                if data.len() < $size {
                    Err(AppError::IncorrectDataSize)
                } else {
                    Ok(<$type>::from_be_bytes(data[0..$size].try_into()?))
                }
            }

            fn to_data(&self) -> Vec<u8> {
                self.to_be_bytes().into()
            }

            fn data_size(&self) -> usize {
                $size
            }
        }
    )*}
}

macro_rules! encodable_enum {
    ($( $type:ty as $astype:ty ,)*) => {$(
        impl Encodable for $type {
            fn from_data(data: &[u8]) -> Result<Self> {
                Ok(Self::try_from(<$astype>::from_data(data)?)?)
            }

            fn to_data(&self) -> Vec<u8> {
                (*self as $astype).to_data()
            }

            fn data_size(&self) -> usize {
                (*self as $astype).data_size()
            }
        }
    )*}
}

encodable_primitive! {
    u8:1, u16:2, u32:4, u64:8,
    i8:1, i16:2, i32:4, i64:8,
}

encodable_enum! {
    OnOff as u8,
    ParameterID as u16,
    EventType as u8,
    DeviceType as u8,
}
