#[actix_web::get("/user")]
pub async fn info_route(
    req: actix_web::HttpRequest,
    pool: actix_web::web::Data<clorinde::deadpool_postgres::Pool>,
    config: actix_web::web::Data<crate::config::Config>,
) -> impl actix_web::Responder {
    use crate::models::user::get_user;
    use actix_web::{HttpResponse, http::header::ContentType};
    use serde_json::to_string;
    
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

    return HttpResponse::Ok()
        .content_type(ContentType::json())
        .body(to_string(&user).unwrap());
}
