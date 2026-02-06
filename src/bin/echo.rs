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
//! ~/maelstrom/maelstrom test -w echo --bin target/debug/echo --node-count 1 --time-limit 10
//!
//! cargo build && ~/maelstrom/maelstrom test -w echo --bin target/debug/echo --node-count 1 --time-limit 10
//!
//! cargo build --bin echo && ~/maelstrom/maelstrom test -w echo --bin target/debug/echo --node-count 1 --time-limit 3
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

use anyhow::{bail, Result};
use gossip_glomers::logic::main_loop;
use gossip_glomers::message::{Message, Payload};
use gossip_glomers::node::Node;
use std::fmt::Debug;
use std::io::StdoutLock;

/// # The Echo Node (Server)
///
/// A simple echo workload: a client sends a message, and expects to get that same message back from our server.
///
/// Maelstrom sets the node ID for our node(s), during the initialization phase.
#[derive(Default, Debug)]
pub struct EchoNode {
    /// A locally-unique integer identifier for a message from a node. It isn't globally-unique.
    pub msg_id: usize,
    /// A unique node name. Maelstrom sets the node ID for our node(s), during the initialization phase.
    pub node_id: Option<String>,
}

impl Node for EchoNode {
    fn new() -> Self {
        Self {
            msg_id: 0,
            node_id: None,
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
            Payload::Echo { echo } => {
                let payload = Payload::EchoOk { echo };
                self.respond(request.src, request.body.msg_id, payload, output_lock)?;
            }
            Payload::EchoOk { .. } => {}
            other => bail!("received unexpected request message type: {other:?}"),
        }

        Ok(())
    }
}

fn main() -> Result<()> {
    main_loop::<EchoNode>()
}
