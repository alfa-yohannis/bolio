use actix::{Actor, StreamHandler, Addr, AsyncContext, Handler, Message};
use actix_web::{web, Error, HttpRequest, HttpResponse};
use actix_web_actors::ws;
use log::info;
use crate::progress_updater::{ProgressUpdater, ProgressUpdate, RegisterClient, UnregisterClient};

/// WebSocket session actor
pub struct WebSocketSession {
    session_id: String,
    progress_addr: Addr<ProgressUpdater>, // Reference to ProgressUpdater
}

impl Actor for WebSocketSession {
    type Context = ws::WebsocketContext<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        info!("WebSocket started for session: {}", self.session_id);

        // Register this WebSocket session in ProgressUpdater
        self.progress_addr.do_send(RegisterClient {
            session_id: self.session_id.clone(),
            addr: ctx.address(),
        });
    }

    fn stopped(&mut self, _ctx: &mut Self::Context) {
        info!("WebSocket stopped for session: {}", self.session_id);

        // Unregister this WebSocket session from ProgressUpdater
        self.progress_addr.do_send(UnregisterClient {
            session_id: self.session_id.clone(),
            addr: _ctx.address(),
        });
    }
}

/// **✅ Implement `Handler<ProgressUpdate>` so WebSocket can receive progress updates**
impl Handler<ProgressUpdate> for WebSocketSession {
    type Result = ();

    fn handle(&mut self, msg: ProgressUpdate, ctx: &mut Self::Context) {
        info!("WebSocket Progress Update: {} for session {}", msg.message, msg.session_id);
        ctx.text(msg.message); // Send message to the WebSocket client
    }
}

/// Handle incoming WebSocket messages
impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for WebSocketSession {
    fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        match msg {
            Ok(ws::Message::Text(text)) => {
                info!("Received message: {}", text);
                ctx.text(format!("Echo: {}", text)); // Echo back the received message
            }
            Ok(ws::Message::Binary(bin)) => {
                info!("Received binary data: {:?}", bin);
                ctx.binary(bin);
            }
            Ok(ws::Message::Ping(msg)) => ctx.pong(&msg),
            Ok(ws::Message::Close(reason)) => {
                info!("WebSocket closed: {:?}", reason);
                ctx.close(reason);
            }
            _ => (),
        }
    }
}

/// WebSocket route handler
pub async fn ws_route(
    req: HttpRequest,
    stream: web::Payload,
    progress_addr: web::Data<Addr<ProgressUpdater>>, // Inject ProgressUpdater
) -> Result<HttpResponse, Error> {
    let session_id = req
        .query_string()
        .split('=')
        .nth(1)
        .unwrap_or("")
        .to_string();

    ws::start(
        WebSocketSession {
            session_id,
            progress_addr: progress_addr.get_ref().clone(),
        },
        &req,
        stream,
    )
}
