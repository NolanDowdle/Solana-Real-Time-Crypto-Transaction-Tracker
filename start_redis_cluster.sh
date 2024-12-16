#!/usr/bin/env bash

# Start all Redis instances in the background
redis-server redis-cluster/7001/redis.conf &
redis-server redis-cluster/7002/redis.conf &
redis-server redis-cluster/7003/redis.conf &
redis-server redis-cluster/7004/redis.conf &
redis-server redis-cluster/7005/redis.conf &
redis-server redis-cluster/7006/redis.conf &

# Wait for Redis instances to fully start
echo "Waiting for Redis instances to start..."
sleep 5

# Create the Redis cluster
echo "Creating Redis cluster..."
yes "yes" | redis-cli --cluster create \
127.0.0.1:7001 \
127.0.0.1:7002 \
127.0.0.1:7003 \
127.0.0.1:7004@7001 \
127.0.0.1:7005@7002 \
127.0.0.1:7006@7003 \
--cluster-replicas 0

# Confirm cluster creation
echo "Cluster creation complete!"
