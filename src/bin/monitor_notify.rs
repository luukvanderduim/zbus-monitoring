use futures_lite::StreamExt;
use zbus::{fdo::MonitoringProxy, Connection, MatchRule, MessageStream};

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

#[tokio::main]
async fn main() -> Result<()> {
    let conn = Connection::session().await?;

    let monitor = MonitoringProxy::builder(&conn)
        .destination("org.freedesktop.DBus")?
        .path("/org/freedesktop/DBus")?
        .build()
        .await?;

    let rule: String = MatchRule::builder()
        .interface("org.freedesktop.DBus.Monitoring")?
        .msg_type(zbus::MessageType::MethodCall)
        .member("Notify")?
        .build()
        .to_string();

    let rule_str = rule.as_str();

    monitor.become_monitor(&[rule_str], 0).await?;

    let mut stream = MessageStream::from(monitor.connection());

    while let Some(Ok(msg)) = stream.next().await {
        println!("Noticed activity: {}", msg);
    }

    Ok(())
}
