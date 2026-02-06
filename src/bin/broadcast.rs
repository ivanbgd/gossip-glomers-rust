//! # The Broadcast Node (Server)
//!
//! In this challenge, you’ll need to implement a broadcast system that gossips messages between all nodes
//! in the cluster. Gossiping is a common way to propagate information across a cluster when you don’t need
//! strong consistency guarantees.
//!
//! [Challenge #3a: Single-Node Broadcast](https://fly.io/dist-sys/3a/)
//! [Challenge #3b: Multi-Node Broadcast](https://fly.io/dist-sys/3b/)
//!
//! A broadcast system. Essentially a test of eventually-consistent set addition,
//! but also provides an initial `topology` message to the cluster with a set of neighbors for each node to use.
//!
//! A topology message is sent at the start of the test, after initialization, and informs the node
//! of an optional network topology to use for broadcast.
//! The topology consists of a map of node IDs to lists of neighbor node IDs.
//!
//! [Workload: Broadcast](https://github.com/jepsen-io/maelstrom/blob/main/doc/workloads.md#workload-broadcast)
//!
//! Run as:
//!
//! ```
//! ~/maelstrom/maelstrom test -w broadcast --bin target/debug/broadcast --node-count 5 --time-limit 20 --rate 10
//!
//! cargo build --bin broadcast && ~/maelstrom/maelstrom test -w broadcast --bin target/debug/broadcast --node-count 5 --time-limit 3 --rate 10
//! ```

use anyhow::{bail, Result};
use gossip_glomers::logic::main_loop;
use gossip_glomers::message::{BroadcastPayload, Message, Payload};
use gossip_glomers::node::Node;
use std::collections::{HashMap, HashSet};
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
    /// Network topology sent to us by Maelstrom - a map of node IDs to list of their neighbor node IDs
    pub topology: HashMap<String, Vec<String>>,
    /// Broadcast messages
    pub messages: HashSet<usize>,
}

impl Node for BroadcastNode {
    fn new() -> Self {
        Self {
            node_id: None,
            msg_id: 0,
            topology: HashMap::new(),
            messages: HashSet::new(),
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
                    self.messages.insert(message);

                    let payload = Payload::Broadcast(BroadcastPayload::BroadcastOk);
                    self.respond(
                        request.src,
                        request.body.msg_id,
                        payload,
                        output_lock,
                        "broadcast_ok",
                    )?;

                    let node_id = self.node_id.clone().expect("expected some self.node_id");
                    if let Some(neighbors) = self.topology.get(&node_id) {
                        for neighbor in neighbors.clone() {
                            let payload =
                                Payload::Broadcast(BroadcastPayload::Broadcast { message });
                            self.request(neighbor, payload, output_lock, "broadcast")?;
                        }
                    }
                }
                BroadcastPayload::Read => {
                    let payload = Payload::Broadcast(BroadcastPayload::ReadOk {
                        messages: self.messages.clone(),
                    });
                    self.respond(
                        request.src,
                        request.body.msg_id,
                        payload,
                        output_lock,
                        "read_ok",
                    )?;
                }
                BroadcastPayload::Topology { topology } => {
                    self.topology = topology;

                    let payload = Payload::Broadcast(BroadcastPayload::TopologyOk);
                    self.respond(
                        request.src,
                        request.body.msg_id,
                        payload,
                        output_lock,
                        "topology_ok",
                    )?;
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
