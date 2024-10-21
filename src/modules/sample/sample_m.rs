use actix_web::{web, Error, HttpRequest};

use crate::system::ctx_sys::CtxSys;

use crate::infrastructure::sample_sql::repository::sample_user_sql::{SampleUserSql};
use crate::modules::sample::sample_r::SampleRouteR;

pub struct SampleM<'a> {
    ctx_sys: &'a CtxSys,
    sample_user_sql: SampleUserSql<'a>,
}

impl<'a> SampleM<'a> {
    pub fn new(ctx_sys: &'a CtxSys) -> Self {
        let sample_user_sql = SampleUserSql::new(&ctx_sys);
        SampleM {
            ctx_sys,
            sample_user_sql,
        }
    }

    pub async fn get_data(
        &self,
        request: web::Json<SampleRouteR::Request>,
    ) -> Result<SampleRouteR::Response, Error> {
        let out = SampleRouteR::Response {
            title: request.title.clone(),
            description: request.description.clone(),
        };
        Ok(out)
    }

    pub async fn init_user_data(&self) -> Result<(), Error> {
        self.sample_user_sql.init_user_data().await;
        Ok(())
    }
}
