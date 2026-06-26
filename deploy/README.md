# Deploy

## Build the enclave shim binary

From this repository:

```bash
cargo build --release --bin aw   # alias of aw
cp target/release/aw ./aw-bin
```

## AWS Nitro

```bash
./deploy/nitro-deploy.sh
```

Builds `Dockerfile.enclave`, runs `nitro-cli`, starts parent `aw proxy`.

Environment:

- `IMAGE`, `EIF`, `CPU_COUNT`, `MEMORY_MIB`, `PROXY_PORT`, `ACME_FLAG`

## AMD SEV-SNP (Azure)

```bash
./deploy/azure-cvm.sh
```

## Intel TDX (GCP)

```bash
./deploy/gcp-tdx.sh
```

## Generic EIF with hello workload

```dockerfile
# See Dockerfile.enclave — copies aw-bin + optional /src for Value X.
# Launch examples/hello-workload.py on :8080 before or alongside aw enclave.
```

## Client verification (any machine)

```bash
aw check --json https://<host>/
```

Use `tls_spki_hash` from JSON to pin subsequent connections (bootstrap once, then cheap).
