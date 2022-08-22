-- Seed data for collaborator invitations

-- owner invitations
INSERT INTO owner_invitations VALUES
(1, 2, 2, '51d52fb511ed33af9314450e328fc4ae376e06903adfe8c4ae60b001c50238cb', TRUE, CURRENT_TIMESTAMP),
(1, 3, 8, 'fc7daaee585ecceae355b1ca34eb503626d45e998f5bdada82f36c28d84fa855', TRUE, CURRENT_TIMESTAMP);

-- collaborator invitations
INSERT INTO owner_invitations VALUES
(1, 4, 4, '109e1760ba805ddafcf1b69ac461de712a3615720ae9e5bb059be06b70801130', FALSE, CURRENT_TIMESTAMP),
(1, 5, 15, 'b7bb8c6da21ef52e34c238105c15ae9f5faf33ad4d11e673535fb852f7abff11', FALSE, CURRENT_TIMESTAMP);
