#!/bin/bash

set -e

echo "=========================================="
echo "Starting Kafka Cluster with Kerberos/GSSAPI"
echo "=========================================="
echo ""

# Check if docker-compose or docker compose is available
if command -v docker-compose &> /dev/null; then
    COMPOSE_CMD="docker-compose"
elif docker compose version &> /dev/null; then
    COMPOSE_CMD="docker compose"
else
    echo "Error: docker-compose or docker compose is not installed"
    exit 1
fi

cd "$(dirname "$0")"

echo "Step 1: Starting Kerberos KDC and initializing principals..."
$COMPOSE_CMD up -d kerberos

echo ""
echo "Waiting for Kerberos to be ready..."
sleep 10

# Wait for Kerberos health check
for i in {1..30}; do
    if docker ps | grep -q "kafka-kerberos.*healthy"; then
        echo "Kerberos is healthy!"
        break
    fi
    echo "Waiting for Kerberos to become healthy... ($i/30)"
    sleep 5
done

echo ""
echo "Step 2: Starting Zookeeper..."
$COMPOSE_CMD up -d zookeeper

sleep 5

echo ""
echo "Step 3: Starting Kafka Broker..."
$COMPOSE_CMD up -d kafka

echo ""
echo "Waiting for Kafka to be ready..."
sleep 15

# Check if Kafka is running
for i in {1..20}; do
    if docker ps | grep -q "kafka-broker.*healthy\|Up.*healthy"; then
        echo "Kafka is healthy!"
        break
    fi
    echo "Waiting for Kafka to become ready... ($i/20)"
    sleep 5
done

echo ""
echo "=========================================="
echo "Kafka Cluster with Kerberos is starting up!"
echo "=========================================="
echo ""
echo "Services:"
echo "  - Kerberos KDC:     kerberos.example.com:88 (UDP/TCP)"
echo "  - Zookeeper:        localhost:2181"
echo "  - Kafka Broker:     localhost:9092 (SASL_PLAINTEXT with GSSAPI)"
echo ""
echo "Principals created:"
echo "  - admin/admin@EXAMPLE.COM (password: admin)"
echo "  - kafka/kafka.example.com@EXAMPLE.COM"
echo "  - kafka/localhost@EXAMPLE.COM"
echo "  - zookeeper/zookeeper.example.com@EXAMPLE.COM"
echo "  - kafka-user@EXAMPLE.COM (password: kafka)"
echo "  - client@EXAMPLE.COM (password: client)"
echo ""
echo "Keytab files available in ./kerberos/keytabs/:"
echo "  - kafka.keytab, zookeeper.keytab, kafka-user.keytab, client.keytab"
echo ""
echo "To check status:"
echo "  docker ps"
echo ""
echo "To view logs:"
echo "  $COMPOSE_CMD logs -f kafka"
echo ""
echo "To run test client:"
echo "  ./test-kafka-kerberos.sh"
echo ""
