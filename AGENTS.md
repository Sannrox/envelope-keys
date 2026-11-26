# Repository guidelines

`envelope-keys` is one library crate: the envelope-encryption key-provider
contract, development providers, and a conformance suite. Read `README.md`
and `docs/decisions/` before changing the contract.

## Commands

- `cargo fmt --all -- --check`
- `cargo test --locked`
- `cargo clippy --locked --all-targets -- -D warnings`
- `bash scripts/validate-release.sh` (release scripts and workflows)

Set `CARGO_TARGET_DIR` to this checkout's `target/`.

## Rules

- The envelope is the contract (ADR 0001). Never change the wrapped-DEK
  serialized form, the AEAD associated data, or the KEK file format without a
  versioned change and a reader for the old format. `tests/sekai_keys_compat.rs`
  must keep passing.
- Key material never reaches `Debug`, errors, or audit rows, and is zeroized
  on drop. Fail closed on grants, revocation, and production profiles.
- Library paths return `KeyError`; no `unwrap` or `expect` outside tests.
- Tests use per-process temp paths with a unique suffix, never fixed files.
- Releases follow `RELEASING.md`. Agents never push `v*` tags or publish to
  crates.io.
- Commit subjects are short and imperative, Conventional Commit style.
