mod commands;
mod events;
mod data;

use crate::error::*;
pub use commands::*;
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

encodable_primitive! {
    u8:1, u16:2, u32:4, u64:8,
    i8:1, i16:2, i32:4, i64:8,
}
