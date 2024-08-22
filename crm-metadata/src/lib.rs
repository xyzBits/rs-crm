pub mod abi;
pub mod pb;

pub mod config;

pub use abi::*;
use futures::Stream;
pub use pb::*;
use std::pin::Pin;
use tonic::{Request, Response, Status, Streaming};

use crate::metadata_server::{Metadata, MetadataServer};
pub use config::*;

#[allow(unused)]
pub struct MetadataService {
    config: AppConfig,
}

type ServiceResult<T> = Result<Response<T>, Status>;
type ResponseStream = Pin<Box<dyn Stream<Item = Result<Content, Status>> + Send>>;

#[tonic::async_trait]
impl Metadata for MetadataService {
    type MaterializeStream = ResponseStream;

    async fn materialize(
        &self,
        request: Request<Streaming<MaterializeRequest>>,
    ) -> ServiceResult<Self::MaterializeStream> {
        let request = request.into_inner();
        self.materialize(request).await
    }
}

impl MetadataService {
    pub fn new(config: AppConfig) -> Self {
        MetadataService { config }
    }

    pub fn into_server(self) -> MetadataServer<Self> {
        MetadataServer::new(self)
    }
}
