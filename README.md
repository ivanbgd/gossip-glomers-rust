# Gossip Glomers in Rust

### Maelstrom Fly.io Distributed Systems Challenges

My implementation of the [Fly.io distributed systems challenges](https://fly.io/dist-sys/) in Rust.

## Prerequisites

- Requires Java Development Kit (JDK), such as OpenJDK,
  and [Maelstrom v0.2.3](https://github.com/jepsen-io/maelstrom/releases/tag/v0.2.3).
- Graphviz and Gnuplot are optional.
- Check out the
  [reference document](https://github.com/jepsen-io/maelstrom/blob/main/doc/01-getting-ready/index.md#prerequisites).
- I'd suggest to install at least Gnuplot, because Maelstrom reports errors in its output when Gnuplot isn't installed,
  and that makes it a little harder to discern whether tests succeeded or not.

## Building and Running

- We first need to build our implementation, in `Debug` or `Release` profile.

```shell
cargo build
# or
cargo build --release
```

- Then we should run Maelstrom as defined for each challenge.
- We need to provide the proper path to our binary, `debug` or `release`.
- For example:

```shell
~/maelstrom/maelstrom test -w echo --bin target/debug/gossip-glomers --node-count 1 --time-limit 10 [--log-stderr]
# or
~/maelstrom/maelstrom test -w echo --bin target/release/gossip-glomers --node-count 1 --time-limit 10 [--log-stderr]
```

- Alternatively, we can execute both commands at once.
- For example:

```shell
cargo build && ~/maelstrom/maelstrom test -w echo --bin target/debug/gossip-glomers --node-count 1 --time-limit 10 --log-stderr
# or
cargo build && ~/maelstrom/maelstrom test -w echo --bin target/release/gossip-glomers --node-count 1 --time-limit 10 --log-stderr
```
