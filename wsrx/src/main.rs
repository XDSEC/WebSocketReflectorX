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
        #[clap(short, long)]
        secret: Option<String>,
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
    /// Launch wsrx server.
    Serve {
        #[clap(long)]
        /// The admin and ws http address to listen on.
        host: Option<String>,
        #[clap(short, long)]
        /// The admin and ws http port to listen on.
        port: Option<u16>,
        #[clap(short, long)]
        secret: Option<String>,
    },
}

#[tokio::main]
async fn main() {
    let cli = WsrxCli::parse();
    match cli {
        WsrxCli::Daemon { host, port, secret } => cli::daemon::launch(host, port, secret).await,
        WsrxCli::Connect {
            address,
            host,
            port,
        } => cli::connect::launch(address, host, port).await,
        WsrxCli::Serve { host, port, secret } => cli::server::launch(host, port, secret).await,
    }
}
