-- Your SQL goes here
ALTER TABLE IF EXISTS contract_upgrade_processor_status
ALTER COLUMN last_updated
SET DEFAULT NOW ();