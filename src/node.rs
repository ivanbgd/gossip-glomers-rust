//! # Generic Node

use crate::message::{Body, InitPayload, Message, Payload};
use anyhow::{bail, Context, Result};
use std::io::{StdoutLock, Write};

pub trait Node {
    /// Creates and returns a new node.
    fn new() -> Self;

    fn get_msg_id(&self) -> usize;
    fn incr_msg_id(&mut self);
    fn get_node_id(&self) -> Option<String>;
    fn set_node_id(&mut self, value: Option<String>);

    /// Respond to initialization by Maelstrom.
    fn init_response(
        &mut self,
        request: Message<InitPayload>,
        output_lock: &mut StdoutLock,
    ) -> Result<()> {
        match request.body.payload {
            InitPayload::Init { node_id, .. } => {
                self.set_node_id(Some(node_id));

                let response = Message {
                    src: self.get_node_id().expect("expected some self.node_id"), // == request.dest,
                    dest: request.src,
                    body: Body {
                        msg_id: Some(self.get_msg_id()),
                        in_reply_to: request.body.msg_id,
                        payload: InitPayload::InitOk,
                    },
                };

                serde_json::to_writer(&mut *output_lock, &response)
                    .context("serialization of response init_ok message failed")?;
                output_lock
                    .write_all(b"\n")
                    .context("failed to write newline")?;

                self.incr_msg_id();
            }
            other => bail!("received unexpected request message type: {other:?}"),
        }

        Ok(())
    }

    /// A processing step in a node's state-machine.
    ///
    /// Works with all message types except the initialization-by-Maelstrom message types.
    fn step(&mut self, request: Message<Payload>, output_lock: &mut StdoutLock) -> Result<()>;

    /// Respond to any request that is not initialization.
    ///
    /// Designed to be used inside the [`Node::step()`] method.
    fn respond(
        &mut self,
        req_src: String,
        req_msg_id: Option<usize>,
        payload: Payload,
        output_lock: &mut StdoutLock,
    ) -> Result<()> {
        let response = Message {
            src: self.get_node_id().expect("expected some self.node_id"), // == request.dest,
            dest: req_src,
            body: Body {
                msg_id: Some(self.get_msg_id()),
                in_reply_to: req_msg_id,
                payload,
            },
        };

        serde_json::to_writer(&mut *output_lock, &response)
            .context("serialization of response echo_ok message failed")?;
        output_lock
            .write_all(b"\n")
            .context("failed to write newline")?;

        self.incr_msg_id();

        Ok(())
    }
}
