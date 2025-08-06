mod error;
mod repo;
mod routes;
mod rpa;
mod service;
mod sys;

use actix_web::{App, HttpServer, web};
use chromiumoxide::{Browser, cdp::browser_protocol::network::CookieParam};
use env_logger::Env;
use futures::StreamExt;
use google_cloud_pubsub::client::{Client, ClientConfig};
use log::warn;
use repo::AddPolicyRepoImp;
use rpa::BindingPortalAutomationImp;
use service::AddPolicyServiceImp;
use sqlx::postgres::PgPoolOptions;
use std::sync::Arc;
use sys::config::{ApplicationConfig, BrowserConfig, PostgresConfig};
use tokio::fs;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenvy::dotenv()
        .inspect_err(|e| warn!("dotenv loading {e}"))
        .ok();

    env_logger::Builder::from_env(Env::default().default_filter_or("info"))
        .init();

    let app_conf: ApplicationConfig = envy::from_env().unwrap();

    let postgres_conf: PostgresConfig =
        envy::prefixed("POSTGRES_").from_env().unwrap();

    let browser_conf: BrowserConfig =
        envy::prefixed("BROWSER_").from_env().unwrap();

    log::debug!("{:?}", postgres_conf.to_url());

    let pg_pool = Arc::new(
        PgPoolOptions::new()
            .max_connections(5)
            .connect(&postgres_conf.to_url())
            .await
            .unwrap(),
    );

    // Init gcp pubsub
    let pubsub_client = ClientConfig::default().with_auth().await.unwrap();
    let client = Client::new(pubsub_client).await.unwrap();
    let topic = client.topic(&app_conf.gcp_binding_result_topic);

    if app_conf.pubsub_emulator_host.is_some()
        && topic.exists(None).await.is_err()
    {
        log::info!("Create topic for Pub/Sub Emualtor");
        topic.create(None, None).await.unwrap()
    }

    let publisher = topic.new_publisher(None);

    let cookies = load_cookies(app_conf.session_path).await.unwrap();
    let repo = Arc::new(AddPolicyRepoImp::new(pg_pool));

    log::info!("Initiate RPA component");

    let rpa = Arc::new(
        BindingPortalAutomationImp::new(
            browser_conf.into(),
            cookies,
            app_conf.portal_url,
            app_conf.screenshot_path,
        )
        .await,
    );

    log::info!("Starting service, :{}", app_conf.http_server_port);
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(Arc::new(AddPolicyServiceImp::new(
                repo.clone(),
                rpa.clone(),
                publisher.clone(),
            ))))
            .service(routes::register())
    })
    .bind(("0.0.0.0", app_conf.http_server_port))?
    .run()
    .await
}

async fn load_cookies(
    path: String,
) -> Result<Vec<CookieParam>, Box<dyn std::error::Error>> {
    let cookie_data = fs::read_to_string(path).await?;
    let cookies = serde_json::from_str::<Vec<CookieParam>>(&cookie_data)?;
    Ok(cookies)
}
