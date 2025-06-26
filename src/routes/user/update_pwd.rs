#[actix_web::patch("/user/pwd")]
pub async fn update_pwd_route(
    req: actix_web::HttpRequest,
    body: actix_web::web::Json<RequestBody>,
    pool: actix_web::web::Data<clorinde::deadpool_postgres::Pool>,
    config: actix_web::web::Data<crate::config::Config>
) -> impl actix_web::Responder {
    use crate::models::user::{get_user,hash_password};
    use clorinde::queries::users::{retrieve_user_permission,update_user_pwd};
    use actix_web::HttpResponse;
    
    let client = match pool.get().await {
        Ok(client) => client,
        Err(_) => return HttpResponse::InternalServerError().body(""),
    };
    
    let headers = req.headers();
    let token = match headers.get("Authorization") {
        Some(token) => token.to_str().unwrap().to_string(),
        None => return HttpResponse::Unauthorized().body("")
    };

    let user = match get_user(&client, token, config.jwt.encryption_key.clone()).await {
        Ok(user) => user,
        Err(_) => return HttpResponse::BadRequest().body("")
    };

    let pwd = match hash_password(body.pwd.to_string()) {
        Ok(pwd) => pwd,
        Err(_) => return HttpResponse::BadRequest().body("")
    };

    match update_user_pwd().bind(&client, &pwd, &user.login).await {
        Ok(_) => return HttpResponse::Ok().body(""),
        Err(_) => return HttpResponse::InternalServerError().body("")
    }
}

#[derive(serde::Deserialize)]
struct RequestBody {
    pwd: String
}
