use crate::{QueryRequest, RawQueryRequest, ResponseStream, ServiceResult, User, UserStatsService};
use chrono::{DateTime, TimeZone, Utc};
use itertools::Itertools;
use prost_types::Timestamp;
use tonic::{Response, Status};

impl UserStatsService {
    pub async fn query(&self, query_request: QueryRequest) -> ServiceResult<ResponseStream> {
        let mut sql = "SELECT email, name FROM user_stats WHERE ".to_string();

        let time_conditions = query_request
            .timestamps
            .into_iter()
            .map(|(k, v)| timestamp_query(&k, v.lower, v.upper))
            .join(" AND "); // 如果 time_conditions 为空，拼出来的 sql 就有问题

        println!("time_conditions: {}", time_conditions);

        sql.push_str(&time_conditions);

        let id_conditions = query_request
            .ids
            .into_iter()
            .map(|(k, v)| ids_query(&k, v.ids))
            .join(" AND ");
        // sql.push_str(" AND ");
        sql.push_str(&id_conditions);

        println!("Generated SQL: {}", sql);

        self.raw_query(RawQueryRequest { query: sql }).await
    }

    pub async fn raw_query(&self, req: RawQueryRequest) -> ServiceResult<ResponseStream> {
        let Ok(ret) = sqlx::query_as::<_, User>(&req.query)
            .fetch_all(&self.inner.pool)
            .await
        else {
            return Err(Status::internal(format!(
                "Failed to fetch data with query: {}",
                req.query
            )));
        };

        Ok(Response::new(Box::pin(futures::stream::iter(
            ret.into_iter().map(Ok),
        ))))
    }
}

fn ids_query(name: &str, ids: Vec<u32>) -> String {
    if ids.is_empty() {
        return "TRUE".to_string();
    }

    // 这个 SQL 拼接感觉有点问题
    format!("array{:?} <@ {}", ids, name)
}

fn timestamp_query(name: &str, lower: Option<Timestamp>, upper: Option<Timestamp>) -> String {
    if lower.is_none() && upper.is_none() {
        return "TRUE".to_string();
    }

    if lower.is_none() {
        let upper = ts_to_utc(upper.unwrap());
        return format!("{} <= '{}'", name, upper.to_rfc3339());
    }

    if upper.is_none() {
        let lower = ts_to_utc(lower.unwrap());
        return format!("{} >= '{}'", name, lower.to_rfc3339());
    }

    format!(
        "{} BETWEEN '{}' AND '{}'",
        name,
        ts_to_utc(upper.unwrap()).to_rfc3339(),
        ts_to_utc(lower.unwrap()).to_rfc3339()
    )
}

fn ts_to_utc(ts: Timestamp) -> DateTime<Utc> {
    Utc.timestamp_opt(ts.seconds, ts.nanos as _).unwrap()
}

#[cfg(test)]
mod tests {
    use crate::{
        AppConfig, IdQuery, QueryRequestBuilder, RawQueryRequest, TimeQuery, UserStatsService,
    };
    use anyhow::Result;
    use chrono::{Days, Utc};
    use futures::StreamExt;
    use itertools::assert_equal;
    use prost_types::Timestamp;
    use std::time::{SystemTime, UNIX_EPOCH};

    #[tokio::test]
    async fn raw_query_should_work() -> Result<()> {
        let app_config = AppConfig::load().expect("Failed to load config");

        let svc = UserStatsService::new(app_config).await;

        let mut stream = svc
            .raw_query(RawQueryRequest {
                query: "select * from user_stats where created_at > '2024-01-01' limit 5"
                    .to_string(),
            })
            .await?
            .into_inner();

        while let Some(res) = stream.next().await {
            println!("{:?}", res?);
        }

        Ok(())
    }

    #[tokio::test]
    async fn query_should_work() -> Result<()> {
        let app_config = AppConfig::load().expect("Failed to load config");
        let svc = UserStatsService::new(app_config).await;

        let request = QueryRequestBuilder::default()
            .timestamp(("created_at".to_string(), time_query(Some(120), None)))
            .timestamp(("last_visited_at".to_string(), time_query(Some(30), None)))
            // .id(("viewed_but_not_started".to_string(), id_query(&[252790])))
            .build()?;

        let mut stream = svc.query(request).await?.into_inner();

        while let Some(res) = stream.next().await {
            println!("{:?}", res?);
        }

        Ok(())
    }

    fn id_query(id: &[u32]) -> IdQuery {
        IdQuery { ids: id.to_vec() }
    }

    fn time_query(lower: Option<i64>, upper: Option<i64>) -> TimeQuery {
        TimeQuery {
            lower: lower.map(day_to_timestamp),
            upper: upper.map(day_to_timestamp), // 如果 upper 为 None，则直接返回 None
        }
    }

    fn day_to_timestamp(days: i64) -> Timestamp {
        let date = Utc::now()
            .checked_sub_signed(chrono::Duration::days(days))
            .unwrap();

        Timestamp {
            seconds: date.timestamp(),
            nanos: date.timestamp_subsec_nanos() as i32,
        }
    }

    #[test]
    fn test_utc_time() -> Result<()> {
        let date = Utc::now()
            .checked_sub_signed(chrono::Duration::days(3))
            .unwrap();
        println!("{}", date);

        let time = Utc::now().checked_sub_days(Days::new(3)).unwrap();
        println!("{}", time);

        let current_mills = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards")
            .as_millis();

        println!("{}", current_mills);

        let date = Some(4).map(day_to_timestamp);

        println!("{:?}", date);

        let date = None.map(day_to_timestamp);
        println!("{:?}", date);

        let maybe_some_string = Some(String::from("Hello, World!"));
        let maybe_some_len = maybe_some_string.map(|s| s.len());
        assert_eq!(maybe_some_len, Some(13));

        let x: Option<&str> = None;
        assert_equal(x.map(|s| s.len()), None::<usize>);

        Ok(())
    }
}
