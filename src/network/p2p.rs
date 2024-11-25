use crate::network::message::{Message, MessageType};
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::sync::{Arc, Mutex};
use std::thread;

/// Struct to represent a peer node in the P2P network.
pub struct Peer {
    pub address: String,
}

/// Struct to manage the P2P network.
pub struct P2PNetwork {
    peers: Arc<Mutex<Vec<Peer>>>, // List of connected peers
}

impl P2PNetwork {
    /// Creates a new P2P network instance.
    pub fn new() -> Self {
        P2PNetwork {
            peers: Arc::new(Mutex::new(Vec::new())),
        }
    }

    /// Starts the node as a server, accepting incoming connections.
    pub fn start_server(&self, address: &str) {
        let listener = TcpListener::bind(address).expect("Failed to bind to address");

        println!("Node listening on: {}", address);

        for stream in listener.incoming() {
            match stream {
                Ok(stream) => {
                    let peers = Arc::clone(&self.peers);
                    thread::spawn(move || {
                        P2PNetwork::handle_connection(stream, peers);
                    });
                }
                Err(e) => {
                    eprintln!("Failed to accept connection: {}", e);
                }
            }
        }
    }

    /// Connects to a peer node and establishes communication.
    pub fn connect_to_peer(&self, peer_address: &str) {
        match TcpStream::connect(peer_address) {
            Ok(mut stream) => {
                println!("Connected to peer: {}", peer_address);

                let message = Message::new(MessageType::Hello, "Hello from node".to_string());
                let serialized_message = bincode::serialize(&message).unwrap();
                stream
                    .write_all(&serialized_message)
                    .expect("Failed to send message");

                // Add peer to the list of connected peers
                let mut peers = self.peers.lock().unwrap();
                peers.push(Peer {
                    address: peer_address.to_string(),
                });
            }
            Err(e) => {
                eprintln!("Failed to connect to peer: {}", e);
            }
        }
    }

    /// Handles incoming connections from other nodes.
    fn handle_connection(mut stream: TcpStream, peers: Arc<Mutex<Vec<Peer>>>) {
        let mut buffer = [0; 1024];
        while match stream.read(&mut buffer) {
            Ok(size) if size > 0 => {
                let message: Message =
                    bincode::deserialize(&buffer[..size]).expect("Failed to deserialize message");
                P2PNetwork::process_message(message, Arc::clone(&peers));
                true
            }
            _ => false,
        } {}
    }

    /// Processes incoming messages based on the message type.
    fn process_message(message: Message, _peers: Arc<Mutex<Vec<Peer>>>) {
        match message.message_type {
            MessageType::Hello => {
                println!("Received hello message: {}", message.payload);
            }
            MessageType::Block => {
                println!("Received block message: {}", message.payload);
                // Handle block processing
            }
            MessageType::Transaction => {
                println!("Received transaction message: {}", message.payload);
                // Handle transaction processing
            }
        }
    }
}
