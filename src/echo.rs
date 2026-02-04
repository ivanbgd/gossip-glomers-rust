//! # The Echo Node (Server)
//!
//! [Challenge #1: Echo](https://fly.io/dist-sys/1/)
//!
//! A simple echo workload: a client sends a message, and expects to get that same message back from our server.
//!
//! https://github.com/jepsen-io/maelstrom/blob/main/doc/workloads.md#workload-echo
//!
//! Run as:
//!
//! ```
//! cargo build && ~/maelstrom/maelstrom test -w echo --bin target/debug/gossip-glomers --node-count 1 --time-limit 10
//! ```
//!
//! This command instructs Maelstrom to run the `echo` workload against our binary.
//! It runs a single node, and it will send `echo` commands for 10 seconds.
//!
//! Maelstrom will only inject network failures, and it will not intentionally crash our node process,
//! so we don’t need to worry about persistence.
//! We can use in-memory data structures for these challenges.
//!
//! Everything looks good! ヽ(‘ー`)ノ

use crate::message::{Body, Message, Payload};
use anyhow::{bail, Context, Result};
use serde::Serialize;
use std::fmt::Debug;
use std::io::{StdoutLock, Write};
use std::marker::PhantomData;

/// # The Echo Node (Server)
///
/// A simple echo workload: a client sends a message, and expects to get that same message back from our server.
///
/// Maelstrom sets the node ID for our node(s), during the initialization phase.
#[derive(Default, Debug)]
pub struct EchoNode<ID> {
    /// A locally-unique integer identifier for a message from a node. It isn't globally-unique.
    pub msg_id: usize,
    /// A unique node name. Maelstrom sets the node ID for our node(s), during the initialization phase.
    pub node_id: Option<String>,
    /// Required as we don't use the `ID` generic type parameter in this node type.
    phantom: PhantomData<ID>,
}

impl<ID> EchoNode<ID>
where
    ID: Debug + Default + Serialize,
{
    /// Creates and returns a new node.
    pub const fn new() -> Self {
        Self {
            msg_id: 0,
            node_id: None,
            phantom: PhantomData,
        }
    }

    /// A processing step in a node's state-machine.
    pub fn step(&mut self, request: Message<ID>, output_lock: &mut StdoutLock) -> Result<()> {
        match request.body.payload {
            Payload::Echo { echo } => {
                let response = Message::<ID> {
                    src: self.node_id.clone().expect("expected some self.node_id"), // == request.dest,
                    dest: request.src,
                    body: Body {
                        msg_id: Some(self.msg_id),
                        in_reply_to: request.body.msg_id,
                        payload: Payload::EchoOk { echo },
                    },
                };

                serde_json::to_writer(&mut *output_lock, &response)
                    .context("serialization of response echo_ok message failed")?;
                output_lock
                    .write_all(b"\n")
                    .context("failed to write newline")?;

                self.msg_id += 1;
            }
            Payload::EchoOk { .. } => {}
            Payload::Init { node_id, .. } => {
                self.node_id = Some(node_id);

                let response = Message {
                    src: self.node_id.clone().expect("expected some self.node_id"), // == request.dest,
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
