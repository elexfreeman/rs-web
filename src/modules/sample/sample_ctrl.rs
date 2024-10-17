use actix_web::{post, web, Error, HttpResponse};

use crate::fa_action;
use crate::modules::sample::sample_m::get_data;
use crate::modules::sample::sample_r::Request as SampleRequest;
use crate::modules::sample::sample_r::Response as SampleResponse;
use crate::system::error_s::response_error;

#[post("/sample_route")]
pub async fn sample_route(request: web::Json<SampleRequest>) -> Result<HttpResponse, Error> {
    log::info!("Request from /some_route");
    fa_action!(get_data(request), SampleResponse, response_error).await
}
