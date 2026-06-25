# attested-workload

a workload inside the tee, served over attested tls.

run an http service **inside a cloud tee**, prove what code is running with a
hardware quote, and serve it over **attested tls** — the certificate spki is
bound into the quote. one engine for **aws nitro**, **amd sev-snp**, and
**intel tdx**. a co-located app on loopback can share the same attested channel
(app-proxy).

## commands

| command | role |
|---|---|
| `aw build` | attested build inside a tee (stage 0) |
| `aw run` | attested runtime with attested tls (stage 1) |
| `aw check <url>` | verify a live endpoint (quote + spki channel binding) |
| `aw check --json <url>` | machine-readable verification (+ `tls_spki_hash`) |
| `aw enclave` | nitro: build + serve in one enclave process |
| `aw proxy --cid N` | nitro parent: tcp:443 → vsock (tls terminates in the enclave) |

`bountynet` is a compatibility alias for the same `aw` binary.

## three checks

1. platform measurement matches (pcr0 / snp measurement / tdx mrtd).
2. value x matches (sha384 over the workload source tree).
3. tls spki is bound into the hardware quote (`sha256(cert_spki) == eat.tls_spki_hash`).

## quick start

```bash
git clone https://github.com/maceip/attested-workload
cd attested-workload && cargo build --release && cargo test
./target/release/aw check --json https://<host>/
```

## platform status

| platform | model | notes |
|---|---|---|
| aws nitro | isolated enclave + vsock | live validated |
| amd sev-snp | confidential vm | whole-vm tls |
| intel tdx | confidential vm | whole-vm tls |

## the stack

- agent platform — [cvm-agent](https://github.com/maceip/cvm-agent)
- attestation service — [attestation-service](https://github.com/maceip/attestation-service)
- quote format — [unified-quote](https://github.com/maceip/unified-quote)
- in-tee runtime — **attested-workload** (here)

pages: https://maceip.github.io/attested-workload/
