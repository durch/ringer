ALTER TABLE checks DROP COLUMN state;
ALTER TABLE checks ADD COLUMN meta JSONB;