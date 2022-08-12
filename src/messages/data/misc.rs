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

#[derive(Debug, Default, Deref)]
#[allow(dead_code)]
pub struct BitFlags(Vec<u8>);

impl Encodable for BitFlags {
    fn from_data(data: &[u8]) -> Result<Self> {
        Ok(Self(<Vec<u8>>::from_data(data)?))
    }

    fn to_data(&self) -> Vec<u8> {
        self.0.to_data()
    }

    fn data_size(&self) -> usize {
        self.0.data_size()
    }
}

#[allow(dead_code)]
impl BitFlags {
    pub fn flag_count(&self) -> usize {
        self.len() * 8
    }

    pub fn get_flag(&self, index: usize) -> Result<bool> {
        if index >= self.flag_count() {
            Err(AppError::IncorrectDataSize)
        } else {
            // Assuming 12 then:
            let byte_index = index / 8; // 1
            let bit_index = index % 8; // 4
            let bit_mask = 1 << (bit_index - 1); // 1 << 3 = b00001000
            Ok((self.0[byte_index] & bit_mask) > 0)
        }
    }

    pub fn to_flags(&self) -> Vec<bool> {
        let mut res = vec![];
        for byte in self.0.iter() {
            for bitshift in 0..8 {
                res.push(((1 << bitshift) & *byte) > 0);
            }
        }
        res
    }
}

define_array_struct! {
    MacAddress[6],
}

define_fixed_size_string! {
    SoftwarePartNumber[8],
}
