mod events;
use events::{InputState, InputMapping};

use zbus::{Connection, proxy};
use futures_util::stream::StreamExt;

#[proxy(
    interface = "org.shadowblip.Input.DBusDevice",
    default_service = "org.shadowblip.InputPlumber",
    default_path = "/org/shadowblip/InputPlumber/devices/target/dbus0"
)]
trait DBusDevice {
    #[zbus(signal)]
    fn input_event(&self, event: String, value: f64) -> zbus::Result<()>;
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let connection = Connection::system().await?;

    let mut niri_socket = niri_ipc::socket::Socket::connect().expect("Failed talking to niri socket");

    let proxy = DBusDeviceProxy::builder(&connection)
        .path("/org/shadowblip/InputPlumber/devices/target/dbus0")?
        .build()
        .await?;

    println!("Waiting for signal...");

    let mut stream = proxy.receive_input_event().await?;

    while let Some(v) = stream.next().await {
        let args = v.args()?;
        let state = InputState::from(args.value);
        // let event = InputDbusEvent::from_str(args.event);
        println!("{}, {:?}", args.event(), state);
        if (args.event() == "ui_launcher") && (state == InputState::Pressed) {
            niri_socket
                .send(niri_ipc::Request::Action(niri_ipc::Action::SpawnSh { command: "dms ipc call spotlight toggle".to_string() }))
                .expect("whatever?")
                .expect("fucking i dont know");
        }
        if (args.event() == "ui_window_up") && (state == InputState::Pressed) {
            niri_socket
                .send(niri_ipc::Request::Action(niri_ipc::Action::FocusWindowOrWorkspaceUp {  }))
                .expect("whatever?")
                .expect("fucking i dont know");
        }
        if (args.event() == "ui_window_down") && (state == InputState::Pressed) {
            niri_socket
                .send(niri_ipc::Request::Action(niri_ipc::Action::FocusWindowOrWorkspaceDown {  }))
                .expect("whatever?")
                .expect("fucking i dont know");
        }
        if (args.event() == "ui_window_left") && (state == InputState::Pressed) {
            niri_socket
                .send(niri_ipc::Request::Action(niri_ipc::Action::FocusColumnOrMonitorLeft {  }))
                .expect("whatever?")
                .expect("fucking i dont know");
        }
        if (args.event() == "ui_window_right") && (state == InputState::Pressed) {
            niri_socket
                .send(niri_ipc::Request::Action(niri_ipc::Action::FocusColumnOrMonitorRight {  }))
                .expect("whatever?")
                .expect("fucking i dont know");
        }
        if (args.event() == "ui_overview") && (state == InputState::Pressed) {
            niri_socket
                .send(niri_ipc::Request::Action(niri_ipc::Action::OpenOverview {  }))
                .expect("whatever?")
                .expect("fucking i dont know");
        }
        if (args.event() == "ui_overview") && (state == InputState::Released) {
            niri_socket
                .send(niri_ipc::Request::Action(niri_ipc::Action::CloseOverview {  }))
                .expect("whatever?")
                .expect("fucking i dont know");
        }
    }

    Ok(())
}
