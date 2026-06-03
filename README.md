# attested-workload

Run an HTTP service **inside a cloud TEE**, prove what code is running with a
hardware quote, and serve it over **attested TLS** where the certificate SPKI is
bound into the quote.

One engine for **AWS Nitro Enclaves**, **AMD SEV-SNP**, and **Intel TDX**. A
co-located app on loopback can share the same attested channel (app-proxy).

This repository is the **infrastructure layer**. Application projects (e.g.
[tenet](https://github.com/maceip/sphinx-tahoe)) consume it; it is not tied to
LLM hackathons, agent cards, or on-chain governance.

## What you get

| Command | Role |
|---------|------|
| `aw build` | Stage 0: attested build inside a TEE (ratchet + Value X) |
| `aw run` | Stage 1: attested runtime with attested TLS |
| `aw check <url>` | Verify a live endpoint (quote + SPKI channel binding) |
| `aw check --json <url>` | Machine-readable verification (+ `tls_spki_hash` for pinning) |
| `aw enclave` | Nitro: build + serve in one enclave process |
| `aw proxy --cid N` | Nitro parent: TCP:443 → vsock (TLS terminates **in** enclave) |

The `bountynet` binary name is a **compatibility alias** for the same `aw` binary
(older EIF images and deploy scripts).

## Three checks (see `docs/INVARIANT.md`)

1. **Platform measurement** matches (PCR0 / SNP measurement / TDX MRTD).
2. **Value X** matches (sha384 over the workload source tree).
3. **TLS SPKI** is bound into the hardware quote (`sha256(cert_spki) == eat.tls_spki_hash`).

## App-proxy (co-located HTTP workload)

On Nitro, attested TLS terminates inside the enclave. Application HTTP (your
matcher, API, etc.) listens on **`127.0.0.1:8080`**. The enclave forwards:

- `/`, `/eat`, KMS, ACME — attestation plane (unchanged)
- `/v1/*`, `/healthz` — **app-proxy** to the loopback workload (streaming)

See `examples/hello-workload.py` for a minimal workload.

## Quick start (local development)

```bash
git clone https://github.com/maceip/attested-workload
cd attested-workload
cargo build --release
cargo test
```

Verify against a running attested endpoint:

```bash
./target/release/aw check --json https://your-host/
```

## Nitro deploy (summary)

On a Nitro-enabled EC2 instance:

```bash
cargo build --release --bin bountynet
cp target/release/bountynet ./bountynet-bin
./deploy/nitro-deploy.sh
```

Full detail: `deploy/README.md`, `docs/BUILD.md`, `docs/STAGES.md`.

## Platform status

| Platform | Model | Deploy script | Notes |
|----------|-------|---------------|-------|
| AWS Nitro | Isolated enclave + vsock | `deploy/nitro-deploy.sh` | App-proxy supported |
| AMD SEV-SNP | Confidential VM | `deploy/azure-cvm.sh` | Whole-VM TLS |
| Intel TDX | Confidential VM | `deploy/gcp-tdx.sh` | Whole-VM TLS |

Hardware regression fixtures: `testdata/chain/` (see `docs/HARDWARE_VALIDATION.md`).

## Lineage

Extracted and focused from the attestation engines previously spread across
`bountynet-genesis`, `unified-quote`, and `runcards`. Those repos may continue
as product experiments; **this repo is the shared TEE runtime** other projects
should pin.

## Consumers

- **[tenet](docs/consumers/tenet.md)** — privacy-preserving expert routing; matcher
  runs as the loopback workload behind app-proxy.

## License

MIT
