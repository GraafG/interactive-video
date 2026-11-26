//! Playing a project: the picture (a real video or a built-in sample clip),
//! the timed interactions on top of it, branching between scenes, and the
//! transport bar with chapters and interaction markers.

use crate::model::{Jump, Kind, Project, Scene};
use crate::session::Session;
use crate::video;
use egui::{pos2, vec2, Align2, Color32, FontId, Rect, Sense, Shape, Stroke, StrokeKind, Ui};
use std::collections::HashSet;

pub fn fmt_time(t: f32) -> String {
    let t = t.max(0.0) as u32;
    format!("{}:{:02}", t / 60, t % 60)
}

static SCALE_BITS: std::sync::atomic::AtomicU32 = std::sync::atomic::AtomicU32::new(0x3FA0_0000); // 1.25

/// The text-size setting (1.0 = normal).
pub fn scale() -> f32 {
    f32::from_bits(SCALE_BITS.load(std::sync::atomic::Ordering::Relaxed))
}

pub fn set_scale(s: f32) {
    SCALE_BITS.store(s.to_bits(), std::sync::atomic::Ordering::Relaxed);
}

/// A size in points, grown with the text-size setting.
pub fn sz(px: f32) -> f32 {
    px * scale()
}

/// Bigger text, buttons, checkboxes and spacing, for small and touch screens.
pub fn apply_style(ctx: &egui::Context) {
    use egui::{FontFamily, TextStyle};
    let s = scale();
    ctx.all_styles_mut(|style| {
        style.text_styles = [
            (TextStyle::Small, FontId::new(11.0 * s, FontFamily::Proportional)),
            (TextStyle::Body, FontId::new(14.0 * s, FontFamily::Proportional)),
            (TextStyle::Button, FontId::new(14.0 * s, FontFamily::Proportional)),
            (TextStyle::Heading, FontId::new(20.0 * s, FontFamily::Proportional)),
            (TextStyle::Monospace, FontId::new(13.0 * s, FontFamily::Monospace)),
        ]
        .into();
        let sp = &mut style.spacing;
        sp.button_padding = vec2(8.0 * s, 4.0 * s);
        sp.item_spacing = vec2(8.0 * s, 6.0 * s);
        sp.interact_size = vec2(40.0 * s, 24.0 * s);
        sp.icon_width = 16.0 * s;
        sp.icon_width_inner = 9.0 * s;
        sp.icon_spacing = 6.0 * s;
        sp.slider_width = 120.0 * s;
        sp.combo_width = 100.0 * s;
        sp.text_edit_width = 240.0 * s;
        sp.scroll.bar_width = 8.0 * s;
    });
}

/// A play / pause button with a drawn icon.
pub fn play_button(ui: &mut Ui, playing: bool, enabled: bool) -> egui::Response {
    let k = scale() * 1.25;
    let (rect, resp) = ui.allocate_exact_size(vec2(36.0 * k, 28.0 * k), if enabled { Sense::click() } else { Sense::hover() });
    let fill = if !enabled {
        Color32::from_gray(50)
    } else if resp.hovered() {
        Color32::from_rgb(255, 215, 110)
    } else {
        Color32::from_rgb(255, 200, 70)
    };
    let p = ui.painter();
    p.rect_filled(rect, 6.0, fill);
    let c = rect.center();
    let ink = if enabled { Color32::from_gray(20) } else { Color32::from_gray(110) };
    if playing {
        for dx in [-4.0, 4.0] {
            p.rect_filled(Rect::from_center_size(c + vec2(dx * k, 0.0), vec2(4.0 * k, 13.0 * k)), 1.0, ink);
        }
    } else {
        p.add(Shape::convex_polygon(vec![c + vec2(-5.0, -7.0) * k, c + vec2(7.0, 0.0) * k, c + vec2(-5.0, 7.0) * k], ink, Stroke::NONE));
    }
    resp
}

/// Fit a 16:9 picture inside the given space.
pub fn video_rect(space: Rect) -> Rect {
    let w = space.width().min(space.height() * 16.0 / 9.0).max(16.0);
    Rect::from_center_size(space.center(), vec2(w, w * 9.0 / 16.0))
}

/// Shows a scene's picture at a time: decoded frames for a video file, or the
/// animated sample clip.
pub struct VideoView {
    stream: Option<video::Stream>,
    tex: Option<egui::TextureHandle>,
    shown: Option<(String, f32)>,
    last_time: f32,
}

impl VideoView {
    pub fn new() -> Self {
        VideoView { stream: None, tex: None, shown: None, last_time: 0.0 }
    }

    pub fn reset(&mut self) {
        self.stream = None;
        self.shown = None;
    }

