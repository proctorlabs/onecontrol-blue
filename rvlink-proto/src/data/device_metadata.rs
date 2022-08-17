use super::*;

define_encodable_struct! {
    DeviceMetadataFull [19] {
        protocol: ProtocolType [0],
        payload_size: u8 [1],
        function_name: FunctionName [2],
        function_instance: u8 [4],
        device_capabilities: u8 [5],
        can_version: u8 [6],
        circuit_number: u32 [7],
        software_part_number: SoftwarePartNumber [11],
    }
    DeviceMetadataBasic [2] {
        protocol: ProtocolType [0],
        payload_size: u8 [1],
    }
}

#[derive(Debug, Default)]
#[allow(dead_code)]
pub enum DeviceMetadata {
    Full(DeviceMetadataFull),
    Basic(DeviceMetadataBasic),
    #[default]
    None,
}

impl Encodable for DeviceMetadata {
    fn from_data(data: &[u8]) -> Result<Self> {
        if data.len() < 2 {
            return Err(AppError::InvalidPayload);
        }
        let protocol = ProtocolType::try_from(data[0])?;
        let payload_size = data[1];
        match (protocol, payload_size) {
            (ProtocolType::Host, 17) | (ProtocolType::Can, _) => {
                if data.len() < 19 {
                    Err(AppError::InvalidPayload)
                } else {
                    Ok(Self::Full(DeviceMetadataFull::from_data(data)?))
                }
            }
            _ => Ok(Self::Basic(DeviceMetadataBasic::from_data(data)?)),
        }
    }

    fn to_data(&self) -> Vec<u8> {
        match self {
            Self::Full(metadata) => metadata.to_data(),
            Self::Basic(metadata) => metadata.to_data(),
            Self::None => vec![0, 0],
        }
    }

    fn data_size(&self) -> usize {
        match self {
            &Self::Full(ref m) => m.data_size(),
            &Self::Basic(ref m) => m.data_size(),
            &Self::None => 2,
        }
    }
}
