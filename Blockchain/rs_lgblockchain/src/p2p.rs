use crate::blockchain::{Block, Chain};

use libp2p::{
    floodsub::{self, Floodsub, Topic},
    identity, mdns, swarm::{self, NetworkBehaviour}, PeerId,
};
use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use tokio::sync::mpsc;

pub static KEYS: Lazy<identity::Keypair> = Lazy::new(identity::Keypair::generate_ed25519);
pub static PEER_ID: Lazy<PeerId> = Lazy::new(|| PeerId::from(KEYS.public()));
pub static CHAIN_TOPIC: Lazy<Topic> = Lazy::new(|| Topic::new("chains"));
pub static BLOCK_TOPIC: Lazy<Topic> = Lazy::new(|| Topic::new("blocks"));

#[derive(Debug, Serialize, Deserialize)]
pub struct ChainResponse {
    pub blocks: Vec<Block>,
    pub receiver: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LocalChainRequest {
    pub from_peer_id: String,
}

pub enum EventType {
    LocalChainResponse(ChainResponse),
    Input(String),
    Init,
}

pub struct App<T> {
    pub app: Chain,
    pub behaviour: AppBehaviour,
    pub init_sender: mpsc::UnboundedSender<T>,
    pub response_sender: mpsc::UnboundedSender<T>,
}

#[derive(NetworkBehaviour)]
#[behaviour(to_swarm = "AppBehaviourEvent")]
pub struct AppBehaviour {
    pub floodsub: Floodsub,
    pub mdns: mdns::tokio::Behaviour,
}

pub enum AppBehaviourEvent {
    Floodsub(floodsub::FloodsubEvent),
    Mdns(mdns::Event)
}

impl From<floodsub::FloodsubEvent> for AppBehaviourEvent {
    fn from(event: floodsub::FloodsubEvent) -> Self {
        AppBehaviourEvent::Floodsub(event)
    }
}

impl From<mdns::Event> for AppBehaviourEvent {
    fn from(event: mdns::Event) -> Self {
        AppBehaviourEvent::Mdns(event)
    }
}

impl<T> App<T> {
    pub fn new(chain: Chain, init_sender: mpsc::UnboundedSender<T>, response_sender: mpsc::UnboundedSender<T>) -> Self {
        let mut behaviour = AppBehaviour {
            floodsub: Floodsub::new(*PEER_ID),
            mdns: mdns::tokio::Behaviour::new(Default::default(), *PEER_ID).expect("can create mdns")
        };
        behaviour.floodsub.subscribe(CHAIN_TOPIC.clone());
        behaviour.floodsub.subscribe(BLOCK_TOPIC.clone());
        Self {
            app: chain,
            init_sender,
            response_sender,
            behaviour
        }
    }
}
