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
    /// Returns a new Message
    pub fn new(class: MessageClass, method: Method) -> Self {
        Self {
            class,
            method,
            message_length: 0, // TODO
            magic_cookie: MAGIC_COOKIE,
            transaction_id: TransactionId::new(),
            attributes: Vec::new(),
        }
    }

    /// Returns the class of the message
    fn class(&self) -> MessageClass {
        self.class
    }

    /// Returns the method of the message
    fn method(&self) -> Method {
        self.method
    }

    /// Returns the Transaction ID reference of the message
    fn transaction_id(&self) -> &TransactionId {
        &self.transaction_id
    }

    /// Add Attribute to the message
    fn add_attribute(&mut self, attr: Attribute) {
        self.attributes.push(attr)
    }

    /// Get the set of Attribute from the message
    fn attributes(&self) -> &[Attribute] {
        self.attributes.as_slice()
    }

    /// To binary
    fn to_binary(&self) -> Vec<u8> {
        // message type
        let message_type: u16 = match self.class {
            MessageClass::Request => 0b00000_0_000_0_0001,
            MessageClass::Indication => 0b00000_0_000_1_0001,
            MessageClass::SuccessResponse => 0b00000_1_000_0_0001,
            MessageClass::ErrorResponse => 0b00000_1_000_1_0001,
        };
        let mut data: Vec<u8> = Vec::new();
        data.extend_from_slice(message_type.to_be_bytes().as_slice());
        data.extend_from_slice(self.message_length.to_be_bytes().as_slice());
        data.extend_from_slice(self.magic_cookie.to_be_bytes().as_slice());
        data.extend_from_slice(self.transaction_id.as_bytes());
        data
    }
}

impl From<Vec<u8>> for Message {
    fn from(data: Vec<u8>) -> Self {
        let message_type_code = ((data[0] as u16) << 8) + (data[1] as u16);

        let message_class: Result<MessageClass, ()> = match message_type_code {
            0b00000_0_000_0_0001 => Ok(MessageClass::Request),
            0b00000_0_000_1_0001 => Ok(MessageClass::Indication),
            0b00000_1_000_0_0001 => Ok(MessageClass::SuccessResponse),
            0b00000_1_000_1_0001 => Ok(MessageClass::ErrorResponse),
            _ => {
                todo!()
            }
        };

        let length = ((data[2] as u16) << 8) + (data[3] as u16);

        let magic_cookie = ((data[4] as u32) << 24)
            + ((data[5] as u32) << 16)
            + ((data[6] as u32) << 8)
            + (data[7] as u32);

        let transaction_id = TransactionId::from(&data[8..20]);

        Self {
            class: message_class.unwrap(),
            method: Method(0),
            message_length: length,
            magic_cookie,
            transaction_id,
            attributes: vec![],
        }
    }
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

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
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

mod test {
    use crate::stun::MessageClass;

    #[test]
    fn message_class() {
        assert_eq!(MessageClass::from_u8(0b00), Some(MessageClass::Request));
        assert_eq!(MessageClass::from_u8(0b01), Some(MessageClass::Indication));
        assert_eq!(
            MessageClass::from_u8(0b10),
            Some(MessageClass::SuccessResponse)
        );
        assert_eq!(
            MessageClass::from_u8(0b11),
            Some(MessageClass::ErrorResponse)
        );
        assert_eq!(MessageClass::from_u8(0b101), None);
    }
}

#[derive(Debug, Copy, Clone)]
pub struct Method(u16);

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

#[derive(Debug, Clone)]
struct TransactionId(Vec<u8>);

impl TransactionId {
    fn new() -> Self {
        let mut transaction_id = [0u8; 12];
        thread_rng().fill(&mut transaction_id[..]);
        let mut bytes = Vec::new();
        bytes.extend_from_slice(&transaction_id[..]);
        Self(bytes)
    }

    fn from(data: &[u8]) -> Self {
        let mut bytes = Vec::new();
        bytes.extend_from_slice(data);
        Self(bytes)
    }

    fn as_bytes(&self) -> &[u8] {
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
