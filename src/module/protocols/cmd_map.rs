use std::collections::HashMap;
use actix::Addr;
use lazy_static::lazy_static;
use crate::module::protocols::ws_message::{Say, SayHello};
use crate::module::controller::trait_cmd::Cmd;

use crate::ws_conn::WsConn;

type RouterFn = fn(addr: Addr<WsConn>, data: String);

lazy_static!{
    pub static ref CMD_MAP: HashMap<&'static str, RouterFn>={
        let mut map = HashMap::new();
        map.insert("SayHello", SayHello::route as RouterFn);
        map.insert("Say", Say::route as RouterFn);
        map
    };
}