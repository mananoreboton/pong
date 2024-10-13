use actix_web::{get, web, App, HttpServer, Responder};

// This function handles all HTTP requests and responds with "pong"
async fn pong() -> impl Responder {
    "pong"
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            // Catch-all route for any HTTP method and path
            .default_service(web::route().to(pong))
    })
    .bind("0.0.0.0:9999")? // Bind the server to localhost on port 8080
    .run()
    .await
}
