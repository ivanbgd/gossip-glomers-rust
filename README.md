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
- In `Cargo.toml`, in the `[features]` section, set the `default` to any *one* of the provided node types -
  the one that you wish to test.

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

- Expected output:

```shell
Everything looks good! ヽ(‘ー`)ノ
```

### All Tests

```shell
cargo build && ~/maelstrom/maelstrom test -w echo --bin target/debug/gossip-glomers --node-count 1 --time-limit 10
cargo build && ~/maelstrom/maelstrom test -w unique-ids --bin target/debug/gossip-glomers --time-limit 30 --rate 1000 --node-count 3 --availability total --nemesis partition
```

## Debugging Maelstrom

- It is possible to run the Maelstrom web server to view our results in more depth.

```shell
~/maelstrom/maelstrom serve
```

- We can then open a web browser to http://localhost:8080 to see results.
- Consult the Maelstrom [result documentation](https://github.com/jepsen-io/maelstrom/blob/main/doc/results.md)
  for further details.
- It is beneficial to have the graph/plotting libraries installed, because of generated visualizations.
