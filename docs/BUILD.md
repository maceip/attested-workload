# Reproducible Enclave Build

How to verify that a running enclave matches source code.

## Prerequisites

- AWS Nitro-enabled instance (e.g., m5.xlarge with enclave support)
- Docker, `nitro-cli`, Rust toolchain

## Build Steps

From this repository:

```bash
cargo build --release --bin bountynet
cp target/release/bountynet ./bountynet-bin
docker build -t aw-demo -f deploy/Dockerfile.enclave .
nitro-cli build-enclave --docker-uri aw-demo:latest --output-file aw-demo.eif
```

Or use the all-in-one script on a Nitro instance:

```bash
./deploy/nitro-deploy.sh
```

The EIF build prints **PCR0** (sha384 hash of the enclave image):

```json
{
  "Measurements": {
    "PCR0": "6a7a3ec78ff901bc2edbd7f0a5b091b1e4c7ab4f459644b0c8271574c1ae918c58e33928579d0106003ec880e0ac0a56"
  }
}
```

Approve PCR0 (or the reported **Value X** from a live `aw check`) in your consumer's
trust policy.

## Verification

A verifier rebuilds the same EIF from the same git commit and compares PCR0, then
checks the live endpoint:

```bash
git clone https://github.com/maceip/attested-workload.git
cd attested-workload
git checkout <pinned-sha>
cargo build --release --bin bountynet
cp target/release/bountynet ./bountynet-bin
docker build -t verify -f deploy/Dockerfile.enclave .
nitro-cli build-enclave --docker-uri verify:latest --output-file verify.eif
# PCR0 must match the running enclave

aw check --json https://<domain>/
```

The TEE signature chain proves the attestation came from real hardware. PCR0 proves
the enclave image matches what you built. Value X proves the measured source tree
inside the enclave matches what you hashed.

## What Each PCR Measures (Nitro)

| PCR | Measures |
|-----|----------|
| PCR0 | Enclave image (hash of EIF — kernel + ramdisk + application) |
| PCR1 | Linux kernel and boot ramfs |
| PCR2 | Application (Docker layer — binary + workload) |

## Running

```bash
nitro-cli run-enclave --eif-path aw-demo.eif --memory 3500 --cpu-count 2
CID=$(nitro-cli describe-enclaves | python3 -c 'import sys,json;print(json.load(sys.stdin)[0]["EnclaveCID"])')
bountynet proxy --cid "$CID" --port 443 --acme
```

## Consumers

Application projects (e.g. [tenet](docs/consumers/tenet.md)) supply their own EIF
Dockerfile that copies `bountynet-bin` from this repo and runs their loopback
workload on `:8080` for app-proxy.
