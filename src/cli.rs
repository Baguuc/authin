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
    pub fn execute(self) {
        use futures::executor::block_on;

        match self {
            Self::Run(args) => { block_on(run(args)); },
            Self::Sync(args) => { block_on(sync(args)); },
            Self::Migrate(args) => { block_on(migrate(args)); }
        };
    }
}

async fn run(args: Args) {
    use colored::Colorize;
    use actix_web::{HttpServer, App, web::Data};
    use futures::executor::block_on;
    use clin::components::{success, error, header};
    use crate::config::Config;
    
    migrate(args.clone()).await;
    println!("");
    sync(args.clone()).await;
    println!("");
    
    header("Running web server");

    let config = W(Config::read(args.clone().config.unwrap_or(String::from("./authin.json"))))
        .or_print_err();
    
    success(format!("Server starting on port {}", config.port.to_string().underline()));
    
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
            error("Cannot bind to port", config.port);
            
            std::process::exit(1);
        }
    };

    binded_server.run().await;
}

async fn sync(args: Args) {
    use clin::components::{progress_bar, header};
    use crate::config::Config;
    use crate::models::{User,Group,Permission};
    
    let config = W(Config::read(args.config.unwrap_or(String::from("./authin.json"))))
        .or_print_err();
    let pool = W(create_pool(config.database.clone()).await)
        .or_print_err();
    
    header("Syncing configuration");
        
    progress_bar(30, 0);

    W(Permission::sync(&config.permissions, &pool).await)
        .or_print_err();
    
    progress_bar(30, 10);
    
    W(Group::sync(&config.groups, &pool).await)
        .or_print_err();
    
    progress_bar(30, 20);
    
    W(User::sync(&config.users, &pool).await)
        .or_print_err();
    
    progress_bar(30, 30);
}

async fn migrate(args: Args) {
    use clin::components::header;
    use crate::config::Config;
    use crate::migrations::migrate;
    
    let config = W(Config::read(args.config.unwrap_or(String::from("./authin.json"))))
        .or_print_err();
    let pool = W(create_pool(config.database.clone()).await)
        .or_print_err();
     
    header("Migrating database");
    
    W(migrate(&pool).await)
        .or_print_err();
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
