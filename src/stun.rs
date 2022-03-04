//! A rust implementation of STUN proto

use rand::{thread_rng, Rng};
use std::fmt;
use std::fmt::Formatter;

const FINGERPRINT: i32 = 0x5354554e;

const MAGIC_COOKIE: u32 = 0x2112_A442;

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
// #[derive(,Clone)]
pub struct Message {
    class: MessageClass,
    method: Method,
    message_length: u16,
    magic_cookie: u32,
    transaction_id: TransactionId,
    attributes: Vec<Attribute>,
}

impl Message {
    pub fn new(class: MessageClass, method: Method) -> Self {
        Self {
            class,
            method,
            message_length: 0, // TODO
            magic_cookie: MAGIC_COOKIE,
            transaction_id: TransactionId::new(),
            attributes: vec![],
        }
    }

    // Returns the class of the message
    fn class(&self) -> MessageClass {
        self.class
    }

    // Returns the method of the message
    fn method(&self) -> Method {
        self.method
    }

    // Returns the Transaction ID of the message
    fn transaction_id(&self) -> TransactionId {
        self.transaction_id
    }

    // Add Attribute to the message
    fn add_attribute(&mut self, attr: Attribute) {
        self.attributes.push(attr)
    }

    // pub fn from_bytes(data: &[u8]) -> Self {
    //     println!("{:?}", data);
    //
    //     // MessageType
    //     let message_type = ((data[0] as u16) << 8) + data[1] as u16;
    //
    //     // MessageLength
    //     let message_length = Self::bytes_to_u16(&data[2..4]);
    //
    //     // MagicCookie
    //     let magic_cookie = Self::bytes_to_u32(&data[4..8]);
    //
    //     // TransactionID
    //     let mut v: Vec<u8> = Vec::new();
    //     v.extend_from_slice(&data[8..]);
    //
    //     // let transaction_id = TransactionId::from(v.as_slice())
    //
    //     Self {
    //         // message_type,
    //         message_length,
    //         magic_cookie,
    //         transaction_id: TransactionId::new(), // TODO
    //     }
    // }

    // pub fn to_binary(&self) -> Vec<u8> {
    //     let mut data: Vec<u8> = Vec::new();
    //     data.extend_from_slice(&self.message_type.to_be_bytes());
    //     data.extend_from_slice(&self.message_length.to_be_bytes());
    //     data.extend_from_slice(&self.magic_cookie.to_be_bytes());
    //     data.extend_from_slice(&self.transaction_id);
    //     data
    // }
}

impl fmt::Debug for Message {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{:?} - {:?} - {:?} - {:?}",
            self.class, self.message_length, self.magic_cookie, self.transaction_id
        )
    }
}

#[derive(Debug, Copy, Clone)]
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

impl MessageClass {
    fn from_u8(value: u8) -> Option<Self> {
        match value {
            0b00 => Some(MessageClass::Request),
            0b01 => Some(MessageClass::Indication),
            0b10 => Some(MessageClass::SuccessResponse),
            0b11 => Some(MessageClass::ErrorResponse),
            _ => None,
        }
    }
}

#[derive(Debug, Copy, Clone)]
struct Method(u16);

impl Method {
    fn new(p: u16) -> Self {
        Self(p)
    }

    fn to_u16(self) -> u16 {
        self.0
    }
}

impl From<u8> for Method {
    fn from(f: u8) -> Self {
        Method(u16::from(f))
    }
}

#[derive(Debug, Copy, Clone)]
struct TransactionId([u8; 12]);

impl TransactionId {
    fn new() -> Self {
        let mut transaction_id: [u8; 12] = [0; 12];
        thread_rng().fill(&mut transaction_id);
        Self(transaction_id)
    }

    fn from(data: [u8; 12]) -> Self {
        Self(data)
    }

    fn as_bytes(&self) -> &[u8; 12] {
        &self.0
    }
}

#[test]
fn transaction_id() {
    let id = TransactionId::new();

    println!("{:?}", id);
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
