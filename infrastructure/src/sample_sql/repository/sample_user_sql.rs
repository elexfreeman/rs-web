use system;
use system::mongo_connect_sys::db_connect;
use mongodb::error::Error;
use mongodb::{
    bson::doc, options::IndexOptions, results::InsertOneResult, Collection, IndexModel,
};

use crate::sample_sql::entity::sample_user_e::User;

pub struct SampleUserSql {
}

const COLL_NAME: &str = "users";

impl<'a> SampleUserSql {
    pub fn new() -> Self {
        Self {  }
    }

    pub async fn add_user(&self, user: &User) -> Result<InsertOneResult, Error> {
        let collection: Collection<User> = db_connect().await.db.collection(COLL_NAME);
        collection.insert_one(user).await
    }

    pub async fn get_user(&self, username: &String) -> Result<Option<User>, Error> {
        let collection: Collection<User> = db_connect().await.db.collection(COLL_NAME);
        collection.find_one(doc! { "username": &username }).await
    }

    pub async fn init_user_data(&self) {
        const COLL_NAME: &str = "users";
        let options = IndexOptions::builder().unique(true).build();
        let model = IndexModel::builder()
            .keys(doc! { "username": 1 })
            .options(options)
            .build();
        db_connect().await.db
            .collection::<User>(COLL_NAME)
            .create_index(model)
            .await
            .expect("creating an index should succeed");
    }
}
