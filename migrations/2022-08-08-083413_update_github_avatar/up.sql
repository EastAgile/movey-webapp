-- Update ava for github
UPDATE accounts 
SET avatar = FORMAT('https://avatars.githubusercontent.com/u/%s', github_id)
WHERE github_id is not NULL;

-- Update ava to retro according to new design
UPDATE accounts 
SET avatar = FORMAT('https://secure.gravatar.com/avatar/%s?s=200&d=retro&r=pg', MD5(LOWER(TRIM(accounts.email))))
WHERE github_id is NULL;
