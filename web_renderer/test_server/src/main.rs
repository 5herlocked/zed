//! Standalone test server that streams mock GPUI scene data over WebSocket.
//!
//! This binary validates the full streaming pipeline without requiring a real
//! Zed instance or Linux. It constructs FrameMessage JSON matching the wire
//! protocol defined in the web_renderer's protocol.ts and broadcasts it to
//! connected browser clients.
//!
//! Usage:
//!   cargo run -p web-renderer-test-server
//!
//! Then open http://localhost:3100 in a browser (with the web_renderer dev
//! server running via `npm run dev`). The browser connects to ws://localhost:3101
//! and renders the streamed scene.

use std::net::SocketAddr;
use std::sync::Arc;
use std::time::Duration;

use async_tungstenite::tungstenite::Message;
use futures::StreamExt;
use futures::channel::mpsc;
use parking_lot::Mutex;
use serde::Serialize;
use smol::Timer;
use smol::net::{TcpListener, TcpStream};

// ---------------------------------------------------------------------------
// Wire format types (mirrors protocol.ts / scene_message.rs)
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Serialize)]
struct FrameMessage {
    frame_id: u64,
    viewport_size: SizeMessage,
    scale_factor: f32,
    atlas_deltas: Vec<()>,
    scene: SceneBody,
}

#[derive(Debug, Clone, Serialize)]
struct SceneBody {
    shadows: Vec<ShadowMessage>,
    quads: Vec<QuadMessage>,
    paths: Vec<()>,
    underlines: Vec<UnderlineMessage>,
    monochrome_sprites: Vec<()>,
    subpixel_sprites: Vec<()>,
    polychrome_sprites: Vec<()>,
}

#[derive(Debug, Clone, Copy, Serialize)]
struct PointMessage {
    x: f32,
    y: f32,
}

#[derive(Debug, Clone, Copy, Serialize)]
struct SizeMessage {
    width: f32,
    height: f32,
}

#[derive(Debug, Clone, Copy, Serialize)]
struct BoundsMessage {
    origin: PointMessage,
    size: SizeMessage,
}

#[derive(Debug, Clone, Copy, Serialize)]
struct ContentMaskMessage {
    bounds: BoundsMessage,
}

#[derive(Debug, Clone, Copy, Serialize)]
struct CornersMessage {
    top_left: f32,
    top_right: f32,
    bottom_right: f32,
    bottom_left: f32,
}

#[derive(Debug, Clone, Copy, Serialize)]
struct EdgesMessage {
    top: f32,
    right: f32,
    bottom: f32,
    left: f32,
}

#[derive(Debug, Clone, Copy, Serialize)]
struct HslaMessage {
    h: f32,
    s: f32,
    l: f32,
    a: f32,
}

#[derive(Debug, Clone, Copy, Serialize)]
struct LinearColorStopMessage {
    color: HslaMessage,
    percentage: f32,
}

#[derive(Debug, Clone, Copy, Serialize)]
struct BackgroundMessage {
    tag: u32,
    color_space: u32,
    solid: HslaMessage,
    gradient_angle_or_pattern_height: f32,
    colors: [LinearColorStopMessage; 2],
}

#[derive(Debug, Clone, Serialize)]
struct QuadMessage {
    order: u32,
    border_style: u32,
    bounds: BoundsMessage,
    content_mask: ContentMaskMessage,
    background: BackgroundMessage,
    border_color: HslaMessage,
    corner_radii: CornersMessage,
    border_widths: EdgesMessage,
}

#[derive(Debug, Clone, Serialize)]
struct ShadowMessage {
    order: u32,
    blur_radius: f32,
    bounds: BoundsMessage,
    corner_radii: CornersMessage,
    content_mask: ContentMaskMessage,
    color: HslaMessage,
}

#[derive(Debug, Clone, Serialize)]
struct UnderlineMessage {
    order: u32,
    bounds: BoundsMessage,
    content_mask: ContentMaskMessage,
    color: HslaMessage,
    thickness: f32,
    wavy: u32,
}

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

const VIEWPORT_W: f32 = 2560.0;
const VIEWPORT_H: f32 = 1440.0;

fn full_mask() -> ContentMaskMessage {
    ContentMaskMessage {
        bounds: BoundsMessage {
            origin: PointMessage { x: 0.0, y: 0.0 },
            size: SizeMessage {
                width: VIEWPORT_W,
                height: VIEWPORT_H,
            },
        },
    }
}

