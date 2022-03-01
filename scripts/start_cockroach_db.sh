#!/bin/sh

# data channel is not encrypted in insecure mode

exec cockroach start-single-node \
--insecure \
--store=db \
--listen-addr=localhost:26257 \
--http-addr=localhost:8080 