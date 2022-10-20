use actix::Addr;
use actix::Actor;
use actix::ActorContext;
use actix::AsyncContext;
use actix::Message as MessageTrait;
use actix::Handler;
use actix::StreamHandler;
use actix_web_actors::ws;
use std::time::Duration;
use std::time::Instant;

const HEARTBEAT_INTERVAL: Duration = Duration::from_secs(5);
const CLIENT_TIMEOUT: Duration = Duration::from_secs(10);

#[derive(Debug)]
pub struct ClientActor {
    hb: Instant
}

impl ClientActor {
    pub fn new() -> ClientActor {
        ClientActor { hb: Instant::now() }
    }
}

impl Actor for ClientActor {
    type Context = ws::WebsocketContext<ClientActor>;

    fn started(&mut self, ctx: &mut ws::WebsocketContext<ClientActor>) {
        ctx.run_interval(HEARTBEAT_INTERVAL, |act, ctx| {
            if Instant::now().duration_since(act.hb) > CLIENT_TIMEOUT {
                ctx.stop();
                return;
            }
            ctx.ping(b"");
        });
    }
}

impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for ClientActor {
    fn handle(
        &mut self,
        msg: Result<ws::Message, ws::ProtocolError>,
        ctx: &mut ws::WebsocketContext<ClientActor>
    ) {
        let msg = match msg {
            Err(_) => {
                ctx.stop();
                return;
            }
            Ok(msg) => msg,
        };

        match msg {
            ws::Message::Ping(msg) => {
                ctx.pong(&msg);
            }
            ws::Message::Pong(_) => {
                self.hb = Instant::now();
            }
            ws::Message::Text(msg) => {
                ctx.text(msg);
            }
            ws::Message::Binary(_) => println!("Unexpected binary"),
            ws::Message::Close(reason) => {
                ctx.close(reason);
                ctx.stop();
            }
            ws::Message::Continuation(_) => {
                ctx.stop();
            }
            ws::Message::Nop => (),
        }
    }
}

