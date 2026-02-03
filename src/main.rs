//! # Gossip Glomers

use anyhow::{Context, Result};
use gossip_glomers::echo::EchoNode;
use gossip_glomers::message::Message;
use std::io;

fn main() -> Result<()> {
    let stdin = io::stdin().lock();
    let input_msgs = serde_json::Deserializer::from_reader(stdin).into_iter::<Message>();

    let mut stdout = io::stdout().lock();

    let mut node = {
        #[cfg(feature = "echo_node")]
        let node = EchoNode { id: 0 };
        #[cfg(feature = "unique_id_gen_node")]
        let node = todo!();
        node
    };

    for input in input_msgs {
        let input = input.context("failed to deserialize input message")?;
        node.step(input, &mut stdout);
    }

    Ok(())
}
