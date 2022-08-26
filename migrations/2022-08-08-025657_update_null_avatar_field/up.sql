-- Your SQL goes here
UPDATE accounts 
SET avatar = FORMAT('https://secure.gravatar.com/avatar/%s?s=200&d=monsterid&r=pg', MD5(LOWER(TRIM(accounts.email))))
WHERE avatar is NULL;
