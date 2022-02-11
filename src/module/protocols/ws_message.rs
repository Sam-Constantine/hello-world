

use actix::{Addr, Message};
use crate::controller::trait_cmd::Cmd;
use crate::ws_conn::WsConn;

#[derive(Message)]
#[rtype(result="()")]
pub struct SayHello{
    pub from: &'static str,
    pub data: String
}

#[derive(Message)]
#[rtype(result="()")]
pub struct Say{
    pub from: &'static str,
    pub data: String
}


impl Cmd for SayHello{
    fn name() -> &'static str {
        "SayHello"
    }

    fn route(addr: Addr<WsConn>, data: String) {
        addr.do_send(Self{ from: "SayHello", data })
    }
}

impl Cmd for Say {
    fn name() -> &'static str {
        "Say"
    }

    fn route(addr: Addr<WsConn>, data: String) {
        addr.do_send(Self{ from: "Say", data })
    }
}