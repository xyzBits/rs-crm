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
use futures::Stream;
pub use in_app::*;
use itertools::WhileSome;
use prost_types::Timestamp;
pub use sms::*;
use std::ops::Deref;
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::mpsc;
use tokio::time::sleep;
use tonic::Status;
use tracing::info;

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
        mut stream: impl Stream<Item = Result<SendRequest, Status>> + Send + 'static + Unpin,
    ) -> ServiceResult<ResponseStream> {
        todo!()
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
