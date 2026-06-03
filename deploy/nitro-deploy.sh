#!/usr/bin/env bash
# AWS Nitro deploy for attested-workload (EIF + nitro-cli + parent proxy).
#
# Fresh attestation quotes are generated inside the enclave at run time.
# Verification (`aw check`) works on any machine via test fixtures; only
# quote generation needs real Nitro hardware.
#
# Prereqs: Nitro-enabled EC2, docker, aws-nitro-enclaves-cli.
# Build from repo root:
#   cargo build --release --bin bountynet
#   cp target/release/bountynet ./bountynet-bin
set -euo pipefail

IMAGE="${IMAGE:-matcher-enclave}"
EIF="${EIF:-matcher.eif}"
CPU_COUNT="${CPU_COUNT:-2}"
MEMORY_MIB="${MEMORY_MIB:-2048}"
PROXY_PORT="${PROXY_PORT:-443}"
ACME_FLAG="${ACME_FLAG:---acme}"   # set to "" to skip Let's Encrypt (e.g. staging)

echo "[deploy] one-time host setup (idempotent)"
sudo amazon-linux-extras install aws-nitro-enclaves-cli -y 2>/dev/null || true
sudo systemctl enable --now nitro-enclaves-allocator

echo "[deploy] build the EIF (reproducible -> PCR0/Value X)"
docker build -t "${IMAGE}:latest" -f deploy/Dockerfile.enclave .
nitro-cli build-enclave --docker-uri "${IMAGE}:latest" --output-file "${EIF}"
# Record the measurements; PCR0 is the value to approve in the trust registry
# and to set as approved_value_x in the client EnclaveTrustPolicy.
nitro-cli describe-eif --eif-path "${EIF}" | python3 -c \
  'import sys,json;m=json.load(sys.stdin)["Measurements"];print("[deploy] PCR0 =",m["PCR0"])'

echo "[deploy] run the enclave"
sudo nitro-cli run-enclave \
    --cpu-count "${CPU_COUNT}" --memory "${MEMORY_MIB}" --eif-path "${EIF}"
CID=$(nitro-cli describe-enclaves \
    | python3 -c 'import sys,json;print(json.load(sys.stdin)[0]["EnclaveCID"])')
echo "[deploy] enclave CID = ${CID}"

echo "[deploy] start the parent vsock bridge (TLS terminates IN the enclave)"
# The parent sees only ciphertext; it provisions the cert via ACME but the TLS
# key and termination live inside the enclave (src/net/vsock.rs app-proxy).
bountynet proxy --cid "${CID}" --port "${PROXY_PORT}" ${ACME_FLAG}

echo "[deploy] up. verify from a client with:  aw check --json https://<this-host>"
