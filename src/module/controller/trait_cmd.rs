use actix::Addr;
use crate::ws_conn::WsConn;

pub trait Cmd{
    /// 指令名
    fn name()->&'static str;

    /// 路由方法
    fn route(addr: Addr<WsConn>, data: String);
}