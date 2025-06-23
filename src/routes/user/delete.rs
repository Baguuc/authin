#[actix_web::delete("/users/{login}")]
pub async fn delete_route(
    req: actix_web::HttpRequest,
    path: actix_web::web::Path<RequestPath>,
    pool: actix_web::web::Data<clorinde::deadpool_postgres::Pool>
) -> impl actix_web::Responder {
    use clorinde::queries::users::delete_user;
    use crate::functions::user::get_user;
    use actix_web::HttpResponse;
    use dotenv::var;
    
    let client = match pool.get().await {
        Ok(client) => client,
        Err(_) => return HttpResponse::InternalServerError(),
    };
    
    let headers = req.headers();
    let token = match headers.get("Authorization") {
        Some(token) => token.to_str().unwrap().to_string(),
        None => return HttpResponse::Unauthorized()
    };

    let user = match get_user(&client, token, var("JWT_KEY").unwrap()).await {
        Ok(user) => user,
        Err(_) => return HttpResponse::BadRequest()
    };

    if user.login != path.login {
        return HttpResponse::Unauthorized();
    }
    
    match delete_user().bind(&client, &path.login).await {
        Ok(_) => (),
        Err(_) => return HttpResponse::BadRequest(),
    };

    return HttpResponse::Ok();
}

#[derive(serde::Deserialize)]
struct RequestPath {
    login: String,
}
