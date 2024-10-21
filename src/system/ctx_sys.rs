use std::sync::MutexGuard;
use std::sync::Mutex;
use actix_web::{ web::Data, HttpRequest};
use mongodb::Client;
use mongodb::Database;

use crate::system::ctx_data_sys::CtxDataSys;

pub struct CtxSys{
    pub req: HttpRequest,
}

impl<'a> CtxSys {
    pub fn new(req: HttpRequest) -> Self {
        CtxSys { req }
    }

    pub fn get_header(&self, name: &str) -> Option<String> {
        self.req
            .headers()
            .get(name)
            .map(|header| header.to_str().unwrap().to_string())
    }

    pub fn get_cookie(&self, name: &str) -> Option<String> {
        self.req
            .cookie(name)
            .map(|cookie| cookie.value().to_string())
    }

    pub fn get_config(&self) -> crate::system::config_sys::Config {
        self.get_sys_data().config.clone()
    }


    pub fn get_mongo_db(&self) -> Database {
        let sys_data = self.get_sys_data();
        let db = &sys_data.db;
        db.clone()
    }

    pub fn get_mongo_client(&self) -> Client {
        let sys_data = self.get_sys_data();
        let client = &sys_data.client;
        client.clone()
    }

    pub fn get_sys_data(&self) -> MutexGuard<'_, CtxDataSys>{
        self.req
            .app_data::<Data<Mutex<CtxDataSys>>>()
            .unwrap()
            .lock()
            .unwrap()
    }
}
