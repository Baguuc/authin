pub mod functions;
pub mod error;
pub mod prelude;

use crate::prelude::*;

#[tokio::main]
async fn main() -> Result<()> {
    dotenv::dotenv().ok();

    let pool = create_pool()
        .await
        .unwrap();
    let client = pool.get()
        .await
        .unwrap();

    crate::functions::user::register(&client, String::from("SomeUser"), String::from("123")).await;
    
    let jwt_key = dotenv::var("JWT_KEY")?;
    let token = crate::functions::user::login(&client, String::from("SomeUser"), String::from("123"), jwt_key.clone()).await.unwrap();
    println!("User: {:?}", crate::functions::user::get_user(&client, token, jwt_key).await.unwrap());
    
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
