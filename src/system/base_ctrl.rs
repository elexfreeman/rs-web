use std::sync::MutexGuard;
use std::sync::Mutex;
use actix_web::{ web::Data, HttpRequest};

use crate::system::sys_data::SysData;

pub struct BaseCtrl {
    pub req: HttpRequest,
}

impl BaseCtrl {
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
