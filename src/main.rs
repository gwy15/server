#[macro_use]
extern crate log;

use actix_web::{web, App, HttpServer};
use anyhow::*;
use clap::Parser;
use server::db;
use server::Config;

async fn load_config() -> Result<Config> {
    let opts = server::cli::Opts::parse();
    let config_content = tokio::fs::read_to_string(opts.config)
        .await
        .context("Failed to load config file.")?;
    let config: Config = toml::from_str(&config_content).context("Failed to parse config file.")?;
    info!("config file loaded.");
    Ok(config)
}

#[actix_web::main]
async fn main() -> Result<()> {
    log4rs::init_file("log4rs.yml", Default::default()).context("log4rs init failed")?;
    let config = load_config().await?;
    let pool = db::connect(config.db_max_connections, &config.db_uri).await?;
    let pool = web::Data::new(pool);

    let mut server: _ = HttpServer::new(move || {
        App::new()
            // middleware
            .wrap(actix_web::middleware::Logger::default())
            // custom error handing
            .app_data(web::FormConfig::default().error_handler(server::handlers::form_error))
            .app_data(web::JsonConfig::default().error_handler(server::handlers::json_error))
            .app_data(web::PathConfig::default().error_handler(server::handlers::path_error))
            .app_data(web::QueryConfig::default().error_handler(server::handlers::query_error))
            .app_data(pool.clone())
            // services
            .default_service(web::route().to(server::handlers::not_found))
    });
    if let Some(workers) = config.workers {
        info!("set workers to {}", workers);
        server = server.workers(workers);
    }

    info!("starting server on {:?}", config.bind);
    for bind in config.bind.iter() {
        server = server.bind(bind)?;
    }

    server.run().await?;

    Ok(())
}
