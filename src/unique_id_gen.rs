//! # The Unique ID Generator Node (Server)
//!
//! In this challenge, you’ll need to implement a globally-unique ID generation system
//! that runs against Maelstrom’s unique-ids workload.
//!
//! Your service should be totally available, meaning that it can continue to operate
//! even in the face of network partitions.
//!
//! [Challenge #2: Unique ID Generation](https://fly.io/dist-sys/2/)
//!
//! A simple workload for ID generation systems.
//! Clients ask servers to generate an ID, and the server should respond with an ID.
//! The test verifies that those IDs are globally unique.
//!
//! Generated IDs may be of any type: strings, booleans, integers, floats, compound JSON values, etc.
//!
//! https://github.com/jepsen-io/maelstrom/blob/main/doc/workloads.md#workload-unique-ids
//!
//! Run as:
//!
//! ```
//! cargo build && ~/maelstrom/maelstrom test -w unique-ids --bin target/debug/gossip-glomers --time-limit 30 --rate 1000 --node-count 3 --availability total --nemesis partition
//! ```
//!
//! This will run a 3-node cluster for 30 seconds and request new IDs at the rate of 1000 requests per second.
//! It checks for total availability and will induce network partitions during the test.
//! It will also verify that all IDs are unique.

use crate::message::{Body, Message, Payload};
use anyhow::{bail, Context, Result};
use serde::Serialize;
use std::fmt::Debug;
use std::io::{StdoutLock, Write};

/// # The Unique ID Generator Node (Server)
///
/// A simple workload for ID generation systems.
/// Clients ask servers to generate an ID, and the server should respond with an ID.
///
/// Generated IDs may be of any type: strings, booleans, integers, floats, compound JSON values, etc.
#[derive(Default, Debug)]
pub struct UniqueIDGeneratorNode<ID> {
    /// A locally-unique integer identifier for a message from a node. It isn't globally-unique.
    pub msg_id: usize,
    /// A unique node name. Maelstrom sets the node ID for our node(s), during the initialization phase.
    pub node_id: Option<String>,
    /// A generated globally-unique ID.
    /// It may be of any type: strings, booleans, integers, floats, compound JSON values, etc.
    pub guid: ID,
}

impl<ID> UniqueIDGeneratorNode<ID>
where
    ID: Clone + Debug + Default + Serialize,
{
    /// Creates and returns a new node.
    pub fn new() -> Self {
        Self {
            msg_id: 0,
            node_id: None,
            guid: ID::default(),
        }
    }

    /// A processing step in a node's state-machine.
    pub fn step(&mut self, request: Message<ID>, output_lock: &mut StdoutLock) -> Result<()> {
        match request.body.payload {
            Payload::Generate => {
                // self.guid = 666; TODO

                let response = Message {
                    src: self.node_id.clone().expect("expected some self.node_id"),
                    dest: request.src,
                    body: Body {
                        msg_id: Some(self.msg_id),
                        in_reply_to: request.body.msg_id,
                        payload: Payload::GenerateOk {
                            id: self.guid.clone(),
                        },
                    },
                };

                serde_json::to_writer(&mut *output_lock, &response)
                    .context("serialization of response generate_ok message failed")?;
                output_lock
                    .write_all(b"\n")
                    .context("failed to write newline")?;

                self.msg_id += 1;
            }
            Payload::GenerateOk { id } => {
                self.guid = id;
            }
            Payload::Init { node_id, .. } => {
                self.node_id = Some(node_id);

                let response = Message {
                    src: self.node_id.clone().expect("expected some self.node_id"),
                    dest: request.src,
                    body: Body {
                        msg_id: Some(self.msg_id),
                        in_reply_to: request.body.msg_id,
                        payload: Payload::<ID>::InitOk,
                    },
                };

                serde_json::to_writer(&mut *output_lock, &response)
                    .context("serialization of response init_ok message failed")?;
                output_lock
                    .write_all(b"\n")
                    .context("failed to write newline")?;

                self.msg_id += 1;
            }
            other => bail!("received unexpected request message type: {other:?}"),
        }

        Ok(())
    }
}
