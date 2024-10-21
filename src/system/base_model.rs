use crate::system::ctx_sys::CtxSys;

pub struct BaseModel<'a> {
    pub ctx_sys: &'a CtxSys,
}

impl<'a> BaseModel<'a> {
    pub fn new(ctx_sys: &'a CtxSys) -> Self {
        BaseModel { ctx_sys }
    }
}
