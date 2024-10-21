use crate::system::base_sql::BaseSql;
use mongodb::{bson::doc, options::IndexOptions, Client, Collection, IndexModel};

struct SampleUserSql {
    base_sql: BaseSql,
}

impl SampleUserSql {
    pub fn new(client: &Client) -> Self {
        let base_sql = BaseSql::new(client);
        Self { base_sql }
    }

    async fn get_user(username: web::Path<String>) -> HttpResponse {
        let username = username.into_inner();
        let collection: Collection<User> = client.database(DB_NAME).collection(COLL_NAME);
        match collection.find_one(doc! { "username": &username }).await {
            Ok(Some(user)) => HttpResponse::Ok().json(user),
            Ok(None) => {
                HttpResponse::NotFound().body(format!("No user found with username {username}"))
            }
            Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
        }
    }
}
