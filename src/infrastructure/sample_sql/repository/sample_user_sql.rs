use crate::infrastructure::sample_sql::entity::sample_user_e::User;
use crate::system::ctx_sys::CtxSys;
use mongodb::error::Error;
use mongodb::{bson::doc, options::IndexOptions, Client, Collection, IndexModel};

pub struct SampleUserSql<'a> {
    ctx_sys: &'a CtxSys,
}

const COLL_NAME: &str = "users";

impl<'a> SampleUserSql<'a> {
    pub fn new(ctx_sys: &'a CtxSys) -> Self {
        Self { ctx_sys }
    }

    pub async fn get_user(self, username: String) -> Result<Option<User>, Error> {
        let collection: Collection<User> = self.ctx_sys.get_mongo_db().collection(COLL_NAME);
        collection.find_one(doc! { "username": &username }).await
    }

    pub async fn init_user_data(&self) {
        const COLL_NAME: &str = "users";
        let options = IndexOptions::builder().unique(true).build();
        let model = IndexModel::builder()
            .keys(doc! { "username": 1 })
            .options(options)
            .build();
        self.ctx_sys
            .get_mongo_db()
            .collection::<User>(COLL_NAME)
            .create_index(model)
            .await
            .expect("creating an index should succeed");
    }
}
