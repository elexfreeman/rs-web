use actix_web::{Error, web};

use crate::modules::sample::sample_r::Request;
use crate::modules::sample::sample_r::Response;

pub async fn get_data(request: web::Json<Request>) -> Result<Response, Error> {
    let out = Response {
        title: request.title.clone(),
        description: request.description.clone(),
    };
    Ok(out)
}