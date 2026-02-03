//! # Gossip Glomers

use anyhow::{Context, Result};
use gossip_glomers::echo::EchoNode;
use gossip_glomers::message::Message;
use std::io;

fn main() -> Result<()> {
    let stdin_lock = io::stdin().lock();
    let requests = serde_json::Deserializer::from_reader(stdin_lock).into_iter::<Message>();

    let mut stdout_lock = io::stdout().lock();

    let mut node = {
        #[cfg(feature = "echo_node")]
        let node = EchoNode::new();
        #[cfg(feature = "unique_id_gen_node")]
        let node = todo!();
        node
    };

    for request in requests {
        let request = request.context("deserialization of input message failed")?;
        node.step(request, &mut stdout_lock)
            .context("node step method failed")?;
    }

    Ok(())
}
