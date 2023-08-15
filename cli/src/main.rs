mod connection;

use clap::{arg, command, crate_version, value_parser};

#[tokio::main]
async fn main() {
    simple_logger::SimpleLogger::new().with_level(log::LevelFilter::Info).init().unwrap();
    let matches = command!()
        .name("WebSocket Reflector X")
        .about("Controlled TCP-over-WebSocket forwarding tunnel.")
        .author("Reverier-Xu <reverier.xu@woooo.tech>")
        .version(crate_version!())
        .arg(arg!([url] "The WebSocket URL to connect to.").required(true))
        .arg(
            arg!(
                -p --port <PORT> "The local TCP port to listen on"
            )
            .required(false).value_parser(value_parser!(u16)),
        )
        .arg(
            arg!(
                -g --global "Listen on all interfaces"
            )
            .required(false).value_parser(value_parser!(bool)),
        )
        .get_matches();

    let url = matches
        .get_one::<String>("url")
        .expect("The aim WebSocket URL is required")
        .to_string();
    let bind_global = matches.get_one::<bool>("global").unwrap_or(&false).to_owned();
    match matches.get_one::<u16>("port") {
        Some(port) => {
            connection::connect(url, Some(port.to_owned()), bind_global)
                .await
                .expect("Failed to connect to WebSocket server");
        }
        None => {
            connection::connect(url, None, bind_global)
                .await
                .expect("Failed to connect to WebSocket server");
        }
    };
}
