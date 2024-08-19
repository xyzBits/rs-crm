// 声明大模块
pub mod abi;
pub mod config;
pub mod pb;

use futures::Stream;
// 声明对外使用
pub use pb::*;
use sqlx::PgPool;
use std::ops::Deref;
use std::pin::Pin;
use std::sync::Arc;
use tonic::{async_trait, Request, Response, Status};

use crate::user_stats_server::{UserStats, UserStatsServer};
pub use abi::*;
pub use config::*;

type ServiceResult<T> = Result<Response<T>, Status>;
type ResponseStream = Pin<Box<dyn Stream<Item = Result<User, Status>> + Send>>;

#[derive(Clone)]
pub struct UserStatsService {
    inner: Arc<UserStatsServiceInner>,
}

#[allow(unused)]
pub struct UserStatsServiceInner {
    config: AppConfig,
    pool: PgPool,
}

#[async_trait] // 不加这个编译不过
impl UserStats for UserStatsService {
    type QueryStream = ResponseStream;
    async fn query(
        &self,
        request: Request<QueryRequest>,
    ) -> Result<Response<Self::QueryStream>, Status> {
        todo!()
    }

    type RawQueryStream = ResponseStream;

    async fn raw_query(
        &self,
        request: Request<RawQueryRequest>,
    ) -> Result<Response<Self::RawQueryStream>, Status> {
        todo!()
    }
}

impl UserStatsService {
    pub async fn new(config: AppConfig) -> Self {
        todo!()
    }

    pub fn into_server(self) -> UserStatsServer<Self> {
        todo!()
    }
}

impl Deref for UserStatsService {
    type Target = UserStatsServiceInner;

    fn deref(&self) -> &Self::Target {
        todo!()
    }
}
