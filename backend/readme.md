# Installation of tools

cargo install cargo-watch
cargo install cargo-audit
cargo install sqlx --no-default-features --features postgres,rustls
cargo install cargo-udeps

cargo install cargo-expand

## Ready to go CI Pipeline

## Capturing requirements:

A. As a blog visitor,
I want to subscribe to the newsletter,
So that I can receive email updates when new content is published on the blog

B. Subscription consent flow

- User receives email with the confirmation link. Once they click on it then they are consented.
- On the backend, user POST /subscriptions request.
  1. adds details to subscriptions table with status equal to pending_confirmation
  2. generate a unique subscription_token
  3. store subscription token against the id in a subscriptions_token table
  4. send an email to the subscriber containing a link structured as https://<api-domain>/subscriptions/consent
  5. return 200 OK
