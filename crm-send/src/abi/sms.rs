use crate::pb::send_request::Msg;
use crate::pb::{SendRequest, SendResponse, SmsMessage};
use crate::{abi, NotificationService};
use tonic::Status;

impl abi::Sender for SmsMessage {
    async fn send(self, svc: NotificationService) -> Result<SendResponse, Status> {
        todo!()
    }
}

impl From<SmsMessage> for Msg {
    fn from(value: SmsMessage) -> Self {
        todo!()
    }
}

impl From<SmsMessage> for SendRequest {
    fn from(value: SmsMessage) -> Self {
        todo!()
    }
}

#[cfg(test)]
impl SmsMessage {
    pub fn fake() -> Self {
        todo!()
    }
}
