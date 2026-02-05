//! # The Main Logic
//!
//! This belongs to the library and contains the main loop.

use crate::message::{InitPayload, Message, Payload};
use crate::node::Node;
use anyhow::{Context, Result};
use std::fmt::Debug;
use std::io::{self, BufRead};

/// The main library loop.
pub fn main_loop<N>() -> Result<()>
where
    N: Node + Debug,
{
    let mut node: N = Node::new();

    let stdin_lock = io::stdin().lock();
    let mut requests = stdin_lock.lines();
    let mut stdout_lock = io::stdout().lock();

    // The initialization message from Maelstrom must always come first.
    let init_request: Message<InitPayload> = serde_json::from_str(
        &requests
            .next()
            .context("expected an initialization message from maelstrom")?
            .context("failed to read init request from stdin")?,
    )
    .context("deserialization of initialization request message failed")?;
    node.init_response(init_request, &mut stdout_lock)
        .context(format!("{node:?}: init_response method failed"))?;

    // Our node (server) is now ready to receive all other messages (but not an init message again).
    for request in requests {
        let request = request.context("failed to read request from stdin")?;
        let request: Message<Payload> =
            serde_json::from_str(&request).context("deserialization of request message failed")?;
        node.step(request, &mut stdout_lock)
            .context(format!("{node:?}: step method failed"))?;
    }

    Ok(())
}

/// The main library loop - alternative implementation (for reference).
pub fn _main_loop<N>() -> Result<()>
where
    N: Node + Debug,
{
    let mut node: N = Node::new();

    let mut stdout_lock = io::stdout().lock();

    // The initialization message from Maelstrom must always come first.
    let stdin_lock = io::stdin().lock();
    let init_request = serde_json::Deserializer::from_reader(stdin_lock)
        .into_iter::<Message<InitPayload>>()
        .next()
        .context("expected an initialization message from maelstrom")?
        .context("deserialization of initialization request message failed")?;
    node.init_response(init_request, &mut stdout_lock)
        .context(format!("{node:?}: init_response method failed"))?;

    // Our node (server) is now ready to receive all other messages (but not an init message again).
    let stdin_lock = io::stdin().lock();
    let requests =
        serde_json::Deserializer::from_reader(stdin_lock).into_iter::<Message<Payload>>();
    for request in requests {
        let request: Message<Payload> =
            request.context("deserialization of request message failed")?;
        node.step(request, &mut stdout_lock)
            .context(format!("{node:?}: step method failed"))?;
    }

    Ok(())
}
