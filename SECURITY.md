# Security Policy

## Supported versions

| Version | Supported |
|---------|-----------|
| `main` (v0.1.x) | Yes |
| Older commits | No |

## Reporting a vulnerability

Please report security issues **privately** via [GitHub Security Advisories](https://github.com/unified-field-dev/orbital/security/advisories/new) on this repository.

Do not open a public issue for a vulnerability.

Include:

- Description of the issue and impact
- Steps to reproduce
- Affected component or crate (if known)
- Suggested fix (if you have one)

## Scope

**In scope**

- Orbital component crates, macros, preview server, and client rendering paths
- XSS or injection issues in Orbital-owned HTML/CSS/JS surfaces
- Auth/session handling in Orbital shell or preview tooling

**Out of scope**

- Vulnerabilities in Leptos, axum, or other upstream dependencies unless Orbital-specific wiring is at fault
- Deployed consumer applications that use Orbital as a dependency
- Denial-of-service via intentionally heavy preview/E2E workloads on maintainer infrastructure

## Response

Orbital is an early release. There is no formal SLA. Maintainers aim to acknowledge reports promptly and land fixes on `main`, with advisories published when appropriate.
