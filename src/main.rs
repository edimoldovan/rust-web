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
use actix_web_static_files::ResourceFiles;
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

include!(concat!(env!("OUT_DIR"), "/generated.rs"));

#[actix_web::main]
async fn main() -> std::io::Result<()> {

  let mut settings = Settings::parse_toml("./config.toml")
    .expect("Failed to parse `Settings` from config.toml");

  Settings::override_field_with_env_var(&mut settings.actix.hosts[0].port, "PORT")?;

  init_logger(&settings);

  HttpServer::new({
    // clone settings into each worker thread
    let settings = settings.clone();
    // init templates
    let tera = Tera::new(concat!(env!("CARGO_MANIFEST_DIR"), "/templates/**/*")).unwrap();

    move || {
      let generated = generate();

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
        .service(ResourceFiles::new("/static", generated))
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