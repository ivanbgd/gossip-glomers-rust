#!/bin/sh

# A test script that builds all binaries (node types) and tests them all in succession.

# Fail immediately if any command has a non-zero exit status, or if a variable hasn't been defined.
set -eu

# Test duration (in seconds)
DURATION=3

# Pause between tests (in seconds)
SLEEP=1.5

(
  # Ensure compile steps are run within the repository directory.
  cd "$(dirname "$0")"
  # Debug profile
  cargo build --target-dir=./target/debug/ --manifest-path Cargo.toml
  # Release profile
  #cargo build --release --target-dir=./target/release/ --manifest-path Cargo.toml
)

# Run all tests (challenges) in succession.

# Echo
#~/maelstrom/maelstrom test -w echo --bin target/debug/echo --node-count 1 --time-limit 10
~/maelstrom/maelstrom test -w echo --bin target/debug/echo --node-count 1 --time-limit $DURATION
sleep $SLEEP && printf "\n\n\n\n\n"

# Unique ID Generator
#~/maelstrom/maelstrom test -w unique-ids --bin target/debug/unique_id_gen --time-limit 30 --rate 1000 --node-count 3 --availability total --nemesis partition
~/maelstrom/maelstrom test -w unique-ids --bin target/debug/unique_id_gen --time-limit $DURATION --rate 1000 --node-count 3 --availability total --nemesis partition
sleep $SLEEP && printf "\n\n\n\n\n"
