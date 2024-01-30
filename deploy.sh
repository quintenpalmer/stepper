#!/bin/bash

cargo check
cargo doc
cargo build --release

cp target/release/stepper ~/.bin/
