use actix::*;
use actix_web::{web, Error, HttpRequest, HttpResponse};
use actix_web_actors::ws;

use crate::core::{Lobby, RoomId, RoomManager, UserId};
use crate::protocol::{PacketClient, PacketServer};
use log::{info, trace, warn};
use std::time::{Duration, Instant};

const HEARTBEAT_INTERVAL: Duration = Duration::from_secs(5);
const CLIENT_TIMEOUT: Duration = Duration::from_secs(10);

pub struct WsSession {
    id: UserId,
    last_heartbeat: Instant,
    lobby: Addr<Lobby>,
    room_manager: Addr<RoomManager>,
    room: Option<RoomId>,
    name: String,
}

impl Actor for WsSession {
    type Context = ws::WebsocketContext<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        self.heartbeat(ctx);

        let me = ctx.address();
        /*
        self.host
            .send(game::Connect {
                addr: me.recipient(),
            })
            .into_actor(self)
            .then(|result, actor, ctx| {
                match result {
                    Ok(result) => actor.id = result.id,
                    _ => ctx.stop(),
                }
                fut::ok(())
            })
            .wait(ctx);
        */
    }

    fn stopping(&mut self, _: &mut Self::Context) -> Running {
        /*
        self.host.do_send(game::Disconnect { id: self.id });
        */
        Running::Stop
    }
}

impl Handler<PacketServer> for WsSession {
    type Result = ();

    fn handle(&mut self, packet: PacketServer, ctx: &mut Self::Context) {
        ctx.text(
            serde_json::to_string(&packet).expect("serializing PacketServer should never fail"),
        )
    }
}

impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for WsSession {
    fn handle(&mut self, message: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        let message = if let Ok(message) = message {
            message
        } else {
            return;
        };
        match message {
            ws::Message::Ping(ping) => {
                self.last_heartbeat = Instant::now();
                ctx.pong(&ping);
            }
            ws::Message::Pong(_) => {
                self.last_heartbeat = Instant::now();
            }
            ws::Message::Binary(_) => {
                trace!("Unexpected binary data received from {}", self.id);
            }

            ws::Message::Close(_) => {
                ctx.stop();
            }
            ws::Message::Text(text) => {
                trace!("{}", &text);
                if let Ok(message) = serde_json::from_str::<PacketClient>(&text) {
                    Handler::handle(self, message, ctx);
                }
            }
            ws::Message::Nop | ws::Message::Continuation(_) => {}
        }
    }
}

impl WsSession {
    fn heartbeat(&self, ctx: &mut ws::WebsocketContext<Self>) {
        ctx.run_interval(HEARTBEAT_INTERVAL, |actor, ctx| {
            if Instant::now().duration_since(actor.last_heartbeat) > CLIENT_TIMEOUT {
                info!("Client {} heartbeat failed.", actor.id);

                // actor.host.do_send(game::Disconnect { id: actor.id });

                ctx.stop();
                return;
            }
            ctx.ping(b"");
        });
    }
}

pub async fn ws(
    req: HttpRequest,
    stream: web::Payload,
    room_manager: web::Data<Addr<RoomManager>>,
    lobby: web::Data<Addr<Lobby>>,
) -> Result<HttpResponse, Error> {
    ws::start(
        // TODO: WsSession을 초기화할 때 적절한 id 및 name 값으로 초기화해야 함.
        WsSession {
            id: 0,
            last_heartbeat: Instant::now(),
            lobby: lobby.as_ref().clone(),
            room_manager: room_manager.as_ref().clone(),
            room: None,
            name: "".to_owned(),
        },
        &req,
        stream,
    )
}
