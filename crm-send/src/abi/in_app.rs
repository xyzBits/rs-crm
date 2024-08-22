use crate::pb::send_request::Msg;
use crate::pb::{InAppMessage, SendRequest, SendResponse};
use crate::{abi, NotificationService};
use tonic::Status;

impl abi::Sender for InAppMessage {
    async fn send(self, svc: NotificationService) -> Result<SendResponse, Status> {
        todo!()
    }
}

impl From<InAppMessage> for Msg {
    fn from(value: InAppMessage) -> Self {
        todo!()
    }
}

impl From<InAppMessage> for SendRequest {
    fn from(value: InAppMessage) -> Self {
        todo!()
    }
}

#[cfg(test)]
impl InAppMessage {
    pub fn fake() -> Self {
        todo!()
    }
}