    pub fn paint(&mut self, ui: &Ui, rect: Rect, scene: &Scene, time: f32, ffmpeg: bool) {
        let painter = ui.painter_at(rect);
        match (&scene.video, ffmpeg) {
            (Some(path), _) if path.contains("youtube.com/") || path.contains("youtu.be/") => {
                // YouTube can't be decoded here; it plays in the web version.
                self.stream = None;
                paint_sample(&painter, rect, scene, time);
                painter.text(
                    rect.left_top() + vec2(12.0, 12.0),
                    Align2::LEFT_TOP,
                    "YouTube video: it plays in the web version (File menu); showing a sample clip",
                    FontId::proportional(sz(13.0)),
                    Color32::from_white_alpha(220),
                );
            }
            (Some(path), true) if !path.is_empty() => {
                // A scene can be one part of a longer video: its time 0 is clip_start.
                let time = scene.clip_start + time;
                let restart = match &self.stream {
                    None => true,
                    Some(s) => s.path != *path || time < self.last_time - 0.05 || time > self.last_time + 1.0,
                };
                if restart {
                    self.stream = Some(video::Stream::open(path, time));
                }
                self.last_time = time;
                if let Some(frame) = self.stream.as_mut().and_then(|s| s.frame_at(time)) {
                    let img = egui::ColorImage::from_rgba_unmultiplied([video::W, video::H], &frame.rgba);
                    match &mut self.tex {
                        Some(t) => t.set(img, egui::TextureOptions::LINEAR),
                        None => self.tex = Some(ui.ctx().load_texture("video-frame", img, egui::TextureOptions::LINEAR)),
                    }
                    self.shown = Some((path.clone(), frame.time));
                }
                painter.rect_filled(rect, 0.0, Color32::BLACK);
                let have_frame = matches!(&self.shown, Some((p, _)) if p == path);
                if let (Some(tex), true) = (&self.tex, have_frame) {
                    painter.image(tex.id(), rect, Rect::from_min_max(pos2(0.0, 0.0), pos2(1.0, 1.0)), Color32::WHITE);
                } else {
                    let failed = self.stream.as_ref().map(|s| s.failed).unwrap_or(false);
                    painter.text(
                        rect.center(),
                        Align2::CENTER_CENTER,
                        if failed { "Couldn't open this video" } else { "Loading video…" },
                        FontId::proportional(sz(18.0)),
                        Color32::GRAY,
                    );
                }
                ui.ctx().request_repaint();
            }
            (Some(path), false) if !path.is_empty() => {
                paint_sample(&painter, rect, scene, time);
                painter.text(
                    rect.left_top() + vec2(12.0, 12.0),
                    Align2::LEFT_TOP,
                    "Install ffmpeg to play the video file; showing a sample clip",
                    FontId::proportional(sz(13.0)),
                    Color32::from_white_alpha(200),
                );
            }
            _ => {
                self.stream = None;
                paint_sample(&painter, rect, scene, time);
            }
        }
    }
}

const LOOK_COLOURS: [(Color32, Color32, Color32); 6] = [
    (Color32::from_rgb(20, 70, 120), Color32::from_rgb(40, 150, 190), Color32::from_rgb(160, 230, 255)),
    (Color32::from_rgb(120, 40, 80), Color32::from_rgb(240, 130, 70), Color32::from_rgb(255, 220, 140)),
    (Color32::from_rgb(20, 70, 40), Color32::from_rgb(70, 150, 80), Color32::from_rgb(200, 240, 160)),
    (Color32::from_rgb(12, 14, 40), Color32::from_rgb(50, 50, 110), Color32::from_rgb(170, 170, 255)),
    (Color32::from_rgb(60, 60, 66), Color32::from_rgb(150, 150, 160), Color32::from_rgb(250, 240, 220)),
    (Color32::from_rgb(130, 50, 140), Color32::from_rgb(240, 120, 170), Color32::from_rgb(255, 230, 120)),
];

