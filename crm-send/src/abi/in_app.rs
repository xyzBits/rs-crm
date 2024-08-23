use crate::abi::utc_now_to_timestamp;
use crate::pb::send_request::Msg;
use crate::pb::send_request::Msg::InApp;
use crate::pb::{InAppMessage, SendRequest, SendResponse};
use crate::{abi, NotificationService};
use tonic::Status;
use tracing::warn;
use uuid::Uuid;

impl abi::Sender for InAppMessage {
    async fn send(self, svc: NotificationService) -> Result<SendResponse, Status> {
        let message_id = self.message_id.clone();

        svc.sender.send(Msg::InApp(self)).await.map_err(|e| {
            warn!("Failed to send in-app message");
            Status::internal("Failed to send in-app message")
        })?;

        Ok(SendResponse {
            message_id,
            timestamp: Some(utc_now_to_timestamp()),
        })
    }
}

impl From<InAppMessage> for Msg {
    fn from(in_app: InAppMessage) -> Self {
        Msg::InApp(in_app)
    }
}

impl From<InAppMessage> for SendRequest {
    fn from(in_app: InAppMessage) -> Self {
        let msg: Msg = in_app.into();
        SendRequest { msg: Some(msg) }
    }
}

#[cfg(test)]
impl InAppMessage {
    pub fn fake() -> Self {
        InAppMessage {
            message_id: Uuid::new_v4().to_string(),
            device_id: Uuid::new_v4().to_string(),
            title: "Hello".to_string(),
            body: "Hello, world!".to_string(),
        }
    }
}
