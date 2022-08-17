use super::Encodable;
use crate::error::*;
use num_enum::{IntoPrimitive, TryFromPrimitive};

// pub use dtc_id::*;
pub use device::*;
pub use device_metadata::*;
pub use device_type::*;
pub use function_name::*;
pub use misc::*;
pub use param_id::*;
pub use product_id::*;
pub use protocol_type::*;
pub use states::*;

#[macro_export]
macro_rules! define_encodable_struct {
    ($( $name:ident [ $struct_size:literal ] {$(
        $fieldname:ident : $fieldtype:ty [ $fieldindex:literal ],
    )*} )*) => {$(
        #[derive(Debug, Default)]
        #[allow(dead_code)]
        pub struct $name {
            $( pub $fieldname: $fieldtype, )*
        }

        impl crate::messages::Encodable for $name {
            fn from_data(data: &[u8]) -> Result<Self> {
                if data.len() < $struct_size {
                    return Err(AppError::InvalidPayload);
                }
                $( let $fieldname: $fieldtype = <$fieldtype>::from_data(&data[$fieldindex..])?; )*
                Ok(Self{$( $fieldname, )*})
            }

            fn to_data(&self) -> Vec<u8> {
                let mut res = Vec::from([0u8; $struct_size]);
                $( for (i, val) in self.$fieldname.to_data().into_iter().enumerate() {
                    res[$fieldindex + i] = val;
                } )*
                res
            }

            fn data_size(&self) -> usize {
                $struct_size
            }
        }
    )*};
}

macro_rules! enum_with_metadata {
    ($(
        $name:ident: $repr:ty; $( $selector:ident : $selector_type:ty : $selector_index:tt ),* {$(
            $variant:ident = $index:literal { $( $val:literal ),* },
    )*}
    )*) => {$(
        #[allow(dead_code)]
        #[derive(Default, Debug, PartialEq, Clone, Copy, TryFromPrimitive, IntoPrimitive)]
        #[repr($repr)]
        pub enum $name {
            #[default]
            $( $variant = $index , )*
        }

        #[allow(dead_code)]
        impl $name {
            fn _variant_metadata(&self) -> ($( $selector_type , )*) {
                match self {$(
                    Self::$variant => ( $( $val.into() , )* ),
                )*}
            }

            $( pub fn $selector(&self) -> $selector_type {self._variant_metadata().$selector_index} )*
        }
    )*};
}

// mod dtc_id;
mod device;
mod device_metadata;
mod device_type;
mod function_name;
mod misc;
mod param_id;
mod product_id;
mod protocol_type;
mod states;