/// The built-in moving picture used when a scene has no video file.
pub fn paint_sample(painter: &egui::Painter, rect: Rect, scene: &Scene, time: f32) {
    let (top, bottom, accent) = LOOK_COLOURS[scene.look % LOOK_COLOURS.len()];
    let mut mesh = egui::Mesh::default();
    mesh.colored_vertex(rect.left_top(), top);
    mesh.colored_vertex(rect.right_top(), top);
    mesh.colored_vertex(rect.left_bottom(), bottom);
    mesh.colored_vertex(rect.right_bottom(), bottom);
    mesh.add_triangle(0, 1, 2);
    mesh.add_triangle(1, 2, 3);
    painter.add(Shape::mesh(mesh));

    let w = rect.width();
    let h = rect.height();
    // Floating soft shapes.
    for i in 0..7 {
        let fi = i as f32;
        let x = rect.left() + w * (0.1 + 0.8 * (0.5 + 0.5 * (time * 0.13 * (1.0 + fi * 0.21) + fi * 1.7).sin()));
        let y = rect.top() + h * (0.15 + 0.55 * (0.5 + 0.5 * (time * 0.09 * (1.0 + fi * 0.17) + fi * 2.3).cos()));
        let r = h * (0.05 + 0.03 * (fi * 0.9).sin().abs());
        painter.circle_filled(pos2(x, y), r, accent.gamma_multiply(0.18 + 0.05 * (fi % 3.0)));
    }
    // A horizon with gentle hills.
    let mut hills = egui::Mesh::default();
    let shade = Color32::from_black_alpha(70);
    let steps = 40;
    for s in 0..=steps {
        let fx = s as f32 / steps as f32;
        let y = rect.top() + h * (0.78 + 0.04 * (fx * 9.0 + time * 0.4).sin() + 0.02 * (fx * 23.0 - time * 0.7).sin());
        hills.colored_vertex(pos2(rect.left() + fx * w, y), shade);
        hills.colored_vertex(pos2(rect.left() + fx * w, rect.bottom()), shade);
        if s > 0 {
            let i = (s * 2) as u32;
            hills.add_triangle(i - 2, i - 1, i);
            hills.add_triangle(i - 1, i, i + 1);
        }
    }
    painter.add(Shape::mesh(hills));

    // A slow progress sweep so time is visible.
    let frac = if scene.duration > 0.0 { (time / scene.duration).clamp(0.0, 1.0) } else { 0.0 };
    painter.circle_filled(pos2(rect.left() + w * (0.08 + 0.84 * frac), rect.top() + h * 0.3), h * 0.07, accent.gamma_multiply(0.85));

    if w < 200.0 {
        return;
    }
    painter.text(
        rect.center() - vec2(0.0, h * 0.06),
        Align2::CENTER_CENTER,
        &scene.name,
        FontId::proportional((h * 0.075).clamp(14.0, 40.0)),
        Color32::from_white_alpha(150),
    );
    painter.text(
        rect.right_top() + vec2(-10.0, 10.0),
        Align2::RIGHT_TOP,
        "sample clip",
        FontId::proportional(sz(12.0)),
        Color32::from_white_alpha(130),
    );
}

/// Where a viewing is: which scene, what time, what is on screen.
pub struct Player {
    pub scene: u32,
    pub time: f32,
    pub playing: bool,
    pub started: bool,
    pub finished: bool,
    pub blocking: Option<u32>,
    fired: HashSet<u32>,
    dismissed: HashSet<u32>,
    pub max_watched: f32,
    pick: Option<usize>,
    checked: Option<bool>,
    open_text: String,
    popup: Option<(String, String)>,
    pub speed: f32,
    pub captions: bool,
    pub view: VideoView,
    pub session: Session,
    pub learner: String,
    /// Set when the viewer asks to see their results.
    pub want_results: bool,
    /// Set when the viewer asks to watch again.
    pub want_restart: bool,
}

impl Player {
    pub fn new(project: &Project, learner: &str) -> Self {
        Player {
            scene: project.start_scene,
            time: 0.0,
            playing: false,
            started: false,
            finished: false,
            blocking: None,
            fired: HashSet::new(),
            dismissed: HashSet::new(),
            max_watched: 0.0,
            pick: None,
            checked: None,
            open_text: String::new(),
            popup: None,
            speed: 1.0,
            captions: true,
            view: VideoView::new(),
            session: Session::new(learner, &project.title, project.max_points(), project.pass_percent),
            learner: learner.to_string(),
            want_results: false,
            want_restart: false,
        }
    }

    fn scene_name(&self, project: &Project) -> String {
        project.scene_name(self.scene)
    }

    fn log(&mut self, project: &Project, verb: &str, object: &str, detail: &str, success: Option<bool>, points: u32) {
        let scene = self.scene_name(project);
        let t = self.time;
        self.session.log(&scene, t, verb, object, detail, success, points);
    }

    pub fn start(&mut self, project: &Project) {
        self.started = true;
        self.playing = true;
        self.session.learner = self.learner.clone();
        let name = self.scene_name(project);
        self.session.path.push(name.clone());
        self.log(project, "started", &project.title, "", None, 0);
        self.log(project, "entered scene", &name, "", None, 0);
    }

    fn reset_card(&mut self) {
        self.pick = None;
        self.checked = None;
        self.open_text.clear();
        self.popup = None;
    }

