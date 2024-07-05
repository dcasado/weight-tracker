-- Add up migration script here
CREATE TABLE measurements (
    id SERIAL PRIMARY KEY,
    user_id SERIAL,
    date_time TIMESTAMPTZ NOT NULL,
    weight DOUBLE PRECISION NOT NULL,
    FOREIGN KEY (user_id)
        REFERENCES public.users (id) MATCH SIMPLE
        ON UPDATE NO ACTION
        ON DELETE NO ACTION
        NOT VALID
)
