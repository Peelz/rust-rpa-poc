mod handler;
mod sys;
use actix_web::{App, HttpServer};
use sys::application_config::load_env;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let app_conf = load_env().unwrap();

    HttpServer::new(|| App::new().service(handler::binding_req::register()))
        .bind(("0.0.0.0", app_conf.http_server_port))?
        .run()
        .await
}
