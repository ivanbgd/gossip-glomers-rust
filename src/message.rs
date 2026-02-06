//! # Message
//!
//! The [Protocol](https://github.com/jepsen-io/maelstrom/blob/main/doc/protocol.md) page contains
//! definitions for messages and their bodies (payloads).
//!
//! Maelstrom nodes receive messages on `STDIN`, send messages on `STDOUT`, and log debugging output on `STDERR`.
//! Maelstrom nodes must not print anything that is not a message to `STDOUT`.
//! Maelstrom will log `STDERR` output to disk for you.
//!
//! Both `STDIN` and `STDOUT` messages are JSON objects, separated by newlines (`\n`).

use crate::IdType;
use serde::{Deserialize, Serialize};

/// Messages
///
/// Both `STDIN` and `STDOUT` messages are JSON objects, separated by newlines (`\n`).
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Message<P> {
    /// A string identifying the node this message came from.
    pub src: String,
    /// A string identifying the node this message is for.
    pub dest: String,
    /// An object: the body (payload) of the message.
    pub body: Body<P>,
}

/// Message bodies
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Body<P> {
    /// (optional) A locally-unique integer identifier for a message from a node. It isn't globally-unique.
    pub msg_id: Option<usize>,
    /// (optional) For req/response, the `msg_id` of the request.
    pub in_reply_to: Option<usize>,
    /// (mandatory) A string identifying the type of message this is, plus optional data contained within.
    #[serde(flatten)]
    pub payload: P,
}

/// The initialization-by-Maelstrom payload types.
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(tag = "type")]
#[serde(rename_all = "snake_case")]
pub enum InitPayload {
    /// At the start of a test, Maelstrom issues a single init message to each node.
    Init {
        /// The `node_id` field indicates the ID of the node which is receiving this message.
        /// Your node should remember this ID and include it as the `src` of any message it sends.
        node_id: String,
        /// The `node_ids` field lists all nodes in the cluster, including the recipient.
        /// All nodes receive an identical list; you may use its order if you like.
        node_ids: Vec<String>,
    },
    /// In response to the `init` message, each node must respond with a message of type `init_ok`.
    InitOk,
}

/// Various payloads for message bodies.
///
/// Inherently contains the mandatory message type (in the nested variant's name), and optionally additional data.
///
/// The various message types and the meanings of their fields are defined in the
/// [workload documentation](https://github.com/jepsen-io/maelstrom/blob/main/doc/workloads.md).
///
/// This does *not* include the initialization-by-Maelstrom payload types.
///
/// Payloads are grouped per node types; they are nested inside the groups.
/// The group names are not serde'd.
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[serde(untagged)]
pub enum Payload {
    Echo(EchoPayload),
    Error(ErrorPayload),
    UniqueIdGen(GeneratePayload),
}

/// A simple echo workload: a client sends a message, and expects to get that same message back from our server.
///
/// Clients send echo messages to servers with an `echo` field containing an arbitrary payload
/// they'd like to have sent back.
///
/// Servers should respond with `echo_ok` messages containing that same payload.
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(tag = "type")]
#[serde(rename_all = "snake_case")]
pub enum EchoPayload {
    Echo { echo: String },
    EchoOk { echo: String },
}

/// In response to a Maelstrom RPC request, a node may respond with an error message,
/// whose body is a JSON object.
///
/// The `type` of error body is always `"error"`.
///
/// As with all RPC responses, the `in_reply_to` field is the `msg_id` of the request which caused this error.
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(tag = "type")]
#[serde(rename_all = "snake_case")]
pub struct ErrorPayload {
    /// The `code` is an integer which indicates the type of error which occurred.
    /// Maelstrom defines several error types, and you can also invent your own.
    /// Codes `0-999` are reserved for Maelstrom's use; codes `1000` and above are free for your own purposes.
    pub code: ErrorCode,
    /// The `text` field is a free-form string.
    /// It is optional, and may contain any explanatory message you like.
    pub text: Option<String>,
}

/// Indicates the type of error which occurred.
///
/// Maelstrom defines several error types, and you can also invent your own.
///
/// Codes `0-999` are reserved for Maelstrom's use; codes `1000` and above are free for your own purposes.
///
/// Custom error codes are always indefinite.
#[derive(Clone, Debug, Serialize, Deserialize)]
#[repr(usize)]
pub enum ErrorCode {
    SomeErrorCode = 1000,
}

/// A simple workload for ID generation systems.
/// Clients ask servers to generate an ID, and the server should respond with an ID.
///
/// Generated IDs may be of any type: strings, booleans, integers, floats, compound JSON values, etc.
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(tag = "type")]
#[serde(rename_all = "snake_case")]
pub enum GeneratePayload {
    Generate,
    GenerateOk { id: IdType },
}
