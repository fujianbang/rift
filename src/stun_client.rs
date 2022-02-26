use crate::stun::Message;
use std::net::IpAddr;

/// STUN client
struct Client {
    /// remote address of STUN server
    server_address: IpAddr,
}

impl Client {
    /// discover the NAT
    fn discover() -> IpAddr {
        let package = Message::new();
        // package.s
        // Message::new();
        todo!()
    }
}
