use crate::prelude::*;

#[derive(serde::Serialize, serde::Deserialize)]
pub struct User {
    pub id: i32,
    pub login: String,
    pub pwd: String
}

pub async fn register(client: &clorinde::deadpool_postgres::Client, login: &String, password: &String) -> Result<()> { 
    use clorinde::queries::users::insert_user; 

    let pwd = hash_password(password.clone())?;

    let result = insert_user()
        .bind(client, &login, &pwd)
        .await?;
    
    return Ok(());
}

pub async fn login(client: &clorinde::deadpool_postgres::Client, login: &String, password: &String, encoding_key: String) -> Result<String> {
    use clorinde::queries::users::retrieve_user; 
    
    let user = retrieve_user()
        .bind(client, login)
        .one()
        .await?;

    if !verify_password(password, &user.pwd) {
        return Err(Error::Generic(String::from("Wrong password")));
    };
    
    let token = generate_jwt(
        user.login,
        (chrono::Utc::now() + chrono::Duration::days(7)).timestamp() as usize,
        encoding_key
    )?;
    
    return Ok(token);
}

pub async fn get_user(client: &clorinde::deadpool_postgres::Client, token: String, encoding_key: String) -> Result<User> {
    let claims = get_claims(token, encoding_key)?;
    let user = clorinde::queries::users::retrieve_user()
        .bind(client, &claims.sub)
        .one()
        .await?;
    
    return Ok(User { id: user.id, login: user.login, pwd: user.pwd });
}

fn hash_password(password: String) -> Result<String> {
    use argon2::{Argon2, PasswordHasher, password_hash::{SaltString, rand_core::OsRng}};
    
    let pwd = password.as_bytes();
    let salt = SaltString::generate(&mut OsRng);

    let password_hash = match Argon2::default().hash_password(pwd, &salt) {
        Ok(hash) => hash,
        Err(err) => return Err(Error::Generic(err.to_string())),
    }
    .to_string();

    return Ok(password_hash);
}

fn verify_password(password: &String, password_hash: &String) -> bool {
    use argon2::{Argon2, PasswordHash, PasswordVerifier, password_hash::Encoding};

    let password_hash = &PasswordHash::parse(password_hash.as_str(), Encoding::B64)
        .unwrap();

    let _ = match Argon2::default().verify_password(password.as_bytes(), password_hash) {
        Ok(_) => return true,
        Err(_) => return false
    };
}

#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct Claims {
    pub sub: String,
    pub exp: usize
}

fn generate_jwt(login: String, expires: usize, key: String) -> Result<String> {
    use jsonwebtoken::{encode, Header, EncodingKey};

    let claims = Claims {
        sub: login,
        exp: expires,
    };

    let encoded = encode(
        &Header::default(),
        &claims, 
        &EncodingKey::from_secret(key.as_ref())
    )?;

    return Ok(encoded);
}

fn get_claims(token: String, encoding_key: String) -> Result<Claims> {
    use jsonwebtoken::{decode, Header, DecodingKey, Validation};
    
    let decoded = decode::<Claims>(
        &token,
        &DecodingKey::from_secret(encoding_key.as_ref()),
        &Validation::default()
    )?;

    return Ok(decoded.claims);
}
