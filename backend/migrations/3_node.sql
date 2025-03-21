CREATE TYPE node_type AS ENUM ('anvil', 'solana');

CREATE TABLE node (
    id uuid DEFAULT uuid_generate_v1mc(),
    user_id uuid NOT NULL,
    challenge_id uuid NOT NULL,
    level TEXT,
    instances TEXT[],
    pod_name TEXT,
    pod_uid TEXT,
    type node_type NOT NULL,
    last_activity TIMESTAMP NOT NULL DEFAULT NOW(),
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP NOT NULL DEFAULT NOW()
);

ALTER TABLE node ADD PRIMARY KEY (id);
