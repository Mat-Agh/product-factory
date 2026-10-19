# Security

**Status:** Active
**Last updated:** {{DATE}}

The threat model and protection boundaries of {{MODULE}}. Implementation rules
live in [../development/06-security-contracts.md](../development/06-security-contracts.md)
and [../development/10-cryptography.md](../development/10-cryptography.md).

## Threat model

Keep one table and update it whenever a change adds a surface, a data class or a
dependency.

| Asset | Threat | Control | Residual risk |
|---|---|---|---|
| `<credentials>` | `<theft, replay>` | `<hashing, rotation, binding>` | `<what remains>` |

State what the system does **not** protect against (for example an authorized live
service reading data it is allowed to decrypt, or a compromised client endpoint).

## Data classification

| Class | Examples | Storage | Logging |
|---|---|---|---|
| Secret | Passwords, tokens, keys | Hash or envelope | Never |
| Restricted content | Private messages | Envelope | Never |
| Personal data | Email, name | Envelope (+ blind index if approved) | Approved identifiers only |
| Routing metadata | IDs, status, timestamps | Plaintext | Yes |

## Boundaries

| Boundary | Mechanism | Fails |
|---|---|---|
| Public edge | TLS 1.3 | Closed |
| Service to service | mTLS, scoped identity | Closed |
| Tenant | Principal-derived scope, RLS where used | Closed |
| Secrets | Mounted files, never env defaults | Startup refuses |

Security changes need a threat-model delta in the merge request.
