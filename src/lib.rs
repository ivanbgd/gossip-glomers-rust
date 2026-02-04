//! # The Gossip Glomers Library

pub mod logic;
pub mod message;
pub mod node;

/// The type of the generated globally-unique ID.
/// It may be any type: strings, booleans, integers, floats, compound JSON values, etc.
pub type IdType = String;