fn no_corners() -> CornersMessage {
    CornersMessage {
        top_left: 0.0,
        top_right: 0.0,
        bottom_right: 0.0,
        bottom_left: 0.0,
    }
}

fn uniform_corners(r: f32) -> CornersMessage {
    CornersMessage {
        top_left: r,
        top_right: r,
        bottom_right: r,
        bottom_left: r,
    }
}

fn no_border() -> EdgesMessage {
    EdgesMessage {
        top: 0.0,
        right: 0.0,
        bottom: 0.0,
        left: 0.0,
    }
}

fn transparent() -> HslaMessage {
    HslaMessage {
        h: 0.0,
        s: 0.0,
        l: 0.0,
        a: 0.0,
    }
}

fn no_gradient() -> [LinearColorStopMessage; 2] {
    [
        LinearColorStopMessage {
            color: transparent(),
            percentage: 0.0,
        },
        LinearColorStopMessage {
            color: transparent(),
            percentage: 1.0,
        },
    ]
}

fn solid_bg(h: f32, s: f32, l: f32, a: f32) -> BackgroundMessage {
    BackgroundMessage {
        tag: 0,
        color_space: 0,
        solid: HslaMessage { h, s, l, a },
        gradient_angle_or_pattern_height: 0.0,
        colors: no_gradient(),
    }
}

fn quad(
    order: u32,
    x: f32,
    y: f32,
    w: f32,
    h: f32,
    bg: BackgroundMessage,
    border_color: HslaMessage,
    corner_radii: CornersMessage,
    border_widths: EdgesMessage,
    border_style: u32,
    content_mask: ContentMaskMessage,
) -> QuadMessage {
    QuadMessage {
        order,
        border_style,
        bounds: BoundsMessage {
            origin: PointMessage { x, y },
            size: SizeMessage {
                width: w,
                height: h,
            },
        },
        content_mask,
        background: bg,
        border_color,
        corner_radii,
        border_widths,
    }
}

// ---------------------------------------------------------------------------
// Scene builder
// ---------------------------------------------------------------------------

