#[actix_web::get("/authorize/{permission_name}")]
pub async fn authorize_route(
    req: actix_web::HttpRequest,
    path: actix_web::web::Path<RequestPath>,
    pool: actix_web::web::Data<clorinde::deadpool_postgres::Pool>
) -> impl actix_web::Responder {
    use clorinde::queries::users::retrieve_user_permission;
    use crate::functions::user::get_user;
    use actix_web::{HttpResponse, http::header::ContentType};
    use dotenv::var;
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
    let jwt_key = var("JWT_KEY").unwrap();

    let user = match get_user(&client, token, jwt_key).await {
        Ok(user) => user,
        Err(_) => return HttpResponse::BadRequest().body("")
    };

    match retrieve_user_permission().bind(&client, &user.login, &path.permission_name).one().await {
        Ok(_) => return HttpResponse::Ok().body(""),
        Err(_) => return HttpResponse::Unauthorized().body("")
    };
}

#[derive(serde::Deserialize)]
struct RequestPath {
    permission_name: String
}
