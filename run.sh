#!/bin/bash

git pull

# for kafka
#cargo run -p jira_case --bin ts5820main

#RUST_LOG=info cargo run -p jira_case --bin jira_case

# example
#RUST_LOG=info cargo run -p jira_case --example consumer_json
#RUST_LOG=info cargo run -p jira_case --example producer_json_loop
# for mqtt
#cargo run -p test_rust_conn --bin flashmq_pub_taosx
# for tuya
#RUST_LOG=info cargo run -p jira_case --example pulsar_producer_tuya_loop

# for pulsar
RUST_LOG=info cargo run -p jira_case --example pulsar_producer_loop
