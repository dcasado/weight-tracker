-- Add down migration script here
ALTER TABLE weight
RENAME COLUMN measured_at TO date_time;

ALTER TABLE weight
RENAME COLUMN kilograms TO weight;

ALTER TABLE weight
RENAME COLUMN weight_id TO id;

ALTER TABLE weight
RENAME TO measurements;
