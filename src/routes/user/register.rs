#[actix_web::post("/users")]
pub async fn register_route(
    body: actix_web::web::Json<RequestBody>,
    pool: actix_web::web::Data<clorinde::deadpool_postgres::Pool>
) -> impl actix_web::Responder {
    use crate::models::user::register;
    use actix_web::HttpResponse;
    
    let client = match pool.get().await {
        Ok(client) => client,
        Err(_) => return HttpResponse::InternalServerError(),
    };
    
    match register(&client, &body.login, &body.pwd).await {
        Ok(token) => token,
        Err(_) => return HttpResponse::BadRequest(),
    };

    return HttpResponse::Ok();
}

#[derive(serde::Deserialize)]
struct RequestBody {
    login: String,
    pwd: String
}
