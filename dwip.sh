#!/bin/bash

docker exec zgcdev1 /bin/bash -c "cd /app/TDdeploy && git config --global --add safe.directory /app/TDdeploy && bash wip_commit.sh"
