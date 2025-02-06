use crate::websocket_handler::WebSocketSession; // Import WebSocketSession from the appropriate module
use actix::{Actor, Addr, Context, Handler, Message, Recipient};
use actix_web_actors::ws::WebsocketContext;
use log::info;
use std::collections::HashMap;

/// WebSocket progress message
#[derive(Message, Clone)]
#[rtype(result = "()")]
pub struct ProgressUpdate {
    pub session_id: String,
    pub message: String,
}

/// Register a WebSocket client
#[derive(Message)]
#[rtype(result = "()")]
pub struct RegisterClient {
    pub session_id: String,
    pub addr: Addr<WebSocketSession>,
}

/// Unregister a WebSocket client
#[derive(Message)]
#[rtype(result = "()")]
pub struct UnregisterClient {
    pub session_id: String,
    pub addr: Addr<WebSocketSession>,
}

/// Actor to manage WebSocket clients
pub struct ProgressUpdater {
    pub clients: HashMap<String, Vec<Addr<WebSocketSession>>>, // Store WebSocket actor addresses
}

impl ProgressUpdater {
    /// Register a WebSocket client
    pub fn register_client(&mut self, session_id: String, addr: Addr<WebSocketSession>) {
        self.clients.entry(session_id).or_default().push(addr);
    }

    /// Unregister a WebSocket client
    pub fn unregister_client(&mut self, session_id: &str, addr: &Addr<WebSocketSession>) {
        if let Some(clients) = self.clients.get_mut(session_id) {
            clients.retain(|c| c != addr);
            if clients.is_empty() {
                self.clients.remove(session_id);
            }
        }
    }
}

impl Actor for ProgressUpdater {
    type Context = Context<Self>;
}

/// Handle progress updates (send updates to connected WebSocket clients)
impl Handler<ProgressUpdate> for ProgressUpdater {
    type Result = ();

    fn handle(&mut self, msg: ProgressUpdate, _ctx: &mut Self::Context) {
        info!(
            "Sending progress update: {} for session {}",
            msg.message, msg.session_id
        );

        if let Some(clients) = self.clients.get(&msg.session_id) {
            for client in clients {
                // Explicitly discard the result to avoid type mismatch
                if let Err(e) = client.try_send(msg.clone()) {
                    info!("Failed to send message to client: {:?}", e);
                };
            }
        }
    }
}



/// Handle WebSocket client registration
impl Handler<RegisterClient> for ProgressUpdater {
    type Result = ();

    fn handle(&mut self, msg: RegisterClient, _ctx: &mut Self::Context) {
        info!("Registering WebSocket for session: {}", msg.session_id);
        self.register_client(msg.session_id, msg.addr);
    }
}

/// Handle WebSocket client unregistration
impl Handler<UnregisterClient> for ProgressUpdater {
    type Result = ();

    fn handle(&mut self, msg: UnregisterClient, _ctx: &mut Self::Context) {
        info!("Unregistering WebSocket for session: {}", msg.session_id);
        self.unregister_client(&msg.session_id, &msg.addr);
    }
}