    /// Move within the current scene. Interactions before the new time count as passed.
    pub fn seek(&mut self, project: &Project, t: f32) {
        let Some(scene) = project.scene(self.scene) else { return };
        let mut t = t.clamp(0.0, scene.duration);
        if project.prevent_skipping {
            t = t.min(self.max_watched);
        }
        self.time = t;
        self.fired = scene.interactions.iter().filter(|i| i.time < t - 0.01).map(|i| i.id).collect();
        self.dismissed.clear();
        self.popup = None;
    }

    pub fn go(&mut self, project: &Project, jump: Jump) {
        self.blocking = None;
        self.reset_card();
        match jump {
            Jump::Continue => self.playing = true,
            Jump::Time(t) => {
                self.max_watched = self.max_watched.max(t);
                self.seek(project, t);
                self.playing = true;
                self.log(project, "jumped", &fmt_time(t), "", None, 0);
            }
            Jump::Scene(id) => {
                if project.scene(id).is_none() {
                    self.finish(project);
                    return;
                }
                self.scene = id;
                self.time = 0.0;
                self.max_watched = 0.0;
                self.fired.clear();
                self.dismissed.clear();
                self.view.reset();
                self.playing = true;
                let name = self.scene_name(project);
                self.session.path.push(name.clone());
                self.log(project, "entered scene", &name, "", None, 0);
            }
            Jump::Finish => self.finish(project),
        }
    }

    fn finish(&mut self, project: &Project) {
        self.finished = true;
        self.playing = false;
        let detail = format!("score {}/{} ({}%)", self.session.score, self.session.max_score, self.session.percent());
        let passed = self.session.passed();
        self.log(project, "completed", &project.title, &detail, Some(passed), self.session.score);
        self.session.finish();
    }

    /// Advance the clock and set off interactions whose time has come.
    pub fn tick(&mut self, project: &Project, dt: f32) {
        if !self.started || self.finished || self.blocking.is_some() || !self.playing {
            return;
        }
        let Some(scene) = project.scene(self.scene) else { return };
        let new_t = (self.time + dt * self.speed).min(scene.duration);
        for i in &scene.interactions {
            if self.fired.contains(&i.id) || i.time > new_t {
                continue;
            }
            self.fired.insert(i.id);
            self.time = i.time.max(self.time);
            match &i.kind {
                Kind::ScoreCheck { min_percent, pass, fail } => {
                    let ok = self.session.percent() >= *min_percent;
                    let jump = if ok { pass.clone() } else { fail.clone() };
                    self.log(project, "score checked", &format!("at least {min_percent}%"), &format!("{}%", self.session.percent()), Some(ok), 0);
                    self.go(project, jump);
                    return;
                }
                k if k.always_blocks() || i.pause => {
                    self.blocking = Some(i.id);
                    self.playing = false;
                    self.reset_card();
                    let summary = k.summary();
                    self.log(project, "reached", &summary, k.type_name(), None, 0);
                    return;
                }
                _ => {}
            }
        }
        self.time = new_t;
        self.max_watched = self.max_watched.max(new_t);
        if new_t >= scene.duration {
            let name = scene.name.clone();
            let end = scene.at_end.clone();
            self.log(project, "watched to the end", &name, "", None, 0);
            let end = if end == Jump::Continue { Jump::Finish } else { end };
            self.go(project, end);
        }
    }

