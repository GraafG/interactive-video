//! Serves the web version (web/) to this computer only, so the browser opens it
//! from a web address. YouTube's player refuses pages opened straight from a file.
//! The address stays the same between runs, so lessons saved in the browser stay.

use std::io::{BufRead, BufReader, Write};
use std::net::{TcpListener, TcpStream};
use std::path::{Component, Path, PathBuf};
use std::sync::OnceLock;

const FIRST_PORT: u16 = 8765;

static PORT: OnceLock<u16> = OnceLock::new();

/// web/index.html beside the program, or up from where it was built, or here.
pub fn find_web_dir() -> Option<PathBuf> {
    let mut starts: Vec<PathBuf> = vec![];
    if let Ok(exe) = std::env::current_exe() {
        starts.extend(exe.ancestors().skip(1).take(4).map(|p| p.to_path_buf()));
    }
    if let Ok(dir) = std::env::current_dir() {
        starts.push(dir);
    }
    starts.into_iter().map(|d| d.join("web")).find(|d| d.join("index.html").is_file())
}

/// Start serving (once) and give the address of the web version.
pub fn url() -> Result<String, String> {
    if let Some(port) = PORT.get() {
        return Ok(format!("http://127.0.0.1:{port}/index.html"));
    }
    let dir = find_web_dir().ok_or("Couldn't find the web version (web/index.html)")?;
    let listener = (FIRST_PORT..FIRST_PORT + 20)
        .find_map(|p| TcpListener::bind(("127.0.0.1", p)).ok())
        .ok_or("No free local port for the web version")?;
    let port = listener.local_addr().map_err(|e| e.to_string())?.port();
    std::thread::spawn(move || {
        for stream in listener.incoming().flatten() {
            let dir = dir.clone();
            std::thread::spawn(move || {
                let _ = serve(stream, &dir);
            });
        }
    });
    let _ = PORT.set(port);
    Ok(format!("http://127.0.0.1:{port}/index.html"))
}

fn content_type(path: &Path) -> &'static str {
    match path.extension().and_then(|e| e.to_str()).unwrap_or("").to_ascii_lowercase().as_str() {
        "html" => "text/html; charset=utf-8",
        "js" => "text/javascript; charset=utf-8",
        "css" => "text/css; charset=utf-8",
        "json" => "application/json",
        "png" => "image/png",
        "jpg" | "jpeg" => "image/jpeg",
        "svg" => "image/svg+xml",
        "mp4" | "m4v" => "video/mp4",
        "webm" => "video/webm",
        _ => "application/octet-stream",
    }
}

fn decode(path: &str) -> String {
    let bytes = path.as_bytes();
    let mut out = vec![];
    let mut i = 0;
    while i < bytes.len() {
        if bytes[i] == b'%' && i + 2 < bytes.len() {
            if let Ok(b) = u8::from_str_radix(&path[i + 1..i + 3], 16) {
                out.push(b);
                i += 3;
                continue;
            }
        }
        out.push(bytes[i]);
        i += 1;
    }
    String::from_utf8_lossy(&out).into_owned()
}

fn serve(stream: TcpStream, dir: &Path) -> std::io::Result<()> {
    let mut reader = BufReader::new(stream.try_clone()?);
    let mut line = String::new();
    reader.read_line(&mut line)?;
    // Skip the headers.
    let mut header = String::new();
    while reader.read_line(&mut header)? > 2 {
        header.clear();
    }
    let mut out = stream;
    let mut parts = line.split_whitespace();
    let method = parts.next().unwrap_or("");
    let target = parts.next().unwrap_or("/");
    let path = decode(target.split(['?', '#']).next().unwrap_or("/"));
    let rel = path.trim_start_matches('/');
    let rel = if rel.is_empty() { "index.html" } else { rel };
    let safe = Path::new(rel).components().all(|c| matches!(c, Component::Normal(_)));
    let file = dir.join(rel);
    let body = if safe && (method == "GET" || method == "HEAD") { std::fs::read(&file).ok() } else { None };
    match body {
        Some(body) => {
            write!(
                out,
                "HTTP/1.1 200 OK\r\nContent-Type: {}\r\nContent-Length: {}\r\nCache-Control: no-cache\r\nConnection: close\r\n\r\n",
                content_type(&file),
                body.len()
            )?;
            if method == "GET" {
                out.write_all(&body)?;
            }
        }
        None => {
            let msg = b"Not found";
            write!(out, "HTTP/1.1 404 Not Found\r\nContent-Type: text/plain\r\nContent-Length: {}\r\nConnection: close\r\n\r\n", msg.len())?;
            out.write_all(msg)?;
        }
    }
    out.flush()
}
