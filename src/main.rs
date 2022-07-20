use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use actix_web::middleware::Logger;
use env_logger::Env;
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
  let port = std::env::var("PORT")
      .ok()
      .map(|val| val.parse::<u16>())
      .unwrap_or(Ok(8080))?;

  env_logger::init_from_env(Env::default().default_filter_or("info"));

  HttpServer::new(|| {

    App::new()
      .wrap(Logger::default())
      .wrap(Logger::new("%a %{User-Agent}i"))
      .service(hello)
      .service(echo)
      .route("/hey", web::get().to(manual_hello))
  })
  .bind(("127.0.0.1", port))?
  .run()
  .await
}