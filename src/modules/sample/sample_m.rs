use actix_web::HttpRequest;
use actix_web::{web, Error};

use crate::system::base_model::BaseModel;

use crate::modules::sample::sample_r::Request;
use crate::modules::sample::sample_r::Response;

pub struct SampleM {
    base_model: BaseModel,
}

impl SampleM {
    pub fn new(req: HttpRequest) -> Self {
        let base_model = BaseModel::new(req);
        Self { base_model }
    }

    pub async fn get_data(&self, request: web::Json<Request>) -> Result<Response, Error> {
        let out = Response {
            title: request.title.clone(),
            description: request.description.clone(),
        };
        Ok(out)
    }
}
