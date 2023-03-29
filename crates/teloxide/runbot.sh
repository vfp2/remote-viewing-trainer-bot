#!/bin/sh
export TELOXIDE_TOKEN="6053787818:AAH7hT3IsWfVEMegGmrYvHkzl6K33I-p8z4"
RUST_LOG=info cargo run --features="full" --example rvt_bot
