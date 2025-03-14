CREATE TYPE blockchain_type AS ENUM ('ethereum', 'solana');

CREATE TABLE challenge (
    id uuid DEFAULT uuid_generate_v1mc(),
    author_id uuid NOT NULL REFERENCES "user"(id) ON DELETE CASCADE,
    title TEXT NOT NULL,
    description TEXT NOT NULL,
    code TEXT NOT NULL,
    bytecode BYTEA NOT NULL,
    difficulty SMALLINT NOT NULL,
    solved INTEGER NOT NULL DEFAULT 0,
    blockchain blockchain_type NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT NOW(), -- TODO maybe use timestampz
    updated_at TIMESTAMP NOT NULL DEFAULT NOW()
);

ALTER TABLE challenge ADD PRIMARY KEY (id);
