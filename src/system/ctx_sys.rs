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
