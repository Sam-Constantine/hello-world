use std::time::{Duration, Instant};

use actix::{Actor, ActorContext, AsyncContext, StreamHandler};
use actix_web::{Error, HttpRequest, HttpResponse, web};
use actix_web_actors::ws;
use actix_web_actors::ws::Message;
use log::{debug, info};

/// How often heartbeat pings are sent
const HEARTBEAT_INTERVAL: Duration = Duration::from_secs(5);
/// How long before lack of client response causes a timeout
const CLIENT_TIMEOUT: Duration = Duration::from_secs(10);

/// do websocket handshake and start `MyWebSocket` actor
pub async fn ws_index(r: HttpRequest, stream: web::Payload) -> Result<HttpResponse, Error> {
    info!("收到请求： \n{:?}", r);
    let res = ws::start(WsConn::new(), &r, stream);
    info!("{:?}", res);
    res
}

/// websocket connection is long running connection, it easier
/// to handle with an actor
struct WsConn {
    /// Client must send ping at least once per 10 seconds (CLIENT_TIMEOUT),
    /// otherwise we drop connection.
    hb: Instant,
}

impl Actor for WsConn {

    type Context = ws::WebsocketContext<Self>;

    /// Method is called on actor start. We start the heartbeat process here.
    fn started(&mut self, ctx: &mut Self::Context) {
        self.hb(ctx);
    }
}

/// Handler for `ws::Message`
impl StreamHandler<Result<Message, ws::ProtocolError>> for WsConn {
    fn handle(&mut self, msg: Result<Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        // process websocket messages
        debug!("WS: {:?}", msg);
        match msg {
            Ok(Message::Ping(msg)) => {
                self.hb = Instant::now();
                ctx.pong(&msg);
            }
            Ok(Message::Pong(_)) => {
                self.hb = Instant::now();
            }
            Ok(Message::Text(text)) => ctx.text(text),
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
    fn new() -> Self {
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
}


