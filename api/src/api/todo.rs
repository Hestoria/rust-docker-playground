use actix_web::{get, web, HttpResponse, Responder};

pub fn todo_api_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("/todos").service(get_todos).service(get_todo));
}

#[get("")]
pub async fn get_todos() -> impl Responder {
    HttpResponse::Ok().body("get todos")
}

#[get("/{id}")]
pub async fn get_todo(id: web::Path<String>) -> impl Responder {
    HttpResponse::Ok().body(format!("todo {id}"))
}
