use super::CRC8;
use crate::error::*;

#[allow(dead_code)]
pub struct COBS;

#[allow(dead_code)]
impl COBS {
    const LIMIT: usize = 381;
    const FRAME_DELIMITER: u8 = 0x00;
    const CRC_SIZE: usize = 1;
    const FRAME_DELIM_SIZE: usize = 1;
    const DATA_BIT_COUNT: usize = 6;
    const FRAME_BYTE_COUNT_LSB: u8 = 1 << Self::DATA_BIT_COUNT;
    const MAX_DATA_BYTES: u8 = Self::FRAME_BYTE_COUNT_LSB - 1;
    const MAX_COMPRESSED_FRAME_BYTES: u8 = 255 - Self::MAX_DATA_BYTES;

    pub fn encode(input: &[u8]) -> Result<Vec<u8>> {
        let mut output: Vec<u8> = vec![Self::FRAME_DELIMITER];
        let limit = Self::LIMIT - Self::CRC_SIZE - Self::FRAME_DELIM_SIZE;
        let mut crc = CRC8::new();

        if input.is_empty() {
            return Ok(output);
        }

        let in_size = input.len();
        if in_size > limit {
            return Err(AppError::IncorrectDataSize);
        }

        let mut input_pos = 0;
        let input_size = in_size + Self::CRC_SIZE;

        while input_pos < input_size {
            let loop_start_pos = output.len();
            let mut loop_data_count = 0;
            output.push(0xFF); // This will be overwritten later by the data count, but we go ahead and push it

            //This loop writes the data until we hit a frame delimiter
            while input_pos < input_size {
                let b: u8;
                if input_pos < in_size {
                    b = input[input_pos];
                    if b == Self::FRAME_DELIMITER {
                        break;
                    }
                    crc.next(b);
                } else {
                    b = crc.cur();
                    if b == Self::FRAME_DELIMITER {
                        break;
                    }
                }
                input_pos = input_pos + 1;
                output.push(b);
                loop_data_count = loop_data_count + 1;
                if loop_data_count >= Self::MAX_DATA_BYTES {
                    break;
                }
            }

            // In this loop we only calculate CRCs and counts, we'll use this to update the length byte
            while input_pos < input_size
                && (if input_pos < input.len() {
                    input[input_pos]
                } else {
                    crc.cur()
                }) == Self::FRAME_DELIMITER
            {
                crc.next(Self::FRAME_DELIMITER);
                input_pos = input_pos + 1;
                loop_data_count += Self::FRAME_BYTE_COUNT_LSB;
                if loop_data_count >= Self::MAX_COMPRESSED_FRAME_BYTES {
                    break;
                }
            }

            // Update the length byte at the start
            output[loop_start_pos] = loop_data_count as u8;
        }
        output.push(Self::FRAME_DELIMITER);
        Ok(output)
    }

    pub fn decode(input: &[u8]) -> Result<Vec<u8>> {
        let mut output: Vec<u8> = vec![];
        let mut code_byte: u8 = 0;
        // let mut crc = CRC8::new();
        let input = if input[0] == Self::FRAME_DELIMITER {
            &input[1..]
        } else {
            input
        };
        for b in input.iter() {
            if *b == Self::FRAME_DELIMITER {
                if code_byte != 0 {
                    return Err(AppError::InvalidPayload);
                }
                if output.len() <= 1 {
                    return Err(AppError::IncorrectDataSize);
                }
                let crcval = output.pop().unwrap();
                if crcval != CRC8::calc(&output) {
                    return Err(AppError::CRCFailure);
                }
                break;
            }
            if code_byte <= 0 {
                code_byte = *b;
            } else {
                code_byte = code_byte - 1;
                output.push(*b);
            }
            if (code_byte & Self::MAX_DATA_BYTES) == 0 {
                while code_byte > 0 {
                    output.push(Self::FRAME_DELIMITER);
                    code_byte = code_byte - Self::FRAME_BYTE_COUNT_LSB;
                }
            }
        }
        Ok(output)
    }
}

#[cfg(test)]
mod tests {
    const INPUTS: &[&[u8]] = &[
        &[0x03, 0x01, 0x10, 0xFF, 0xFF],
        &[0x06u8, 0x03, 0x01, 0x10, 0xFF, 0xFF],
        &[0x06u8, 0x03, 0x01, 0x10, 0, 0, 0, 0xFF, 0xFF],
        &[0u8, 0, 0, 0, 0, 0, 0, 0, 0, 0],
    ];

    #[test]
    fn encoder_decoder_tests() {
        for (i, data) in INPUTS.iter().enumerate() {
            let encoded = super::COBS::encode(data).unwrap();

            let decoded = super::COBS::decode(&encoded).unwrap();
            // assert_eq!(*expected_crc, calculated_crc);
            println!("Loop {} original datas: {:?}", i, data);
            println!("Loop {} encoded result: {:?}", i, encoded);
            println!("Loop {} decoded result: {:?}", i, decoded);
        }
    }

    const DECODER_INPUTS: &[&[u8]] = &[
        &[0x00, 0x06, 0x03, 0x01, 0x10, 0xff, 0xff, 0x78, 0x00],
        &[
            0x00, 0x87, 0x06, 0x01, 0x07, 0x81, 0xff, 0x01, 0x21, 0x01, 0xc1, 0x00,
        ],
        // Example command payloads
        &[0x00, 0x40, 0x42, 0x01, 0x60, 0x01, 0xa8, 0x00],
        &[0x00, 0x40, 0x04, 0x02, 0x60, 0x01, 0x12, 0x00],
        &[
            0x00, 0x40, 0x43, 0x03, 0x40, 0x01, 0x04, 0x08, 0x09, 0x07, 0xa0, 0x00,
        ],
        // Example responses
        &[
            0x00, 0x49, 0x0c, 0x01, 0x0a, 0x21, 0x0b, 0x21, 0x0c, 0x21, 0x0d, 0x03, 0x0e, 0x21,
            0x50, 0x00,
        ],
        &[
            0x00, 0x41, 0x04, 0x41, 0x09, 0x41, 0xd6, 0x43, 0x01, 0x10, 0x80, 0x01, 0x5e, 0x00,
        ],
        &[
            0x00, 0x41, 0x02, 0x42, 0x01, 0x81, 0xc3, 0x05, 0x05, 0x18, 0x40, 0xc1, 0x04, 0x03,
            0x02, 0x78, 0x2e, 0x00,
        ],
        &[0x00, 0x40, 0x42, 0x07, 0x60, 0x01, 0x79, 0x00],
        &[0x00, 0x40, 0x43, 0x09, 0x40, 0x01, 0x02, 0x07, 0xfe, 0x00],
    ];

    #[test]
    fn decoder_tests() {
        for (i, data) in DECODER_INPUTS.iter().enumerate() {
            let decoded = super::COBS::decode(data).unwrap();
            println!("Loop {} original datas: {:?}", i, data);
            println!("Loop {} decoded result: {:?}", i, decoded);
        }
    }
}
