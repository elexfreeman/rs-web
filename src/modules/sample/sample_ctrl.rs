use actix_web::{post, web, Error, HttpRequest, HttpResponse};

use crate::fa_action;
use crate::system::base_ctrl::BaseCtrl;
use crate::system::error_s::response_error;

use crate::modules::sample::sample_r::SampleRouteR;

use crate::modules::sample::sample_m::SampleM;

struct SampleCtrl {
    base_ctrl: BaseCtrl,
    sample_m: SampleM,
}

impl SampleCtrl {
    fn new(req: HttpRequest) -> Self {
        let base_ctrl = BaseCtrl::new(req.clone());
        let sample_m = SampleM::new(req.clone());
        Self {
            base_ctrl,
            sample_m,
        }
    }

    async fn sample_route_one(
        &self,
        request: web::Json<SampleRouteR::Request>,
    ) -> Result<HttpResponse, Error> {
        log::info!("Request from /some_route");

        let user_data = &self.base_ctrl.get_sys_data();
        log::info!("User data str: {}", user_data.sample_string);

        fa_action!(
            &self.sample_m.get_data(request),
            SampleRouteR::Response,
            response_error
        )
        .await
    }
    async fn sample_route_two(
        &self,
        request: web::Json<SampleRouteR::Request>,
    ) -> Result<HttpResponse, Error> {
        log::info!("Request from /some_route_two");
        fa_action!(
            &self.sample_m.get_data(request),
            SampleRouteR::Response,
            response_error
        )
        .await
    }
}

#[post("/sample_route_one")]
pub async fn sample_route_one(
    body: web::Json<SampleRouteR::Request>,
    req: HttpRequest,
) -> Result<HttpResponse, Error> {
    SampleCtrl::new(req).sample_route_one(body).await
}

#[post("/sample_route_two")]
pub async fn sample_route_two(
    body: web::Json<SampleRouteR::Request>,
    req: HttpRequest,
) -> Result<HttpResponse, Error> {
    SampleCtrl::new(req).sample_route_two(body).await
}
