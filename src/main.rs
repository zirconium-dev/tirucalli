mod events;
use events::{InputState, InputMapping};

use log::log;
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

fn call_ipc_quick(action: niri_ipc::Request::Action) {
    niri_socket.send(action).expect("Failure").expect("No idea");
}


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();

    log::debug!("amogus");
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

        if (state == InputState::Pressed) {
            match args.event() {
                "ui_launcher" => {
                    call_ipc_quick(niri_ipc::Request::Action(niri_ipc::Action::SpawnSh { command: "dms ipc call spotlight toggle".to_string() }));
                }
                "ui_closewindow" => {
                    call_ipc_quick(niri_ipc::Request::Action(niri_ipc::Action::CloseWindow { id: None }));
                }
                "ui_window_up" => {
                    call_ipc_quick(niri_ipc::Request::Action(niri_ipc::Action::FocusWindowOrWorkspaceUp {  }));
                }
                "ui_window_down" => {
                    call_ipc_quick(niri_ipc::Request::Action(niri_ipc::Action::FocusWindowOrWorkspaceDown {  }));
                }
                "ui_window_left" => {
                    call_ipc_quick(niri_ipc::Request::Action(niri_ipc::Action::FocusWindowOrWorkspaceLeft {  }));
                }
                "ui_window_right" => {
                    call_ipc_quick(niri_ipc::Request::Action(niri_ipc::Action::FocusWindowOrWorkspaceRight {  }));
                }
                "ui_overview" => {
                    call_ipc_quick(niri_ipc::Request::Action(niri_ipc::Action::OpenOverview {  }));
                }
            }

        }

        if (state == InputState::Released) {
            match args.event() {
                "ui_overview" => {
                    call_ipc_quick(niri_ipc::Request::Action(niri_ipc::Action::CloseOverview {  }));
                }
                _ => (),
            }
        }
    }

    Ok(())
}
