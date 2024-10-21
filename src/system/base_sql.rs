use super::ctx_data_sys::CtxDataSys;

pub struct BaseSql {
    pub sys_data: CtxDataSys,
}

impl BaseSql {
    pub fn new(sys_data: CtxDataSys) -> Self {
        Self { sys_data }
    }
}
