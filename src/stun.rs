//! A rust implementation of STUN proto

use rand::{thread_rng, Rng};
use std::fmt;
use std::fmt::{Formatter, Pointer};

const FINGERPRINT: i32 = 0x5354554e;

/// STUN Message definition
///
/// STUN message are encoded in binary using network-oriented format.
/// All STUN messages comprise a 20-byte header followed by zero or more attributes.
///
///  0                   1                   2                   3
///  0 1 2 3 4 5 6 7 8 9 0 1 2 3 4 5 6 7 8 9 0 1 2 3 4 5 6 7 8 9 0 1
/// +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
/// |0 0|     STUN Message Type     |         Message Length        |
/// +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
/// |                         Magic Cookie                          |
/// +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
/// |                                                               |
/// |                     Transaction ID (96 bits)                  |
/// |                                                               |
/// +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
pub struct Message {
    message_type: u16,
    message_length: u16,
    magic_cookie: u32,
    transaction_id: [u8; 12],
}

impl Message {
    pub fn new(class: MessageClass) -> Self {
        // generate transaction_id
        let mut transaction_id: [u8; 12] = [0; 12];
        thread_rng().fill(&mut transaction_id);

        Self {
            message_type: match class {
                MessageClass::Request => 0b00000_0_000_0_0001,
                MessageClass::Indication => 0b00000_0_000_1_0001,
                MessageClass::SuccessResponse => 0b00000_1_000_0_0001,
                MessageClass::ErrorResponse => 0b00000_1_000_1_0001,
            },
            message_length: 0, // TODO
            magic_cookie: 0x2112A442,
            transaction_id,
        }
    }

    pub fn from_bytes(data: &[u8]) -> Self {
        println!("{:?}", data);

        // MessageType
        let message_type = ((data[0] as u16) << 8) + data[1] as u16;

        // MessageLength
        let message_length = Self::bytes_to_u16(&data[2..4]);

        // MagicCookie
        let magic_cookie = Self::bytes_to_u32(&data[4..8]);

        // TransactionID
        let mut v: Vec<u8> = Vec::new();
        v.extend_from_slice(&data[8..]);
        let transaction_id: [u8; 12] = v.as_slice().try_into().expect("error");

        Self {
            message_type,
            message_length,
            magic_cookie,
            transaction_id,
        }
    }
    pub fn to_binary(&self) -> Vec<u8> {
        let mut data: Vec<u8> = Vec::new();
        data.extend_from_slice(&self.message_type.to_be_bytes());
        data.extend_from_slice(&self.message_length.to_be_bytes());
        data.extend_from_slice(&self.magic_cookie.to_be_bytes());
        data.extend_from_slice(&self.transaction_id);
        data
    }

    fn bytes_to_u16(s: &[u8]) -> u16 {
        let mut val: u16 = 0;
        for (i, &v) in s.iter().enumerate() {
            val = val + ((v as u16) << (8 * (s.len() - i - 1)));
        }
        val
    }

    fn bytes_to_u32(s: &[u8]) -> u32 {
        let mut val: u32 = 0;
        for (i, &v) in s.iter().enumerate() {
            val = val + ((v as u32) << (8 * (s.len() - i - 1)));
        }
        val
    }
}

impl fmt::Debug for Message {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{:?} - {:?} - {:?} - {:?}",
            self.message_type, self.message_length, self.magic_cookie, self.transaction_id
        )
    }
}

#[test]
fn message_serialize() {
    let m = Message::new(MessageClass::Request);
    let m2 = Message::from_bytes(m.to_binary().as_slice());
    // println!("origin: {:?}", m);
    // println!("after: {:?}", m2);
    assert_eq!(m.to_binary(), m2.to_binary());
}

pub enum MessageClass {
    /// 0b00
    Request,
    /// 0b01
    Indication,
    /// 0b10
    SuccessResponse,
    /// 0b11
    ErrorResponse,
}

/// STUN Attributes
///
///  0                   1                   2                   3
///  0 1 2 3 4 5 6 7 8 9 0 1 2 3 4 5 6 7 8 9 0 1 2 3 4 5 6 7 8 9 0 1
/// +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
/// |         Type                  |            Length             |
/// +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
/// |                         Value (variable)                ....
/// +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
struct Attribute {}

/// MAPPED_ADDRESS
///
///  0                   1                   2                   3
///  0 1 2 3 4 5 6 7 8 9 0 1 2 3 4 5 6 7 8 9 0 1 2 3 4 5 6 7 8 9 0 1
/// +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
/// |0 0 0 0 0 0 0 0|    Family     |           Port                |
/// +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
/// |                                                               |
/// |                 Address (32 bits or 128 bits)                 |
/// |                                                               |
/// +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
struct MappedAddress {}

/// XOR-MAPPED-ADDRESS
///
///  0                   1                   2                   3
///  0 1 2 3 4 5 6 7 8 9 0 1 2 3 4 5 6 7 8 9 0 1 2 3 4 5 6 7 8 9 0 1
/// +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
/// |0 0 0 0 0 0 0 0|    Family     |         X-Port                |
/// +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
/// |                X-Address (Variable)
/// +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
struct XorMappedAddress {}

/// ERROR_CODE
///
/// 0                   1                   2                   3
/// 0 1 2 3 4 5 6 7 8 9 0 1 2 3 4 5 6 7 8 9 0 1 2 3 4 5 6 7 8 9 0 1
/// +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
/// |           Reserved, should be 0         |Class|     Number    |
/// +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
/// |      Reason Phrase (variable)                                ..
/// +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
enum ErrorCode {
    TryAlternate = 300,
    BadRequest = 400,
    Unauthenticated = 401,
    UnknownAttribute = 420,
    StaleNonce = 438,
    ServerError = 500,
}

/// PASSWORD-ALGORITHMS
///
/// 0                   1                   2                   3
/// 0 1 2 3 4 5 6 7 8 9 0 1 2 3 4 5 6 7 8 9 0 1 2 3 4 5 6 7 8 9 0 1
/// +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
/// |         Algorithm 1           | Algorithm 1 Parameters Length |
/// +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
/// |                    Algorithm 1 Parameters (variable)
/// +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
/// |         Algorithm 2           | Algorithm 2 Parameters Length |
/// +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
/// |                    Algorithm 2 Parameters (variable)
/// +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
/// |                                                             ...
struct PasswordAlgorithms {}

/// PASSWORD_ALGORITHM
///   0                   1                   2                   3
///  0 1 2 3 4 5 6 7 8 9 0 1 2 3 4 5 6 7 8 9 0 1 2 3 4 5 6 7 8 9 0 1
/// +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
/// |          Algorithm           |  Algorithm Parameters Length   |
/// +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
/// |                    Algorithm Parameters (variable)
/// +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
struct PasswordAlgorithm {}

/// UNKNOWN-ATTRIBUTES
///
///   0                   1                   2                   3
///  0 1 2 3 4 5 6 7 8 9 0 1 2 3 4 5 6 7 8 9 0 1 2 3 4 5 6 7 8 9 0 1
/// +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
/// |      Attribute 1 Type         |       Attribute 2 Type        |
/// +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
/// |      Attribute 3 Type         |       Attribute 4 Type    ...
/// +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
struct UnknownAttributes {}
