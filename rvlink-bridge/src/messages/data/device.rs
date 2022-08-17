use super::*;

#[derive(Debug, Default)]
#[allow(dead_code)]
pub enum Device {
    Full {
        protocol: ProtocolType,  //[0]
        payload_size: u8,        //[1]
        device_type: DeviceType, //[2]
        device_instance: u8,     //[3]
        product_id: ProductID,   //[4]
        mac_address: MacAddress, //[6]
    },
    Basic {
        protocol: ProtocolType, //[0]
        payload_size: u8,       //[1]
    },
    #[default]
    None,
}

impl Encodable for Device {
    fn from_data(data: &[u8]) -> Result<Self> {
        if data.len() < 2 {
            return Err(AppError::InvalidPayload);
        }
        let protocol = ProtocolType::try_from(data[0])?;
        let payload_size = data[1];
        match (protocol, payload_size) {
            (ProtocolType::Host, 10) | (ProtocolType::Can, _) => {
                if data.len() < 12 {
                    Err(AppError::InvalidPayload)
                } else {
                    let device_type = <DeviceType>::from_data(&data[2..])?;
                    let device_instance = <u8>::from_data(&data[3..])?;
                    let product_id = <ProductID>::from_data(&data[4..])?;
                    let mac_address: MacAddress = <MacAddress>::from_data(&data[6..12])?;
                    Ok(Self::Full {
                        protocol,
                        payload_size,
                        device_type,
                        device_instance,
                        product_id,
                        mac_address,
                    })
                }
            }
            _ => Ok(Self::Basic {
                protocol,
                payload_size,
            }),
        }
    }

    fn to_data(&self) -> Vec<u8> {
        match self {
            Device::Full {
                protocol,
                payload_size,
                device_type,
                device_instance,
                product_id,
                mac_address,
            } => {
                let mut res = vec![
                    *protocol as u8,
                    *payload_size,
                    *device_type as u8,
                    *device_instance,
                ];
                // let pidbytes = product_id.to_be_bytes();
                res.append(&mut Vec::from(product_id.to_data()));
                res.append(&mut Vec::from(&mac_address[..]));
                res
            }
            Device::Basic {
                protocol,
                payload_size,
            } => vec![*protocol as u8, *payload_size],
            Device::None => vec![0, 0],
        }
    }

    fn data_size(&self) -> usize {
        match self {
            &Self::Full { .. } => 12,
            &Self::Basic { .. } => 2,
            &Self::None => 2,
        }
    }
}
