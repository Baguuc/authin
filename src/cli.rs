use crate::prelude::*;

#[derive(clap::Parser)] // requires `derive` feature
#[command(name = "authin")]
#[command(bin_name = "authin")]
pub enum MainCli {
    Sync,
    Run,
    Grant {
        object: String,
        subject: String
    },
    Revoke {
        object: String,
        subject: String
    }
}

impl MainCli {
    pub fn execute(self) -> Result<()> {
        match self {
            Self::Sync => {
                use crate::models::{permission::sync_permissions, group::sync_groups, user::sync_users};
                use futures::executor::block_on;
                use crate::config::Config;

                let config = Config::read(String::from("./config.json"))?;
                let pool = block_on(create_pool(config.database.clone()))?;
                let client = match block_on(pool.get()) {
                    Ok(client) => client,
                    Err(_) => return Err(Error::Generic(String::from("Cannot create postgres client from the pool")))
                };
                
                block_on(sync_permissions(&client, &config.permissions))?;
                block_on(sync_groups(&client, &config.groups))?;
                block_on(sync_users(&client, &config.users))?;
            },
            Self::Run => {
                use actix_web::{HttpServer, App, web::Data};
                use futures::executor::block_on;
                use crate::config::Config;
                
                let config = Config::read(String::from("./config.json"))?;
                println!("Server starting on port {}", &config.port);
                
                block_on(
                    HttpServer::new(|| {
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
                    .bind(("127.0.0.1", config.port))?
                    .run()
                );
            },
            Self::Grant { subject, object } => {
                use clorinde::queries::groups::grant_group;
                use futures::executor::block_on;
                use crate::config::Config;
                
                let config = Config::read(String::from("./config.json")).unwrap();
                let pool = block_on(create_pool(config.database.clone())).unwrap();
                let client = block_on(pool.get()).unwrap();

                block_on(grant_group().bind(&client, &subject, &object));
            },
            Self::Revoke { subject, object } => {
                use clorinde::queries::groups::revoke_group;
                use futures::executor::block_on;
                use crate::config::Config;
                
                let config = Config::read(String::from("./config.json")).unwrap();
                let pool = block_on(create_pool(config.database.clone())).unwrap();
                let client = block_on(pool.get()).unwrap();

                block_on(revoke_group().bind(&client, &subject, &object));
            }
        };

        return Ok(());
    }
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
