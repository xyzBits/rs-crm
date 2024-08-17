use anyhow::Result;
use sqlx::postgres::PgPool;
use sqlx::Executor;

#[derive(Debug, sqlx::FromRow)]
struct UserStat {
    name: String,
    email: String,
}

#[tokio::main]
async fn main() -> Result<()> {
    let db_url = "postgres://postgres:pass123@localhost:5432/stats";
    // let pool = PgPool::connect(&std::env::var("DATABASE_URL")?).await?;
    let pool = PgPool::connect(db_url).await?;

    let result = sqlx::query_as::<_, UserStat>("select * from user_stats")
        .fetch_all(&pool)
        .await?;

    println!("{:?}", result);

    Ok(())
}
