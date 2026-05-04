# Security Policy

## Supported Versions

| Version | Supported |
|---------|-----------|
| 0.1.x   | Yes       |

## Reporting a Vulnerability

If you discover a security vulnerability in RavenRustRAG, please report it responsibly.

**Do not open a public GitHub issue for security vulnerabilities.**

Instead, use one of the following methods:

1. **GitHub Private Vulnerability Reporting**: Go to the [Security Advisories](https://github.com/egkristi/ravenrustrag/security/advisories) page and click "Report a vulnerability".
2. **Email**: Contact the maintainer directly at the email address listed in the repository profile.

### What to include

- Description of the vulnerability
- Steps to reproduce
- Affected versions
- Potential impact
- Suggested fix (if any)

### Response timeline

- **Acknowledgment**: Within 48 hours
- **Initial assessment**: Within 7 days
- **Fix or mitigation**: Depends on severity, typically within 30 days for critical issues

### Severity classification

We use the following severity levels:

- **Critical**: Remote code execution, authentication bypass, data exfiltration
- **High**: Privilege escalation, denial of service affecting availability
- **Medium**: Information disclosure, CORS misconfiguration, missing rate limits
- **Low**: Minor information leaks, hardening recommendations

## Security Measures

RavenRustRAG implements the following security controls:

- `unsafe_code = "forbid"` enforced workspace-wide
- Constant-time authentication comparison (`subtle::ConstantTimeEq`)
- Parameterized SQL queries (no injection)
- Symlink traversal protection in file loader
- Request body size limit (10MB)
- Automated `cargo-audit` in CI pipeline
- Clippy pedantic linting with `unwrap_used` warnings

## Known Issues

See the [issue tracker](https://github.com/egkristi/ravenrustrag/issues?q=label%3Asecurity) for open security-related issues.