fn build_scene(frame_id: u64, time_secs: f64) -> FrameMessage {
    let mut quads = Vec::new();
    let mut shadows = Vec::new();
    let mut underlines = Vec::new();

    // Animate a subtle hue shift on the background
    let bg_hue = 0.7083 + (time_secs * 0.02).sin() as f32 * 0.02;

    // --- Background ---
    quads.push(quad(
        1,
        0.0,
        0.0,
        VIEWPORT_W,
        VIEWPORT_H,
        solid_bg(bg_hue, 0.25, 0.12, 1.0),
        transparent(),
        no_corners(),
        no_border(),
        0,
        full_mask(),
    ));

    // --- Title bar ---
    quads.push(quad(
        2,
        0.0,
        0.0,
        VIEWPORT_W,
        64.0,
        solid_bg(bg_hue, 0.2, 0.16, 1.0),
        HslaMessage {
            h: 0.0,
            s: 0.0,
            l: 0.5,
            a: 0.1,
        },
        no_corners(),
        EdgesMessage {
            top: 0.0,
            right: 0.0,
            bottom: 2.0,
            left: 0.0,
        },
        0,
        full_mask(),
    ));

    // --- Sidebar ---
    quads.push(quad(
        3,
        0.0,
        64.0,
        500.0,
        VIEWPORT_H - 64.0,
        solid_bg(bg_hue, 0.2, 0.14, 1.0),
        HslaMessage {
            h: 0.0,
            s: 0.0,
            l: 0.5,
            a: 0.1,
        },
        no_corners(),
        EdgesMessage {
            top: 0.0,
            right: 2.0,
            bottom: 0.0,
            left: 0.0,
        },
        0,
        full_mask(),
    ));

    // --- Editor area ---
    quads.push(quad(
        4,
        502.0,
        64.0,
        VIEWPORT_W - 502.0,
        VIEWPORT_H - 64.0 - 60.0,
        solid_bg(bg_hue, 0.25, 0.12, 1.0),
        transparent(),
        no_corners(),
        no_border(),
        0,
        full_mask(),
    ));

    // --- Status bar ---
    quads.push(quad(
        5,
        502.0,
        VIEWPORT_H - 60.0,
        VIEWPORT_W - 502.0,
        60.0,
        solid_bg(bg_hue, 0.2, 0.16, 1.0),
        HslaMessage {
            h: 0.0,
            s: 0.0,
            l: 0.5,
            a: 0.1,
        },
        no_corners(),
        EdgesMessage {
            top: 2.0,
            right: 0.0,
            bottom: 0.0,
            left: 0.0,
        },
        0,
        full_mask(),
    ));

    // --- Tab bar ---
    quads.push(quad(
        20,
        560.0,
        100.0,
        1900.0,
        48.0,
        solid_bg(bg_hue, 0.2, 0.16, 1.0),
        HslaMessage {
            h: 0.0,
            s: 0.0,
            l: 0.5,
            a: 0.08,
        },
        no_corners(),
        EdgesMessage {
            top: 0.0,
            right: 0.0,
            bottom: 2.0,
            left: 0.0,
        },
        0,
        full_mask(),
    ));

    // --- Active tab indicator ---
    quads.push(quad(
        30,
        560.0,
        148.0,
        120.0,
        48.0,
        solid_bg(bg_hue, 0.25, 0.12, 1.0),
        HslaMessage {
            h: 0.6167,
            s: 0.7,
            l: 0.6,
            a: 1.0,
        },
        no_corners(),
        EdgesMessage {
            top: 0.0,
            right: 0.0,
            bottom: 4.0,
            left: 0.0,
        },
        0,
        full_mask(),
    ));

    // --- Sidebar selected item (animating position) ---
    let selected_y = 120.0 + ((time_secs * 0.5).sin() as f32 * 0.5 + 0.5) * 300.0;
    quads.push(quad(
        10,
        20.0,
        selected_y,
        460.0,
        56.0,
        solid_bg(0.6167, 0.7, 0.5, 0.15),
        transparent(),
        uniform_corners(8.0),
        no_border(),
        0,
        ContentMaskMessage {
            bounds: BoundsMessage {
                origin: PointMessage { x: 0.0, y: 64.0 },
                size: SizeMessage {
                    width: 500.0,
                    height: VIEWPORT_H - 64.0,
                },
            },
        },
    ));

    // --- Line number gutter ---
    quads.push(quad(
        40,
        560.0,
        196.0,
        16.0,
        1140.0,
        solid_bg(0.0, 0.0, 0.5, 0.08),
        HslaMessage {
            h: 0.0,
            s: 0.0,
            l: 0.5,
            a: 0.05,
        },
        no_corners(),
        EdgesMessage {
            top: 0.0,
            right: 2.0,
            bottom: 0.0,
            left: 0.0,
        },
        0,
        full_mask(),
    ));

    // --- Current line highlight (animating vertical position) ---
    let cursor_line_y = 300.0 + ((time_secs * 0.3).sin() as f32 * 0.5 + 0.5) * 600.0;
    let editor_mask = ContentMaskMessage {
        bounds: BoundsMessage {
            origin: PointMessage { x: 502.0, y: 64.0 },
            size: SizeMessage {
                width: VIEWPORT_W - 502.0,
                height: VIEWPORT_H - 64.0 - 60.0,
            },
        },
    };

    quads.push(quad(
        50,
        576.0,
        cursor_line_y,
        1940.0,
        44.0,
        solid_bg(0.6167, 0.7, 0.5, 0.08),
        transparent(),
        no_corners(),
        no_border(),
        0,
        editor_mask,
    ));

    // --- Cursor bar ---
    let cursor_blink = ((time_secs * 2.0).sin() as f32 * 0.5 + 0.5).round();
    quads.push(quad(
        55,
        700.0,
        cursor_line_y,
        4.0,
        44.0,
        solid_bg(0.6167, 0.8, 0.65, cursor_blink),
        transparent(),
        uniform_corners(2.0),
        no_border(),
        0,
        editor_mask,
    ));

    // --- Floating dialog with shadow ---
    let dialog_x = 1700.0 + (time_secs * 0.4).cos() as f32 * 80.0;
    let dialog_y = 300.0 + (time_secs * 0.3).sin() as f32 * 40.0;
    let dialog_w = 500.0;
    let dialog_h = 320.0;

    shadows.push(ShadowMessage {
        order: 99,
        blur_radius: 32.0,
        bounds: BoundsMessage {
            origin: PointMessage {
                x: dialog_x,
                y: dialog_y,
            },
            size: SizeMessage {
                width: dialog_w,
                height: dialog_h,
            },
        },
        corner_radii: uniform_corners(16.0),
        content_mask: full_mask(),
        color: HslaMessage {
            h: 0.0,
            s: 0.0,
            l: 0.0,
            a: 0.6,
        },
    });

    // Dialog background
    quads.push(quad(
        100,
        dialog_x,
        dialog_y,
        dialog_w,
        dialog_h,
        solid_bg(bg_hue, 0.2, 0.18, 1.0),
        HslaMessage {
            h: 0.0,
            s: 0.0,
            l: 0.5,
            a: 0.3,
        },
        uniform_corners(16.0),
        EdgesMessage {
            top: 2.0,
            right: 2.0,
            bottom: 2.0,
            left: 2.0,
        },
        0,
        full_mask(),
    ));

    // Dialog gradient header
    let dialog_mask = ContentMaskMessage {
        bounds: BoundsMessage {
            origin: PointMessage {
                x: dialog_x,
                y: dialog_y,
            },
            size: SizeMessage {
                width: dialog_w,
                height: dialog_h,
            },
        },
    };

    quads.push(QuadMessage {
        order: 101,
        border_style: 0,
        bounds: BoundsMessage {
            origin: PointMessage {
                x: dialog_x + 20.0,
                y: dialog_y + 20.0,
            },
            size: SizeMessage {
                width: dialog_w - 40.0,
                height: 56.0,
            },
        },
        content_mask: dialog_mask,
        background: BackgroundMessage {
            tag: 1,
            color_space: 0,
            solid: transparent(),
            gradient_angle_or_pattern_height: 90.0,
            colors: [
                LinearColorStopMessage {
                    color: HslaMessage {
                        h: 0.6167,
                        s: 0.7,
                        l: 0.5,
                        a: 1.0,
                    },
                    percentage: 0.0,
                },
                LinearColorStopMessage {
                    color: HslaMessage {
                        h: 0.8,
                        s: 0.7,
                        l: 0.5,
                        a: 1.0,
                    },
                    percentage: 1.0,
                },
            ],
        },
        border_color: transparent(),
        corner_radii: uniform_corners(8.0),
        border_widths: no_border(),
    });

    // Dialog rows
    for i in 0..3 {
        let row_y = dialog_y + 96.0 + i as f32 * 64.0;
        let (bg, bc) = if i == 2 {
            (
                solid_bg(0.35, 0.6, 0.35, 0.2),
                HslaMessage {
                    h: 0.35,
                    s: 0.6,
                    l: 0.4,
                    a: 0.5,
                },
            )
        } else {
            (
                solid_bg(0.0, 0.0, 1.0, 0.05),
                HslaMessage {
                    h: 0.0,
                    s: 0.0,
                    l: 0.5,
                    a: 0.1,
                },
            )
        };
        quads.push(quad(
            102,
            dialog_x + 20.0,
            row_y,
            dialog_w - 40.0,
            44.0,
            bg,
            bc,
            uniform_corners(8.0),
            EdgesMessage {
                top: 2.0,
                right: 2.0,
                bottom: 2.0,
                left: 2.0,
            },
            0,
            dialog_mask,
        ));
    }

    // --- Wavy underline ---
    underlines.push(UnderlineMessage {
        order: 60,
        bounds: BoundsMessage {
            origin: PointMessage {
                x: 700.0,
                y: cursor_line_y + 60.0,
            },
            size: SizeMessage {
                width: 500.0,
                height: 12.0,
            },
        },
        content_mask: editor_mask,
        color: HslaMessage {
            h: 0.0,
            s: 0.85,
            l: 0.55,
            a: 1.0,
        },
        thickness: 4.0,
        wavy: 1,
    });

    // --- Straight underline ---
    underlines.push(UnderlineMessage {
        order: 60,
        bounds: BoundsMessage {
            origin: PointMessage {
                x: 700.0,
                y: cursor_line_y + 110.0,
            },
            size: SizeMessage {
                width: 500.0,
                height: 4.0,
            },
        },
        content_mask: editor_mask,
        color: HslaMessage {
            h: 0.6167,
            s: 0.7,
            l: 0.65,
            a: 1.0,
        },
        thickness: 4.0,
        wavy: 0,
    });

    // --- Multiple "file" entries in the sidebar ---
    let sidebar_mask = ContentMaskMessage {
        bounds: BoundsMessage {
            origin: PointMessage { x: 0.0, y: 64.0 },
            size: SizeMessage {
                width: 500.0,
                height: VIEWPORT_H - 64.0,
            },
        },
    };

    for i in 0..12 {
        let file_y = 100.0 + i as f32 * 56.0;
        if (file_y - selected_y).abs() < 28.0 {
            continue; // skip the selected one, it's already drawn
        }
        // Subtle hover effect on one item
        let alpha = if i == 3 { 0.06 } else { 0.0 };
        if alpha > 0.0 {
            quads.push(quad(
                9,
                20.0,
                file_y,
                460.0,
                48.0,
                solid_bg(0.0, 0.0, 1.0, alpha),
                transparent(),
                uniform_corners(6.0),
                no_border(),
                0,
                sidebar_mask,
            ));
        }
    }

    FrameMessage {
        frame_id,
        viewport_size: SizeMessage {
            width: VIEWPORT_W,
            height: VIEWPORT_H,
        },
        scale_factor: 2.0,
        atlas_deltas: vec![],
        scene: SceneBody {
            shadows,
            quads,
            paths: vec![],
            underlines,
            monochrome_sprites: vec![],
            subpixel_sprites: vec![],
            polychrome_sprites: vec![],
        },
    }
}

