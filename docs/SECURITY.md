# Security Policy

## Supported Versions

| Version | Supported |
|---------|-----------|
| latest  | ✅        |

Only the latest release receives security fixes. Please upgrade before reporting.

## Reporting a Vulnerability

**Do not open a public GitHub issue for security vulnerabilities.**

Report vulnerabilities privately via [GitHub Security Advisories](https://github.com/karafra/cenyslovensko-api/security/advisories/new).

Please include:
- A clear description of the vulnerability
- Steps to reproduce or a proof-of-concept
- Affected versions
- Suggested fix (if any)

We aim to acknowledge reports within **48 hours** and to release a fix within **14 days** for confirmed critical issues.

## Scope

This policy covers:
- `cenyslovensko` - umbrella crate
- `cenyslovensko_web_client` - HTTP client
- `cenyslovensko_version_api` - version API client
- `cenyslovensko_rpc_server` - JSON-RPC server binary

Third-party dependencies (e.g. `reqwest`, `tokio`) should be reported upstream to their respective maintainers.
