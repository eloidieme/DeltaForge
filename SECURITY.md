# Security Policy

## Supported version

DeltaForge 1.0 is the supported line. Security fixes are made on the default branch and included in the next release.

## Reporting a vulnerability

Please do not open a public issue for a suspected vulnerability. Use [GitHub's private vulnerability reporting form](https://github.com/eloidieme/DeltaForge/security/advisories/new) and include the affected version, operating system, reproduction steps, and likely impact.

DeltaForge runs a loopback HTTP service that can execute pack-defined commands under the current user's account. Reports involving the capability token, Host/Origin checks, project-creation path boundary, command execution, or filesystem containment are especially useful. Do not include real credentials or sensitive learner files in a report.

You should receive an acknowledgement within seven days. We will coordinate remediation and disclosure through the private advisory.
