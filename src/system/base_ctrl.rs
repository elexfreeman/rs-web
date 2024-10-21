use crate::system::ctx_sys::CtxSys;

pub struct BaseCtrl<'a> {
    pub ctx_sys: &'a CtxSys,
}

impl<'a> BaseCtrl<'a> {
    pub fn new(ctx_sys: &'a CtxSys) -> Self {
        BaseCtrl { ctx_sys }
    }
}
