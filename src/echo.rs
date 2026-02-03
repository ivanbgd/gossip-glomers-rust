//! # The Echo Node

use crate::message::Message;
use std::io::StdoutLock;

pub struct EchoNode {
    pub id: usize,
}

impl EchoNode {
    pub fn step(&mut self, input_msg: Message, output: &mut StdoutLock) {
        todo!()
    }
}
