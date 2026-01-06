use actix_web::web;
use super::handlers;

pub fn config(config: &mut web::ServiceConfig) {

    config.service(
            web::scope("/home")
                .service(handlers::home_handler::greet)// /home/handler
                .service(handlers::home_handler::test) // / home/test
        );
    // .service adds a service to the config
}
