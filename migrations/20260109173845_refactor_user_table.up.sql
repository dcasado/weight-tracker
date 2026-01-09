-- Add up migration script here
ALTER TABLE users
RENAME TO person;

ALTER TABLE person
RENAME COLUMN id TO person_id;

ALTER TABLE person
ADD birthdate TEXT;

ALTER TABLE person
ADD height REAL;
