use crate::stun::Message;
use futures::executor::block_on;
use stun_server::Server;

mod stun;
mod stun_client;
mod stun_server;

fn main() {
    let server = Server::new().run();
    block_on(server);
}
