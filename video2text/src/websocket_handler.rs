use actix::{Actor, StreamHandler};
use actix_web::{web, Error, HttpRequest, HttpResponse};
use actix_web_actors::ws;
use log::info;

/// WebSocket route handler
pub async fn ws_route(req: HttpRequest, stream: web::Payload) -> Result<HttpResponse, Error> {
    ws::start(WebSocketSession {}, &req, stream)
}

/// WebSocket session actor
struct WebSocketSession;

impl Actor for WebSocketSession {
    type Context = ws::WebsocketContext<Self>;
}

/// Implement `StreamHandler` to handle incoming WebSocket messages
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
