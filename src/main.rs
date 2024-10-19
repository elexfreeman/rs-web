use actix_web::{web, App, HttpServer};
mod modules;
use modules::sample::sample_ctrl::*;

mod interfaces;

mod system;
use crate::system::sys_data::SysData;

//async fn get_data() -> Result<ProductPageI, Error> {
//    // URL, на который будем отправлять запрос
//    let url = "http://lavacacao.ru:1337/api/page-data-common?populate=deep";
//    // Выполняем GET-запрос
//    let data = reqwest::get(url)
//        .await
//        .map_err(actix_web::error::ErrorInternalServerError)?; // <- convert it into an actix_web::Error
//                                                               // Err("error code: 13".to_string()).map_err(actix_web::error::ErrorInternalServerError)?; // <- convert it into an actix_web::Error
//    data.json::<ProductPageI>()
//        .await
//        .map_err(actix_web::error::ErrorInternalServerError) // <- convert it into an actix_web::Error
//}


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));
    log::info!("starting HTTP server at http://localhost:8080");
    let user_data = web::Data::new(SysData {
        sample_string: "default_value".to_string(),
    });
    HttpServer::new(move || {
        App::new()
            .app_data(user_data.clone())
            .service(sample_route_one)
            .service(sample_route_two)
    })
    .workers(4)
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
