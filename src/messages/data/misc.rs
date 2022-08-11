use super::*;

macro_rules! define_array_struct {
    ($( $name:ident [ $size:literal ] , )*) => {$(
        #[derive(Debug, Default, Deref)]
        #[allow(dead_code)]
        pub struct $name([u8; $size]);

        impl Encodable for $name {
            fn from_data(data: &[u8]) -> Result<Self> {
                if data.len() < $size {
                    return Err(AppError::InvalidPayload);
                }
                Ok(Self((&data[0..$size]).try_into()?))
            }

            fn to_data(&self) -> Vec<u8> {
                self.0.into()
            }

            fn data_size(&self) -> usize {
                $size
            }
        }
    )*};
}

macro_rules! define_fixed_size_string {
    ($( $name:ident [ $size:literal ] , )*) => {$(
        #[derive(Debug, Default, Deref)]
        #[allow(dead_code)]
        pub struct $name(String);

        impl Encodable for $name {
            fn from_data(data: &[u8]) -> Result<Self> {
                if data.len() < $size {
                    return Err(AppError::InvalidPayload);
                }
                Ok(Self(std::str::from_utf8(&data[0..$size])?.to_string()))
            }

            fn to_data(&self) -> Vec<u8> {
                self.0.as_bytes().to_vec()
            }

            fn data_size(&self) -> usize {
                $size
            }
        }
    )*};
}

define_array_struct! {
    MacAddress[6],
}

define_fixed_size_string! {
    SoftwarePartNumber[8],
}
