pub mod error;
pub mod prelude;
pub mod models;
pub mod routes;
pub mod config;
pub mod cli;

use crate::prelude::*;

#[tokio::main]
async fn main() -> Result<()> {
    use clap::Parser;
    /*
    use actix_web::{HttpServer, App, web::Data};
    use futures::executor::block_on;
    use crate::config::Config;
    
    let config = Config::read(String::from("./config.json"))?;
 
    println!("Server listening on port {}", config.port);
    
    HttpServer::new(|| {
        // i will not fight the borrow checker for 0.5s speed gain parsing the config :|
        let config = Config::read(String::from("./config.json")).unwrap();
        let pool = block_on(create_pool(config.database.clone())).unwrap();

        App::new()
            .app_data(Data::new(pool))
            .app_data(Data::new(config.clone()))
            .service(crate::routes::user::login::login_route)
            .service(crate::routes::user::register::register_route)
            .service(crate::routes::user::delete::delete_route)
            .service(crate::routes::user::info::info_route)
            .service(crate::routes::user::authorize::authorize_route)
    })
    .bind(("127.0.0.1", config.port.clone()))?
    .run()
    .await;
    */

    let cli = crate::cli::MainCli::parse();
    cli.execute();

    return Ok(());
}

async fn create_pool(config: crate::config::DatabaseConfig) -> Result<clorinde::deadpool_postgres::Pool> {
    let mut cfg = clorinde::deadpool_postgres::Config::new();
    
    cfg.user = Some(config.user);
    cfg.password = Some(config.password);
    cfg.host = Some(config.host);
    cfg.port = Some(config.port);
    cfg.dbname = Some(config.database);

    return Ok(cfg.create_pool(
        Some(clorinde::deadpool_postgres::Runtime::Tokio1),
        clorinde::tokio_postgres::NoTls)?
    );
}
