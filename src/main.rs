use zbus::{Connection, proxy};
use futures_util::stream::StreamExt;

#[proxy(
    interface = "org.shadowblip.Input.DBusDevice",
    default_service = "org.shadowblip.InputPlumber",
    default_path = "/org/shadowblip/InputPlumber/devices/target/dbus0"
)]
trait DBusDevice {
    #[zbus(signal)]
    fn input_event(&self, event: String, value: f32) -> zbus::Result<()>;
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let connection = Connection::system().await?;

    let proxy = DBusDeviceProxy::builder(&connection)
        .destination(":1.12")? // Using the unique name from your command
        .path("/org/shadowblip/InputPlumber/devices/target/dbus0")?
        .build()
        .await?;

    println!("Waiting for signal...");

    let mut stream = proxy.receive_input_event().await?;

    // 4. Wait for the first occurrence (mimics 'wait')
    while let Some(v) = stream.next().await {
        let args = v.args()?;
        println!("{}, {}", args.event(), args.value());
    }

    Ok(())
}