// ---------------------------------------------------------------------------
// WebSocket server
// ---------------------------------------------------------------------------

type Clients = Arc<Mutex<Vec<mpsc::UnboundedSender<Arc<String>>>>>;

async fn handle_connection(stream: TcpStream, addr: SocketAddr, clients: Clients) {
    let ws = match async_tungstenite::accept_async(stream).await {
        Ok(ws) => ws,
        Err(e) => {
            eprintln!("[{addr}] handshake failed: {e}");
            return;
        }
    };

    println!("[{addr}] connected");

    let (write, mut read) = ws.split();
    let (tx, mut rx) = mpsc::unbounded::<Arc<String>>();
    clients.lock().push(tx);

    // Write task
    let write_task = smol::spawn(async move {
        let mut sink = write;
        while let Some(json) = rx.next().await {
            let msg = Message::Text((*json).clone().into());
            if futures::SinkExt::send(&mut sink, msg).await.is_err() {
                break;
            }
        }
    });

    // Read task (drain input, log it)
    while let Some(Ok(msg)) = read.next().await {
        match msg {
            Message::Text(text) => {
                println!("[{addr}] input: {}", &text[..text.len().min(120)]);
            }
            Message::Close(_) => break,
            _ => {}
        }
    }

    write_task.cancel().await;
    println!("[{addr}] disconnected");
}

