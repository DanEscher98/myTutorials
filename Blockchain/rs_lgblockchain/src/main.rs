use lgblockchain::p2p;
use tokio::sync::mpsc;
use libp2p::{identity::Keypair, noise::Config};

#[tokio::main]
async fn main() {
    pretty_env_logger::init();

    log::info!("Peer Id: {}", p2p::PEER_ID.clone());
    let (response_sender, mut response_rcv) = mpsc::unbounded_channel::<String>();
    let (init_sender, mut init_rcv) = mpsc::unbounded_channel::<String>();

    let auth_keys = Keypair::new();
}
