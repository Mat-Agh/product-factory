# Operations

**Status:** Active
**Last updated:** {{DATE}}

Runbooks for the people who install and run {{MODULE}}. Every runbook states the
preconditions, the exact commands, how to verify success, and how to back out.

## Install and update

Commands for a first install and for an update, with the verification step
(health, readiness, migrations complete) and the expected output.

## Backup and rollback

What is backed up (databases, configuration, keys), how often, how it is
encrypted, where it is stored, and the restore drill. A rollback restores the
previous artifact set and data snapshot; never roll a database migration back by
hand.

## Monitoring

Health and readiness endpoints, the metrics that matter, log locations, and the
alert thresholds. Logs and metrics never contain secrets or personal data.

## Runbooks

| Situation | Runbook |
|---|---|
| Service will not start | `<link>` |
| Migration failed | `<link>` |
| Certificate expiring | `<link>` |
| Restore from backup | `<link>` |
