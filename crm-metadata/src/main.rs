use crm_metadata::{AppConfig, MetadataService};
use tonic::transport::Server;
use tracing::info;
use tracing::level_filters::LevelFilter;
use tracing_subscriber::fmt::Layer;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;
use tracing_subscriber::Layer as _;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let layer = Layer::new().with_filter(LevelFilter::INFO);
    tracing_subscriber::registry().with(layer).init();

    let config = AppConfig::load().expect("Metadata filed to load config");
    let port = config.server.port;
    let address = format!("[::1]:{}", port).parse()?;

    info!("Metadata service listening on {}", address);

    let svc = MetadataService::new(config).into_server();

    Server::builder().add_service(svc).serve(address).await?;

    Ok(())
}
