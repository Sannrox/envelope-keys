# ADR 0001: Extract the key-provider contract

- Status: Accepted
- Date: 2026-11-26
- Issue: [#1](https://github.com/Sannrox/envelope-keys/issues/1)

## Context

The key-provider contract lived as `sekai-keys` inside one product's
repository. Several independent products depend on it, so each one pulled
in that repository to get a small, dependency-light crate. A product that
wanted no tie to that repository had to fork the contract, and two copies of
an envelope format drift.

## Decision

1. The contract is its own crate, `envelope-keys`, published to crates.io.
2. The envelope is the contract. The wrapped-DEK fields and serialized form,
   the AEAD and its associated data (tenant, purpose, DEK id), and the KEK
   file format stay byte-compatible with `sekai-keys` 1.0.0. A change to any
   of them is a versioned change and ships with a reader for the old format.
3. The crate holds the contract, the development providers (memory, file),
   and the conformance suite. KMS, HSM and network providers live in the
   product that speaks to them until a second product needs the same one.

## Consequences

- Consumers move from a git dependency to `envelope-keys = "X.Y.Z"` with an
  import rename and no data migration.
- `tests/sekai_keys_compat.rs` keeps fixtures written by `sekai-keys` 1.0.0
  readable.

## Rejected

- Copy the contract into each product: forks the envelope format.
- Keep it in the original repository: every consumer depends on an unrelated
  product.
