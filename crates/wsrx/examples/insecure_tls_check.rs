//! Manual end-to-end check for the `accept_invalid_certs` tunnel option.
//!
//! Spawns:
//!   1. a plain WebSocket echo server,
//!   2. a `socat` OpenSSL listener terminating TLS with a freshly generated
//!      self-signed (i.e. unverifiable) certificate in front of it,
//!
//! then verifies that a [`Tunnel`] can proxy through the `wss://` endpoint
//! with `TunnelOptions::new(true)` and fails to echo anything without it.
//!
//! Run with:
//! ```sh
//! cargo run -p wsrx --example insecure_tls_check --features client
//! ```

use std::{process::Command, time::Duration};

use futures_util::{SinkExt, StreamExt};
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::{TcpListener, TcpStream},
};
use tokio_tungstenite::accept_async;
use wsrx::tunnel::{Tunnel, TunnelOptions};

const PLAIN_PORT: u16 = 19190;
const TLS_PORT: u16 = 19191;

#[tokio::main]
async fn main() {
    // Real wsrx binaries (CLI and desktop) install a process-wide crypto
    // provider before creating any TLS connection; mirror that here so the
    // default (verifying) connect path works as in production.
    if rustls::crypto::aws_lc_rs::default_provider()
        .install_default()
        .is_err()
    {
        rustls::crypto::ring::default_provider()
            .install_default()
            .expect("no rustls crypto backend available");
    }

    // 1. Plain WebSocket echo server.
    tokio::spawn(async move {
        let listener = TcpListener::bind(("127.0.0.1", PLAIN_PORT))
            .await
            .expect("failed to bind plain echo server");
        loop {
            let Ok((stream, _)) = listener.accept().await else {
                continue;
            };
            tokio::spawn(async move {
                if let Ok(ws) = accept_async(stream).await {
                    let (mut write, mut read) = ws.split();
                    while let Some(Ok(msg)) = read.next().await {
                        if msg.is_binary() || msg.is_text() {
                            write.send(msg).await.ok();
                        }
                    }
                }
            });
        }
    });

    // 2. Self-signed certificate + socat TLS terminator in front of it.
    let dir = std::env::temp_dir().join("wsrx-insecure-tls-check");
    std::fs::create_dir_all(&dir).unwrap();
    let cert = dir.join("cert.pem");
    let key = dir.join("key.pem");
    assert!(
        Command::new("openssl")
            .args(["req", "-x509", "-newkey", "rsa:2048", "-nodes", "-keyout",])
            .arg(&key)
            .args(["-out"])
            .arg(&cert)
            .args(["-days", "1", "-subj", "/CN=127.0.0.1",])
            .output()
            .expect("openssl not available")
            .status
            .success(),
        "failed to generate self-signed certificate"
    );
    let tls_addr = format!(
        "OPENSSL-LISTEN:{TLS_PORT},cert={},key={},verify=0,fork,reuseaddr",
        cert.display(),
        key.display(),
    );
    let mut socat = Command::new("socat")
        .arg(tls_addr)
        .arg(format!("TCP:127.0.0.1:{PLAIN_PORT}"))
        .spawn()
        .expect("socat not available");

    // Give socat a moment to bind.
    tokio::time::sleep(Duration::from_millis(500)).await;

    // Optionally probe an externally provided `wss://` endpoint (e.g. one
    // terminated with an expired certificate) instead of the local one.
    let remote = std::env::var("WSS_URL").unwrap_or_else(|_| format!("wss://127.0.0.1:{TLS_PORT}"));
    println!("probing {remote}");

    // 3. Positive case: invalid certs accepted -> data flows through.
    let echoed = proxy_round_trip(&remote, true).await;
    assert_eq!(
        echoed,
        Some(b"hello wsrx".to_vec()),
        "tunnel with accept_invalid_certs=true should echo through the invalid-cert wss server"
    );
    println!("PASS: accepted connection with invalid certificate");

    // 4. Negative case: verification enabled -> no data flows.
    let echoed = proxy_round_trip(&remote, false).await;
    assert_eq!(
        echoed, None,
        "tunnel with accept_invalid_certs=false should reject the invalid certificate"
    );
    println!("PASS: rejected connection with invalid certificate");

    socat.kill().ok();
    socat.wait().ok();
    println!("all checks passed");
}

/// Creates a tunnel to `remote`, sends `hello wsrx` through it and waits for
/// the echo. Returns `None` when nothing came back (or the connection died).
async fn proxy_round_trip(remote: &str, accept_invalid_certs: bool) -> Option<Vec<u8>> {
    let listener = TcpListener::bind(("127.0.0.1", 0))
        .await
        .expect("failed to bind tunnel listener");
    let local = listener.local_addr().expect("no local addr");
    let _tunnel = Tunnel::with_options(remote, listener, TunnelOptions::new(accept_invalid_certs));

    let mut tcp = TcpStream::connect(local)
        .await
        .expect("failed to connect to tunnel");
    tcp.write_all(b"hello wsrx").await.unwrap();

    let mut buf = vec![0u8; 16];
    match tokio::time::timeout(Duration::from_secs(10), tcp.read(&mut buf)).await {
        Ok(Ok(n)) if n > 0 => Some(buf[..n].to_vec()),
        _ => None,
    }
}
