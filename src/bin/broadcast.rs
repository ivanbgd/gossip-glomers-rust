//! # The Broadcast Node (Server)
//!
//! In this challenge, you’ll need to implement a broadcast system that gossips messages between all nodes
//! in the cluster. Gossiping is a common way to propagate information across a cluster when you don’t need
//! strong consistency guarantees.
//!
//! [Challenge #3a: Single-Node Broadcast](https://fly.io/dist-sys/3a/)
//!
//! A broadcast system. Essentially a test of eventually-consistent set addition,
//! but also provides an initial `topology` message to the cluster with a set of neighbors for each node to use.
//!
//! [Workload: Broadcast](https://github.com/jepsen-io/maelstrom/blob/main/doc/workloads.md#workload-broadcast)
//!
//! Run as:
//!
//! ```
//! ~/maelstrom/maelstrom test -w broadcast --bin target/debug/broadcast --node-count 1 --time-limit 20 --rate 10
//!
//! cargo build --bin broadcast && ~/maelstrom/maelstrom test -w broadcast --bin target/debug/broadcast --node-count 1 --time-limit 3 --rate 10
//! ```

use anyhow::{bail, Result};
use gossip_glomers::logic::main_loop;
use gossip_glomers::message::{BroadcastPayload, Message, Payload};
use gossip_glomers::node::Node;
use std::fmt::Debug;
use std::io::StdoutLock;

/// # The Broadcast Node (Server)
///
/// A broadcast system. Essentially a test of eventually-consistent set addition,
/// but also provides an initial `topology` message to the cluster with a set of neighbors for each node to use.
#[derive(Default, Debug)]
struct BroadcastNode {
    /// A unique node name. Maelstrom sets the node ID for our node(s), during the initialization phase.
    pub node_id: Option<String>,
    /// A locally-unique integer identifier for a message from a node. It isn't globally-unique.
    pub msg_id: usize,
    /// Broadcast messages
    pub messages: Vec<usize>,
}

impl Node for BroadcastNode {
    fn new() -> Self {
        Self {
            node_id: None,
            msg_id: 0,
            messages: Vec::new(),
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
            Payload::Broadcast(broadcast_paylod) => match broadcast_paylod {
                BroadcastPayload::Broadcast { message } => {
                    self.messages.push(message);
                    let payload = Payload::Broadcast(BroadcastPayload::BroadcastOk);
                    self.respond(request.src, request.body.msg_id, payload, output_lock)?;
                }
                BroadcastPayload::Read => {
                    let payload = Payload::Broadcast(BroadcastPayload::ReadOk {
                        messages: self.messages.clone(),
                    });
                    self.respond(request.src, request.body.msg_id, payload, output_lock)?;
                }
                BroadcastPayload::Topology { .. } => {
                    let payload = Payload::Broadcast(BroadcastPayload::TopologyOk);
                    self.respond(request.src, request.body.msg_id, payload, output_lock)?;
                }
                BroadcastPayload::BroadcastOk
                | BroadcastPayload::ReadOk { .. }
                | BroadcastPayload::TopologyOk => {}
            },
            other => bail!("received unexpected request message type: {other:?}"),
        }

        Ok(())
    }
}

fn main() -> Result<()> {
    main_loop::<BroadcastNode>()
}
