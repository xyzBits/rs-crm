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
//todo 为什么这个类型需要使用 Pin 呢
type ResponseStream = Pin<Box<dyn Stream<Item = Result<User, Status>> + Send>>;
// cannot be unpinned
// type ResponseStream = Box<dyn Stream<Item = Result<User, Status>> + Send>;

#[derive(Clone)]
pub struct UserStatsService {
    inner: Arc<UserStatsServiceInner>,
}

#[allow(unused)]
pub struct UserStatsServiceInner {
    config: AppConfig,
    pool: PgPool,
}

// impl protobuf 中定义的 grpc 接口
#[async_trait] // 不加这个编译不过
impl UserStats for UserStatsService {
    type QueryStream = ResponseStream;
    async fn query(
        &self,
        request: Request<QueryRequest>,
    ) -> Result<Response<Self::QueryStream>, Status> {
        let query_request = request.into_inner();
        self.query(query_request).await
    }

    type RawQueryStream = ResponseStream;

    async fn raw_query(
        &self,
        request: Request<RawQueryRequest>,
    ) -> Result<Response<Self::RawQueryStream>, Status> {
        let raw_query_request = request.into_inner();
        self.raw_query(raw_query_request).await
    }
}

impl UserStatsService {
    /// 将 服务器的配置加载进来
    /// 并且将 数据库连接池放在 Arc 中
    pub async fn new(config: AppConfig) -> Self {
        let pool = PgPool::connect(&config.server.db_url)
            .await
            .expect("Failed to connect to db");

        let inner = UserStatsServiceInner { config, pool };

        Self {
            inner: Arc::new(inner),
        }
    }

    pub fn into_server(self) -> UserStatsServer<Self> {
        UserStatsServer::new(self)
    }
}

impl Deref for UserStatsService {
    type Target = UserStatsServiceInner;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
