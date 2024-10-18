use actix_web::{post, web, Error, HttpRequest, HttpResponse};

use crate::fa_action;
use crate::modules::sample::sample_m::get_data;
use crate::modules::sample::sample_r::Request as SampleRequest;
use crate::modules::sample::sample_r::Response as SampleResponse;
use crate::system::error_s::response_error;

use crate::system::BaseCtrl;


struct SampleCtrl {
    base_ctrl: BaseCtrl,
}

impl SampleCtrl {
    fn new(req: HttpRequest) -> Self {
        let base_ctrl = BaseCtrl::new(req);
        Self { base_ctrl }
    }

    async fn sample_route_one(
        &self,
        request: web::Json<SampleRequest>,
    ) -> Result<HttpResponse, Error> {
        let user_data = &self.base_ctrl.get_user_data();
        log::info!("Request from /some_route");
        log::info!("User data str: {}", user_data.sample_string);
        fa_action!(get_data(request), SampleResponse, response_error).await
    }
    async fn sample_route_two(
        &self,
        request: web::Json<SampleRequest>,
    ) -> Result<HttpResponse, Error> {
        log::info!("Request from /some_route_two");
        fa_action!(get_data(request), SampleResponse, response_error).await
    }
}

//
//
//
//trait SampleCtrlTrait {
//    async fn sample_route_one(
//        &self,
//        request: web::Json<SampleRequest>,
//    ) -> Result<HttpResponse, Error>;
//
//    async fn sample_route_two(
//        &self,
//        request: web::Json<SampleRequest>,
//    ) -> Result<HttpResponse, Error>;
//}
//
//
//impl SampleCtrlTrait for BaseCtrl {
//    async fn sample_route_one(
//        &self,
//        request: web::Json<SampleRequest>,
//    ) -> Result<HttpResponse, Error> {
//        let data = &self.req.app_data::<Data<Mutex<UserData>>>().unwrap();
//        let mut user_data = data.lock().unwrap();
//        log::info!("Request from /some_route");
//        log::info!("User data str: {}", user_data.sample_string);
//        fa_action!(get_data(request), SampleResponse, response_error).await
//    }
//
//    async fn sample_route_two(
//        &self,
//        request: web::Json<SampleRequest>,
//    ) -> Result<HttpResponse, Error> {
//        log::info!("Request from /some_route_two");
//        let data = &self.req.app_data::<Data<Mutex<UserData>>>().unwrap();
//        let mut user_data = data.lock().unwrap();
//        user_data.sample_string = String::from("new_value");
//        fa_action!(get_data(request), SampleResponse, response_error).await
//    }
//}
//
#[post("/sample_route_one")]
pub async fn sample_route_one(
    body: web::Json<SampleRequest>,
    req: HttpRequest,
) -> Result<HttpResponse, Error> {
//    let sample_ctrl = SampleCtrl::new(req);
    SampleCtrl::new(req).sample_route_one(body).await
}

#[post("/sample_route_two")]
pub async fn sample_route_two(
    body: web::Json<SampleRequest>,
    req: HttpRequest,
) -> Result<HttpResponse, Error> {
    SampleCtrl::new(req).sample_route_two(body).await
}
