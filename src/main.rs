//! # Gossip Glomers

use anyhow::Result;
use gossip_glomers::logic::main_loop;

fn main() -> Result<()> {
    main_loop()
}
