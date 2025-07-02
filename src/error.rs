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
        match self.0 {
            Ok(value) => return value,
            Err(err) => {
                match err {
                    Error::Generic(err) => print_error("generic error", err),
                    Error::IO(err) => print_error("IO error", err),
                    Error::Serde(err) => print_error("json format error", err),
                    Error::Actix(err) => print_error("web server error", err),
                    Error::Sqlx(err) => print_error("database error", err),
                    Error::JWT(err) => print_error("JWT token error", err),
                    Error::PasswordHash(err) => print_error("password hashing error", err),
                    Error::Clap(err) => print_error("cli arguments error", err),
                };

                std::process::exit(1);
            }
        }
    }
}

pub fn print_error(msg: impl std::fmt::Display, error: impl std::fmt::Display) {
    use colored::Colorize; 
    
    println!("{} {} {}", "error:".red(), msg, error);
}

pub fn print_ok(msg: impl std::fmt::Display) {
    use colored::Colorize;

    println!("{} {}", "ok:".green(), msg);
}
