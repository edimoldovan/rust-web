use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use actix_web::middleware::Logger;
use env_logger::Env;
use askama_actix::{Template};
use std::{env, net::SocketAddr};

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
  let port = env::var("PORT")
    .ok()
    .and_then(|port| port.parse().ok())
    .unwrap_or_else(|| 8186);
  let address = SocketAddr::from(([127, 0, 0, 1], port));

  env_logger::init_from_env(Env::default().default_filter_or("info"));

  HttpServer::new(|| {

    App::new()
      .wrap(Logger::default())
      .wrap(Logger::new("%a %{User-Agent}i"))
      .service(hello)
      .service(echo)
      .route("/hey", web::get().to(manual_hello))
  })
  .bind(address)?
  .run()
  .await
}