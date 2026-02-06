#!/bin/sh

# A test script that builds all binaries (node types) and tests them all in succession.

# The first test variants are defined and recommended by Fly.io.
# We can shorten the time limits for debugging.

# Fail immediately if any command has a non-zero exit status, or if a variable hasn't been defined.
set -eu

# "debug" or "release" profile
PROFILE="debug"

# Test duration (in seconds)
DURATION=3

# Pause between tests (in seconds)
SLEEP=1.5

# Ensure compile steps are run within the repository directory.
cd "$(dirname "$0")"

if [ "$PROFILE" = "debug" ]; then
  # Debug profile
  cargo build --all-targets
elif [ "$PROFILE" = "release" ]; then
  # Release profile
  cargo build --release --all-targets
else
  echo "Profile should be \"debug\" or \"release\"."
  exit 1
fi

# This makes it so that if a test fails, we continue with other tests that come after it.
# To fail fast, after the first failed test, comment this out.
set +e

# Run all tests (challenges) in succession.

# Echo
printf "\n\n\n\n\n\n"
#~/maelstrom/maelstrom test -w echo --bin target/"$PROFILE"/echo --node-count 1 --time-limit 10
~/maelstrom/maelstrom test -w echo --bin target/"$PROFILE"/echo --node-count 1 --time-limit "$DURATION"

# Unique ID Generator
printf "\n\n\n\n\n\n" && sleep "$SLEEP"
#~/maelstrom/maelstrom test -w unique-ids --bin target/"$PROFILE"/unique_id_gen --time-limit 30 --rate 1000 --node-count 3 --availability total --nemesis partition
~/maelstrom/maelstrom test -w unique-ids --bin target/"$PROFILE"/unique_id_gen --time-limit "$DURATION" --rate 1000 --node-count 3 --availability total --nemesis partition

# Broadcast
printf "\n\n\n\n\n\n" && sleep "$SLEEP"
#~/maelstrom/maelstrom test -w broadcast --bin target/"$PROFILE"/broadcast --node-count 1 --time-limit 20 --rate 10
~/maelstrom/maelstrom test -w broadcast --bin target/"$PROFILE"/broadcast --node-count 1 --time-limit "$DURATION" --rate 10
