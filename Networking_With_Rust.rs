// use std::net::TcpListener;
// use std::io::{Read, Write};

// fn main() -> std::io::Result<()> {
//     let listener = TcpListener::bind("127.0.0.1:7878")?;
//     println!("☕ Server started on port 7878");

//     for stream in listener.incoming() {
//         let mut stream = stream?;
//         let mut buffer = [0; 512];
//         stream.read(&mut buffer)?;
//         println!("📝 Received: {}", String::from_utf8_lossy(&buffer[..]));
//         stream.write(b"HTTP/1.1 200 OK\r\n\r\nWelcome to Rust Café!")?;
//     }

//     Ok(())
// }
// src/main.rs
use warp::Filter;
use warp::ws::{Message, WebSocket, Ws};
use futures_util::{StreamExt, SinkExt};
use std::convert::Infallible;

#[tokio::main]
async fn main() {
    let html = warp::path::end()
        .map(|| warp::reply::html(INDEX_HTML));

    let ws_route = warp::path("ws")
        .and(warp::ws())
        .and_then(handle_ws);

    let routes = html.or(ws_route);

    println!("📡 Rust Chat Café is open at http://127.0.0.1:8080");
    warp::serve(routes).run(([127, 0, 0, 1], 8080)).await;
}

async fn handle_ws(ws: Ws) -> Result<impl warp::Reply, Infallible> {
    Ok(ws.on_upgrade(handle_socket))
}

async fn handle_socket(socket: WebSocket) {
    let (mut tx, mut rx) = socket.split();
    println!("👋 Customer entered!");

    while let Some(result) = rx.next().await {
        if let Ok(msg) = result {
            if msg.is_text() {
                let text = msg.to_str().unwrap();
                println!("🧑 Customer says: {}", text);
                let reply = format!("☕ Here’s your {}!", text);
                tx.send(Message::text(reply)).await.unwrap();
            }
        }
    }

    println!("👋 Customer left.");
}

const INDEX_HTML: &str = r#"<!DOCTYPE html>
<html lang="en">
<head>
  <meta charset="UTF-8" />
  <title>Rust Chat Café ☕📡</title>
  <style>
    body { font-family: sans-serif; max-width: 600px; margin: auto; padding: 2rem; background: #fff7fc; }
    h1 { color: #6b2d5c; }
    #messages { border: 1px solid #ccc; padding: 1rem; height: 200px; overflow-y: auto; margin-bottom: 1rem; background: white; }
    input, button { padding: 0.5rem; font-size: 1rem; }
    button { background: #6b2d5c; color: white; border: none; }
  </style>
</head>
<body>
  <h1>🕸️ Rust Chat Café ☕📡</h1>
  <div id="messages">Connecting to the café...</div>
  <input type="text" id="orderInput" placeholder="Type your order (e.g., Latte 😄)" />
  <button onclick="sendOrder()">Order ☕</button>

  <script>
    const ws = new WebSocket("ws://" + location.host + "/ws");
    const messages = document.getElementById("messages");

    ws.onopen = () => addMsg("✅ Connected to Rust Café");
    ws.onmessage = (e) => addMsg("📩 Café: " + e.data);
    ws.onerror = () => addMsg("❌ Connection error");

    function addMsg(msg) {
      messages.innerHTML += "<div>" + msg + "</div>";
      messages.scrollTop = messages.scrollHeight;
    }

    function sendOrder() {
      const input = document.getElementById("orderInput");
      if (ws.readyState === WebSocket.OPEN && input.value.trim()) {
        ws.send(input.value);
        addMsg("🧑 You: " + input.value);
        input.value = "";
      }
    }
  </script>
</body>
</html>"#;
