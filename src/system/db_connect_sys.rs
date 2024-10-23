use mongodb::{Client, Database};
use tokio::sync::OnceCell;

// Структура, которая будет синглтоном
pub struct DbConnectSys {
    pub db: Database,
    pub client: Client,
}

impl DbConnectSys {
    async fn new() -> Self {
        let config = crate::system::config_sys::ConfigSys::get_instance();
        crate::system::config_sys::print_config(&config);
        let client = Client::with_uri_str(config.get_mongo_uri())
            .await
            .expect("MongoDB failed to connect");
        let db = client.database(&config.mongo_config.db_name.clone());
        DbConnectSys { db, client }
    }
}

static ONCE: OnceCell<DbConnectSys> = OnceCell::const_new();

pub async fn db_connect() -> &'static DbConnectSys {
    ONCE.get_or_init(|| async { DbConnectSys::new().await })
        .await
}