    /// Draw the picture and everything on top of it.
    pub fn show(&mut self, ui: &mut Ui, project: &Project, ffmpeg: bool, space: Rect) {
        let rect = video_rect(space);
        ui.painter().rect_filled(space, 0.0, Color32::from_rgb(10, 10, 14));
        let Some(scene) = project.scene(self.scene).cloned() else {
            ui.painter().text(space.center(), Align2::CENTER_CENTER, "This lesson has no scenes yet", FontId::proportional(sz(18.0)), Color32::GRAY);
            return;
        };
        self.view.paint(ui, rect, &scene, self.time, ffmpeg);
        let now = ui.input(|i| i.time) as f32;
        // Click on the picture toggles play.
        let bg = ui.interact(rect, ui.scope_id().with("video-bg"), Sense::click());
        if bg.clicked() && self.started && !self.finished && self.blocking.is_none() {
            self.playing = !self.playing;
        }

        // Captions.
        if self.captions {
            if let Some(c) = scene.captions.iter().find(|c| self.time >= c.start && self.time <= c.end) {
                let font = FontId::proportional((rect.height() * 0.045).clamp(13.0, 24.0));
                let galley = ui.painter().layout(c.text.clone(), font, Color32::WHITE, rect.width() * 0.8);
                let size = galley.size() + vec2(20.0, 10.0);
                let r = Rect::from_center_size(pos2(rect.center().x, rect.bottom() - size.y / 2.0 - 14.0), size);
                ui.painter().rect_filled(r, 6.0, Color32::from_black_alpha(170));
                ui.painter().galley(r.min + vec2(10.0, 5.0), galley, Color32::WHITE);
            }
        }

        // Cards, hotspots and links that are visible now.
        let mut actions: Vec<(String, String, Jump, u32)> = vec![];
        let mut card_y = rect.top() + 12.0;
        for i in &scene.interactions {
            let visible_window = self.time >= i.time && self.time <= i.time + i.show_for;
            let shown = (visible_window || self.blocking == Some(i.id)) && !self.dismissed.contains(&i.id) && self.started;
            if !shown {
                continue;
            }
            match &i.kind {
                Kind::Info { title, body } => {
                    let w = (rect.width() * 0.36).max(200.0);
                    let card = Rect::from_min_size(pos2(rect.left() + 12.0, card_y), vec2(w, 10.0));
                    let mut close = false;
                    let resp = ui.scope_builder(egui::UiBuilder::new().max_rect(card.with_max_y(rect.bottom())), |ui| {
                        egui::Frame::new()
                            .fill(Color32::from_rgba_unmultiplied(15, 20, 35, 225))
                            .stroke(Stroke::new(1.5, i.kind.colour()))
                            .corner_radius(8.0)
                            .inner_margin(10.0)
                            .show(ui, |ui| {
                                ui.set_width(w - 20.0);
                                ui.horizontal(|ui| {
                                    ui.label(egui::RichText::new(title).strong().color(Color32::WHITE).size(sz(15.0)));
                                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                                        if ui.small_button("✕").on_hover_text("Close").clicked() {
                                            close = true;
                                        }
                                    });
                                });
                                ui.label(egui::RichText::new(body).color(Color32::from_gray(220)));
                            });
                    });
                    card_y = resp.response.rect.bottom() + 8.0;
                    if close {
                        self.dismissed.insert(i.id);
                        self.log(project, "closed", title, "", None, 0);
                    }
                }
                Kind::Hotspot { rect: r, label, info, jump } => {
                    let hr = Rect::from_min_size(
                        pos2(rect.left() + r[0] * rect.width(), rect.top() + r[1] * rect.height()),
                        vec2(r[2] * rect.width(), r[3] * rect.height()),
                    );
                    let pulse = 0.5 + 0.5 * (now * 3.0).sin();
                    let resp = ui.interact(hr, ui.scope_id().with(("hotspot", i.id)), Sense::click());
                    let col = i.kind.colour();
                    let fill = if resp.hovered() { col.gamma_multiply(0.35) } else { col.gamma_multiply(0.12 + 0.1 * pulse) };
                    ui.painter().rect_filled(hr, 10.0, fill);
                    ui.painter().rect_stroke(hr, 10.0, Stroke::new(2.0 + pulse, col), StrokeKind::Inside);
                    let chip = ui.painter().layout_no_wrap(label.clone(), FontId::proportional(sz(14.0)), Color32::BLACK);
                    let cr = Rect::from_min_size(hr.left_top() + vec2(6.0, 6.0), chip.size() + vec2(12.0, 6.0));
                    ui.painter().rect_filled(cr, 10.0, col);
                    ui.painter().galley(cr.min + vec2(6.0, 3.0), chip, Color32::BLACK);
                    if resp.on_hover_cursor(egui::CursorIcon::PointingHand).clicked() {
                        actions.push((label.clone(), info.clone(), jump.clone(), i.id));
                    }
                }
                Kind::Link { label, url } => {
                    let br = Rect::from_min_size(rect.right_bottom() - vec2(sz(170.0), sz(96.0)), vec2(sz(150.0), sz(36.0)));
                    let resp = ui.put(br, egui::Button::new(egui::RichText::new(format!("{label}  ↗")).size(sz(15.0)).color(Color32::BLACK)).fill(i.kind.colour()).corner_radius(18.0));
                    if resp.clicked() {
                        ui.ctx().open_url(egui::OpenUrl::new_tab(url));
                        self.log(project, "opened link", label, url, None, 0);
                    }
                }
                _ => {}
            }
        }
        for (label, info, jump, _id) in actions {
            self.log(project, "clicked hotspot", &label, "", None, 0);
            if !info.is_empty() {
                self.popup = Some((label, info));
            }
            if jump != Jump::Continue {
                self.go(project, jump);
            }
        }
        if let Some((title, body)) = self.popup.clone() {
            let w = (rect.width() * 0.4).max(220.0);
            let r = Rect::from_min_size(pos2(rect.right() - w - 12.0, rect.top() + 12.0), vec2(w, 10.0));
            ui.scope_builder(egui::UiBuilder::new().max_rect(r.with_max_y(rect.bottom())), |ui| {
                egui::Frame::new().fill(Color32::from_rgba_unmultiplied(250, 250, 245, 240)).corner_radius(8.0).inner_margin(10.0).show(ui, |ui| {
                    ui.set_width(w - 20.0);
                    ui.horizontal(|ui| {
                        ui.label(egui::RichText::new(&title).strong().color(Color32::BLACK).size(sz(15.0)));
                        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                            if ui.small_button("✕").clicked() {
                                self.popup = None;
                            }
                        });
                    });
                    ui.label(egui::RichText::new(&body).color(Color32::from_gray(40)));
                });
            });
        }

        // A blocking interaction: dim the picture and show its card.
        if let Some(id) = self.blocking {
            if let Some(inter) = scene.interactions.iter().find(|i| i.id == id).cloned() {
                match &inter.kind {
                    Kind::Hotspot { .. } | Kind::Info { .. } | Kind::Link { .. } => {
                        let br = Rect::from_center_size(pos2(rect.center().x, rect.bottom() - 70.0), vec2(sz(150.0), sz(36.0)));
                        if ui.put(br, egui::Button::new(egui::RichText::new("Continue  →").size(sz(15.0)))).clicked() {
                            self.go(project, Jump::Continue);
                        }
                    }
                    _ => {
                        ui.painter().rect_filled(rect, 0.0, Color32::from_black_alpha(150));
                        self.blocking_card(ui, project, rect, &inter);
                    }
                }
            }
        }

        if !self.started {
            ui.painter().rect_filled(rect, 0.0, Color32::from_black_alpha(140));
            let card = Rect::from_center_size(rect.center(), vec2((rect.width() * 0.6).max(260.0), 10.0));
            ui.scope_builder(egui::UiBuilder::new().max_rect(card.with_max_y(rect.bottom())), |ui| {
                ui.vertical_centered(|ui| {
                    ui.label(egui::RichText::new(&project.title).size(sz(26.0)).strong().color(Color32::WHITE));
                    if !project.description.is_empty() {
                        ui.label(egui::RichText::new(&project.description).color(Color32::from_gray(220)));
                    }
                    ui.add_space(12.0);
                    let b = egui::Button::new(egui::RichText::new("Start").size(sz(20.0)).color(Color32::BLACK))
                        .fill(Color32::from_rgb(255, 200, 70))
                        .corner_radius(22.0)
                        .min_size(vec2(sz(150.0), sz(44.0)));
                    if ui.add(b).clicked() {
                        self.start(project);
                    }
                    ui.add_space(6.0);
                    ui.label(egui::RichText::new("Videos play muted.").small().color(Color32::from_gray(180)));
                });
            });
        }

        if self.finished {
            ui.painter().rect_filled(rect, 0.0, Color32::from_black_alpha(170));
            let card = Rect::from_center_size(rect.center(), vec2((rect.width() * 0.55).max(280.0), 10.0));
            let s = &self.session;
            let (score, max, pct, passed, answered, path) = (s.score, s.max_score, s.percent(), s.passed(), s.questions_answered(), s.path.join("  →  "));
            ui.scope_builder(egui::UiBuilder::new().max_rect(card.with_max_y(rect.bottom())), |ui| {
                egui::Frame::new().fill(Color32::from_rgb(28, 32, 44)).corner_radius(12.0).inner_margin(18.0).show(ui, |ui| {
                    ui.vertical_centered(|ui| {
                        ui.label(egui::RichText::new("Finished!").size(sz(26.0)).strong().color(Color32::WHITE));
                        if max > 0 {
                            ui.label(egui::RichText::new(format!("Score {score} / {max}  ({pct}%)")).size(sz(18.0)).color(Color32::WHITE));
                            let (txt, col) = if passed { ("Passed", Color32::from_rgb(110, 220, 130)) } else { ("Not passed yet", Color32::from_rgb(255, 140, 120)) };
                            ui.label(egui::RichText::new(txt).size(sz(16.0)).color(col));
                        }
                        ui.label(egui::RichText::new(format!("{answered} question(s) answered")).color(Color32::from_gray(200)));
                        ui.label(egui::RichText::new(format!("Your path: {path}")).small().color(Color32::from_gray(180)));
                        ui.add_space(10.0);
                        ui.horizontal(|ui| {
                            if ui.button("Watch again").clicked() {
                                self.want_restart = true;
                            }
                            if ui.button("See results").clicked() {
                                self.want_results = true;
                            }
                        });
                    });
                });
            });
        }
    }

    fn blocking_card(&mut self, ui: &mut Ui, project: &Project, rect: Rect, inter: &crate::model::Interaction) {
        let w = (rect.width() * 0.62).clamp(280.0, 640.0);
        let card = Rect::from_center_size(rect.center() - vec2(0.0, rect.height() * 0.12), vec2(w, 10.0));
        let col = inter.kind.colour();
        let mut next: Option<Jump> = None;
        ui.scope_builder(egui::UiBuilder::new().max_rect(card.with_max_y(rect.bottom() - 8.0)), |ui| {
            egui::Frame::new()
                .fill(Color32::from_rgb(250, 250, 246))
                .stroke(Stroke::new(3.0, col))
                .corner_radius(12.0)
                .inner_margin(16.0)
                .show(ui, |ui| {
                    ui.set_width(w - 32.0);
                    let dark = Color32::from_gray(25);
                    ui.style_mut().visuals = egui::Visuals::light();
                    ui.label(egui::RichText::new(inter.kind.type_name().to_uppercase()).small().strong().color(col.gamma_multiply(0.8)));
                    match &inter.kind {
                        Kind::Question { prompt, options, points, allow_retry, on_correct, on_wrong } => {
                            ui.label(egui::RichText::new(prompt).size(sz(18.0)).strong().color(dark));
                            ui.add_space(6.0);
                            for (n, o) in options.iter().enumerate() {
                                let selected = self.pick == Some(n);
                                let mut text = egui::RichText::new(&o.text).size(sz(16.0)).color(dark);
                                if self.checked.is_some() && o.correct {
                                    text = text.color(Color32::from_rgb(20, 130, 50)).strong();
                                }
                                let r = ui.add_enabled(self.checked.is_none(), egui::RadioButton::new(selected, text));
                                if r.clicked() {
                                    self.pick = Some(n);
                                }
                            }
                            ui.add_space(8.0);
                            match self.checked {
                                None => {
                                    if ui.add_enabled(self.pick.is_some(), egui::Button::new(egui::RichText::new("Check answer").size(sz(15.0)))).clicked() {
                                        let n = self.pick.unwrap_or(0);
                                        let ok = options.get(n).map(|o| o.correct).unwrap_or(false);
                                        self.checked = Some(ok);
                                        let first = self.session.answered.insert(inter.id);
                                        let gained = if ok && first { *points } else { 0 };
                                        self.session.score += gained;
                                        let answer = options.get(n).map(|o| o.text.clone()).unwrap_or_default();
                                        self.log(project, "answered", prompt, &answer, Some(ok), gained);
                                    }
                                }
                                Some(ok) => {
                                    let fb = self.pick.and_then(|n| options.get(n)).map(|o| o.feedback.clone()).unwrap_or_default();
                                    let (head, c) = if ok { ("Correct!", Color32::from_rgb(20, 130, 50)) } else { ("Not quite.", Color32::from_rgb(190, 50, 40)) };
                                    ui.label(egui::RichText::new(head).size(sz(16.0)).strong().color(c));
                                    if !fb.is_empty() {
                                        ui.label(egui::RichText::new(fb).color(dark));
                                    }
                                    ui.horizontal(|ui| {
                                        if !ok && *allow_retry && ui.button("Try again").clicked() {
                                            self.checked = None;
                                            self.pick = None;
                                        }
                                        if ui.button(egui::RichText::new("Continue  →").size(sz(15.0))).clicked() {
                                            next = Some(if ok { on_correct.clone() } else { on_wrong.clone() });
                                        }
                                    });
                                }
                            }
                        }
                        Kind::OpenQuestion { prompt } => {
                            ui.label(egui::RichText::new(prompt).size(sz(18.0)).strong().color(dark));
                            ui.add(egui::TextEdit::multiline(&mut self.open_text).desired_rows(3).desired_width(f32::INFINITY).hint_text("Type your answer…"));
                            ui.horizontal(|ui| {
                                if ui.add_enabled(!self.open_text.trim().is_empty(), egui::Button::new("Submit")).clicked() {
                                    let answer = self.open_text.trim().to_string();
                                    self.session.answered.insert(inter.id);
                                    self.log(project, "answered", prompt, &answer, None, 0);
                                    next = Some(Jump::Continue);
                                }
                                if ui.button("Skip").clicked() {
                                    self.log(project, "skipped", prompt, "", None, 0);
                                    next = Some(Jump::Continue);
                                }
                            });
                        }
                        Kind::Choice { prompt, options } => {
                            ui.label(egui::RichText::new(prompt).size(sz(18.0)).strong().color(dark));
                            ui.add_space(6.0);
                            for o in options {
                                let b = egui::Button::new(egui::RichText::new(&o.text).size(sz(16.0)).color(Color32::WHITE))
                                    .fill(col.gamma_multiply(0.9))
                                    .corner_radius(8.0)
                                    .min_size(vec2(w - 32.0, sz(36.0)));
                                if ui.add(b).clicked() {
                                    self.log(project, "chose", prompt, &o.text, None, 0);
                                    next = Some(o.jump.clone());
                                }
                            }
                        }
                        Kind::PauseScreen { text } => {
                            ui.label(egui::RichText::new(text).size(sz(18.0)).color(dark));
                            ui.add_space(6.0);
                            if ui.button(egui::RichText::new("Continue  →").size(sz(15.0))).clicked() {
                                next = Some(Jump::Continue);
                            }
                        }
                        _ => {
                            if ui.button("Continue").clicked() {
                                next = Some(Jump::Continue);
                            }
                        }
                    }
                });
        });
        if let Some(j) = next {
            self.go(project, j);
        }
    }

    /// Play button, time, the seek bar with markers, speed and captions.
    pub fn transport(&mut self, ui: &mut Ui, project: &Project) {
        let Some(scene) = project.scene(self.scene).cloned() else { return };
        ui.horizontal(|ui| {
            let can_play = self.started && !self.finished && self.blocking.is_none();
            if play_button(ui, self.playing, can_play || !self.started).on_hover_text("Play / pause (Space)").clicked() {
                if !self.started {
                    self.start(project);
                } else {
                    self.playing = !self.playing;
                }
            }
            if ui.button("⟲").on_hover_text("Back 10 seconds").clicked() && self.blocking.is_none() {
                let t = self.time - 10.0;
                self.seek(project, t);
            }
            ui.label(egui::RichText::new(format!("{} / {}", fmt_time(self.time), fmt_time(scene.duration))).monospace());

            let bar_w = (ui.available_width() - sz(240.0)).max(120.0);
            let (bar, resp) = ui.allocate_exact_size(vec2(bar_w, sz(30.0)), Sense::click_and_drag());
            let p = ui.painter();
            let track = Rect::from_center_size(bar.center(), vec2(bar.width(), sz(7.0)));
            p.rect_filled(track, 3.0, Color32::from_gray(60));
            let x_of = |t: f32| track.left() + track.width() * (t / scene.duration.max(0.01)).clamp(0.0, 1.0);
            if project.prevent_skipping {
                p.rect_filled(Rect::from_min_max(track.min, pos2(x_of(self.max_watched), track.max.y)), 3.0, Color32::from_gray(95));
            }
            p.rect_filled(Rect::from_min_max(track.min, pos2(x_of(self.time), track.max.y)), 3.0, Color32::from_rgb(255, 200, 70));
            for c in &scene.chapters {
                let x = x_of(c.time);
                p.line_segment([pos2(x, track.top() - 5.0), pos2(x, track.bottom() + 5.0)], Stroke::new(1.5, Color32::from_gray(200)));
            }
            for i in &scene.interactions {
                p.circle_filled(pos2(x_of(i.time), track.center().y), sz(6.0), i.kind.colour());
            }
            p.circle_filled(pos2(x_of(self.time), track.center().y), sz(8.0), Color32::WHITE);
            if let Some(pos) = resp.hover_pos() {
                let t = (pos.x - track.left()) / track.width() * scene.duration;
                let tip = scene
                    .interactions
                    .iter()
                    .find(|i| (x_of(i.time) - pos.x).abs() < 6.0)
                    .map(|i| format!("{} · {}", i.kind.type_name(), i.kind.summary()))
                    .or_else(|| scene.chapters.iter().rev().find(|c| c.time <= t).map(|c| c.title.clone()))
                    .unwrap_or_default();
                resp.clone().on_hover_text(format!("{}  {}", fmt_time(t), tip));
            }
            if (resp.clicked() || resp.dragged()) && self.started && self.blocking.is_none() && !self.finished {
                if let Some(pos) = resp.interact_pointer_pos() {
                    let t = (pos.x - track.left()) / track.width() * scene.duration;
                    self.seek(project, t);
                }
            }

            egui::ComboBox::from_id_salt("speed")
                .width(60.0)
                .selected_text(format!("{}×", self.speed))
                .show_ui(ui, |ui| {
                    for s in [0.5, 0.75, 1.0, 1.5, 2.0] {
                        ui.selectable_value(&mut self.speed, s, format!("{s}×"));
                    }
                });
            ui.toggle_value(&mut self.captions, "CC").on_hover_text("Captions");
        });
    }
}
