//! Keys wrapped by `sekai-keys` 1.0.0, the crate this one was extracted from,
//! unwrap here unchanged. The fixtures are a development KEK file and one
//! wrapped DEK written by that release; they hold no production key.

use std::fs;
use std::path::PathBuf;

use envelope_keys::{
    FileKeyProvider, KeyProvider, KeyPurpose, RuntimeProfile, TenantId, UnwrapGrant, WrappedDek,
};

const FIXTURES: &str = concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/tests/fixtures/sekai-keys-1.0.0"
);

/// Copies the fixture KEK into a per-test directory, so parallel runs never
/// share a provider file or its lock.
fn provider_copy(test: &str) -> (PathBuf, FileKeyProvider) {
    let dir = std::env::temp_dir().join(format!(
        "envelope-keys-compat-{}-{test}-{}",
        std::process::id(),
        rand_suffix()
    ));
    fs::create_dir_all(&dir).expect("temp dir");
    let kek = dir.join("provider.kek");
    fs::copy(format!("{FIXTURES}/provider.kek"), &kek).expect("copy kek");
    let provider = FileKeyProvider::open(&kek, RuntimeProfile::Development).expect("open kek");
    (dir, provider)
}

fn rand_suffix() -> u64 {
    use std::time::{SystemTime, UNIX_EPOCH};
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("clock")
        .as_nanos() as u64
}

fn fixture_wrapped() -> WrappedDek {
    let json = fs::read_to_string(format!("{FIXTURES}/wrapped-dek.json")).expect("read wrapped");
    serde_json::from_str(&json).expect("decode wrapped")
}

fn grant() -> UnwrapGrant {
    UnwrapGrant {
        tenant_id: TenantId::new("tenant-a").expect("tenant"),
        purpose: KeyPurpose::Store,
        expires_unix_secs: i64::MAX,
    }
}

#[test]
fn unwraps_a_dek_wrapped_by_sekai_keys_1_0_0() {
    let (dir, provider) = provider_copy("unwrap");
    let wrapped = fixture_wrapped();
    assert_eq!(provider.describe().kek_id, wrapped.kek_id);

    let outcome = provider
        .unwrap(&wrapped, &grant(), "compat", 0)
        .expect("unwrap");
    let expected: [u8; 32] = std::array::from_fn(|i| i as u8);
    assert_eq!(outcome.dek.as_bytes(), &expected);

    drop(provider);
    fs::remove_dir_all(dir).expect("cleanup");
}

#[test]
fn wrapped_dek_json_round_trips_unchanged() {
    let json = fs::read_to_string(format!("{FIXTURES}/wrapped-dek.json")).expect("read wrapped");
    let wrapped: WrappedDek = serde_json::from_str(&json).expect("decode wrapped");
    let again = serde_json::to_string_pretty(&wrapped).expect("encode wrapped") + "\n";
    assert_eq!(again, json);
}
