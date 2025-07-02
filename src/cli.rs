use crate::prelude::*;

#[derive(clap::Parser)] // requires `derive` feature
#[command(name = "authin")]
#[command(bin_name = "authin")]
#[command(about = "A simple API for managing users and permissions in closed systems", long_about = None)]
pub enum MainCli {
    #[command(about = "Run the HTTP server", long_about = None)]
    Run(Args),
    #[command(about = "Sync permissions, groups and users data defined in supplied config.json", long_about = None)]
    Sync(Args),
    #[command(about = "Run migrations on the database")]
    Migrate(Args)
}

#[derive(clap::Args, Clone)]
pub struct Args {
    #[clap(long, short)]
    config: Option<String>
}

impl MainCli {
    pub fn execute(self) -> Result<()> {
        use colored::Colorize; 
        
        match self {
            Self::Sync(args) => {
                use futures::executor::block_on;
                use crate::config::Config;
                use crate::models::{User,Group,Permission};

                let config = match Config::read(args.config.unwrap_or(String::from("./authin.json"))) {
                    Ok(config) => config,
                    Err(err) => {
                        println!("{} reading config: {}", "error:".red(), err);
                        std::process::exit(1);
                    }
                };
                let client = match block_on(create_pool(config.database.clone())) {
                    Ok(pool) => pool,
                    Err(err) => {
                        println!("{} Error connecting to the database: {}", "error:".red(), err);
                        std::process::exit(1);
                    }
                };

                println!("{} Syncing permissions...", "+".green());
                match block_on(Permission::sync(&config.permissions, &client)) {
                    Ok(_) => (),
                    Err(err) => {
                        println!("{} {}", "error:".red(), err);

                        std::process::exit(1);
                    }
                };
                
                println!("{} Syncing groups...", "+".green());
                match block_on(Group::sync(&config.groups, &client)) {
                    Ok(_) => (),
                    Err(err) => {
                        println!("{} {}", "error:".red(), err);
                        
                        std::process::exit(1);
                    }
                };
                
                println!("{} Syncing users...", "+".green());
                match block_on(User::sync(&config.users, &client)) {
                    Ok(_) => (),
                    Err(err) => {
                        println!("{} {}", "error:".red(), err);
                        
                        std::process::exit(1);
                    }
                }; 
                
                println!("{} Done.", "+".green());
            },
            Self::Run(args) => {
                use actix_web::{HttpServer, App, web::Data};
                use futures::executor::block_on;
                use crate::config::Config;
                
                let config = match Config::read(args.clone().config.unwrap_or(String::from("./authin.json"))) {
                    Ok(config) => config,
                    Err(err) => {
                        println!("{} Reading config: {}", "error:".red(), err);
                        std::process::exit(1);
                    }
                };
                println!("{} Server starting on port {}", "+".green(), config.port.to_string().underline());
                
                let bind_result = HttpServer::new(move || {
                    let config = match Config::read(args.clone().config.unwrap_or(String::from("./authin.json"))) {
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
            },
            Self::Migrate(args) => {
                use futures::executor::block_on;
                use crate::config::Config;
                use crate::migrations::migrate;

                let config = match Config::read(args.clone().config.unwrap_or(String::from("./authin.json"))) {
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

                match block_on(migrate(&pool)) {
                    Ok(_) => (),
                    Err(err) => {
                        println!("{} Running migrations: {}", "error".red(), err);
                        std::process::exit(1);
                    }
                };
            }
        };

        return Ok(());
    }
}

async fn create_pool(config: crate::config::DatabaseConfig) -> Result<sqlx::postgres::PgPool> {
    use sqlx::postgres::PgPool;

    let connection_string = format!(
        "postgres://{}:{}@{}:{}/{}",
        config.user,
        config.password,
        config.host,
        config.port,
        config.database
    );
    let pool = PgPool::connect(connection_string.as_str()).await?;

    return Ok(pool);
}
