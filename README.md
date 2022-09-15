# rust-web
A simple and opinionated web framework mostly based on the `actix-web`, server side includes and javascript import maps

## usage
...

## features
- watch and restart the server with `cargo watch -x run`
- using `askama` as template engine
- middlewares, starting with a request logger

## upcoming features
- automatic page reload: ws https://actix.rs/docs/websockets/
- better css tooling
- session handling, start with cookies: https://docs.rs/actix-session/latest/actix_session/
- server side inclundes
- serve static files from `public` folder: https://actix.rs/docs/static-files/
- minify css `brew install tdewolff/tap/minify`
- parse and use `config.toml`: 
- use javascript import maps installed with `npm` and configured in `config.toml`
- automatic page reload with a simple socket signalling
- JSON API exammples: https://auth0.com/blog/build-an-api-in-rust-with-jwt-authentication-using-actix-web/
- JWT examples, both issuing a new token at login and reading it

- session handling, start with cookies
- server side inclundes
