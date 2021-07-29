use actix_web::{get, web, App, HttpServer, Responder};

#[get("/{name}/index.html")]
async fn index(name: web::Path<String>) -> impl Responder {
    format!("Hello World! name:{}", name)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(index))
        .bind("127.0.0.1:8080")?
        .run()
        .await
}
