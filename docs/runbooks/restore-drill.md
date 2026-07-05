# Runbook: Restore Drill

> A backup you haven't restored is not a backup. Run this drill monthly.

## What gets backed up

- SQLite ledger (`ledger.db`) — the tamper-evident request log
- Redis snapshot (RDB) — the cache (nice-to-have, not critical)
- Config / secrets manifests

## Restore procedure

_TODO: fill in once `scripts/backup.sh` and `scripts/restore.sh` exist._

1. Provision a scratch environment.
2. Pull the latest backup from OCI Object Storage.
3. Run `scripts/restore.sh <backup-id>`.
4. Verify ledger hash-chain integrity end-to-end.
5. Verify the gateway starts and serves `/health`.

## Success criteria

- Ledger chain verifies with no gaps.
- Gateway health check passes against restored data.
- Time-to-restore recorded and under target (TODO: set target).
