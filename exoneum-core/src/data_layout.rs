/// Data structures stored in blockchain
use chrono::{DateTime, Utc};
use exonum::crypto::{Hash, PublicKey};
use pnet::datalink;
use std::net;

encoding_struct! {
    /// User
    struct User {
        /// Public key
        public_key: &PublicKey,
        /// Name
        name: &str,
        /// Current balance
        balance: u64,
        /// User public IP address in network {ip:port} for comminicate between nodes
        user_address: std::net::Ipv4Addr,
    }
}
