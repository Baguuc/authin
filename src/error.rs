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

pub type Result<T> = std::result::Result<T, crate::error::Error>;

impl<T> crate::prelude::W<Result<T>> {
    pub fn or_print_err(self) -> T {
        use clin::components::error;
        
        match self.0 {
            Ok(value) => return value,
            Err(err) => {
                match err {
                    Error::Generic(err) => error("generic error", err),
                    Error::IO(err) => error("IO error", err),
                    Error::Serde(err) => error("json format error", err),
                    Error::Actix(err) => error("web server error", err),
                    Error::Sqlx(err) => error("database error", err),
                    Error::JWT(err) => error("JWT token error", err),
                    Error::PasswordHash(err) => error("password hashing error", err),
                    Error::Clap(err) => error("cli arguments error", err),
                };

                std::process::exit(1);
            }
        }
    }
}
