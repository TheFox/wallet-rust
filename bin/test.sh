#!/usr/bin/env bash

SCRIPT_BASEDIR=$(dirname "$0")
TEST_OPTS=${TEST_OPTS:-}

which rustc &> /dev/null || { echo 'ERROR: rustc not found in PATH'; exit 1; }
which cargo &> /dev/null || { echo 'ERROR: cargo not found in PATH'; exit 1; }
which strip &> /dev/null || { echo 'ERROR: strip not found in PATH'; exit 1; }

cd "${SCRIPT_BASEDIR}/.."

#export RUST_BACKTRACE=full
export RUST_BACKTRACE=1
#export RUSTFLAGS=-Awarnings

echo "TEST_OPTS: '${TEST_OPTS}'"

# Clean up
rm -rf tmp/tests

mkdir -p tmp/tests

set -x

pushd wallet_lib/ &> /dev/null
cargo test ${TEST_OPTS} --lib $*

# popd &> /dev/null
# cargo test ${TEST_OPTS}
