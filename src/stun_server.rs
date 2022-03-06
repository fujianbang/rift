use crate::Message;
use std::net::UdpSocket;

const MAX_UDP_BUFFER_SIZE: usize = 256;

pub struct Server {
    socket: UdpSocket,
    // to_send: Option<(usize, SocketAddr)>;
}

impl Server {
    pub(crate) fn new() -> Self {
        let sock = UdpSocket::bind("0.0.0.0:8080").unwrap();

        Self { socket: sock }
    }

    /// Run UDP server
    pub(crate) async fn run(self) {
        // let sock = UpS
        let Server { socket } = self;

        println!("Ready to receiving data");
        loop {
            let mut buf = [0; MAX_UDP_BUFFER_SIZE];

            match socket.recv_from(&mut buf) {
                Ok((size, addr)) => {
                    let data = &buf[0..size];
                    println!("{:?} bytes received from {:?}", data.len(), addr);

                    let m = Message::from(data);
                    println!("{:?}", m);
                }
                Err(e) => {
                    println!("{:?}", e);
                    break;
                }
            };
        }
    }
}
