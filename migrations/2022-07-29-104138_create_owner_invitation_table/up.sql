CREATE TABLE owner_invitations (
     invited_user_id INTEGER NOT NULL REFERENCES accounts (id) ON DELETE CASCADE,
     invited_by_user_id INTEGER NOT NULL REFERENCES accounts (id) ON DELETE CASCADE,
     package_id INTEGER NOT NULL REFERENCES packages (id) ON DELETE CASCADE,
     token TEXT NOT NULL,
     created_at TIMESTAMP NOT NULL DEFAULT now(),

     PRIMARY KEY (invited_user_id, package_id)
);
