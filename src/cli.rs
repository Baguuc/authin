use crate::prelude::*;

#[derive(clap::Parser)] // requires `derive` feature
#[command(name = "authin")]
#[command(bin_name = "authin")]
pub enum MainCli {
    Sync,
    Run
}

impl MainCli {
    pub fn execute(self) -> Result<()> {
        use colored::Colorize; 
        
        match self {
            Self::Sync => {
                use crate::models::{permission::sync_permissions, group::sync_groups, user::sync_users};
                use futures::executor::block_on;
                use crate::config::Config;

                let config = match Config::read(String::from("./config.json")) {
                    Ok(config) => config,
                    Err(err) => {
                        println!("{} reading config: {}", "error:".red(), err);
                        std::process::exit(1);
                    }
                };
                let pool = match block_on(create_pool(config.database.clone())) {
                    Ok(pool) => pool,
                    Err(err) => {
                        println!("{} Error connecting to the database: {}", "error:".red(), err);
                        std::process::exit(1);
                    }
                };
                let mut client = match block_on(pool.get()) {
                    Ok(client) => client,
                    Err(_) => {
                        println!("{} Cannot create postgres client from the pool", "error:".red());
                        std::process::exit(1);
                    }
                };
                let tx = block_on(client.transaction())?;

                println!("{} Syncing permissions...", "+".green());
                match block_on(sync_permissions(&tx, &config.permissions)) {
                    Ok(_) => (),
                    Err(err) => {
                        println!("{} {}", "error:".red(), err);
                        std::process::exit(1);
                    }
                };
                
                println!("{} Syncing groups...", "+".green());
                match block_on(sync_groups(&tx, &config.groups)) {
                    Ok(_) => (),
                    Err(err) => {
                        println!("{} {}", "error:".red(), err);
                        std::process::exit(1);
                    }
                };
                
                println!("{} Syncing users...", "+".green());
                match block_on(sync_users(&tx, &config.users)) {
                    Ok(_) => (),
                    Err(err) => {
                        println!("{} {}", "error:".red(), err);
                        std::process::exit(1);
                    }
                };

                match block_on(tx.commit()) {
                    Ok(_) => println!("{} Done.", "+".green()),
                    Err(err) => {
                        println!("{} {}", "error:".red(), err);
                        std::process::exit(1);
                    }
                };
            },
            Self::Run => {
                use actix_web::{HttpServer, App, web::Data};
                use futures::executor::block_on;
                use crate::config::Config;
                
                let config = match Config::read(String::from("./config.json")) {
                    Ok(config) => config,
                    Err(err) => {
                        println!("{} Reading config: {}", "error:".red(), err);
                        std::process::exit(1);
                    }
                };
                println!("{} Server starting on port {}", "+".green(), config.port.to_string().underline());
                
                let bind_result = HttpServer::new(|| {
                    let config = match Config::read(String::from("./config.json")) {
                        Ok(config) => config,
                        Err(err) => {
                            println!("{} Reading config: {}", "error:".red(), err);
                            std::process::exit(1);
                        }
                    };
                    let pool = match block_on(create_pool(config.database.clone())) {
                        Ok(pool) => pool,
                        Err(err) => {
                            println!("{} Connecting to the database: {}", "error:".red(), err);
                            std::process::exit(1);
                        }
                    };
                    let mut client = match block_on(pool.get()) {
                        Ok(client) => client,
                        Err(_) => {
                            println!("{} Cannot create postgres client from the pool", "error:".red());
                            std::process::exit(1);
                        }
                    };
                    
                    App::new()
                        .app_data(Data::new(pool))
                        .app_data(Data::new(config.clone()))
                        .service(crate::routes::user::login::login_route)
                        .service(crate::routes::user::info::info_route)
                        .service(crate::routes::user::authorize::authorize_route)
                        .service(crate::routes::user::update_pwd::update_pwd_route)
                })
                .bind(("127.0.0.1", config.port.clone()));

                match bind_result {
                    Ok(server) => block_on(server.run()),
                    Err(_) => {
                        println!("{} Cannot bind server to port {}", "error:".red(), config.port);
                        
                        std::process::exit(1);
                    }
                };
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
