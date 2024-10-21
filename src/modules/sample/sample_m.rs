use actix_web::{web, Error, HttpRequest};
use mongodb::{bson::doc, options::IndexOptions, Client, Collection, IndexModel};

use crate::system::ctx_sys::{CtxSys};
use crate::system::base_model::BaseModel;

use crate::modules::sample::sample_r::SampleRouteR;
use crate::infrastructure::sample_sql::entity::sample_user_e::User;
    

pub struct SampleM<'a> {
    pub base_model: BaseModel<'a>,
}

impl<'a> SampleM<'a> {
    pub fn new(ctx_sys: &'a CtxSys) -> Self {
        let base_model = BaseModel::new(ctx_sys);
        SampleM { base_model }
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

    pub async fn init_user_data(&self) {
        const COLL_NAME: &str = "users";
        let options = IndexOptions::builder().unique(true).build();
        let model = IndexModel::builder()
            .keys(doc! { "username": 1 })
            .options(options)
            .build();
        self.base_model.ctx_sys
            .get_mongo_db()
            .collection::<User>(COLL_NAME)
            .create_index(model)
            .await
            .expect("creating an index should succeed");
    }
}
