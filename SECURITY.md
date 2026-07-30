# Security Policy

## Reporting a Vulnerability

If you find a security vulnerability in binaryornot-rs, please report it through [GitHub's private vulnerability reporting](https://github.com/opticsWolf/binaryornot-rs/security/advisories/new). This keeps details private while we work on a fix.

Please include:

- What you found and how to reproduce it
- Which version you're using
- Any relevant logs or output (redact secrets)

## Security Measures

- **CodeQL** scans code for vulnerabilities using the `security-extended` query suite
- **Dependabot** keeps dependencies updated with a 7-day cooldown
- **All actions pinned by SHA** with version comments
- **Minimal workflow permissions** (`permissions: {}` at top level, scoped per job)
- **`persist-credentials: false`** on checkout steps

## Response Times

This is a volunteer-maintained open source project. Security reports are taken seriously, but there are no guaranteed response times.

## Supported Versions

Security fixes are applied to the latest release on the `main` branch. There is no backport policy for older versions.
