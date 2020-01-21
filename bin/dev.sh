#!/usr/bin/env bash

SCRIPT_BASEDIR=$(dirname "$0")

which rustc &> /dev/null || { echo 'ERROR: rustc not found in PATH'; exit 1; }
which cargo &> /dev/null || { echo 'ERROR: cargo not found in PATH'; exit 1; }

cd "${SCRIPT_BASEDIR}/.."

mkdir -p tmp

export RUST_BACKTRACE=full
export RUSTFLAGS=-Awarnings

set -x

cargo run -- "${@}"
