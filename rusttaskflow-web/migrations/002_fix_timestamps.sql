-- Fix timestamp types to use TIMESTAMPTZ for proper timezone handling
-- This migration fixes the mismatch between Rust DateTime<Utc> and PostgreSQL TIMESTAMP

ALTER TABLE users 
    ALTER COLUMN created_at TYPE TIMESTAMPTZ,
    ALTER COLUMN updated_at TYPE TIMESTAMPTZ;

ALTER TABLE tasks 
    ALTER COLUMN created_at TYPE TIMESTAMPTZ,
    ALTER COLUMN updated_at TYPE TIMESTAMPTZ,
    ALTER COLUMN started_at TYPE TIMESTAMPTZ,
    ALTER COLUMN completed_at TYPE TIMESTAMPTZ,
    ALTER COLUMN due_date TYPE TIMESTAMPTZ;