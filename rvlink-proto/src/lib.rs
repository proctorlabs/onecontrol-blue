#[macro_use]
extern crate derive_more;

pub mod commands;
pub mod data;
pub mod encoding;
pub mod events;

pub use commands::*;
pub use data::*;
pub use events::*;
use fixed::{types::extra::U8, FixedU16};
use rvlink_common::error::*;

pub trait Encodable: Sized {
    fn from_data(data: &[u8]) -> Result<Self>;
    fn to_data(&self) -> Vec<u8>;
    fn data_size(&self) -> usize;

    fn decode_buffer(data: &[u8]) -> Result<Vec<Self>> {
        let mut index = 0;
        let mut result = vec![];
        while index < data.len() {
            match Self::from_data(&data[index..]) {
                Ok(res) => {
                    index = index + res.data_size();
                    result.push(res);
                }
                Err(_) => break,
            }
        }
        Ok(result)
    }
}

/// Implement Encodable trait for primitive values
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

/// Implement encodable trait for a fixed-size variant-only enum (e.g. an enum with #[repr()])
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

impl Encodable for Vec<u8> {
    fn from_data(data: &[u8]) -> Result<Self> {
        Ok(data.into())
    }

    fn to_data(&self) -> Vec<u8> {
        self.clone()
    }

    fn data_size(&self) -> usize {
        self.len()
    }
}

encodable_primitive! {
    u8:1, u16:2, u32:4, u64:8,
    i8:1, i16:2, i32:4, i64:8,
    FixedU16<U8>:2,
}

encodable_enum! {
    OnOff as u8,
    ParameterID as u16,
    DeviceType as u8,
    ProductID as u16,
    ProtocolType as u8,
    FunctionName as u16,
    CommandType as u8,
}
