# rust-web
A simple and opinionated web framework mostly based on the `actix-web`, server side includes and javascript import maps

## usage
...

## features
- watch and restart the server with `cargo watch -x run`
- using `askama` as template engine
- middlewares, starting with a request logger
- App is easily serving static files from `public` folder like this: `/public/some.file` available on `/static/some.file` url.serve static files from `public`

## upcoming features
- automatic page reload: ws https://actix.rs/docs/websockets/ , https://crates.io/crates/actix-ws
- better css tooling
- session handling, start with cookies: https://docs.rs/actix-session/latest/actix_session/
- server side inclundes

- minify css `brew install tdewolff/tap/minify`
- parse and use `config.toml`: 
- use javascript import maps installed with `npm` and configured in `config.toml`
- automatic page reload with a simple socket signalling
- JSON API exammples: https://auth0.com/blog/build-an-api-in-rust-with-jwt-authentication-using-actix-web/
- JWT examples, both issuing a new token at login and reading it: https://github.com/Keats/jsonwebtoken 

- session handling, start with cookies
- server side inclundes
