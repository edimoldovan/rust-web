use actix_settings::{ApplySettings as _, Settings, Mode};
use actix_web::{
  get, 
  post, 
  web, 
  middleware::{Condition, Compress, Logger},
  App, 
  HttpResponse, 
  HttpServer, 
  Responder,
  
};
use actix_files as fs;
// use askama_actix::{Template};
use std::{
  path::Path,
  fs::File,
  io::prelude::*,
};
use lightningcss::{
  bundler::{Bundler, FileProvider},
  stylesheet::{ParserOptions, PrinterOptions}, //MinifyOptions
};
use tera::{Tera, Context};

#[get("/")]
async fn index(
  tmpl: web::Data<tera::Tera>,
) -> impl Responder {
  let name = "World";
  let mut ctx = Context::new();
  ctx.insert("name", name);
  HttpResponse::Ok()
    .content_type("text/html")
    .body(tmpl.render("index.html", &ctx).unwrap())
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
  HttpResponse::Ok().body(req_body)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {

  let mut settings = Settings::parse_toml("./config.toml")
    .expect("Failed to parse `Settings` from config.toml");

  Settings::override_field_with_env_var(&mut settings.actix.hosts[0].port, "PORT")?;

  init_logger(&settings);

  // maybe use later
  // println!("The current directory is {}", env::current_exe().unwrap().display());
  // write bundled css
  let fs = FileProvider::new();
  let mut bundler = Bundler::new(&fs, None, ParserOptions::default());
  let stylesheet = bundler.bundle(Path::new("./assets/css/global.css")).unwrap();
  // figure out minify later
  // stylesheet.minify(MinifyOptions::default());
  // println!("`{:?}`", stylesheet.minify(MinifyOptions::default()));
  // println!("{}", stylesheet.to_css(PrinterOptions::default()).unwrap().code);
  let mut file = File::create("./public/global.min.css")?;
  write!(file, "{}", stylesheet.to_css(PrinterOptions::default()).unwrap().code)?;

  HttpServer::new({
    // clone settings into each worker thread
    let settings = settings.clone();
    // init templates
    let tera = Tera::new(concat!(env!("CARGO_MANIFEST_DIR"), "/templates/**/*")).unwrap();

    move || {
      App::new()
        // Include this `.wrap()` call for compression settings to take effect
        .wrap(Condition::new(
          settings.actix.enable_compression,
          Compress::default(),
        ))
        .wrap(Logger::default())
        // figure out how to use this in route handlers
        .app_data(web::Data::new(settings.clone()))
        // pass template engine to route handlers
        .app_data(web::Data::new(tera.clone()))
        .service(index)
        .service(echo)
        // serve static files
        .service(fs::Files::new("/static", "./public"))
    }
  })
  .apply_settings(&settings)
  .run()
  .await
}

/// Initialize the logging infrastructure.
fn init_logger(settings: &Settings) {
  if !settings.actix.enable_log {
    return;
  }

  std::env::set_var(
    "RUST_LOG",
    match settings.actix.mode {
      Mode::Development => "actix_web=debug",
      Mode::Production => "actix_web=info",
    },
  );

  std::env::set_var("RUST_BACKTRACE", "1");
  env_logger::init();
}