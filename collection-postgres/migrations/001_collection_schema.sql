CREATE TABLE IF NOT EXISTS storage_collection_schema
(
    name       VARCHAR(36) PRIMARY KEY,
    fields     JSONB                DEFAULT '[]'::jsonb NOT NULL,
    created_at timestamptz NOT NULL DEFAULT NOW()::timestamp,
    updated_at timestamptz NOT NULL DEFAULT NOW()::timestamp
);
