//! A rust implementation of STUN proto

use rand::{thread_rng, Rng};

const MAGIC_COOKIE: i32 = 0x2112A442;
const FINGERPRINT: i32 = 0x5354554e;

/// STUN Message definition
///
/// STUN message are encoded in binary using network-oriented format.
/// All STUN messages comprise a 20-byte header followed by zero or more attributes.
///
/// 0                   1                   2                   3
/// 0 1 2 3 4 5 6 7 8 9 0 1 2 3 4 5 6 7 8 9 0 1 2 3 4 5 6 7 8 9 0 1
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
    header: [u8; 20],
    // attributes: Vec<Attribute>,
}

struct Attribute {}

impl Message {
    pub fn new() -> Self {
        // generate header

        Self {
            header: [0; 20],
            // attributes: vec![],
        }
    }
    /// Set MessageType (12-bits)
    ///
    /// 0            1
    /// 2  3  4 5 6 7 8 9 0 1 2 3 4 5
    ///+--+--+-+-+-+-+-+-+-+-+-+-+-+-+
    ///|M |M |M|M|M|C|M|M|M|C|M|M|M|M|
    ///|11|10|9|8|7|1|6|5|4|0|3|2|1|0|
    ///+--+--+-+-+-+-+-+-+-+-+-+-+-+-+/
    ///
    /// message class
    /// C1 and C0 represent a 2-bit encoding of the class
    /// * request(0b00)
    /// * indication(0b01)
    /// * success response(0b10)
    /// * error response(0b11)
    ///
    /// message method
    /// * 0b000000000001 (Binding)
    fn set_message_type(&mut self) {
        todo!()
    }
    fn set_magic_cookie(&mut self) {
        MAGIC_COOKIE
            .to_be_bytes()
            .iter()
            .enumerate()
            .for_each(|(i, &v)| {
                self.header[4 + i] = v;
            });
    }
    /// a randomly selected 96-bits number
    ///
    /// allow the client to associate the response with the Request that generated it;
    /// for indications, the transaction ID serves as a debugging aid.
    fn set_transaction_id(&mut self) {
        let mut rnd: [u8; 12] = [0; 12];
        thread_rng().fill(&mut rnd);

        rnd.iter()
            .enumerate()
            .for_each(|(i, &v)| self.header[8 + i] = v);
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
    SuccessResponse(),
    /// 0b11
    ErrorResponse,
}
