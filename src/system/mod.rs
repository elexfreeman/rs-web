use std::sync::MutexGuard;
use std::sync::Mutex;
use actix_web::{ web::Data, HttpRequest};

pub mod macros_s;
pub mod error_s;


pub struct UserData {
    pub sample_string: String,
}

pub struct BaseCtrl {
    pub req: HttpRequest,
}

impl BaseCtrl {
    pub fn new(req: HttpRequest) -> Self {
        Self { req }
    }

    pub fn get_user_data(&self) -> MutexGuard<'_, UserData>{
        self.req
            .app_data::<Data<Mutex<UserData>>>()
            .unwrap()
            .lock()
            .unwrap()
    }
}