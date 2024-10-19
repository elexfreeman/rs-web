use std::sync::MutexGuard;
use std::sync::Mutex;
use actix_web::{ web::Data, HttpRequest};

use crate::system::sys_data::SysData;

pub struct BaseModel {
    pub req: HttpRequest,
}

impl BaseModel {
    pub fn new(req: HttpRequest) -> Self {
        Self { req }
    }

    pub fn get_sys_data(&self) -> MutexGuard<'_, SysData>{
        self.req
            .app_data::<Data<Mutex<SysData>>>()
            .unwrap()
            .lock()
            .unwrap()
    }
}