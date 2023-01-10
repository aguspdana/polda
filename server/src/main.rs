use actix_files::NamedFile;
use actix::Actor;
use actix_cors::Cors;
use actix_web::App;
use actix_web::Error;
use actix_web::HttpRequest;
use actix_web::HttpResponse;
use actix_web::HttpServer;
use actix_web::middleware::Logger;
use actix_web::Responder;
use actix_web::web;
use actix_web_actors::ws;
use std::env;

mod broker;
mod client;
mod document;
mod executor;

use client::Client;
use broker::Broker;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    let hostname = env::var("HOSTNAME")
        .unwrap_or(String::from("localhost"));
    let port = env::var("PORT")
        .map(|p| p.parse::<u16>().expect("Invalid PORT environment variable"))
        .unwrap_or(8080);
    let origin = env::var("ORIGIN")
        .unwrap_or(String::from("http://localhost:3000"));

    log::info!("starting HTTP server at http://{}:{}", hostname, port);

    let broker = Broker::new().start();

    HttpServer::new(move || {
        let cors = Cors::default()
            .allowed_origin(&*origin)
            .allowed_origin_fn(|origin, _req_head| {
                origin.as_bytes().starts_with(b"http://localhost:")
            })
            .allow_any_method()
            .allow_any_header()
            .supports_credentials()
            .max_age(3600);

        App::new()
            .app_data(web::Data::new(broker.clone()))
            .wrap(Logger::default())
            .wrap(cors)
            .service(web::resource("/").to(index))
            .route("/client", web::get().to(client))
            .route("/ws", web::get().to(ws))
    })
    .bind((hostname, port))?
    .shutdown_timeout(60)
    .run()
    .await
    .unwrap();

    Ok(())
}

async fn index() -> impl Responder {
    "Hello worker"
}

async fn client() -> impl Responder {
    NamedFile::open_async("./static/index.html").await.unwrap()
}

async fn ws(
    req: HttpRequest,
    stream: web::Payload,
) -> Result<HttpResponse, Error> {
    ws::start(
        Client::new(),
        &req,
        stream,
    )
}

