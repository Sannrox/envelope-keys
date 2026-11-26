# Changelog

## [Unreleased]

## [1.0.0]

First release. Extracted unchanged from `sekai-keys` 1.0.0 apart from the
crate name, including the KEK zeroization fixes that followed that release.
Envelopes and KEK files written by `sekai-keys` 1.0.0 read here unchanged.

### Added

- `KeyProvider` contract, `Dek`, `WrappedDek`, `UnwrapGrant`, `UnwrapAudit`,
  `KeyId`, `TenantId`, `KeyPurpose`, `RuntimeProfile` and `admit_provider`.
- `MemoryKeyProvider` and `FileKeyProvider` for development and tests.
- `conformance::assert_provider_contract` for downstream providers.
