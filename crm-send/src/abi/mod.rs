mod email;
mod in_app;
mod sms;

use crate::config::AppConfig;
use crate::pb::notification_server::NotificationServer;
use crate::pb::send_request::Msg;
use crate::pb::{SendRequest, SendResponse};
use crate::{NotificationService, NotificationServiceInner, ResponseStream, ServiceResult};
use chrono::Utc;
pub use email::*;
use futures::{Stream, StreamExt};
pub use in_app::*;
use itertools::WhileSome;
use prost_types::Timestamp;
pub use sms::*;
use std::ops::Deref;
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::mpsc;
use tokio::time::sleep;
use tonic::codegen::tokio_stream::wrappers::ReceiverStream;
use tonic::{Response, Status};
use tracing::{info, warn};

const CHANNEL_SIZE: usize = 1024;

pub trait Sender {
    async fn send(self, svc: NotificationService) -> Result<SendResponse, Status>;
}

impl NotificationService {
    pub fn new(config: AppConfig) -> Self {
        let sender = dummy_sender();
        let inner = NotificationServiceInner { config, sender };

        Self {
            inner: Arc::new(inner),
        }
    }

    pub fn into_server(self) -> NotificationServer<Self> {
        NotificationServer::new(self)
    }

    pub async fn send(
        &self,
        mut stream:
        impl Stream<Item=Result<SendRequest, Status>>
        + Send
        + 'static +
        Unpin,
    ) -> ServiceResult<ResponseStream> {
        let (sender, receiver) = mpsc::channel(CHANNEL_SIZE);
        let service = self.clone();

        tokio::spawn(async move {
            while let Some(Ok(req)) = stream.next().await {
                let cloned_service = service.clone();
                let res = match req.msg {
                    Some(Msg::Email(email)) => email.send(cloned_service).await,
                    Some(Msg::Sms(sms)) => sms.send(cloned_service).await,
                    Some(Msg::InApp(in_app)) => in_app.send(cloned_service).await,
                    None => {
                        warn!("Invalid request");
                        Err(Status::invalid_argument("Invalid request"))
                    }
                };
                sender.send(res).await.unwrap();
            }
        });

        let stream = ReceiverStream::new(receiver);
        Ok(Response::new(Box::pin(stream)))
    }
}

impl Deref for NotificationService {
    type Target = NotificationServiceInner;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

fn dummy_sender() -> mpsc::Sender<Msg> {
    let (sender, mut receiver) = mpsc::channel(CHANNEL_SIZE * 10);

    tokio::spawn(async move {
        while let Some(msg) = receiver.recv().await {
            info!("Sending message: {:?}", msg);
            sleep(Duration::from_millis(300)).await;
        }
    });

    sender
}

fn utc_now_to_timestamp() -> Timestamp {
    let now = Utc::now();
    Timestamp {
        seconds: now.timestamp(),
        nanos: now.timestamp_subsec_nanos() as i32,
    }
}

#[cfg(test)]
mod tests {
    use crate::config::AppConfig;
    use crate::pb::{EmailMessage, InAppMessage, SmsMessage};
    use crate::NotificationService;
    use futures::StreamExt;
    use tonic::codegen::tokio_stream;

    #[tokio::test]
    async fn send_should_work() -> anyhow::Result<()> {
        let config = AppConfig::load()?;
        let service = NotificationService::new(config);

        let stream = tokio_stream::iter(vec![
            Ok(EmailMessage::fake().into()),
            Ok(SmsMessage::fake().into()),
            Ok(InAppMessage::fake().into()),
        ]);

        let response = service.send(stream).await?;

        let ret =
            response
                .into_inner()
                .collect::<Vec<_>>()
                .await;

        assert_eq!(ret.len(), 3);
        Ok(())
    }
}
