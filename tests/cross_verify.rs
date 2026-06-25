//! Cross-layer compatibility: the receipts attested-workload emits are in the
//! unified-quote EAT format. This test verifies the runtime's own testdata
//! receipts using the *canonical* `unified-quote` crate's verifier (a real
//! cargo dependency, not this crate's copy) — proving the in-tee runtime and
//! the base layer agree on the wire format and verify identically.

use std::path::Path;

use unified_quote::eat::EatToken;
use unified_quote::quote::verify::verify_platform_quote;

fn load(name: &str) -> Vec<u8> {
    let p = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("testdata/chain")
        .join(name);
    std::fs::read(&p).unwrap_or_else(|e| panic!("read {}: {e}", p.display()))
}

/// Verify a token and walk its build->runtime chain with the base-layer
/// verifier. Returns (all_quotes_verified, value_x_stable, stage_count).
fn verify_with_base_layer(bytes: &[u8]) -> (bool, bool, usize) {
    let token = EatToken::from_cbor(bytes).expect("decode eat with unified-quote");

    let mut chain = vec![token.clone()];
    let mut cursor = token.clone();
    while let Ok(Some(prev)) = cursor.decode_previous() {
        chain.push(prev.clone());
        cursor = prev;
    }

    let all_ok = chain.iter().all(|t| match t.platform_enum() {
        Some(p) if !t.platform_quote.is_empty() => {
            verify_platform_quote(p, &t.platform_quote, &t.binding_bytes()).is_ok()
        }
        // No hardware quote: software witness, not a verification failure here.
        _ => true,
    });

    let value_x_stable = chain.windows(2).all(|w| w[0].value_x == w[1].value_x);
    (all_ok, value_x_stable, chain.len())
}

#[test]
fn tdx_chain_verifies_under_base_layer() {
    let (ok, stable, stages) = verify_with_base_layer(&load("tdx_stage1.cbor"));
    assert!(ok, "tdx chain quotes must verify under the unified-quote verifier");
    assert!(stable, "value_x must be stable across the build->runtime chain");
    assert_eq!(stages, 2, "tdx_stage1 should walk back to stage0");
}

#[test]
fn nitro_receipt_verifies_under_base_layer() {
    let (ok, stable, stages) = verify_with_base_layer(&load("nitro_stage0.cbor"));
    assert!(ok, "nitro quote must verify under the unified-quote verifier");
    assert!(stable);
    assert_eq!(stages, 1);
}

#[test]
#[ignore = "snp verify fetches the VCEK from AMD KDS (needs network); run with --ignored"]
fn snp_chain_verifies_under_base_layer() {
    let (ok, stable, _) = verify_with_base_layer(&load("snp_stage1.cbor"));
    assert!(ok, "snp chain quotes must verify under the unified-quote verifier");
    assert!(stable, "value_x must be stable across the snp chain");
}
