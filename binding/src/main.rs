mod handler;
mod repo;
mod svc;
mod sys;
mod rpa;

use std::sync::Arc;

use actix_web::{App, HttpServer};
use repo::add_policy::AddPolicyRepoImp;
use sqlx::{PgPool, postgres::PgPoolOptions};
use sys::application_config::load_env;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let app_conf = load_env().unwrap();

    let pg_pool = Arc::new(
        PgPoolOptions::new()
            .max_connections(5)
            .connect(&app_conf.postgres.to_url())
            .await
            .unwrap(),
    );

    let repo = AddPolicyRepoImp::new(pg_pool);

    HttpServer::new(|| App::new().service(handler::register()))
        .bind(("0.0.0.0", app_conf.http_server_port))?
        .run()
        .await
}
