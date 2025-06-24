use crate::prelude::*;

#[derive(clap::Parser)] // requires `derive` feature
#[command(name = "authin")]
#[command(bin_name = "authin")]
pub enum MainCli {
    Sync
}

impl MainCli {
    pub fn execute(self) -> Result<()> {
        use crate::models::{permission::sync_permissions, group::sync_groups};
        use futures::executor::block_on;
        use crate::config::Config;

        match self {
            Self::Sync => {
                let config = Config::read(String::from("./config.json"))?;
                let pool = block_on(create_pool(config.database.clone()))?;
                let client = match block_on(pool.get()) {
                    Ok(client) => client,
                    Err(_) => return Err(Error::Generic(String::from("Cannot create postgres client from the pool")))
                };
                
                block_on(sync_permissions(&client, &config.permissions))?;
                block_on(sync_groups(&client, &config.groups))?;
            },
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
