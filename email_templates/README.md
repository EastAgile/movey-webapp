## Configuring Email Support
Jelly provides simple setup for transactional email. It's configured to work with [Postmark](https://postmarkapp.com), but you can replace it with whatever you want.

## Setting Up
- Sign up on [Postmark](https://postmarkapp.com). Do your standard domain configuration pieces as need be, and store your API key in your `.env` file.
- Take the contents of `layout.html` and add them as a Layout on Postmark.
- For each html file in this directory that's _not_ `layout.html`, add it to Postmark as a Template. Make sure you set the layout as "Use an existing layout", selecting the layout you added in the last step. The key for each template should be the filename without the extension - so, for `password-was-reset.html`, make the key `password-was-reset`.

The templates in here are verified to work with most common email clients. You can add a logo URL in `layout.html`, and configure anything else as necessary.

Enjoy!
