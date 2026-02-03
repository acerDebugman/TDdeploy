#!/bin/bash

echo "=========================================="
echo "Testing Kafka with Kerberos/GSSAPI"
echo "=========================================="
echo ""

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

echo "Starting Kafka test client..."
$COMPOSE_CMD run --rm kafka-client bash -c "
echo 'Getting Kerberos ticket...'
kinit -kt /keytabs/kafka-user.keytab kafka-user@EXAMPLE.COM
klist

echo ''
echo 'Creating test topic...'
kafka-topics.sh \
    --bootstrap-server kafka.example.com:9092 \
    --command-config /dev/stdin \
    --create --topic test-topic --partitions 1 --replication-factor 1 <<EOFCONFIG
security.protocol=SASL_PLAINTEXT
sasl.mechanism=GSSAPI
sasl.kerberos.service.name=kafka
sasl.jaas.config=com.sun.security.auth.module.Krb5LoginModule required useKeyTab=true keyTab=\"/keytabs/kafka-user.keytab\" principal=\"kafka-user@EXAMPLE.COM\";
EOFCONFIG

echo ''
echo 'Listing topics...'
kafka-topics.sh \
    --bootstrap-server kafka.example.com:9092 \
    --command-config /dev/stdin \
    --list <<EOFCONFIG
security.protocol=SASL_PLAINTEXT
sasl.mechanism=GSSAPI
sasl.kerberos.service.name=kafka
sasl.jaas.config=com.sun.security.auth.module.Krb5LoginModule required useKeyTab=true keyTab=\"/keytabs/kafka-user.keytab\" principal=\"kafka-user@EXAMPLE.COM\";
EOFCONFIG

echo ''
echo 'Producing message...'
echo 'Hello from Kerberos authenticated Kafka!' | kafka-console-producer.sh \
    --bootstrap-server kafka.example.com:9092 \
    --producer.config /dev/stdin \
    --topic test-topic <<EOFCONFIG
security.protocol=SASL_PLAINTEXT
sasl.mechanism=GSSAPI
sasl.kerberos.service.name=kafka
sasl.jaas.config=com.sun.security.auth.module.Krb5LoginModule required useKeyTab=true keyTab=\"/keytabs/kafka-user.keytab\" principal=\"kafka-user@EXAMPLE.COM\";
EOFCONFIG

echo ''
echo 'Consuming message...'
kafka-console-consumer.sh \
    --bootstrap-server kafka.example.com:9092 \
    --consumer.config /dev/stdin \
    --topic test-topic \
    --from-beginning \
    --max-messages 1 \
    --timeout-ms 10000 <<EOFCONFIG
security.protocol=SASL_PLAINTEXT
sasl.mechanism=GSSAPI
sasl.kerberos.service.name=kafka
sasl.jaas.config=com.sun.security.auth.module.Krb5LoginModule required useKeyTab=true keyTab=\"/keytabs/kafka-user.keytab\" principal=\"kafka-user@EXAMPLE.COM\";
EOFCONFIG

echo ''
echo 'Test completed!'
"

echo ""
echo "=========================================="
echo "Test finished!"
echo "=========================================="
