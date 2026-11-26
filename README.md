# envelope-keys

**envelope-keys lets a store encrypt with data keys it can only unwrap
through a key provider, never by holding the master key.**

A store generates a data key (DEK), asks a provider to wrap it under a key
encryption key (KEK), and keeps only the wrapped bytes. To read, it presents
the wrapped DEK with a time-bounded grant for one tenant and purpose. The
provider unwraps and returns an audit row the store must record.

[![CI](https://img.shields.io/github/actions/workflow/status/Sannrox/envelope-keys/ci.yml?branch=main&label=ci)](https://github.com/Sannrox/envelope-keys/actions/workflows/ci.yml)
[![crates.io](https://img.shields.io/crates/v/envelope-keys)](https://crates.io/crates/envelope-keys)
[![docs.rs](https://img.shields.io/docsrs/envelope-keys)](https://docs.rs/envelope-keys)
[![License](https://img.shields.io/crates/l/envelope-keys)](https://github.com/Sannrox/envelope-keys/blob/main/LICENSE)

[Quickstart](#quickstart) · [When not to use](#when-not-to-use-envelope-keys) · [API docs](https://docs.rs/envelope-keys) · [Design](https://github.com/Sannrox/envelope-keys/blob/main/docs/decisions/0001-extract-the-key-provider-contract.md) · [Changelog](https://github.com/Sannrox/envelope-keys/blob/main/CHANGELOG.md)

- **One provider contract.** `KeyProvider` wraps, rewraps, unwraps, rotates
  and revokes. Hardware and network providers implement it in the product
  that speaks to them.
- **Bound envelopes.** AES-256-GCM with tenant, purpose and DEK id in the
  associated data, so a wrapped DEK cannot be replayed under another tenant
  or purpose.
- **Grants and audit.** Every unwrap needs an unexpired grant for the
  envelope's tenant and purpose, and returns an `UnwrapAudit` with no key
  material.
- **Rotation with overlap.** A rotated provider still unwraps envelopes under
  the previous KEK until rotation completes; a second rotation during overlap
  fails closed.
- **Fail closed in production.** `admit_provider` refuses the memory and file
  providers under `RuntimeProfile::Production`.
- **Conformance suite.** `conformance::assert_provider_contract` checks any
  provider against the contract.
- **Zeroized keys.** DEKs and KEKs are wiped on drop and redacted in `Debug`.

## When not to use envelope-keys

- You need a KMS or HSM client. This crate defines the contract; the client
  for your KMS lives in your product and implements `KeyProvider`.
- You encrypt one file with one password. Use a password-based format.
- You need key distribution across hosts. Providers here are local; the
  memory and file providers are for development and tests.

## Quickstart

```sh
cargo add envelope-keys
```

Wrap a data key, unwrap it under a grant, and see a wrong tenant refused:

```rust
use envelope_keys::{
    Dek, KeyError, KeyProvider, KeyPurpose, MemoryKeyProvider, RuntimeProfile, TenantId,
    UnwrapGrant,
};

fn main() -> Result<(), KeyError> {
    // Development provider; production uses a KMS or HSM provider.
    let provider = MemoryKeyProvider::new(RuntimeProfile::Development)?;
    let tenant = TenantId::new("tenant-a")?;

    let dek = Dek::generate();
    let wrapped = provider.wrap(&tenant, KeyPurpose::Store, &dek)?;

    let grant = UnwrapGrant {
        tenant_id: tenant.clone(),
        purpose: KeyPurpose::Store,
        expires_unix_secs: 10_000,
    };
    let outcome = provider.unwrap(&wrapped, &grant, "orders-store", 0)?;
    assert_eq!(outcome.dek.as_bytes(), dek.as_bytes());

    // The same envelope under another tenant's grant is refused.
    let other = UnwrapGrant { tenant_id: TenantId::new("tenant-b")?, ..grant };
    assert_eq!(
        provider.unwrap(&wrapped, &other, "orders-store", 0).unwrap_err(),
        KeyError::WrongTenant
    );
    Ok(())
}
```

Store `wrapped` next to the ciphertext it protects, and record every
`outcome.audit`.

## Providers

| Provider | Use |
| --- | --- |
| `MemoryKeyProvider` | Tests |
| `FileKeyProvider` | Development; one owner-only KEK file per path, held by one process |
| Your KMS or HSM client | Production; implement `KeyProvider` and run the conformance suite |

## Compatibility

The wrapped-DEK envelope, its serialized form, and the KEK file format are
unchanged from `sekai-keys` 1.0.0, the crate this one was extracted from.
Envelopes and KEK files written by it read here unchanged; the
`sekai_keys_compat` test checks fixtures written by that release.

## Development

```sh
cargo fmt --all -- --check
cargo test --locked
cargo clippy --locked --all-targets -- -D warnings
bash scripts/validate-release.sh
```

## License

Apache-2.0
