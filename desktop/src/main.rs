#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

//! ClipQuest: make and play interactive videos — timed questions, hotspots,
//! branching scenes, chapters and captions — and see what viewers did.

mod app;
mod model;
mod player;
mod session;
mod video;
mod web_server;

fn main() -> eframe::Result {
    let args: Vec<String> = std::env::args().collect();
    // --serve-for N: only serve the web version for N seconds (no window), for checks.
    if let Some(secs) = args.iter().position(|a| a == "--serve-for").and_then(|i| args.get(i + 1)).and_then(|v| v.parse::<u64>().ok()) {
        match web_server::url() {
            Ok(url) => println!("{url}"),
            Err(e) => eprintln!("{e}"),
        }
        std::thread::sleep(std::time::Duration::from_secs(secs));
        return Ok(());
    }
    let shot = args
        .iter()
        .position(|a| a == "--screenshot")
        .and_then(|i| args.get(i + 1))
        .map(|p| app::Shot::new(std::path::PathBuf::from(p)));
    let quiet = shot.is_some();
    let value_of = |flag: &str| args.iter().position(|a| a == flag).and_then(|i| args.get(i + 1)).cloned();
    let example = value_of("--example").and_then(|v| v.parse::<usize>().ok());
    let tab = value_of("--tab");
    let video_file = value_of("--video");
    let open_web = value_of("--open-web");

    let mut viewport = egui::ViewportBuilder::default()
        .with_title("ClipQuest — interactive video")
        .with_inner_size([1360.0, 820.0])
        .with_min_inner_size([640.0, 480.0]);
    if quiet {
        viewport = viewport.with_active(false);
        #[cfg(windows)]
        {
            viewport = viewport.with_window_level(egui::WindowLevel::AlwaysOnBottom);
        }
    }
    #[allow(unused_mut)]
    let mut options = eframe::NativeOptions { viewport, ..Default::default() };
    #[cfg(target_os = "macos")]
    if quiet {
        options.event_loop_builder = Some(Box::new(|builder| {
            use eframe::egui_winit::winit::platform::macos::EventLoopBuilderExtMacOS;
            builder.with_activate_ignoring_other_apps(false);
        }));
    }

    eframe::run_native(
        "ClipQuest",
        options,
        Box::new(move |cc| {
            cc.egui_ctx.set_visuals(egui::Visuals::dark());
            player::apply_style(&cc.egui_ctx);
            // --open-web <options>: also open the web version (e.g. "example=3") in the browser.
            if let Some(opts) = &open_web {
                if let Ok(url) = web_server::url() {
                    cc.egui_ctx.open_url(egui::OpenUrl::new_tab(format!("{url}#{opts}")));
                }
            }
            Ok(Box::new(app::App::new(shot).with_start(example, tab.as_deref(), video_file)))
        }),
    )
}
