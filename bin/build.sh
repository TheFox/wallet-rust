#!/usr/bin/env bash

SCRIPT_BASEDIR=$(dirname "$0")

which rustc &> /dev/null || { echo 'ERROR: rustc not found in PATH'; exit 1; }
which cargo &> /dev/null || { echo 'ERROR: cargo not found in PATH'; exit 1; }
which strip &> /dev/null || { echo 'ERROR: strip not found in PATH'; exit 1; }

cd "${SCRIPT_BASEDIR}/.."

# cargo build -Z unstable-options --build-plan > build_plan.json
cargo build --release
ls -la target/release/wallet

strip target/release/wallet
ls -la target/release/wallet
