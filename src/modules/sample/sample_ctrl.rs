use actix_web::{post, web, Error, HttpRequest, HttpResponse};

use crate::fa_action;
use crate::system::ctx_sys::CtxSys;
use crate::system::error_s::response_error;

use crate::modules::sample::sample_m::SampleM;
use crate::modules::sample::sample_r::SampleRouteR;

struct SampleCtrl<'a> {
    ctx_sys: &'a CtxSys,
    sample_m: SampleM<'a>,
}

impl<'a> SampleCtrl<'a> {
    fn new(ctx_sys: &'a CtxSys) -> Self {
        let sample_m = SampleM::new(&ctx_sys);
        Self { sample_m, ctx_sys }
    }

    async fn sample_route_one(
        &self,
        request: web::Json<SampleRouteR::Request>,
    ) -> Result<HttpResponse, Error> {
        log::info!("Request from /some_route");

        let user_data = &self.ctx_sys.get_sys_data();
        log::info!("User data str: {}", user_data.sample_string);

        fa_action!(
            self.sample_m.get_data(request),
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
            self.sample_m.get_data(request),
            SampleRouteR::Response,
            response_error
        )
        .await
    }

    async fn init_user_data(&self) -> Result<HttpResponse, Error> {
        log::info!("Request from /some_route_two");
        match self.sample_m.init_user_data().await {
            Ok(_) => Ok(HttpResponse::Ok().body("OK")),
            Err(e) => Err(e),
        }
    }
}

#[post("/sample_route_one")]
pub async fn sample_route_one(
    body: web::Json<SampleRouteR::Request>,
    req: HttpRequest,
) -> Result<HttpResponse, Error> {
    let ctx = CtxSys::new(req);
    let ctrl = SampleCtrl::new(&ctx);
    ctrl.sample_route_one(body).await
}

#[post("/sample_route_two")]
pub async fn sample_route_two(
    body: web::Json<SampleRouteR::Request>,
    req: HttpRequest,
) -> Result<HttpResponse, Error> {
    let ctx = CtxSys::new(req);
    let ctrl = SampleCtrl::new(&ctx);
    ctrl.sample_route_two(body).await
}

#[post("/sample_init_user_data")]
pub async fn sample_init_user_data(
    body: web::Json<SampleRouteR::Request>,
    req: HttpRequest,
) -> Result<HttpResponse, Error> {
    let ctx = CtxSys::new(req);
    let ctrl = SampleCtrl::new(&ctx);
    ctrl.init_user_data().await
}
