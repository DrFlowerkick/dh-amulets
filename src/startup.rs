//! src/startup.rs

use crate::configuration::Settings;
use crate::routes::{four_player, home, three_player, two_player};
use actix_web::{dev::Server, web, App, HttpServer};
use actix_files::Files;
use anyhow::{Context, Result};
use std::net::TcpListener;
use tracing_actix_web::TracingLogger;

pub struct Application {
    port: u16,
    server: Server,
}

impl Application {
    pub async fn build(configuration: Settings) -> Result<Self> {
        let address = format!(
            "{}:{}",
            configuration.application.host, configuration.application.port
        );
        let listener = TcpListener::bind(address).context("Failed to bind to address")?;
        let port = listener.local_addr().unwrap().port();
        let server = run(listener).await?;

        Ok(Self { port, server })
    }

    pub fn port(&self) -> u16 {
        self.port
    }

    pub async fn run_until_stopped(self) -> Result<()> {
        self.server.await.context("Failed to run server.")
    }
}

async fn run(listener: TcpListener) -> Result<Server> {
    let server = HttpServer::new(move || {
        App::new()
            .wrap(TracingLogger::default())
            .route("/", web::get().to(home))
            .route("/two_player", web::get().to(two_player))
            .route("/three_player", web::get().to(three_player))
            .route("/four_player", web::get().to(four_player))
            .service(Files::new("/static", "static").show_files_listing())
    })
    .listen(listener)
    .context("Failed to start listening on HttpServer.")?
    .run();
    Ok(server)
}
