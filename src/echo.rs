//! # The Echo Node (Server)
//!
//! A simple echo workload: a client sends a message, and expects to get that same message back from our server.
//!
//! https://github.com/jepsen-io/maelstrom/blob/main/doc/workloads.md#workload-echo

use crate::message::{Body, Message, Payload};
use anyhow::{bail, Context, Result};
use std::io::{StdoutLock, Write};

/// A simple echo workload: a client sends a message, and expects to get that same message back from our server.
///
/// Maelstrom sets the node ID for our node(s), during the initialization phase.
#[derive(Default, Debug)]
pub struct EchoNode {
    pub msg_id: usize,
    pub node_id: Option<String>,
}

impl EchoNode {
    pub const fn new() -> Self {
        Self {
            msg_id: 0,
            node_id: None,
        }
    }

    pub fn step(&mut self, request: Message, output_lock: &mut StdoutLock) -> Result<()> {
        match request.body.payload {
            Payload::Echo { echo } => {
                let response = Message {
                    src: self.node_id.clone().expect("expected some self.node_id"), // == request.dest,
                    dest: request.src,
                    body: Body {
                        msg_id: Some(self.msg_id),
                        in_reply_to: request.body.msg_id,
                        payload: Payload::EchoOk { echo },
                    },
                };

                serde_json::to_writer(&mut *output_lock, &response)
                    .context("serialization of output echo_ok message failed")?;
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
                        payload: Payload::InitOk,
                    },
                };

                serde_json::to_writer(&mut *output_lock, &response)
                    .context("serialization of output init_ok message failed")?;
                output_lock
                    .write_all(b"\n")
                    .context("failed to write newline")?;

                self.msg_id += 1;
            }
            other => bail!("received unexpected input message type: {other:?}"),
        }

        Ok(())
    }
}
