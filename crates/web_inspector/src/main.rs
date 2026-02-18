use anyhow::{Context, Result};
use async_tungstenite::tungstenite::Message;
use clap::{Parser, Subcommand};
use futures::StreamExt;
use prost::Message as ProstMessage;
use rpc::proto::{Envelope, envelope::Payload};
use serde_json::json;
use smol::net::TcpStream;

#[derive(Parser)]
#[command(name = "web_inspector", about = "Inspect WebSocket proto messages from Zed's remote server")]
struct Args {
    #[command(subcommand)]
    command: Option<Command>,

    /// WebSocket URL to connect to (for default listen mode)
    #[arg(short, long, default_value = "ws://localhost:8080")]
    url: String,

    /// Filter by message type name (e.g., "UpdateWorktree")
    #[arg(short = 't', long = "type")]
    message_type: Option<String>,

    /// Show full payload details
    #[arg(short, long)]
    verbose: bool,
}

#[derive(Subcommand)]
enum Command {
    /// Serve the web inspector HTML page on an HTTP port
    Serve {
        /// HTTP port to serve the inspector page on
        #[arg(short, long, default_value = "8081")]
        port: u16,
    },
}

fn payload_type_name(payload: &Payload) -> &'static str {
    match payload {
        Payload::Hello(_) => "Hello",
        Payload::Ack(_) => "Ack",
        Payload::Error(_) => "Error",
        Payload::Ping(_) => "Ping",
        Payload::Test(_) => "Test",
        Payload::EndStream(_) => "EndStream",
        Payload::CreateRoom(_) => "CreateRoom",
        Payload::CreateRoomResponse(_) => "CreateRoomResponse",
        Payload::JoinRoom(_) => "JoinRoom",
        Payload::JoinRoomResponse(_) => "JoinRoomResponse",
        Payload::UpdateWorktree(_) => "UpdateWorktree",
        Payload::UpdateWorktreeSettings(_) => "UpdateWorktreeSettings",
        Payload::CreateBufferForPeer(_) => "CreateBufferForPeer",
        Payload::UpdateBuffer(_) => "UpdateBuffer",
        Payload::UpdateDiagnosticSummary(_) => "UpdateDiagnosticSummary",
        Payload::OpenBufferByPath(_) => "OpenBufferByPath",
        Payload::OpenBufferResponse(_) => "OpenBufferResponse",
        _ => "Unknown",
    }
}

fn payload_summary(payload: &Payload) -> serde_json::Value {
    match payload {
        Payload::Hello(hello) => json!({
            "peer_id": hello.peer_id,
        }),
        Payload::Ack(_) => json!({}),
        Payload::Error(error) => json!({
            "message": error.message,
        }),
        Payload::UpdateWorktree(update) => json!({
            "project_id": update.project_id,
            "worktree_id": update.worktree_id,
            "updated_entries_count": update.updated_entries.len(),
            "removed_entries_count": update.removed_entries.len(),
        }),
        Payload::CreateBufferForPeer(create) => json!({
            "project_id": create.project_id,
            "peer_id": create.peer_id,
        }),
        Payload::UpdateBuffer(update) => json!({
            "project_id": update.project_id,
            "buffer_id": update.buffer_id,
        }),
        Payload::UpdateDiagnosticSummary(update) => json!({
            "project_id": update.project_id,
            "worktree_id": update.worktree_id,
        }),
        _ => json!({}),
    }
}

const INSPECTOR_HTML: &str = include_str!("../static/inspector.html");

fn main() -> Result<()> {
    let args = Args::parse();

    match args.command {
        Some(Command::Serve { port }) => serve_inspector(port),
        None => listen_cli(args),
    }
}

fn serve_inspector(port: u16) -> Result<()> {
    smol::block_on(async {
        let listener = smol::net::TcpListener::bind(format!("0.0.0.0:{port}"))
            .await
            .context(format!("failed to bind HTTP server on port {port}"))?;

        eprintln!("Inspector page: http://localhost:{port}");

        loop {
            let (stream, addr) = listener.accept().await?;
            smol::spawn(async move {
                if let Err(error) = handle_http_request(stream).await {
                    eprintln!("HTTP error from {addr}: {error:?}");
                }
            })
            .detach();
        }
    })
}

async fn handle_http_request(mut stream: smol::net::TcpStream) -> Result<()> {
    use smol::io::{AsyncReadExt, AsyncWriteExt};

    let mut buf = vec![0u8; 4096];
    let n = stream.read(&mut buf).await?;
    let request = String::from_utf8_lossy(&buf[..n]);

    let (status, content_type, body) = if request.starts_with("GET / ")
        || request.starts_with("GET /inspector")
    {
        ("200 OK", "text/html; charset=utf-8", INSPECTOR_HTML)
    } else {
        ("404 Not Found", "text/plain", "Not found")
    };

    let response = format!(
        "HTTP/1.1 {status}\r\nContent-Type: {content_type}\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{body}",
        body.len(),
    );
    stream.write_all(response.as_bytes()).await?;
    Ok(())
}

fn listen_cli(args: Args) -> Result<()> {
    smol::block_on(async {
        let url = &args.url;
        let host_port = url
            .strip_prefix("ws://")
            .context("URL must start with ws://")?;

        let tcp_stream = TcpStream::connect(host_port)
            .await
            .context(format!("failed to connect to {host_port}"))?;

        let (ws_stream, _) = async_tungstenite::client_async(url, tcp_stream)
            .await
            .context("WebSocket handshake failed")?;

        let (_sink, mut source) = ws_stream.split();

        eprintln!("Connected to {url}");
        eprintln!("Listening for proto messages...");

        let mut message_count: u64 = 0;
        let mut total_bytes: u64 = 0;

        while let Some(msg) = source.next().await {
            match msg {
                Ok(Message::Binary(data)) => {
                    total_bytes += data.len() as u64;
                    match Envelope::decode(data.as_ref()) {
                        Ok(envelope) => {
                            message_count += 1;
                            let type_name = envelope
                                .payload
                                .as_ref()
                                .map(payload_type_name)
                                .unwrap_or("Empty");

                            if let Some(ref filter) = args.message_type {
                                if type_name != filter.as_str() {
                                    continue;
                                }
                            }

                            let summary = envelope
                                .payload
                                .as_ref()
                                .map(payload_summary)
                                .unwrap_or(json!(null));

                            let output = if args.verbose {
                                json!({
                                    "seq": message_count,
                                    "id": envelope.id,
                                    "responding_to": envelope.responding_to,
                                    "type": type_name,
                                    "bytes": data.len(),
                                    "summary": summary,
                                    "total_messages": message_count,
                                    "total_bytes": total_bytes,
                                })
                            } else {
                                json!({
                                    "seq": message_count,
                                    "id": envelope.id,
                                    "type": type_name,
                                    "summary": summary,
                                })
                            };

                            println!("{}", serde_json::to_string(&output)?);
                        }
                        Err(error) => {
                            eprintln!("Failed to decode envelope: {error:?}");
                        }
                    }
                }
                Ok(Message::Close(_)) => {
                    eprintln!("Server closed connection");
                    break;
                }
                Ok(_) => {}
                Err(error) => {
                    eprintln!("WebSocket error: {error:?}");
                    break;
                }
            }
        }

        eprintln!(
            "Disconnected. Total: {message_count} messages, {total_bytes} bytes"
        );
        Ok(())
    })
}
