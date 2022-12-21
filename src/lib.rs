pub mod configuration;
pub mod routes;
pub mod startup;

use std::net::TcpListener;

use actix_web::{
    dev::Server,
    web::{self, Form},
    App, HttpRequest, HttpResponse, HttpServer, Responder,
};

#[derive(serde::Deserialize)]
pub struct FormData {
    email: String,
    name: String,
}

async fn greet(req: HttpRequest) -> impl Responder {
    let name = req.match_info().get("name").unwrap_or("World");
    format!("Hello {}!", &name)
}

async fn suscriptions(_form: Form<FormData>) -> HttpResponse {
    HttpResponse::Ok().finish()
}

async fn health_check(_req: HttpRequest) -> impl Responder {
    HttpResponse::Ok()
}

pub fn run(listener: TcpListener) -> Result<Server, std::io::Error> {
    let server = HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(greet))
            .route("/greet/{name}", web::get().to(greet))
            .route("/health_check", web::get().to(health_check))
            .route("/suscriptions", web::post().to(suscriptions))
    })
    .listen(listener)?
    .run();
    Ok(server)
}
