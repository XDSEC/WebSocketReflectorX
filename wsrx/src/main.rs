use clap::{command, Parser};

mod cli;

/// wsrx is a controlled WS-TCP tunnel for Ret2Shell platform.
#[derive(Parser)]
#[command(name = "wsrx")]
#[command(bin_name = "wsrx")]
enum WsrxCli {
    /// Launch wsrx daemon.
    Daemon {
        #[clap(long)]
        /// The admin and ws http address to listen on.
        host: Option<String>,
        #[clap(short, long)]
        /// The admin and ws http port to listen on.
        port: Option<u16>,
    },
    /// Launch wsrx client.
    Connect {
        /// The address to connect to.
        address: String,
        #[clap(long)]
        /// The admin and ws http address to listen on.
        host: Option<String>,
        #[clap(short, long)]
        /// The admin and ws http port to listen on.
        port: Option<u16>,
    },
}

#[tokio::main]
async fn main() {
    let cli = WsrxCli::parse();
    match cli {
        WsrxCli::Daemon { host, port } => cli::daemon::launch_daemon(host, port).await,
        WsrxCli::Connect {
            address,
            host,
            port,
        } => cli::connect::launch_client(address, host, port).await,
    }
}
