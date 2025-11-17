-- Add up migration script here
ALTER TABLE measurements
RENAME TO weight;

ALTER TABLE weight
RENAME COLUMN id TO weight_id;

ALTER TABLE weight
RENAME COLUMN weight TO kilograms;

ALTER TABLE weight
RENAME COLUMN date_time TO measured_at;
