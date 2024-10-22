use actix_web::{web, Error, error, HttpRequest, HttpResponse};

use crate::infrastructure::sample_sql::entity::sample_user_e::User;
use crate::system::ctx_sys::CtxSys;

use crate::infrastructure::sample_sql::repository::sample_user_sql::SampleUserSql;
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
        request: web::Json<SampleRouteR::SampleRoute::Request>,
    ) -> Result<SampleRouteR::SampleRoute::Response, Error> {
        let out = SampleRouteR::SampleRoute::Response {
            title: request.title.clone(),
            description: request.description.clone(),
        };
        Ok(out)
    }

    pub async fn add_user(
        &self,
        request: web::Json<SampleRouteR::SampleAddUser::Request>,
    ) -> Result<SampleRouteR::SampleAddUser::Response, Error> {
        let user = User {
            first_name: request.first_name.clone(),
            last_name: request.last_name.clone(),
            username: request.username.clone(),
            email: request.email.clone(),
        };
        self.sample_user_sql.add_user(&user).await.map(|res| {
            SampleRouteR::SampleAddUser::Response {
                id: res.inserted_id.to_string(),
            }
        }).map_err(|e| error::ErrorInternalServerError(e))
    }
    pub async fn init_user_data(&self) -> Result<(), Error> {
        self.sample_user_sql.init_user_data().await;
        Ok(())
    }
}
