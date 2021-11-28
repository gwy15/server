#[macro_use]
extern crate log;

use actix_web::{web, App, HttpServer};
use anyhow::*;
use clap::Parser;

#[actix_web::main]
async fn main() -> Result<()> {
    log4rs::init_file("log4rs.yml", Default::default()).context("log4rs 初始化失败")?;

    let opts = server::cli::Opts::parse();

    let mut server = HttpServer::new(|| {
        App::new().default_service(web::route().to(server::handlers::not_found))
    });
    if let Some(workers) = opts.workers {
        info!("set workers to {}", workers);
        server = server.workers(workers);
    }

    info!("starting server on {}", opts.bind);
    server.bind(opts.bind)?.run().await?;

    Ok(())
}
