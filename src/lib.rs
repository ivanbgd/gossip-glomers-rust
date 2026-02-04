//! # The Gossip Glomers Library

pub mod echo;
pub mod message;
pub mod node;
pub mod unique_id_gen;

/// The type of the generated globally-unique ID.
/// It may be any type: strings, booleans, integers, floats, compound JSON values, etc.
pub type IdType = String;
