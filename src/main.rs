use actix_settings::{ApplySettings as _, Settings, Mode};
use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use actix_web::middleware::{Condition, Compress, Logger};
use askama_actix::{Template};

#[derive(Template)]
#[template(path = "hello.html")]
struct HelloTemplate<'a> {
  name: &'a str,
}

#[get("/")]
async fn hello() -> impl Responder {
  HelloTemplate { name: "world" }
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
  HttpResponse::Ok().body(req_body)
}

async fn manual_hello() -> impl Responder {
  HttpResponse::Ok().body("Hey there!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
  let mut settings = Settings::parse_toml("./config.toml")
    .expect("Failed to parse `Settings` from config.toml");

  Settings::override_field_with_env_var(&mut settings.actix.hosts[0].port, "PORT")?;

  init_logger(&settings);

  HttpServer::new({
    // clone settings into each worker thread
    let settings = settings.clone();

    move || {
      App::new()
        // Include this `.wrap()` call for compression settings to take effect
        .wrap(Condition::new(
          settings.actix.enable_compression,
          Compress::default(),
        ))
        .wrap(Logger::default())
        .app_data(web::Data::new(settings.clone()))
        .service(hello)
        .service(echo)
        .route("/hey", web::get().to(manual_hello))
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