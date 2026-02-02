#!/bin/bash

echo "Stopping Kafka Cluster with Kerberos..."

cd "$(dirname "$0")"

# Check if docker-compose or docker compose is available
if command -v docker-compose &> /dev/null; then
    COMPOSE_CMD="docker-compose"
elif docker compose version &> /dev/null; then
    COMPOSE_CMD="docker compose"
else
    echo "Error: docker-compose or docker compose is not installed"
    exit 1
fi

$COMPOSE_CMD down -v

echo "Kafka cluster stopped and volumes removed."
