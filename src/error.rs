#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Generic {0}")]
    Generic(String),
    
    #[error(transparent)]
    IO(#[from] std::io::Error),
    
    #[error("Serde {0}")]
    Serde(#[from] serde_json::Error),
    
    #[error(transparent)]
    Actix(#[from] actix_web::Error),

    #[error(transparent)]
    Sqlx(#[from] sqlx::Error),
    
    #[error(transparent)]
    JWT(#[from] jsonwebtoken::errors::Error),

    #[error(transparent)]
    PasswordHash(#[from] argon2::password_hash::Error),
    
    #[error(transparent)]
    Clap(#[from] clap::Error)
}
