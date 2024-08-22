use crate::pb::send_request::Msg;
use crate::pb::{EmailMessage, SendRequest, SendResponse};
use crate::{abi, NotificationService};
use tonic::Status;

impl abi::Sender for EmailMessage {
    async fn send(self, svc: NotificationService) -> Result<SendResponse, Status> {
        todo!()
    }
}

#[cfg(test)]
impl EmailMessage {
    pub fn fake() -> Self {
        todo!()
    }
}

impl From<EmailMessage> for Msg {
    fn from(email: EmailMessage) -> Self {
        todo!()
    }
}

impl From<EmailMessage> for SendRequest {
    fn from(value: EmailMessage) -> Self {
        todo!()
    }
}
