use futures_lite::StreamExt;
use zbus::{fdo::MonitoringProxy, Connection, MessageStream};

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

#[tokio::main]
async fn main() -> Result<()> {
    let conn = Connection::session().await?;

    // A monitoring proxy with the default destination, path, and interface
    let monitor = MonitoringProxy::new(&conn).await?;

    let rules = &[""];
    monitor.become_monitor(rules, 0).await?;

    let mut stream = MessageStream::from(monitor.connection());

    while let Some(Ok(msg)) = stream.next().await {
        println!("Noticed activity: {}", msg);
    }

    Ok(())
}
