use actix::{Actor, Context, Handler, Message, Recipient};
use log::info;

/// WebSocket progress message
#[derive(Message, Clone)]
#[rtype(result = "()")]
pub struct ProgressUpdate {
    pub session_id: String,
    pub message: String,  // Instead of percentage, we send a message
}

/// Actor to manage progress updates
pub struct ProgressUpdater {
    pub clients: Vec<Recipient<ProgressUpdate>>,
}

impl Actor for ProgressUpdater {
    type Context = Context<Self>;
}

impl Handler<ProgressUpdate> for ProgressUpdater {
    type Result = ();

    fn handle(&mut self, msg: ProgressUpdate, _ctx: &mut Self::Context) {
        info!("Sending progress update: {} for session {}", msg.message, msg.session_id);
        
        for client in &self.clients {
            let _ = client.do_send(msg.clone());
        }
    }
}
