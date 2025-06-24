#[actix_web::post("/user")]
pub async fn login_route(
    body: actix_web::web::Json<RequestBody>,
    pool: actix_web::web::Data<clorinde::deadpool_postgres::Pool>
) -> impl actix_web::Responder {
    use crate::models::user::login;
    use actix_web::HttpResponse;
    use dotenv::var;
    
    let client = match pool.get().await {
        Ok(client) => client,
        Err(_) => return HttpResponse::InternalServerError().body("")
    };
    
    let token = match login(&client, &body.login, &body.pwd, var("JWT_KEY").unwrap()).await {
        Ok(token) => token,
        Err(_) => return HttpResponse::Unauthorized().body("")
    };

    return HttpResponse::Ok()
        .body(token);
}

#[derive(serde::Deserialize)]
struct RequestBody {
    login: String,
    pwd: String
}
