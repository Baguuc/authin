pub mod error;
pub mod prelude;
pub mod functions;
pub mod routes;

use crate::prelude::*;

#[tokio::main]
async fn main() -> Result<()> {
    use dotenv::{dotenv,var};
    use actix_web::{HttpServer, App, web::Data};
    use futures::executor::block_on;

    dotenv().ok();

    let port: u16 = var("HOST_PORT")?
        .parse()
        .unwrap();
    // these are to make sure env has everything
    let _ = var("POSTGRES_USER")?;
    let _ = var("POSTGRES_PASSWORD")?;
    let _ = var("POSTGRES_HOST")?;
    let _ = var("POSTGRES_PORT")?;
    let _ = var("POSTGRES_DATABASE")?;
    let _ = var("JWT_KEY")?;

    println!("Server listening on port {}", port);
    
    HttpServer::new(|| {
        let pool = block_on(create_pool()).unwrap();

        App::new()
            .app_data(Data::new(pool))
            .service(crate::routes::user::login::login_route)
            .service(crate::routes::user::register::register_route)
            .service(crate::routes::user::delete::delete_route)
    })
    .bind(("127.0.0.1", port))?
    .run()
    .await;
    
    return Ok(());
}

async fn create_pool() -> Result<clorinde::deadpool_postgres::Pool> {
    let mut cfg = clorinde::deadpool_postgres::Config::new();

    let username = dotenv::var("POSTGRES_USER")?;
    let password = dotenv::var("POSTGRES_PASSWORD")?;
    let host = dotenv::var("POSTGRES_HOST")?;
    let port: u16 = dotenv::var("POSTGRES_PORT")?
        .parse()
        .unwrap_or(5432);
    let database_name = dotenv::var("POSTGRES_DATABASE")?;
    
    cfg.user = Some(username);
    cfg.password = Some(password);
    cfg.host = Some(host);
    cfg.port = Some(port);
    cfg.dbname = Some(database_name);

    return Ok(cfg.create_pool(
        Some(clorinde::deadpool_postgres::Runtime::Tokio1),
        clorinde::tokio_postgres::NoTls)?
    );
}
