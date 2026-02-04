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
//!
//! Everything looks good! ヽ(‘ー`)ノ

use crate::message::{Body, Message, Payload};
use crate::node::Node;
use crate::IdType;
use anyhow::{bail, Context, Result};
use std::fmt::Debug;
use std::io::{StdoutLock, Write};

/// # The Unique ID Generator Node (Server)
///
/// A simple workload for ID generation systems.
/// Clients ask servers to generate an ID, and the server should respond with an ID.
///
/// Generated IDs may be of any type: strings, booleans, integers, floats, compound JSON values, etc.
#[derive(Default, Debug)]
pub struct UniqueIDGeneratorNode {
    /// A locally-unique integer identifier for a message from a node. It isn't globally-unique.
    pub msg_id: usize,
    /// A unique node name. Maelstrom sets the node ID for our node(s), during the initialization phase.
    pub node_id: Option<String>,
    /// A generated globally-unique ID.
    /// It may be of any type: strings, booleans, integers, floats, compound JSON values, etc.
    pub guid: IdType,
}

impl Node for UniqueIDGeneratorNode {
    fn new() -> Self {
        Self {
            msg_id: 0,
            node_id: None,
            guid: IdType::new(),
        }
    }

    fn get_msg_id(&self) -> usize {
        self.msg_id
    }

    fn incr_msg_id(&mut self) {
        self.msg_id += 1;
    }

    fn get_node_id(&self) -> Option<String> {
        self.node_id.clone()
    }

    fn set_node_id(&mut self, value: Option<String>) {
        self.node_id = value;
    }

    fn step(&mut self, request: Message<Payload>, output_lock: &mut StdoutLock) -> Result<()> {
        match request.body.payload {
            Payload::Generate => {
                let this = self.node_id.clone().expect("expected some self.node_id");
                let msg_id = self.msg_id;
                self.guid = format!("{this}-{msg_id}");

                let response = Message {
                    src: this,
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
            Payload::GenerateOk { .. } => {}
            other => bail!("received unexpected request message type: {other:?}"),
        }

        Ok(())
    }
}
