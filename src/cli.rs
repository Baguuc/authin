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

                let config = W(Config::read(args.config.unwrap_or(String::from("./authin.json"))))
                    .or_print_err();
                let pool = W(block_on(create_pool(config.database.clone())))
                    .or_print_err();

                println!("{} Syncing permissions...", "+".green());
                W(block_on(Permission::sync(&config.permissions, &pool)))
                    .or_print_err();
                
                println!("{} Syncing groups...", "+".green());
                W(block_on(Group::sync(&config.groups, &pool)))
                    .or_print_err();
                
                println!("{} Syncing users...", "+".green());
                W(block_on(User::sync(&config.users, &pool)))
                    .or_print_err();
                
                println!("{} Done.", "+".green());
            },
            Self::Run(args) => {
                use actix_web::{HttpServer, App, web::Data};
                use futures::executor::block_on;
                use crate::config::Config;
                
                let config = W(Config::read(args.clone().config.unwrap_or(String::from("./authin.json"))))
                    .or_print_err();
                
                println!("{} Server starting on port {}", "+".green(), config.port.to_string().underline());
                
                let server = HttpServer::new(move || {
                    let config = W(Config::read(args.clone().config.unwrap_or(String::from("./authin.json"))))
                        .or_print_err();
                    let pool = W(block_on(create_pool(config.database.clone())))
                        .or_print_err();
                    
                    App::new()
                        .app_data(Data::new(pool))
                        .app_data(Data::new(config.clone()))
                        .service(crate::routes::user::login::login_route)
                        .service(crate::routes::user::info::info_route)
                        .service(crate::routes::user::authorize::authorize_route)
                        .service(crate::routes::user::update_pwd::update_pwd_route)
                });

                let binded_server = match server.bind(("127.0.0.1", config.port.clone())) {
                    Ok(server) => server,
                    Err(_) => {
                        crate::error::print_error("Cannot bind to port", config.port);
                        
                        std::process::exit(1);
                    }
                };

                block_on(binded_server.run());
            },
            Self::Migrate(args) => {
                use futures::executor::block_on;
                use crate::config::Config;
                use crate::migrations::migrate;

                let config = W(Config::read(args.config.unwrap_or(String::from("./authin.json"))))
                    .or_print_err();
                let pool = W(block_on(create_pool(config.database.clone())))
                    .or_print_err();
                
                W(block_on(migrate(&pool)))
                    .or_print_err();
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
