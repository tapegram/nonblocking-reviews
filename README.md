# Nonblocking Reviews with ML

Any ideas for a good app name or just general language we should be using?

## Installing/Running

Setup our prepush hook (to protect main)
```bash
git config core.hooksPath .git-hooks

```

```bash
cargo run
```

You can also leverage `cargo-watch`

```bash
cargo install cargo-watch

# Then you can do the following
cargo watch -x run
```

## Testing webhooks

Using smee client we can forward webhooks to localhost.

To install globally
`npm install -g smee-client`

Then you can run the following

```
smee -u WEBHOOK_PROXY_URL -t http://localhost:3000/github-webhook
```

Where WEBHOOK_PROXY_URL is whatever we set in the app (which for now is a smee endpoint): https://smee.io/fMWUzysZfMg1hpki

so 
`smee -u https://smee.io/fMWUzysZfMg1hpki -t http://localhost:3000/github-webhook`


## Code generation

We are experimenting with code generation tools (using plopjs). run `./generate.sh` to enter the code generation dialogue.

## Architecture & Tech Stack

### Frontend Client

This application required `bun` be installed. If you get an error regarding bun not being found, [please install bun](https://bun.sh/).

The `web-client` crate is responsible for building our client-side assets.
Assets for the client are reusable UI components, bespoke web controls (to extend our hypermedia client - the browser).
This crate leverage bun as our bundler for our TypeScript modules and TailwindCss (coming soon) for css.
See the `web-client` [README.md](./web-client/README.md) for more.

The `web-htmx` crate serves as "the backend for the frontend" using HTMX as the means to deliver a more rich UI w/out relying on custom JavaScript.
