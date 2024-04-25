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
        /// Log in json format.
        #[clap(short, long)]
        log_json: Option<bool>,
        /// The heartbeat interval in seconds.
        /// If not set, the daemon will not automatically exit when heartbeat timeout.
        #[clap(long)]
        heartbeat: Option<u64>,
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
        /// Log in json format.
        #[clap(short, long)]
        log_json: Option<bool>,
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
        /// Log in json format.
        #[clap(short, long)]
        log_json: Option<bool>,
    },
}

#[tokio::main]
async fn main() {
    let cli = WsrxCli::parse();
    match cli {
        WsrxCli::Daemon {
            host,
            port,
            secret,
            log_json,
            heartbeat,
        } => cli::daemon::launch(host, port, secret, log_json, heartbeat).await,
        WsrxCli::Connect {
            address,
            host,
            port,
            log_json,
        } => cli::connect::launch(address, host, port, log_json).await,
        WsrxCli::Serve {
            host,
            port,
            secret,
            log_json,
        } => cli::serve::launch(host, port, secret, log_json).await,
    }
}
