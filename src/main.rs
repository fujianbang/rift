use crate::stun::Message;
use std::net::UdpSocket;
use std::str;

mod stun;
mod stun_client;
mod stun_server;

fn main() -> std::io::Result<()> {
    let socket = UdpSocket::bind("0.0.0.0:10000")?;
    let mut buf = [0; 2048];

    loop {
        let (c, src) = socket.recv_from(&mut buf)?;

        let buf = &mut buf[..c];
        // socket.send_to(buf, &src);
        println!("receive {:?}", buf);

        // let m = Message::from_bytes(buf);
        // println!("receive {:?}", m);
    }
}
