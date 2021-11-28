#[macro_use]
extern crate log;

use actix_web::{web, App, HttpServer};
use anyhow::*;
use clap::Parser;

use server::handlers::example;

#[actix_web::main]
async fn main() -> Result<()> {
    log4rs::init_file("log4rs.yml", Default::default()).context("log4rs init failed")?;

    let opts = server::cli::Opts::parse();

    let mut server = HttpServer::new(|| {
        App::new()
            // middleware
            .wrap(actix_web::middleware::Logger::default())
            // custom error handing
            .app_data(web::FormConfig::default().error_handler(server::handlers::form_error))
            .app_data(web::JsonConfig::default().error_handler(server::handlers::json_error))
            .app_data(web::PathConfig::default().error_handler(server::handlers::path_error))
            .app_data(web::QueryConfig::default().error_handler(server::handlers::query_error))
            // services
            .service(example::service())
            .default_service(web::route().to(server::handlers::not_found))
    });
    if let Some(workers) = opts.workers {
        info!("set workers to {}", workers);
        server = server.workers(workers);
    }

    info!("starting server on {}", opts.bind);
    server.bind(opts.bind)?.run().await?;

    Ok(())
}
