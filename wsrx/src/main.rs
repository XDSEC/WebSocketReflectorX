use std::process;

use clap::{command, Parser};
use rustls::crypto;
use tracing::{error, info, warn};

mod cli;

/// wsrx is a controlled WS-TCP tunnel for Ret2Shell platform.
#[derive(Parser)]
#[command(name = "wsrx", bin_name = "wsrx", version, about)]
enum WsrxCli {
    #[clap(alias("d"))]
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
        /// If not set, the daemon will not automatically exit when heartbeat
        /// timeout.
        #[clap(long)]
        heartbeat: Option<u64>,
    },
    #[clap(alias("c"))]
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
    #[clap(alias("s"))]
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
    match crypto::aws_lc_rs::default_provider().install_default() {
        Ok(_) => info!("using `AWS Libcrypto` as default crypto backend."),
        Err(err) => {
            error!("`AWS Libcrypto` is not available: {:?}", err);
            warn!("try to use `ring` as default crypto backend.");
            crypto::ring::default_provider()
                .install_default()
                .inspect_err(|err| {
                    error!("`ring` is not available: {:?}", err);
                    error!("All crypto backend are not available, exiting...");
                    process::exit(1);
                })
                .ok();
            info!("using `ring` as default crypto backend.");
        }
    }
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
