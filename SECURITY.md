# Security policy

envelope-keys guards data keys, so a way to obtain a DEK or KEK without the
contract's checks is a security vulnerability.

## Reporting a vulnerability

Report vulnerabilities privately through GitHub private vulnerability
reporting: <https://github.com/Sannrox/envelope-keys/security/advisories/new>.
Do not open a public issue, pull request, or discussion.

Include the crate version, the provider, and a minimal reproduction.

We aim to acknowledge reports within a week and to publish a fix and an
advisory before the details become public.

## In scope

- An unwrap that succeeds with an expired grant, or a grant for another
  tenant or purpose.
- An unwrap after the DEK or KEK was revoked, or under a previous KEK after
  rotation completed.
- `admit_provider` admitting the memory or file provider under
  `RuntimeProfile::Production`.
- Key material in `Debug` output, audit rows, errors, or left unzeroized in
  memory the crate owns after drop.
- A KEK file created with permissions wider than owner-only on Unix.

## Out of scope

- Providers implemented outside this crate.
- An attacker who can read the process memory or the KEK file.
