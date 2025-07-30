mod error;
mod models;
mod repo;
mod routes;
mod rpa;
mod service;
mod sys;
use std::sync::Arc;

use actix_web::{App, HttpServer};
use chromiumoxide::cdp::browser_protocol::network::{Cookie, CookieParam};
use google_cloud_pubsub::{
    client::{Client, ClientConfig},
    topic,
};
use repo::AddPolicyRepoImp;
use rpa::{BindingPortalAutomation, BindingPortalAutomationImp};
use service::AddPolicyServiceImp;
use sqlx::{PgPool, postgres::PgPoolOptions};
use sys::application_config::load_env;
use tokio::{fs, io};

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

    // Init gcp pubsub
    let config = ClientConfig::default().with_auth().await.unwrap();
    let client = Client::new(config).await.unwrap();
    let topic = client.topic(&app_conf.gcp_binding_result_topic);
    if app_conf.gcp_pubsub_emulator == true && topic.exists(None).await.unwrap()
    {
        topic.create(None, None).await.unwrap()
    }

    let publisher = topic.new_publisher(None);

    let repo = AddPolicyRepoImp::new(pg_pool);
    let cookies = load_cookies(app_conf.session_path).await.unwrap();
    let rpa =
        BindingPortalAutomationImp::new(cookies, app_conf.portal_url).await;
    let service =
        AddPolicyServiceImp::new(Box::new(repo), Box::new(rpa), publisher);

    HttpServer::new(|| App::new().service(routes::register()))
        .bind(("0.0.0.0", app_conf.http_server_port))?
        .run()
        .await
}

async fn load_cookies(
    path: String,
) -> Result<Vec<CookieParam>, Box<dyn std::error::Error>> {
    todo!()
    // let cookie_data = fs::read_to_string(path).await?;
    // let cookies = serde_json::from_str::<Vec<Cookie>>(&cookie_data)?;
}
