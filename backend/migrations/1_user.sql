CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

-- TODO make a 1_setup.sql with extension and trigger
-- TODO add updated_at et created_at
CREATE TABLE "user" (
    id uuid DEFAULT uuid_generate_v1mc(),
    email TEXT NOT NULL,
    username TEXT NOT NULL,
    password TEXT NOT NULL
);

ALTER TABLE "user" ADD PRIMARY KEY (id);
ALTER TABLE "user" ADD UNIQUE (email);
ALTER TABLE "user" ADD UNIQUE (username);
