use mongodb::{Client, Database};

use super::config_sys::Config;

pub struct CtxDataSys {
    pub sample_string: String,
    pub client: Client,
    pub db: Database,
    pub config: Config,
}
