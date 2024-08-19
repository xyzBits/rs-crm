use crate::{QueryRequest, RawQueryRequest, ResponseStream, ServiceResult, UserStatsService};
use chrono::{DateTime, Utc};
use prost_types::Timestamp;

impl UserStatsService {
    pub async fn query(&self, query: QueryRequest) -> ServiceResult<ResponseStream> {
        todo!()
    }

    pub async fn raw_query(&self, req: RawQueryRequest) -> ServiceResult<ResponseStream> {
        todo!()
    }
}

fn ids_query(name: &str, ids: Vec<u32>) -> String {
    todo!()
}

fn timestamp_query(name: &str, lower: Option<Timestamp>, upper: Option<Timestamp>) -> String {
    todo!()
}

fn ts_to_utc(ts: Timestamp) -> DateTime<Utc> {
    todo!()
}

#[cfg(test)]
mod tests {}
