use std::any::Any;
use std::time::{Duration, Instant};

use actix::{Actor, ActorContext, Addr, AsyncContext, Handler, StreamHandler};
use actix_web::{Error, get, HttpRequest, HttpResponse, web};
use actix_web_actors::ws;
use actix_web_actors::ws::Message;
use log::debug;
use crate::module::protocols::ws_message::{Say, SayHello};

/// How often heartbeat pings are sent
const HEARTBEAT_INTERVAL: Duration = Duration::from_secs(5);
/// How long before lack of client response causes a timeout
const CLIENT_TIMEOUT: Duration = Duration::from_secs(10);

/// websocket connection is long running connection, it easier
/// to handle with an actor
pub struct WsConn {
    /// Client must send ping at least once per 10 seconds (CLIENT_TIMEOUT),
    /// otherwise we drop connection.
    hb: Instant,

}

impl Actor for WsConn {
    type Context = ws::WebsocketContext<Self>;

    /// Method is called on actor start. We start the heartbeat process here.
    fn started(&mut self, ctx: &mut Self::Context) {
        self.hb(ctx);
        debug!("Websocket heartbeat process started.");

        self.report_light_sources(ctx);
        debug!("Websocket report light sources started.");
    }

    /// 断开连接
    fn stopped(&mut self, _: &mut Self::Context) {
        debug!("Disconnects websocket.");
    }
}

/// Handler for `ws::Message`
impl StreamHandler<Result<Message, ws::ProtocolError>> for WsConn {
    fn handle(&mut self, msg: Result<Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        // process websocket messages
        //debug!("WS: {:?}", msg);
        match msg {
            Ok(Message::Ping(msg)) => {
                self.hb = Instant::now();
                ctx.pong(&msg);
            }
            Ok(Message::Pong(_)) => {
                self.hb = Instant::now();
            }
            Ok(Message::Text(text)) => ctx.text("Repeat: ".to_string() + &text),
            Ok(Message::Binary(bin)) => ctx.binary(bin),
            Ok(Message::Close(reason)) => {
                ctx.close(reason);
                ctx.stop();
            }
            _ => ctx.stop(),
        }
    }
}

impl WsConn {
    pub fn new() -> Self {
        Self { hb: Instant::now() }
    }

    /// helper method that sends ping to client every second.
    ///
    /// also this method checks heartbeats from client
    fn hb(&self, ctx: &mut <Self as Actor>::Context) {
        
        ctx.run_interval(HEARTBEAT_INTERVAL, |act, ctx| {
            // check client heartbeats
            if Instant::now().duration_since(act.hb) > CLIENT_TIMEOUT {
                // heartbeat timed out
                debug!("Websocket Client heartbeat failed, disconnecting!");

                // stop actor
                ctx.stop();

                // don't try to send a ping
                return;
            }
            ctx.ping(b"");
        });


    }

    /// 返回光源数据
    fn report_light_sources(&self, ctx: &mut <Self as Actor>::Context) {
        let mut cnt = 0;
        ctx.run_interval(Duration::from_millis(50), move |_act, ctx| {

            let msg = cnt.to_string();
            //debug!("light_sources {}", msg);
            ctx.text(msg);
            cnt += 1;
        });
    }
}


impl Handler<SayHello> for WsConn{
    type Result = ();

    fn handle(&mut self, msg: SayHello, ctx: &mut Self::Context) -> Self::Result {
        ctx.text("hello!");
    }
}

impl Handler<Say> for WsConn {
    type Result = ();

    fn handle(&mut self, msg: Say, ctx: &mut Self::Context) -> Self::Result {
        ctx.text(msg.data);
    }
}

