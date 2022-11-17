use actix_web::{get, post, web, HttpResponse, Responder};
use serde::Deserialize;

pub fn user_api_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/users")
            .service(get_users)
            .service(get_user)
            .service(new_user)
            .service(edit_user),
    );
}
#[derive(Deserialize, Debug)]
pub struct NewUserReqBody {
    first_name: String,
    last_name: String,
}

#[get("")]
async fn get_users() -> impl Responder {
    HttpResponse::Ok().body("get users list")
}

#[post("")]
async fn new_user(_body: web::Json<NewUserReqBody>) -> impl Responder {
    println!("{:?}", _body);
    HttpResponse::Ok().body(format!("name:{} {}", _body.first_name, _body.last_name))
}

#[get("/{id}")]
async fn get_user(id: web::Path<String>) -> impl Responder {
    if id.len() >= 5 {
        HttpResponse::Ok().body(format!("hello {id}"))
    } else if id.len() == 0 {
        HttpResponse::NotFound().body(format!("user not found"))
    } else {
        HttpResponse::BadRequest().body(format!("user id should be longer then 5"))
    }
}

#[post("/{id}")]
async fn edit_user(id: web::Path<String>, _body: web::Json<NewUserReqBody>) -> impl Responder {
    HttpResponse::Ok().body(format!(
        "id:{} name:{} {}",
        id, _body.first_name, _body.last_name
    ))
}
