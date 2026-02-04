//! # Gossip Glomers

use anyhow::{Context, Result};
use gossip_glomers::message::{InitPayload, Message, Payload};
use gossip_glomers::node::Node;
use std::io;

fn main() -> Result<()> {
    let mut node = {
        #[cfg(feature = "echo_node")]
        let node = gossip_glomers::echo::EchoNode::new();
        #[cfg(feature = "unique_id_gen_node")]
        let node = gossip_glomers::unique_id_gen::UniqueIDGeneratorNode::new();
        node
    };

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
