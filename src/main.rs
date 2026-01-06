use actix_web::{App, HttpServer, Responder, get, middleware::Logger, web};
mod routes;
mod utils;

// #[get("/hello/{name}")]
// async fn greet(name: web::Path<String>) -> impl Responder {
//     format!("Hello {name}!")
// }

#[actix_web::main] // or #[tokio::main]
async fn main() -> std::io::Result<()> {
    
    if std::env::var_os("RUST_LOG").is_none(){
        unsafe {
            std::env::set_var("RUST_LOG", "actix_web=info");// this is unsafe in recent rust, so just mark it as unsafe and use it
        }
    }

    dotenv::dotenv().ok();

    env_logger::init();
    let port = utils::constants::PORT.clone();
    let address = utils::constants::ADDRESS.clone();

    HttpServer::new(|| {
        App::new()
            .wrap(Logger::default()) // this is a middleware for logging request and response summaries to the terminal. This middleware uses the log crate to output information. Enable log's output for the "actix_web" scope using env_logger   or similar crate.
            .configure(routes::home_routes::config)
            // .service(greet)
    })
    .bind((address, port))?
    .run()
    .await
}

