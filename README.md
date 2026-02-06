# Gossip Glomers in Rust

### Maelstrom Fly.io Distributed Systems Challenges

- My implementation of the [Fly.io distributed systems challenges](https://fly.io/dist-sys/) in Rust.
- Contains implementations for various types of nodes (servers), that Maelstrom clients interact with.
- Tests are run on individual basis, per node type.
- Everything runs locally in our machine.
- The network and its various conditions are simulated by Maelstrom.
- Maelstrom orchestrates clients that send requests to our implementation of nodes.
- Our nodes act as servers and are expected to respond to clients with appropriate messages.
- Various network conditions, including failures, partitions, are induced by Maelstrom.
- Maelstrom collects results in the end and analyzes them.
- It gives us a quick report on command line, with some details, statistics and final verdict, but we can also run it
  as a web server to see more detailed analyses, stats and visualizations in our web browser.

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

- That will build all existing binaries, i.e., node types, at once.
- `cargo build [--release] --bin <node-type>` can be used to build only a specific binary (node type).


- Then we should run Maelstrom as defined for each challenge.
- We need to provide the proper path to our binary, `debug` or `release`.
- For example:

```shell
~/maelstrom/maelstrom test -w echo --bin target/debug/echo --node-count 1 --time-limit 10 [--log-stderr]
# or
~/maelstrom/maelstrom test -w echo --bin target/release/echo --node-count 1 --time-limit 10 [--log-stderr]
```

- Alternatively, we can execute both commands at once.
- For example:

```shell
cargo build --bin echo && ~/maelstrom/maelstrom test -w echo --bin target/debug/echo --node-count 1 --time-limit 10 --log-stderr
# or
cargo build --bin echo && ~/maelstrom/maelstrom test -w echo --bin target/release/echo --node-count 1 --time-limit 10 --log-stderr
```

- Expected output:

```shell
Everything looks good! ヽ(‘ー`)ノ
```

### All Tests

- A shell script is provided to build all binaries (node types) and to test them all in succession.
- It is configurable.
- It contains default values for all tests as specified by Fly.io, and also values for quicker debugging.

```shell
chmod +x ./test.sh  # run once
./test.sh
```

- Or, manually:

```shell
cargo build --all-targets
~/maelstrom/maelstrom test -w echo --bin target/debug/echo --node-count 1 --time-limit 10
~/maelstrom/maelstrom test -w unique-ids --bin target/debug/unique_id_gen --time-limit 30 --rate 1000 --node-count 3 --availability total --nemesis partition
~/maelstrom/maelstrom test -w broadcast --bin target/debug/broadcast --node-count 1 --time-limit 20 --rate 10
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

## Official Documentation

- [Jepsen](https://github.com/jepsen-io/jepsen)
- [Maelstrom](https://github.com/jepsen-io/maelstrom)
- [Maelstrom: Protocol](https://github.com/jepsen-io/maelstrom/blob/main/doc/protocol.md)
- [Maelstrom: Workloads](https://github.com/jepsen-io/maelstrom/blob/main/doc/workloads.md)
- [Maelstrom: Understanding Test Results](https://github.com/jepsen-io/maelstrom/blob/main/doc/results.md)
