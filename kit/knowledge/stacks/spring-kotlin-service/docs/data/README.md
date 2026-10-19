# Data

**Status:** Active
**Last updated:** {{DATE}}

The physical schema of each durable service. Flyway migrations are the source of
truth; this folder explains them.

## Schema

One section per service database: tables, their purpose, tenant column, key
constraints and indexes. Update it in the same change as the migration.

| Service | Database | Migrations |
|---|---|---|
| `<service>` | `<db>` | `application/<service>/src/main/resources/db/migration/` |

## Model catalogue

| Table | Owner feature | Classification of sensitive columns | Notes |
|---|---|---|---|
| `<table>` | `<feature>` | `<envelope / plaintext>` | |

## Relations and flow

Cross-service references are logical UUIDs, never foreign keys. Draw the flow of
facts between services (which events create which rows) as a diagram source with a
rendered image, and regenerate any ERD after schema changes.
