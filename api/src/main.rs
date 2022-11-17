mod api;

use actix_web::{web, App, HttpServer};
use api::{todo, user};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let server = HttpServer::new(|| {
        App::new().service(
            web::scope("/api")
                .configure(user::user_api_routes)
                .configure(todo::todo_api_routes),
        )
    })
    .bind(("0.0.0.0", 8080))?
    // .bind(("127.0.0.1", 8080))?
    .run();
    println!("server running on 0.0.0.0:8080");
    server.await
}
