-- Add down migration script here
ALTER TABLE person
DROP COLUMN birthdate;

ALTER TABLE person
DROP COLUMN height;

ALTER TABLE person
RENAME COLUMN person_id TO id;

ALTER TABLE person
RENAME TO users;
