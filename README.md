# rust-web
A simple and opinionated web framework mostly based on the `actix-web`, server side includes and javascript import maps

```docs being updated on a regular basis, also becoming more detailed```

## developer tooling
- watch and restart the server with `cargo watch -x run` where `.gitignore` serves as ignore for watch

## better css
Basic design system included based on [CubeCSS](https://cube.fyi/). Key principles:
- use progressive enhancement
- structure the CSS in these four groups: composition styles, utilities, blocks and exceptions
The most import part is that we should try to guide the browser to do what it does best (rendering) in a context that it finds itself in.
Design system is composed of a few files: defined colors, spacing values and text sizes, along with a global reset and global styling to bring all browsers on the same page. These are used to build the actual, fluid, styling of the pages.

## easily serve static files (css/js/images/etc)
App is easily serving static files from `public` folder like this: `/public/some.file` available on `/static/some.file` url.

## html templates
- using `Tera` as template engine

## middleware
- Middleware example, starting with a preconfigure logger


## upcoming features
- automatic page reload: ws https://actix.rs/docs/websockets/ , https://crates.io/crates/actix-ws
- session handling, start with cookies: https://docs.rs/actix-session/latest/actix_session/
- server side inclundes
- parse and use `config.toml`: 
- use javascript import maps installed with `npm` and configured in `config.toml`
- JSON API exammples: https://auth0.com/blog/build-an-api-in-rust-with-jwt-authentication-using-actix-web/
- JWT examples, both issuing a new token at login and reading it: https://github.com/Keats/jsonwebtoken 




## product plan
1. product name, description
2. design system
   1. brand (light and dark)
      1. primary colors (brand + others) -- with pickers
      2. secondary colors -- with pickers
      3. colors shades generated from primary and secondary colors but also editable
      4. neutral colors generated but also editable
   2. shadows
      1. outer
      2. inner
   3. box ratios
   4. fluid type