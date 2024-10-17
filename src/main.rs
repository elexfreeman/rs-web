use actix_web::{get, App, Error, HttpResponse, HttpServer, Result};
mod modules;
use modules::sample::sample_ctrl::sample_route;
use crate::error_s::response_error;

mod interfaces;
use crate::interfaces::product_i::*;

mod system;
use crate::system::*;

async fn get_data() -> Result<ProductPageI, Error> {
    // URL, на который будем отправлять запрос
    let url = "http://lavacacao.ru:1337/api/page-data-common?populate=deep";
    // Выполняем GET-запрос
    let data = reqwest::get(url)
        .await
        .map_err(actix_web::error::ErrorInternalServerError)?; // <- convert it into an actix_web::Error
   // Err("error code: 13".to_string()).map_err(actix_web::error::ErrorInternalServerError)?; // <- convert it into an actix_web::Error
    data.json::<ProductPageI>()
        .await
        .map_err(actix_web::error::ErrorInternalServerError) // <- convert it into an actix_web::Error
}



#[get("/some_route")]
async fn some_route() -> Result<HttpResponse, Error> {
    log::info!("Request from /some_route");
    fa_action!(get_data(), ProductPageI, response_error).await
}


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));
    log::info!("starting HTTP server at http://localhost:8080");
    HttpServer::new(|| {
        App::new()
            .service(some_route)
            .service(sample_route)
    })
    .workers(4)
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
