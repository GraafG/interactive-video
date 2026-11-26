//! Real video playback through the ffmpeg program, when it is installed.
//! Frames are decoded in a background thread and piped in as raw RGBA. Audio is
//! never decoded, so videos always play muted.

use std::io::Read;
use std::process::{Child, Command, Stdio};
use std::sync::mpsc::{sync_channel, Receiver};

pub const W: usize = 640;
pub const H: usize = 360;
pub const FPS: f32 = 25.0;

fn command(program: &str) -> Command {
    #[allow(unused_mut)]
    let mut c = Command::new(program);
    #[cfg(windows)]
    {
        use std::os::windows::process::CommandExt;
        const CREATE_NO_WINDOW: u32 = 0x0800_0000;
        c.creation_flags(CREATE_NO_WINDOW);
    }
    c
}

/// Whether ffmpeg can be started on this computer.
pub fn ffmpeg_available() -> bool {
    command("ffmpeg")
        .arg("-version")
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status()
        .map(|s| s.success())
        .unwrap_or(false)
}

/// The length of a video file in seconds, through ffprobe.
pub fn probe_duration(path: &str) -> Option<f32> {
    let out = command("ffprobe")
        .args(["-v", "error", "-show_entries", "format=duration", "-of", "csv=p=0", path])
        .output()
        .ok()?;
    String::from_utf8_lossy(&out.stdout).trim().parse::<f32>().ok()
}

pub struct Frame {
    pub time: f32,
    pub rgba: Vec<u8>,
}

/// One running decode, from a start time onwards.
pub struct Stream {
    pub path: String,
    child: Option<Child>,
    rx: Receiver<Frame>,
    pending: Option<Frame>,
    pub failed: bool,
}

impl Stream {
    pub fn open(path: &str, start: f32) -> Stream {
        let (tx, rx) = sync_channel::<Frame>(6);
        let filter = format!(
            "fps={FPS},scale={W}:{H}:force_original_aspect_ratio=decrease,pad={W}:{H}:(ow-iw)/2:(oh-ih)/2:black"
        );
        let child = command("ffmpeg")
            .args(["-hide_banner", "-loglevel", "error", "-ss", &format!("{start:.3}"), "-i", path])
            .args(["-an", "-sn", "-vf", &filter, "-f", "rawvideo", "-pix_fmt", "rgba", "-"])
            .stdin(Stdio::null())
            .stdout(Stdio::piped())
            .stderr(Stdio::null())
            .spawn();
        let mut failed = false;
        let child = match child {
            Ok(mut c) => {
                if let Some(mut out) = c.stdout.take() {
                    std::thread::spawn(move || {
                        let mut i = 0u32;
                        loop {
                            let mut buf = vec![0u8; W * H * 4];
                            if out.read_exact(&mut buf).is_err() {
                                break;
                            }
                            let frame = Frame { time: start + i as f32 / FPS, rgba: buf };
                            if tx.send(frame).is_err() {
                                break;
                            }
                            i += 1;
                        }
                    });
                }
                Some(c)
            }
            Err(_) => {
                failed = true;
                None
            }
        };
        Stream { path: path.to_string(), child, rx, pending: None, failed }
    }

    /// The newest frame due at `time`, if a new one is ready.
    pub fn frame_at(&mut self, time: f32) -> Option<Frame> {
        let mut latest = None;
        loop {
            let next = match self.pending.take() {
                Some(f) => Some(f),
                None => self.rx.try_recv().ok(),
            };
            match next {
                Some(f) if f.time <= time + 0.5 / FPS => latest = Some(f),
                Some(f) => {
                    self.pending = Some(f);
                    break;
                }
                None => break,
            }
        }
        latest
    }
}

impl Drop for Stream {
    fn drop(&mut self) {
        if let Some(mut c) = self.child.take() {
            let _ = c.kill();
            let _ = c.wait();
        }
    }
}
