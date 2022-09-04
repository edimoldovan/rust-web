# rust-web
A simple and opinionated web framework mostly based on the `actix-web`, server side includes and javascript import maps

## usage
...

## features
- watch and restart the server with `cargo watch -x run`
- using `askama` as template engine

## upcoming features
- automatic page reload
- better css tooling
- session handling, start with cookies
- server side inclundes
- serve static files from `public` folder 
- minify css `brew install tdewolff/tap/minify`
- parse and use `config.toml`
- use javascript import maps installed with `npm` and configured in `config.toml`
- automatic page reload with a simple socket signalling
- JSON API exammples
- JWT examples, both issuing a new token at login and reading it
- middlewares, starting with a request logger
- session handling, start with cookies
- server side inclundes
