//! A rust implementation of STUN proto

use rand::{thread_rng, Rng};

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
#[derive(Debug)]
pub struct Message {
    message_type: u16,
    message_length: u16,
    magic_cookie: u32,
    transaction_id: [u8; 12],
    // attributes: Vec<Attribute>,
}

struct Attribute {}

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
}

#[test]
fn test_set_magic_cookie() {
    let mut a = Message::new();
    a.set_magic_cookie();
    a.set_transaction_id();
    a.set_transaction_id();
    println!("{:?}", a)
}

enum MessageClass {
    /// 0b00
    Request,
    /// 0b01
    Indication,
    /// 0b10
    SuccessResponse,
    /// 0b11
    ErrorResponse,
}
