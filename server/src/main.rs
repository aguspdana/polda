use actix_files::NamedFile;
use actix::Actor;
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

    log::info!("starting HTTP server at http://{}:{}", hostname, port);

    let broker = Broker::new().start();

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(broker.clone()))
            .wrap(Logger::default())
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