fn main() {
    let addr = "0.0.0.0:3101";

    smol::block_on(async {
        let listener = TcpListener::bind(addr).await.expect("Failed to bind");
        println!("Web streaming test server listening on ws://{addr}");
        println!("Open http://localhost:3100 in your browser (with npm run dev)");
        println!();

        let clients: Clients = Arc::new(Mutex::new(Vec::new()));

        // Spawn the accept loop
        let accept_clients = clients.clone();
        smol::spawn(async move {
            loop {
                if let Ok((stream, addr)) = listener.accept().await {
                    let c = accept_clients.clone();
                    smol::spawn(handle_connection(stream, addr, c)).detach();
                }
            }
        })
        .detach();

        // Frame broadcast loop targeting ~60fps
        let mut frame_id: u64 = 0;
        let start = std::time::Instant::now();

        loop {
            let time_secs = start.elapsed().as_secs_f64();
            let scene = build_scene(frame_id, time_secs);

            let json = serde_json::to_string(&scene).expect("serialize");
            let json = Arc::new(json);

            let mut lock = clients.lock();
            lock.retain(|tx| tx.unbounded_send(json.clone()).is_ok());
            let client_count = lock.len();
            drop(lock);

            if frame_id % 60 == 0 {
                let kb = json.len() as f64 / 1024.0;
                println!(
                    "frame {frame_id} | {client_count} client(s) | {kb:.1} KB/frame | t={time_secs:.1}s"
                );
            }

            frame_id += 1;
            Timer::after(Duration::from_millis(16)).await;
        }
    });
}
