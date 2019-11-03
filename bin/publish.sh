#!/usr/bin/env bash

SCRIPT_BASEDIR=$(dirname "$0")

which rustc &> /dev/null || { echo 'ERROR: rustc not found in PATH'; exit 1; }
which cargo &> /dev/null || { echo 'ERROR: cargo not found in PATH'; exit 1; }

cd "${SCRIPT_BASEDIR}/.."

pushd wallet_lib/ &> /dev/null
cargo publish

popd &> /dev/null
cargo publish
