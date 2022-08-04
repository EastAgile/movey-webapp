CREATE TABLE pending_invitations (
     pending_user_email TEXT NOT NULL,
     invited_by_user_id INTEGER NOT NULL REFERENCES accounts (id) ON DELETE CASCADE,
     package_id INTEGER NOT NULL REFERENCES packages (id) ON DELETE CASCADE,
     created_at TIMESTAMP NOT NULL DEFAULT now(),

     PRIMARY KEY (pending_user_email, package_id)
);
