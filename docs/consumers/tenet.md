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
attested-workload @ e039216
https://github.com/maceip/attested-workload
```

Do not mix verifier from `runcards` and enclave shim from `bountynet-genesis`.

## Out of scope here

Outfox mixnet, oblivious matcher algorithms, and directory semantics live in
sphinx-tahoe. This repo only provides **TEE + attested TLS + app-proxy**.
