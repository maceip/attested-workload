#!/usr/bin/env bash
# EIF entry: demo workload on :8080 + attested TLS enclave (app-proxy enabled).
set -euo pipefail

python3 /app/hello-workload.py &
exec aw enclave /app --cmd true
