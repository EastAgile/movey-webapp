CREATE TABLE api_tokens (
    id SERIAL PRIMARY KEY,
    account_id integer NOT NULL REFERENCES accounts(id),
    token character varying NOT NULL UNIQUE,
    name character varying NOT NULL,
    created_at timestamp with time zone DEFAULT now() NOT NULL,
    last_used_at timestamp with time zone,
    UNIQUE (account_id, name)
);

CREATE INDEX api_tokens_index ON api_tokens (token);
