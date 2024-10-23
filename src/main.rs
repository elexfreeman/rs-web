use actix_web::{web, App, HttpServer};
use mongodb::{bson::doc, options::IndexOptions, Client, Collection, IndexModel};

mod modules;
use modules::sample::sample_ctrl;

mod infrastructure;
mod interfaces;

mod system;
use crate::system::ctx_data_sys::CtxDataSys;


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

/// Creates an index on the "username" field to force the values to be unique.
//async fn create_username_index(client: &Client) {
//    const DB_NAME: &str = "myApp";
//    const COLL_NAME: &str = "users";
//    let options = IndexOptions::builder().unique(true).build();
//    let model = IndexModel::builder()
//        .keys(doc! { "username": 1 })
//        .options(options)
//        .build();
//    client
//        .database(DB_NAME)
//        .collection::<User>(COLL_NAME)
//        .create_index(model)
//        .await
//        .expect("creating an index should succeed");
//}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));
    let config = crate::system::config_sys::ConfigSys::get_instance();
    crate::system::config_sys::print_config(&config);
    let app_port = config.app_config.port;

    
    let user_data = web::Data::new(CtxDataSys {
        sample_string: "default_value".to_string(),
    });

    log::info!(
        "starting HTTP server at http://[::1]:{}",
        app_port.to_string()
    );

    HttpServer::new(move || {
        App::new()
            .app_data(user_data.clone())
            .service(sample_ctrl::sample_route_one)
            .service(sample_ctrl::sample_route_two)
            .service(sample_ctrl::sample_init_user_data)
    })
    .workers(4)
    .bind(format!("[::1]:{}", app_port))?
    .run()
    .await
}
