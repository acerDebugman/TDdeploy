#!/bin/bash
set -e

echo "=========================================="
echo "Setting up Kerberos KDC"
echo "=========================================="

# Install Kerberos server
yum install -y krb5-server krb5-libs krb5-workstation 2>&1 | tail -5

# Create KDC database
echo "Creating KDC database..."
kdb5_util create -r EXAMPLE.COM -s -P password <<EOF
password
password
EOF

# Create admin ACL
cat > /var/kerberos/krb5kdc/kadm5.acl << 'EOF'
admin/admin@EXAMPLE.COM  *
EOF

# Start kadmind in background
echo "Starting kadmind..."
kadmind &

# Start krb5kdc in background
echo "Starting krb5kdc..."
krb5kdc &

# Create principals
echo "Creating principals..."
kadmin.local -q 'addprinc -pw admin admin/admin@EXAMPLE.COM'
kadmin.local -q 'addprinc -randkey zookeeper/zookeeper.example.com@EXAMPLE.COM'
kadmin.local -q 'addprinc -randkey kafka/kafka.example.com@EXAMPLE.COM'
kadmin.local -q 'addprinc -randkey kafka/localhost@EXAMPLE.COM'
kadmin.local -q 'addprinc -pw kafka kafka-user@EXAMPLE.COM'
kadmin.local -q 'addprinc -pw client client@EXAMPLE.COM'

# Export keytabs
mkdir -p /keytabs
echo "Exporting keytabs..."
kadmin.local -q 'ktadd -k /keytabs/zookeeper.keytab zookeeper/zookeeper.example.com@EXAMPLE.COM'
kadmin.local -q 'ktadd -k /keytabs/kafka.keytab kafka/kafka.example.com@EXAMPLE.COM'
kadmin.local -q 'ktadd -k /keytabs/kafka.keytab kafka/localhost@EXAMPLE.COM'
kadmin.local -q 'ktadd -k /keytabs/kafka-user.keytab kafka-user@EXAMPLE.COM'
kadmin.local -q 'ktadd -k /keytabs/client.keytab client@EXAMPLE.COM'
chmod 644 /keytabs/*

echo "=========================================="
echo "Kerberos KDC Setup Complete!"
echo "Principals created:"
kadmin.local -q 'listprincs'
echo "=========================================="
echo "Keytabs:"
ls -la /keytabs/
echo "=========================================="

# Keep the container running
tail -f /dev/null
