use crate::stun::{Message, MessageClass};
use std::net::UdpSocket;

/// STUN client
struct Client {
    /// remote address of STUN server
    server_address: String,
}

impl Client {
    fn new(addr: String) -> Self {
        Self {
            server_address: addr,
        }
    }
    /// discover the NAT
    fn discover(&self) -> Result<(), StunError> {
        let package = Message::new(MessageClass::Request);
        package.binary();

        // network handle
        let socket = UdpSocket::bind("0.0.0.0:0");
        let socket = match socket {
            Ok(udp_socket) => udp_socket,
            Err(e) => {
                println!("-> {:?}", e);
                panic!();
            }
        };

        match socket.send_to("-> hello".as_bytes(), self.server_address.as_str()) {
            Ok(a) => a,
            Err(e) => {
                println!("{:?}", e);
                return Err(StunError::Network);
            }
        };

        Ok(())
    }
}

#[test]
fn test_discover() {
    let c = Client::new("127.0.0.1:10000".to_string());
    c.discover();
}

enum StunError {
    Unknown,
    Network,
}
