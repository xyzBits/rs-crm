use crate::abi::utc_now_to_timestamp;
use crate::pb::send_request::Msg;
use crate::pb::{SendRequest, SendResponse, SmsMessage};
use crate::{abi, NotificationService};
use fake::faker::phone_number::en::PhoneNumber;
use fake::Fake;
use tonic::Status;
use tracing::warn;
use uuid::Uuid;

impl abi::Sender for SmsMessage {
    async fn send(self, svc: NotificationService) -> Result<SendResponse, Status> {
        let message_id = self.message_id.clone();

        svc.sender.send(Msg::Sms(self)).await.map_err(|e| {
            warn!("Failed to send sms message: {:?}", e);
            Status::internal("Failed to send sms message")
        })?;

        Ok(SendResponse {
            message_id,
            timestamp: Some(utc_now_to_timestamp()),
        })
    }
}

/// From<A> for B
/// 实现一个转换方法，将 A 转换为 B
impl From<SmsMessage> for Msg {
    fn from(sms_message: SmsMessage) -> Self {
        println!("convert SmsMessage into Msg");
        Msg::Sms(sms_message)
    }
}

impl From<SmsMessage> for SendRequest {
    fn from(sms_message: SmsMessage) -> Self {
        let msg: Msg = sms_message.into();
        SendRequest { msg: Some(msg) }
    }
}

#[cfg(test)]
impl SmsMessage {
    pub fn fake() -> Self {
        SmsMessage {
            message_id: Uuid::new_v4().to_string(),
            sender: PhoneNumber().fake(),
            recipients: vec![PhoneNumber().fake()],
            body: "Hello, world!".to_string(),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::pb::send_request::Msg;
    use crate::pb::SmsMessage;

    #[test]
    fn test_from() {
        let sms_message = SmsMessage::fake();
        println!("{:?}", sms_message);

        let msg: Msg = sms_message.into();
    }
}
