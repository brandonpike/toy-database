use std::{net::Ipv4Addr, time::Duration};

use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::{TcpListener, TcpStream},
    task::JoinSet,
    time::{interval_at, Instant},
};
use toy_database::{qp::QueryProcessor, storage::Storage, AppResult};

#[tokio::main]
async fn main() -> AppResult<()> {
    tracing_subscriber::fmt::init();

    let listener = TcpListener::bind((Ipv4Addr::LOCALHOST, 12345)).await?;
    let qp = QueryProcessor::new(Storage::new());

    let mut connections = JoinSet::new();
    let mut interval = interval_at(
        Instant::now() + Duration::from_secs(5),
        Duration::from_secs(5),
    );

    loop {
        tokio::select! {
            res = listener.accept() => {
                let (stream, peer_addr) = res?;
                tracing::debug!(%peer_addr, "accepted connection");
                connections.spawn(handle(stream, qp.clone()));
            },

            _ = interval.tick() => {
                tracing::info!("Current connections: {}", connections.len());
            }
        }
    }
}

async fn handle(mut stream: TcpStream, qp: QueryProcessor) -> AppResult<()> {
    loop {
        let mut buf = String::new();
        stream.read_to_string(&mut buf).await?;
        tracing::debug!(buf, "received message from client");

        let response = qp.process(buf).await?;
        tracing::debug!(
            response = %String::from_utf8_lossy(&response),
            "received message from storage"
        );
        stream.write_all(&response).await?;
    }
}
