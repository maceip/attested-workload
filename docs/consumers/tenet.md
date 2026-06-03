# tenet

[tenet](https://github.com/maceip/sphinx-tahoe) (package name `por`) is a
privacy-preserving expert-routing network. Its **matcher/mailbox** is a Python
HTTP service that must run inside a TEE without the operator reading traffic.

## How tenet uses attested-workload

1. **Build EIF** with `bountynet-bin` from this repo and tenet's matcher entry
   (`deploy/run_matcher.py` in sphinx-tahoe).
2. **Inside enclave:** matcher listens on `127.0.0.1:8080`; `aw enclave` serves
   attested TLS and **app-proxies** `/v1/*` to it.
3. **Client:** `por/enclave_attest.py` shells out to `aw check --json`, reads
   `value_x` + `tls_spki_hash`, then talks to `/v1/match` over pinned TLS.

## Pinning

Record in sphinx-tahoe `DEPENDENCIES.md`:

```text
attested-workload @ 79a5ea2328f2b30192e57b53913355dcd5e0201e
https://github.com/maceip/attested-workload
```

Do not mix verifier from `runcards` and enclave shim from `bountynet-genesis`.

## Live deployment (2026-06-03)

| Field | Value |
|-------|-------|
| URL | https://d851588d3b41.aeon.site/ |
| Value X | `d851588d3b413cbf7513d9d5fa93d466b42ad1603e1c7fdfd408cfd635a7cf6882412ce99c8fbb3aeac197c3e6c5f361` |
| tls_spki_hash | `b880512378622821deebd4cb395a82eae271069acd491b805940145c97d1eab1` |
| Instance | AWS Nitro, eu-central-1 (`tenet-matcher-nitro`) |

```bash
aw check --json https://d851588d3b41.aeon.site/
curl -s https://d851588d3b41.aeon.site/healthz
```

Validation log: sphinx-tahoe `deploy/HARDWARE_VALIDATION_2026-06-03.md`.

## Out of scope here

Outfox mixnet, oblivious matcher algorithms, and directory semantics live in
sphinx-tahoe. This repo only provides **TEE + attested TLS + app-proxy**.
