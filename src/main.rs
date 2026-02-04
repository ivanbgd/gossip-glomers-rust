//! # Gossip Glomers

use anyhow::{Context, Result};
use gossip_glomers::message::Message;
use gossip_glomers::IdType;
use std::io;

fn main() -> Result<()> {
    let stdin_lock = io::stdin().lock();
    let requests = serde_json::Deserializer::from_reader(stdin_lock).into_iter::<Message<_>>();

    let mut stdout_lock = io::stdout().lock();

    let mut node = {
        #[cfg(feature = "echo_node")]
        let node = gossip_glomers::echo::EchoNode::<IdType>::new();
        #[cfg(feature = "unique_id_gen_node")]
        let node = gossip_glomers::unique_id_gen::UniqueIDGeneratorNode::<IdType>::new();
        node
    };

    for request in requests {
        let request: Message<_> = request.context("deserialization of request message failed")?;
        node.step(request, &mut stdout_lock)
            .context(format!("{node:?}: step method failed"))?;
    }

    Ok(())
}
