#!/bin/bash

git pull

#cargo run -p jira_case --bin ts5820main
#cargo run -p jira_case --bin jira_case
cargo run --package jira_case --bin jira_case

