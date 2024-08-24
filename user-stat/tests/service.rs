use anyhow::Result;
use std::net::SocketAddr;

const PORT_BASE: u32 = 60000;

async fn raw_query_should_work() -> Result<()> {
    Ok(())
}

async fn start_server(port: u32) -> Result<String, SocketAddr> {
    todo!()
}
