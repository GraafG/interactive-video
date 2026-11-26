//! The main window: Watch (play the lesson), Edit (scenes, timeline and
//! interactions), Story map (how scenes connect) and Results (what viewers did).

use crate::model::{self, AnswerOption, Caption, Chapter, ChoiceOption, Jump, Kind, Project, LOOKS};
use crate::player::{fmt_time, paint_sample, sz, video_rect, Player, VideoView};
use crate::session::{self, Session};
use crate::video;
use egui::{pos2, vec2, Align2, Color32, FontId, Key, Rect, RichText, Sense, Stroke, StrokeKind, Ui};
use std::path::PathBuf;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

#[derive(Clone, Copy, PartialEq)]
enum Tab {
    Watch,
    Edit,
    Map,
    Results,
}

pub struct Shot {
    pub path: PathBuf,
    frames: u32,
    requested: bool,
    done: Arc<AtomicBool>,
}

impl Shot {
    pub fn new(path: PathBuf) -> Self {
        Shot { path, frames: 0, requested: false, done: Arc::new(AtomicBool::new(false)) }
    }
}

pub struct App {
    project: Project,
    file: Option<PathBuf>,
    dirty: bool,
    tab: Tab,
    player: Player,
    ffmpeg: bool,
    history: Vec<Session>,
    selected_session: usize,
    learner: String,
    // editor
    sel_scene: u32,
    sel_item: Option<u32>,
    edit_time: f32,
    edit_playing: bool,
    edit_view: VideoView,
    // story map
    map_offset: egui::Vec2,
    status: String,
    shot: Option<Shot>,
    /// The Watch tab's side panel; hidden on its own when the window gets narrow.
    show_side: bool,
    last_width: f32,
}

impl App {
    pub fn new(shot: Option<Shot>) -> Self {
        let project = model::example(0);
        let learner = "Guest".to_string();
        let mut player = Player::new(&project, &learner);
        if shot.is_some() {
            // Show a lesson mid-way: the first question waiting for an answer.
            player.start(&project);
            player.max_watched = 15.8;
            player.seek(&project, 15.8);
        }
        let ffmpeg = video::ffmpeg_available();
        App {
            sel_scene: project.start_scene,
            project,
            file: None,
            dirty: false,
            tab: Tab::Watch,
            player,
            ffmpeg,
            history: vec![],
            selected_session: 0,
            learner,
            sel_item: None,
            edit_time: 0.0,
            edit_playing: false,
            edit_view: VideoView::new(),
            map_offset: vec2(20.0, 20.0),
            status: String::new(),
            show_side: true,
            last_width: 0.0,
            shot,
        }
    }

    /// Open on a given example and tab (`--example N`, `--tab edit|map|results`).
    pub fn with_start(mut self, example: Option<usize>, tab: Option<&str>, video_file: Option<String>) -> Self {
        if example.is_some() || video_file.is_some() {
            let shot = self.shot.is_some();
            let mut p = model::example(example.unwrap_or(0));
            if let Some(v) = video_file {
                let start = p.start_scene;
                if let Some(s) = p.scene_mut(start) {
                    if let Some(d) = video::probe_duration(&v) {
                        s.duration = d;
                    }
                    s.video = Some(v);
                }
            }
            self.load_project(p, None);
            if shot {
                self.player.start(&self.project);
                if example.is_none() {
                    self.player.max_watched = 2.0;
                    self.player.seek(&self.project, 2.0);
                }
            }
        }
        self.tab = match tab {
            Some("edit") => Tab::Edit,
            Some("map") => Tab::Map,
            Some("results") => Tab::Results,
            _ => self.tab,
        };
        if self.tab == Tab::Edit {
            if let Some(i) = self.project.scene(self.sel_scene).and_then(|s| s.interactions.get(1)).map(|i| (i.id, i.time)) {
                self.sel_item = Some(i.0);
                self.edit_time = i.1;
            }
        }
        self
    }

    fn archive_session(&mut self) {
        let fresh = Session::new(&self.learner, &self.project.title, self.project.max_points(), self.project.pass_percent);
        let old = std::mem::replace(&mut self.player.session, fresh);
        if old.has_activity() {
            self.history.push(old);
        }
    }

    fn restart_player(&mut self) {
        self.archive_session();
        self.player = Player::new(&self.project, &self.learner);
    }

    fn load_project(&mut self, p: Project, file: Option<PathBuf>) {
        self.archive_session();
        self.sel_scene = p.start_scene;
        self.project = p;
        self.file = file;
        self.dirty = false;
        self.sel_item = None;
        self.edit_time = 0.0;
        self.edit_view.reset();
        self.player = Player::new(&self.project, &self.learner);
    }

    fn save(&mut self, as_new: bool) {
        let path = if as_new || self.file.is_none() {
            rfd::FileDialog::new()
                .add_filter("Interactive video lesson", &["json"])
                .set_file_name(format!("{}.json", self.project.title))
                .save_file()
        } else {
            self.file.clone()
        };
        if let Some(path) = path {
            match std::fs::write(&path, self.project.to_json()) {
                Ok(()) => {
                    self.status = format!("Saved {}", path.file_name().map(|n| n.to_string_lossy().to_string()).unwrap_or_default());
                    self.file = Some(path);
                    self.dirty = false;
                }
                Err(e) => self.status = format!("Couldn't save: {e}"),
            }
        }
    }

    fn open(&mut self) {
        if let Some(path) = rfd::FileDialog::new().add_filter("Interactive video lesson", &["json"]).pick_file() {
            match std::fs::read_to_string(&path).map_err(|e| e.to_string()).and_then(|t| Project::from_json(&t)) {
                Ok(p) => {
                    self.load_project(p, Some(path));
                    self.status = "Lesson opened".into();
                }
                Err(e) => self.status = format!("Couldn't open: {e}"),
            }
        }
    }

    fn handle_screenshot(&mut self, ctx: &egui::Context) {
        let Some(shot) = &mut self.shot else { return };
        shot.frames += 1;
        ctx.request_repaint();
        if shot.frames >= 120 && !shot.requested {
            shot.requested = true;
            let path = shot.path.clone();
            let done = shot.done.clone();
            let ctx2 = ctx.clone();
            ctx.request_screenshot(move |img| {
                let [w, h] = img.size;
                let mut bytes = Vec::with_capacity(w * h * 4);
                for p in &img.pixels {
                    bytes.extend_from_slice(&[p.r(), p.g(), p.b(), 255]);
                }
                if let Some(dir) = path.parent() {
                    let _ = std::fs::create_dir_all(dir);
                }
                if let Err(e) = image::save_buffer(&path, &bytes, w as u32, h as u32, image::ColorType::Rgba8) {
                    eprintln!("could not save screenshot: {e}");
                }
                done.store(true, Ordering::SeqCst);
                ctx2.request_repaint();
            });
        }
        if shot.done.load(Ordering::SeqCst) || shot.frames > 1200 {
            ctx.send_viewport_cmd(egui::ViewportCommand::Close);
        }
    }

    // ------------------------------------------------------------------ top bar

    fn top_bar(&mut self, ui: &mut Ui) {
        egui::MenuBar::new().ui(ui, |ui| {
            ui.menu_button("File", |ui| {
                if ui.button("New lesson").clicked() {
                    self.load_project(Project::new_blank(), None);
                    self.tab = Tab::Edit;
                    ui.close();
                }
                if ui.button("Open…").clicked() {
                    ui.close();
                    self.open();
                }
                if ui.button("Save").clicked() {
                    ui.close();
                    self.save(false);
                }
                if ui.button("Save as…").clicked() {
                    ui.close();
                    self.save(true);
                }
                ui.separator();
                if ui.button("Open the web version in your browser").clicked() {
                    ui.close();
                    self.status = match open_web_version(ui.ctx()) {
                        Ok(()) => "Web version opened in your browser".into(),
                        Err(e) => e,
                    };
                }
                ui.separator();
                ui.menu_button("Examples", |ui| {
                    for (n, name) in model::example_names().iter().enumerate() {
                        if ui.button(*name).clicked() {
                            self.load_project(model::example(n), None);
                            self.tab = Tab::Watch;
                            ui.close();
                        }
                    }
                });
            });
            ui.menu_button("View", |ui| {
                ui.label(RichText::new("Text and button size").strong());
                for (name, s) in [("Normal", 1.0), ("Large", 1.25), ("Extra large", 1.5)] {
                    let current = (crate::player::scale() - s).abs() < 0.01;
                    if ui.radio(current, name).clicked() {
                        crate::player::set_scale(s);
                        crate::player::apply_style(ui.ctx());
                        self.last_width = 0.0;
                        ui.close();
                    }
                }
                ui.separator();
                if ui.checkbox(&mut self.show_side, "Side panel while watching").clicked() {
                    ui.close();
                }
            });
            ui.separator();
            for (tab, label) in [(Tab::Watch, "Watch"), (Tab::Edit, "Edit"), (Tab::Map, "Story map"), (Tab::Results, "Results")] {
                if ui.selectable_label(self.tab == tab, RichText::new(label).size(sz(15.0))).clicked() {
                    self.tab = tab;
                }
            }
            if self.tab == Tab::Watch {
                ui.separator();
                ui.toggle_value(&mut self.show_side, "Side panel").on_hover_text("Show or hide progress and chapters");
            }
            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                let ff = if self.ffmpeg { "video files: on" } else { "video files: install ffmpeg" };
                ui.label(RichText::new(ff).small().weak());
                ui.separator();
                let name = if self.dirty { format!("{} •", self.project.title) } else { self.project.title.clone() };
                ui.label(RichText::new(name).strong());
                if !self.status.is_empty() {
                    ui.label(RichText::new(&self.status).small().weak());
                }
            });
        });
    }

    // ------------------------------------------------------------------ watch

    fn watch_tab(&mut self, ui: &mut Ui) {
        let dt = ui.input(|i| i.stable_dt).min(0.1);
        self.player.tick(&self.project, dt);
        if self.player.playing {
            ui.ctx().request_repaint();
        }
        if !ui.ctx().egui_wants_keyboard_input() {
            let (space, left, right) = ui.input(|i| (i.key_pressed(Key::Space), i.key_pressed(Key::ArrowLeft), i.key_pressed(Key::ArrowRight)));
            let free = self.player.started && !self.player.finished && self.player.blocking.is_none();
            if space && free {
                self.player.playing = !self.player.playing;
            }
            if free && (left || right) {
                let t = self.player.time + if left { -5.0 } else { 5.0 };
                self.player.seek(&self.project, t);
            }
        }

        // On a narrow window the side panel folds away so the video gets the room.
        let width = ui.ctx().content_rect().width();
        let narrow = |w: f32| w < sz(820.0);
        if self.last_width == 0.0 || narrow(width) != narrow(self.last_width) {
            self.show_side = !narrow(width);
        }
        self.last_width = width;
        let mut open = self.show_side;
        egui::Panel::right("watch-side").default_size(sz(260.0)).resizable(true).show_collapsible(ui, &mut open, |ui| {
            egui::ScrollArea::vertical().show(ui, |ui| self.watch_side(ui));
        });
        self.show_side = open;
        egui::Panel::bottom("transport").frame(egui::Frame::side_top_panel(ui.style()).inner_margin(8.0)).show(ui, |ui| {
            self.player.transport(ui, &self.project);
        });
        egui::CentralPanel::default().frame(egui::Frame::new().fill(Color32::from_rgb(10, 10, 14))).show(ui, |ui| {
            let space = ui.available_rect_before_wrap();
            self.player.show(ui, &self.project, self.ffmpeg, space);
        });

        if self.player.want_restart {
            self.restart_player();
            self.player.start(&self.project);
        }
        if self.player.want_results {
            self.player.want_results = false;
            self.tab = Tab::Results;
        }
    }

    fn watch_side(&mut self, ui: &mut Ui) {
        ui.add_space(6.0);
        ui.label(RichText::new("Viewer").strong());
        if ui.text_edit_singleline(&mut self.learner).changed() {
            self.player.learner = self.learner.clone();
            if !self.player.started {
                self.player.session.learner = self.learner.clone();
            }
        }
        ui.add_space(8.0);
        ui.label(RichText::new("Progress").strong());
        let s = &self.player.session;
        egui::Grid::new("progress").num_columns(2).show(ui, |ui| {
            ui.label("Scene");
            ui.label(self.project.scene_name(self.player.scene));
            ui.end_row();
            if s.max_score > 0 {
                ui.label("Score");
                ui.label(format!("{} / {}  ({}%)", s.score, s.max_score, s.percent()));
                ui.end_row();
                ui.label("Pass mark");
                ui.label(format!("{}%", self.project.pass_percent));
                ui.end_row();
            }
            ui.label("Answered");
            ui.label(s.questions_answered().to_string());
            ui.end_row();
        });
        if s.max_score > 0 {
            ui.add(egui::ProgressBar::new(s.percent() as f32 / 100.0).desired_height(8.0));
        }
        if !s.path.is_empty() {
            ui.add_space(6.0);
            ui.label(RichText::new("Path so far").strong());
            ui.label(RichText::new(s.path.join("  →  ")).small());
        }

        ui.add_space(10.0);
        ui.label(RichText::new("Chapters").strong());
        let Some(scene) = self.project.scene(self.player.scene).cloned() else { return };
        if scene.chapters.is_empty() {
            ui.label(RichText::new("No chapters in this scene").weak());
        }
        let current = scene.chapters.iter().rposition(|c| c.time <= self.player.time + 0.01);
        for (n, c) in scene.chapters.iter().enumerate() {
            let locked = self.project.prevent_skipping && c.time > self.player.max_watched + 0.01;
            let text = format!("{}  {}{}", fmt_time(c.time), c.title, if locked { "  (not reached)" } else { "" });
            let r = ui.add_enabled(!locked && self.player.blocking.is_none(), egui::Button::selectable(current == Some(n), text));
            if r.clicked() {
                if !self.player.started {
                    self.player.start(&self.project);
                }
                self.player.seek(&self.project, c.time);
                self.player.playing = true;
            }
        }

        ui.add_space(10.0);
        ui.label(RichText::new("In this scene").strong());
        for i in &scene.interactions {
            let done = i.time < self.player.time;
            ui.horizontal(|ui| {
                ui.label(RichText::new("●").color(i.kind.colour()));
                let t = RichText::new(format!("{}  {}", fmt_time(i.time), i.kind.type_name())).small();
                ui.label(if done { t.weak() } else { t });
            });
        }
        ui.add_space(10.0);
        if self.project.prevent_skipping {
            ui.label(RichText::new("Skipping ahead is turned off for this lesson.").small().weak());
        }
        if ui.button("Restart lesson").clicked() {
            self.restart_player();
        }
    }

    // ------------------------------------------------------------------ edit

    fn scene_choices(&self) -> Vec<(u32, String)> {
        self.project.scenes.iter().map(|s| (s.id, s.name.clone())).collect()
    }

    fn edit_tab(&mut self, ui: &mut Ui) {
        if self.project.scene(self.sel_scene).is_none() {
            self.sel_scene = self.project.scenes.first().map(|s| s.id).unwrap_or(0);
        }
        let dt = ui.input(|i| i.stable_dt).min(0.1);
        if self.edit_playing {
            if let Some(s) = self.project.scene(self.sel_scene) {
                self.edit_time += dt;
                if self.edit_time >= s.duration {
                    self.edit_time = s.duration;
                    self.edit_playing = false;
                }
            }
            ui.ctx().request_repaint();
        }
        egui::Panel::left("scenes").default_size(sz(220.0)).resizable(true).show(ui, |ui| {
            egui::ScrollArea::vertical().show(ui, |ui| self.scene_list(ui));
        });
        egui::Panel::right("inspector").default_size(sz(330.0)).resizable(true).show(ui, |ui| {
            egui::ScrollArea::vertical().show(ui, |ui| {
                if self.sel_item.is_some() {
                    self.item_inspector(ui);
                } else {
                    self.scene_inspector(ui);
                }
            });
        });
        egui::CentralPanel::default().show(ui, |ui| self.edit_center(ui));
    }

    fn scene_list(&mut self, ui: &mut Ui) {
        ui.add_space(4.0);
        ui.label(RichText::new("Lesson").strong());
        let mut changed = false;
        changed |= ui.add(egui::TextEdit::singleline(&mut self.project.title).hint_text("Title")).changed();
        changed |= ui.add(egui::TextEdit::multiline(&mut self.project.description).desired_rows(2).hint_text("Short description shown at the start")).changed();
        ui.horizontal(|ui| {
            ui.label("Pass mark");
            changed |= ui.add(egui::Slider::new(&mut self.project.pass_percent, 0..=100).suffix("%")).changed();
        });
        changed |= ui.checkbox(&mut self.project.prevent_skipping, "Don't let viewers skip ahead").changed();
        ui.label(RichText::new(format!("{} points in total", self.project.max_points())).small().weak());
        ui.separator();
        ui.label(RichText::new("Scenes").strong());
        let start = self.project.start_scene;
        let ids: Vec<(u32, String, usize)> = self.project.scenes.iter().map(|s| (s.id, s.name.clone(), s.interactions.len())).collect();
        for (id, name, n) in ids {
            let label = format!("{}{}  ({n})", if id == start { "Start · " } else { "" }, name);
            if ui.add(egui::Button::selectable(self.sel_scene == id, label)).clicked() {
                self.sel_scene = id;
                self.sel_item = None;
                self.edit_time = 0.0;
                self.edit_view.reset();
            }
        }
        ui.add_space(4.0);
        ui.horizontal_wrapped(|ui| {
            if ui.button("+ Scene").clicked() {
                let n = self.project.scenes.len();
                let pos = [60.0 + (n % 3) as f32 * 260.0, 40.0 + (n / 3) as f32 * 120.0];
                let id = self.project.add_scene(&format!("Scene {}", n + 1), 30.0, n % LOOKS.len(), pos);
                self.sel_scene = id;
                self.sel_item = None;
                changed = true;
            }
            if ui.button("Duplicate").clicked() {
                if let Some(mut s) = self.project.scene(self.sel_scene).cloned() {
                    s.id = self.project.alloc_id();
                    s.name = format!("{} copy", s.name);
                    s.map_pos[1] += 90.0;
                    for i in &mut s.interactions {
                        i.id = self.project.alloc_id();
                    }
                    self.sel_scene = s.id;
                    self.project.scenes.push(s);
                    changed = true;
                }
            }
            if ui.add_enabled(self.project.scenes.len() > 1, egui::Button::new("Delete")).clicked() {
                let id = self.sel_scene;
                self.project.scenes.retain(|s| s.id != id);
                if self.project.start_scene == id {
                    self.project.start_scene = self.project.scenes[0].id;
                }
                self.sel_scene = self.project.scenes[0].id;
                self.sel_item = None;
                changed = true;
            }
            if ui.add_enabled(self.sel_scene != start, egui::Button::new("Make start")).clicked() {
                self.project.start_scene = self.sel_scene;
                changed = true;
            }
        });
        ui.separator();
        if ui.button("Preview in Watch").clicked() {
            self.restart_player();
            self.tab = Tab::Watch;
        }
        if changed {
            self.dirty = true;
        }
    }

    fn edit_center(&mut self, ui: &mut Ui) {
        let scene_id = self.sel_scene;
        let Some(scene) = self.project.scene(scene_id).cloned() else {
            ui.label("Add a scene to begin.");
            return;
        };
        self.edit_time = self.edit_time.clamp(0.0, scene.duration);
        let t = self.edit_time;

        ui.horizontal_wrapped(|ui| {
            ui.label(RichText::new(format!("Add at {}:", fmt_time(t))).strong());
            let kinds: Vec<(&str, Kind)> = vec![
                ("Question", Kind::Question {
                    prompt: "New question".into(),
                    options: vec![
                        AnswerOption { text: "Right answer".into(), correct: true, feedback: "Well done.".into() },
                        AnswerOption { text: "Wrong answer".into(), correct: false, feedback: "Have another look.".into() },
                    ],
                    points: 10,
                    allow_retry: true,
                    on_correct: Jump::Continue,
                    on_wrong: Jump::Continue,
                }),
                ("Choice", Kind::Choice {
                    prompt: "What next?".into(),
                    options: vec![
                        ChoiceOption { text: "Option A".into(), jump: Jump::Continue },
                        ChoiceOption { text: "Option B".into(), jump: Jump::Continue },
                    ],
                }),
                ("Hotspot", Kind::Hotspot { rect: [0.4, 0.35, 0.2, 0.25], label: "Look here".into(), info: "More about this.".into(), jump: Jump::Continue }),
                ("Text card", Kind::Info { title: "Did you know?".into(), body: "Some extra detail.".into() }),
                ("Open question", Kind::OpenQuestion { prompt: "What do you think?".into() }),
                ("Pause", Kind::PauseScreen { text: "Take a moment, then continue.".into() }),
                ("Score check", Kind::ScoreCheck { min_percent: self.project.pass_percent, pass: Jump::Continue, fail: Jump::Continue }),
                ("Link", Kind::Link { label: "Learn more".into(), url: "https://example.com".into() }),
            ];
            for (label, kind) in kinds {
                let col = kind.colour();
                if ui.add(egui::Button::new(RichText::new(label).color(Color32::BLACK)).fill(col)).clicked() {
                    let id = self.project.add_interaction(scene_id, t, kind);
                    self.sel_item = Some(id);
                    self.dirty = true;
                }
            }
            ui.separator();
            if ui.button("+ Chapter").clicked() {
                if let Some(s) = self.project.scene_mut(scene_id) {
                    s.chapters.push(Chapter { time: t, title: "New chapter".into() });
                    s.chapters.sort_by(|a, b| a.time.total_cmp(&b.time));
                }
                self.sel_item = None;
                self.dirty = true;
            }
            if ui.button("+ Caption").clicked() {
                if let Some(s) = self.project.scene_mut(scene_id) {
                    s.captions.push(Caption { start: t, end: (t + 4.0).min(s.duration), text: "New caption".into() });
                    s.captions.sort_by(|a, b| a.start.total_cmp(&b.start));
                }
                self.sel_item = None;
                self.dirty = true;
            }
        });
        ui.add_space(4.0);

        let timeline_h = sz(128.0);
        let avail = ui.available_rect_before_wrap();
        let space = Rect::from_min_max(avail.min, pos2(avail.max.x, (avail.max.y - timeline_h - sz(64.0)).max(avail.min.y + 120.0)));
        ui.painter().rect_filled(space, 4.0, Color32::from_rgb(10, 10, 14));
        let rect = video_rect(space);
        self.edit_view.paint(ui, rect, &scene, t, self.ffmpeg);
        ui.allocate_rect(space, Sense::hover());

        // Captions and the selected item, as a viewer would see them.
        if let Some(c) = scene.captions.iter().find(|c| t >= c.start && t <= c.end) {
            ui.painter().text(pos2(rect.center().x, rect.bottom() - 20.0), Align2::CENTER_CENTER, &c.text, FontId::proportional(sz(16.0)), Color32::WHITE);
        }
        let mut select: Option<u32> = None;
        let mut moved: Option<(u32, [f32; 4])> = None;
        for i in &scene.interactions {
            let selected = self.sel_item == Some(i.id);
            let visible = t >= i.time && t <= i.time + i.show_for.max(0.5);
            match &i.kind {
                Kind::Hotspot { rect: r, label, .. } if visible || selected => {
                    let hr = Rect::from_min_size(
                        pos2(rect.left() + r[0] * rect.width(), rect.top() + r[1] * rect.height()),
                        vec2(r[2] * rect.width(), r[3] * rect.height()),
                    );
                    let col = i.kind.colour();
                    ui.painter().rect_filled(hr, 8.0, col.gamma_multiply(if selected { 0.3 } else { 0.15 }));
                    ui.painter().rect_stroke(hr, 8.0, Stroke::new(if selected { 3.0 } else { 1.5 }, col), StrokeKind::Inside);
                    ui.painter().text(hr.left_top() + vec2(6.0, 4.0), Align2::LEFT_TOP, label, FontId::proportional(sz(14.0)), Color32::WHITE);
                    let body = ui.interact(hr, ui.scope_id().with(("edit-hs", i.id)), Sense::click_and_drag());
                    let handle = Rect::from_center_size(hr.right_bottom(), vec2(14.0, 14.0));
                    if selected {
                        ui.painter().rect_filled(handle, 2.0, col);
                    }
                    let h = ui.interact(handle, ui.scope_id().with(("edit-hs-size", i.id)), Sense::drag());
                    let mut nr = *r;
                    if body.clicked() || body.drag_started() {
                        select = Some(i.id);
                    }
                    if selected && h.dragged() {
                        let d = h.drag_delta();
                        nr[2] = (nr[2] + d.x / rect.width()).clamp(0.03, 1.0 - nr[0]);
                        nr[3] = (nr[3] + d.y / rect.height()).clamp(0.03, 1.0 - nr[1]);
                        moved = Some((i.id, nr));
                    } else if body.dragged() {
                        let d = body.drag_delta();
                        nr[0] = (nr[0] + d.x / rect.width()).clamp(0.0, 1.0 - nr[2]);
                        nr[1] = (nr[1] + d.y / rect.height()).clamp(0.0, 1.0 - nr[3]);
                        moved = Some((i.id, nr));
                    }
                    if body.hovered() {
                        ui.ctx().set_cursor_icon(egui::CursorIcon::Grab);
                    }
                }
                k if selected || (visible && !matches!(k, Kind::Hotspot { .. })) => {
                    let text = format!("{} · {}", k.type_name(), k.summary());
                    let galley = ui.painter().layout(text, FontId::proportional(sz(15.0)), Color32::BLACK, rect.width() * 0.7);
                    let r = Rect::from_center_size(pos2(rect.center().x, rect.top() + 30.0 + galley.size().y / 2.0), galley.size() + vec2(20.0, 12.0));
                    ui.painter().rect_filled(r, 8.0, k.colour());
                    ui.painter().galley(r.min + vec2(10.0, 6.0), galley, Color32::BLACK);
                }
                _ => {}
            }
        }
        if let Some((id, nr)) = moved {
            if let Some(Kind::Hotspot { rect, .. }) = self.project.scene_mut(scene_id).and_then(|s| s.interactions.iter_mut().find(|i| i.id == id)).map(|i| &mut i.kind) {
                *rect = nr;
                self.dirty = true;
            }
        }
        if let Some(id) = select {
            self.sel_item = Some(id);
        }

        ui.add_space(6.0);
        ui.horizontal(|ui| {
            if crate::player::play_button(ui, self.edit_playing, true).on_hover_text("Preview the clip (interactions don't stop it here)").clicked() {
                self.edit_playing = !self.edit_playing;
            }
            ui.label(RichText::new(format!("{} / {}", fmt_time(t), fmt_time(scene.duration))).monospace());
            ui.label(RichText::new("How to edit (hover)").small().weak()).on_hover_text("Drag the playhead, click a marker to edit it, drag markers to move them in time. Drag hotspots on the picture.");
        });
        self.timeline(ui, &scene, timeline_h);
    }

    fn timeline(&mut self, ui: &mut Ui, scene: &model::Scene, height: f32) {
        let (rect, resp) = ui.allocate_exact_size(vec2(ui.available_width(), height), Sense::click_and_drag());
        let p = ui.painter_at(rect);
        p.rect_filled(rect, 6.0, Color32::from_gray(28));
        let left = rect.left() + 80.0;
        let right = rect.right() - 12.0;
        let dur = scene.duration.max(0.1);
        let x_of = |t: f32| left + (right - left) * (t / dur).clamp(0.0, 1.0);
        let t_of = |x: f32| ((x - left) / (right - left)).clamp(0.0, 1.0) * dur;
        let rows = ["Time", "Interactions", "Chapters", "Captions"];
        let row_h = height / rows.len() as f32;
        for (n, name) in rows.iter().enumerate() {
            let y = rect.top() + row_h * n as f32;
            p.text(pos2(rect.left() + 8.0, y + row_h / 2.0), Align2::LEFT_CENTER, *name, FontId::proportional(sz(12.0)), Color32::from_gray(160));
            if n > 0 {
                p.line_segment([pos2(rect.left(), y), pos2(rect.right(), y)], Stroke::new(1.0, Color32::from_gray(45)));
            }
        }
        let step = if dur > 240.0 { 30.0 } else if dur > 90.0 { 10.0 } else { 5.0 };
        let mut s = 0.0;
        while s <= dur + 0.01 {
            let x = x_of(s);
            p.line_segment([pos2(x, rect.top() + row_h * 0.55), pos2(x, rect.top() + row_h)], Stroke::new(1.0, Color32::from_gray(90)));
            p.text(pos2(x + 2.0, rect.top() + 3.0), Align2::LEFT_TOP, fmt_time(s), FontId::monospace(sz(10.0)), Color32::from_gray(150));
            s += step;
        }
        // Scrub by clicking or dragging on empty space.
        if resp.clicked() || resp.dragged() {
            if let Some(pos) = resp.interact_pointer_pos() {
                self.edit_time = t_of(pos.x);
            }
        }
        // Interaction markers.
        let iy = rect.top() + row_h * 1.5;
        let mut retime: Option<(u32, f32)> = None;
        for i in &scene.interactions {
            let x = x_of(i.time);
            let w = ((x_of(i.time + i.show_for) - x).max(12.0)).min(right - x + 6.0);
            let w = if i.kind.always_blocks() || matches!(i.kind, Kind::ScoreCheck { .. }) { 12.0 } else { w };
            let r = Rect::from_min_size(pos2(x - 6.0, iy - row_h * 0.32), vec2(w, row_h * 0.64));
            let selected = self.sel_item == Some(i.id);
            p.rect_filled(r, 4.0, i.kind.colour().gamma_multiply(if selected { 1.0 } else { 0.75 }));
            if selected {
                p.rect_stroke(r, 4.0, Stroke::new(2.0, Color32::WHITE), StrokeKind::Outside);
            }
            let m = ui.interact(r, ui.scope_id().with(("marker", i.id)), Sense::click_and_drag());
            if m.clicked() || m.drag_started() {
                self.sel_item = Some(i.id);
                self.edit_time = i.time;
            }
            if m.dragged() {
                if let Some(pos) = m.interact_pointer_pos() {
                    retime = Some((i.id, (t_of(pos.x) * 10.0).round() / 10.0));
                }
            }
            m.on_hover_text(format!("{} at {}: {}", i.kind.type_name(), fmt_time(i.time), i.kind.summary()));
        }
        if let Some((id, nt)) = retime {
            if let Some(s) = self.project.scene_mut(scene.id) {
                if let Some(i) = s.interactions.iter_mut().find(|i| i.id == id) {
                    i.time = nt;
                }
                s.interactions.sort_by(|a, b| a.time.total_cmp(&b.time));
            }
            self.edit_time = nt;
            self.dirty = true;
        }
        // Chapters.
        let cy = rect.top() + row_h * 2.5;
        for c in &scene.chapters {
            let x = x_of(c.time);
            p.line_segment([pos2(x, cy - row_h * 0.35), pos2(x, cy + row_h * 0.35)], Stroke::new(2.0, Color32::from_gray(220)));
            p.text(pos2(x + 4.0, cy), Align2::LEFT_CENTER, &c.title, FontId::proportional(sz(11.0)), Color32::from_gray(220));
        }
        // Captions.
        let ky = rect.top() + row_h * 3.5;
        for c in &scene.captions {
            let r = Rect::from_min_max(pos2(x_of(c.start), ky - row_h * 0.28), pos2(x_of(c.end).max(x_of(c.start) + 3.0), ky + row_h * 0.28));
            p.rect_filled(r, 3.0, Color32::from_rgb(70, 80, 110));
        }
        // Playhead.
        let px = x_of(self.edit_time);
        p.line_segment([pos2(px, rect.top()), pos2(px, rect.bottom())], Stroke::new(2.0, Color32::from_rgb(255, 200, 70)));
    }

    fn scene_inspector(&mut self, ui: &mut Ui) {
        let choices = self.scene_choices();
        let ffmpeg = self.ffmpeg;
        let edit_time = self.edit_time;
        let Some(scene) = self.project.scene_mut(self.sel_scene) else { return };
        let mut changed = false;
        ui.add_space(4.0);
        ui.label(RichText::new("Scene").strong().size(sz(16.0)));
        changed |= ui.text_edit_singleline(&mut scene.name).changed();
        ui.add_space(6.0);
        ui.label(RichText::new("Video").strong());
        match scene.video.clone() {
            Some(path) => {
                let name = std::path::Path::new(&path).file_name().map(|n| n.to_string_lossy().to_string()).unwrap_or(path.clone());
                ui.label(name).on_hover_text(&path);
            }
            None => {
                ui.label(RichText::new("Sample clip (no video file)").weak());
            }
        }
        ui.horizontal(|ui| {
            if ui.button("Choose video file…").clicked() {
                if let Some(f) = rfd::FileDialog::new().add_filter("Video", &["mp4", "mov", "m4v", "webm", "mkv", "avi"]).pick_file() {
                    let path = f.to_string_lossy().to_string();
                    if let Some(d) = video::probe_duration(&path) {
                        scene.duration = d;
                    }
                    scene.video = Some(path);
                    changed = true;
                }
            }
            if scene.video.is_some() && ui.button("Remove").clicked() {
                scene.video = None;
                changed = true;
            }
        });
        if !ffmpeg {
            ui.label(RichText::new("Video files need ffmpeg installed; until then a sample clip plays.").small().weak());
        }
        if scene.video.is_some() {
            ui.horizontal(|ui| {
                ui.label("Starts at");
                changed |= ui
                    .add(egui::DragValue::new(&mut scene.clip_start).range(0.0..=36000.0).speed(1.0).custom_formatter(|v, _| fmt_time(v as f32)))
                    .on_hover_text("A scene can be one part of a longer video")
                    .changed();
                ui.label("into the video");
            });
        }
        ui.horizontal(|ui| {
            ui.label("Length");
            changed |= ui.add(egui::DragValue::new(&mut scene.duration).range(2.0..=7200.0).speed(0.5).suffix(" s")).changed();
        });
        if scene.video.is_none() {
            egui::ComboBox::from_label("Sample look").selected_text(LOOKS[scene.look % LOOKS.len()]).show_ui(ui, |ui| {
                for (n, l) in LOOKS.iter().enumerate() {
                    changed |= ui.selectable_value(&mut scene.look, n, *l).changed();
                }
            });
        }
        ui.add_space(6.0);
        ui.label(RichText::new("When the scene ends").strong());
        let duration = scene.duration;
        changed |= jump_ui(ui, "at-end", &mut scene.at_end, &choices, duration);

        ui.add_space(8.0);
        ui.separator();
        ui.label(RichText::new("Chapters (bookmarks)").strong());
        let mut remove = None;
        for (n, c) in scene.chapters.iter_mut().enumerate() {
            ui.horizontal(|ui| {
                changed |= ui.add(egui::DragValue::new(&mut c.time).range(0.0..=7200.0).speed(0.1).custom_formatter(|v, _| fmt_time(v as f32))).changed();
                changed |= ui.add(egui::TextEdit::singleline(&mut c.title).desired_width(150.0)).changed();
                if ui.small_button("✕").clicked() {
                    remove = Some(n);
                }
            });
        }
        if let Some(n) = remove {
            scene.chapters.remove(n);
            changed = true;
        }
        if ui.small_button(format!("+ Chapter at {}", fmt_time(edit_time))).clicked() {
            scene.chapters.push(Chapter { time: edit_time, title: "New chapter".into() });
            changed = true;
        }
        scene.chapters.sort_by(|a, b| a.time.total_cmp(&b.time));

        ui.add_space(8.0);
        ui.separator();
        ui.label(RichText::new("Captions").strong());
        let mut remove = None;
        for (n, c) in scene.captions.iter_mut().enumerate() {
            ui.horizontal(|ui| {
                changed |= ui.add(egui::DragValue::new(&mut c.start).range(0.0..=7200.0).speed(0.1).custom_formatter(|v, _| format!("{v:.1}"))).changed();
                changed |= ui.add(egui::DragValue::new(&mut c.end).range(0.0..=7200.0).speed(0.1).custom_formatter(|v, _| format!("{v:.1}"))).changed();
                if ui.small_button("✕").clicked() {
                    remove = Some(n);
                }
            });
            changed |= ui.add(egui::TextEdit::singleline(&mut c.text).desired_width(f32::INFINITY)).changed();
        }
        if let Some(n) = remove {
            scene.captions.remove(n);
            changed = true;
        }
        if ui.small_button(format!("+ Caption at {}", fmt_time(edit_time))).clicked() {
            scene.captions.push(Caption { start: edit_time, end: edit_time + 4.0, text: "New caption".into() });
            changed = true;
        }
        scene.captions.sort_by(|a, b| a.start.total_cmp(&b.start));

        ui.add_space(8.0);
        ui.separator();
        ui.label(RichText::new("Interactions").strong());
        let items: Vec<(u32, String, egui::Color32)> = scene
            .interactions
            .iter()
            .map(|i| (i.id, format!("{}  {} · {}", fmt_time(i.time), i.kind.type_name(), i.kind.summary()), i.kind.colour()))
            .collect();
        if items.is_empty() {
            ui.label(RichText::new("Use the coloured buttons above the picture to add one at the playhead.").small().weak());
        }
        let mut pick = None;
        for (id, text, col) in items {
            ui.horizontal(|ui| {
                ui.label(RichText::new("●").color(col));
                if ui.add(egui::Button::selectable(false, text)).clicked() {
                    pick = Some(id);
                }
            });
        }
        if changed {
            self.dirty = true;
        }
        if let Some(id) = pick {
            self.sel_item = Some(id);
            if let Some(t) = self.project.scene(self.sel_scene).and_then(|s| s.interactions.iter().find(|i| i.id == id)).map(|i| i.time) {
                self.edit_time = t;
            }
        }
    }

    fn item_inspector(&mut self, ui: &mut Ui) {
        let choices = self.scene_choices();
        let pass = self.project.pass_percent;
        let scene_id = self.sel_scene;
        let Some(item_id) = self.sel_item else { return };
        let Some(scene) = self.project.scene_mut(scene_id) else { return };
        let duration = scene.duration;
        let Some(idx) = scene.interactions.iter().position(|i| i.id == item_id) else {
            self.sel_item = None;
            return;
        };
        let mut changed = false;
        let mut delete = false;
        let mut duplicate = false;
        let mut back = false;
        {
            let it = &mut scene.interactions[idx];
            ui.add_space(4.0);
            ui.horizontal(|ui| {
                if ui.small_button("← Scene").clicked() {
                    back = true;
                }
                ui.label(RichText::new(it.kind.type_name()).strong().size(sz(16.0)).color(it.kind.colour()));
            });
            ui.horizontal(|ui| {
                ui.label("Appears at");
                changed |= ui.add(egui::DragValue::new(&mut it.time).range(0.0..=duration).speed(0.1).custom_formatter(|v, _| format!("{} ({v:.1}s)", fmt_time(v as f32)))).changed();
            });
            let timed = matches!(it.kind, Kind::Info { .. } | Kind::Hotspot { .. } | Kind::Link { .. });
            if timed {
                ui.horizontal(|ui| {
                    ui.label("Visible for");
                    changed |= ui.add(egui::DragValue::new(&mut it.show_for).range(0.5..=600.0).speed(0.1).suffix(" s")).changed();
                });
                changed |= ui.checkbox(&mut it.pause, "Pause the video while it shows").changed();
            } else if !matches!(it.kind, Kind::ScoreCheck { .. }) {
                ui.label(RichText::new("The video pauses until the viewer responds.").small().weak());
            }
            ui.separator();
            match &mut it.kind {
                Kind::Info { title, body } => {
                    ui.label("Title");
                    changed |= ui.text_edit_singleline(title).changed();
                    ui.label("Text");
                    changed |= ui.add(egui::TextEdit::multiline(body).desired_rows(4).desired_width(f32::INFINITY)).changed();
                }
                Kind::Question { prompt, options, points, allow_retry, on_correct, on_wrong } => {
                    ui.label("Question");
                    changed |= ui.add(egui::TextEdit::multiline(prompt).desired_rows(2).desired_width(f32::INFINITY)).changed();
                    ui.label(RichText::new("Answers (tick the right ones)").strong());
                    let mut remove = None;
                    for (n, o) in options.iter_mut().enumerate() {
                        egui::Frame::group(ui.style()).show(ui, |ui| {
                            ui.horizontal(|ui| {
                                changed |= ui.checkbox(&mut o.correct, "").on_hover_text("Correct answer").changed();
                                changed |= ui.add(egui::TextEdit::singleline(&mut o.text).desired_width(190.0)).changed();
                                if ui.small_button("✕").clicked() {
                                    remove = Some(n);
                                }
                            });
                            changed |= ui.add(egui::TextEdit::singleline(&mut o.feedback).hint_text("Feedback for this answer").desired_width(f32::INFINITY)).changed();
                        });
                    }
                    if let Some(n) = remove {
                        options.remove(n);
                        changed = true;
                    }
                    if ui.small_button("+ Answer").clicked() {
                        options.push(AnswerOption { text: "Another answer".into(), correct: false, feedback: String::new() });
                        changed = true;
                    }
                    ui.horizontal(|ui| {
                        ui.label("Points");
                        changed |= ui.add(egui::DragValue::new(points).range(0..=1000)).changed();
                    });
                    changed |= ui.checkbox(allow_retry, "Allow another try after a wrong answer").changed();
                    ui.label(RichText::new("After a right answer").strong());
                    changed |= jump_ui(ui, "on-correct", on_correct, &choices, duration);
                    ui.label(RichText::new("After a wrong answer").strong());
                    changed |= jump_ui(ui, "on-wrong", on_wrong, &choices, duration);
                }
                Kind::OpenQuestion { prompt } => {
                    ui.label("Question");
                    changed |= ui.add(egui::TextEdit::multiline(prompt).desired_rows(3).desired_width(f32::INFINITY)).changed();
                    ui.label(RichText::new("Answers are recorded in Results, not scored.").small().weak());
                }
                Kind::Hotspot { rect, label, info, jump } => {
                    ui.label("Label");
                    changed |= ui.text_edit_singleline(label).changed();
                    ui.label("Shown when clicked");
                    changed |= ui.add(egui::TextEdit::multiline(info).desired_rows(3).desired_width(f32::INFINITY)).changed();
                    ui.label(RichText::new("Clicking it also").strong());
                    changed |= jump_ui(ui, "hs-jump", jump, &choices, duration);
                    ui.label(RichText::new("Position (drag it on the picture too)").strong());
                    for (n, name) in ["Left", "Top", "Width", "Height"].iter().enumerate() {
                        ui.horizontal(|ui| {
                            ui.label(*name);
                            changed |= ui.add(egui::Slider::new(&mut rect[n], 0.0..=1.0).fixed_decimals(2)).changed();
                        });
                    }
                }
                Kind::Choice { prompt, options } => {
                    ui.label("Question shown to the viewer");
                    changed |= ui.add(egui::TextEdit::multiline(prompt).desired_rows(2).desired_width(f32::INFINITY)).changed();
                    ui.label(RichText::new("Buttons and where each leads").strong());
                    let mut remove = None;
                    for (n, o) in options.iter_mut().enumerate() {
                        egui::Frame::group(ui.style()).show(ui, |ui| {
                            ui.horizontal(|ui| {
                                changed |= ui.add(egui::TextEdit::singleline(&mut o.text).desired_width(210.0)).changed();
                                if ui.small_button("✕").clicked() {
                                    remove = Some(n);
                                }
                            });
                            changed |= jump_ui(ui, ("choice", n), &mut o.jump, &choices, duration);
                        });
                    }
                    if let Some(n) = remove {
                        options.remove(n);
                        changed = true;
                    }
                    if ui.small_button("+ Button").clicked() {
                        options.push(ChoiceOption { text: "Another option".into(), jump: Jump::Continue });
                        changed = true;
                    }
                }
                Kind::PauseScreen { text } => {
                    ui.label("Message");
                    changed |= ui.add(egui::TextEdit::multiline(text).desired_rows(3).desired_width(f32::INFINITY)).changed();
                }
                Kind::ScoreCheck { min_percent, pass: ok, fail } => {
                    ui.label(RichText::new(format!("Checks the viewer's score so far (lesson pass mark is {pass}%).")).small().weak());
                    ui.horizontal(|ui| {
                        ui.label("At least");
                        changed |= ui.add(egui::Slider::new(min_percent, 0..=100).suffix("%")).changed();
                    });
                    ui.label(RichText::new("If reached").strong());
                    changed |= jump_ui(ui, "pass", ok, &choices, duration);
                    ui.label(RichText::new("If not").strong());
                    changed |= jump_ui(ui, "fail", fail, &choices, duration);
                }
                Kind::Link { label, url } => {
                    ui.label("Button text");
                    changed |= ui.text_edit_singleline(label).changed();
                    ui.label("Web address");
                    changed |= ui.text_edit_singleline(url).changed();
                }
            }
            ui.separator();
            ui.horizontal(|ui| {
                if ui.button("Duplicate").clicked() {
                    duplicate = true;
                }
                if ui.button(RichText::new("Delete").color(Color32::from_rgb(230, 90, 80))).clicked() {
                    delete = true;
                }
            });
        }
        if changed {
            scene.interactions.sort_by(|a, b| a.time.total_cmp(&b.time));
            self.dirty = true;
        }
        if delete {
            scene.interactions.retain(|i| i.id != item_id);
            self.sel_item = None;
            self.dirty = true;
        } else if duplicate {
            let copy = scene.interactions.iter().find(|i| i.id == item_id).cloned();
            if let Some(mut c) = copy {
                c.id = self.project.alloc_id();
                c.time = (c.time + 2.0).min(duration);
                self.sel_item = Some(c.id);
                if let Some(s) = self.project.scene_mut(scene_id) {
                    s.interactions.push(c);
                    s.interactions.sort_by(|a, b| a.time.total_cmp(&b.time));
                }
                self.dirty = true;
            }
        }
        if back {
            self.sel_item = None;
        }
    }

    // ------------------------------------------------------------------ story map

    fn map_tab(&mut self, ui: &mut Ui) {
        ui.horizontal(|ui| {
            ui.label(RichText::new("Story map").strong().size(sz(16.0)));
            ui.label(RichText::new("Each box is a scene; arrows show where choices, answers and scene endings lead. Drag boxes to arrange; double-click to edit.").weak());
        });
        ui.horizontal(|ui| {
            if ui.button("+ Scene").clicked() {
                let n = self.project.scenes.len();
                let id = self.project.add_scene(&format!("Scene {}", n + 1), 30.0, n % LOOKS.len(), [100.0 - self.map_offset.x, 320.0 - self.map_offset.y]);
                self.sel_scene = id;
                self.dirty = true;
            }
            if ui.button("Tidy up").clicked() {
                self.tidy_map();
            }
        });
        let (resp, painter) = ui.allocate_painter(ui.available_size(), Sense::click_and_drag());
        let area = resp.rect;
        painter.rect_filled(area, 6.0, Color32::from_rgb(22, 24, 30));
        let grid = 40.0;
        let mut x = area.left() + self.map_offset.x.rem_euclid(grid);
        while x < area.right() {
            painter.line_segment([pos2(x, area.top()), pos2(x, area.bottom())], Stroke::new(1.0, Color32::from_gray(32)));
            x += grid;
        }
        let mut y = area.top() + self.map_offset.y.rem_euclid(grid);
        while y < area.bottom() {
            painter.line_segment([pos2(area.left(), y), pos2(area.right(), y)], Stroke::new(1.0, Color32::from_gray(32)));
            y += grid;
        }
        let node_size = vec2(sz(190.0), sz(74.0));
        let origin = area.min + self.map_offset;
        let rect_of = |p: [f32; 2]| Rect::from_min_size(origin + vec2(p[0], p[1]), node_size);

        // Arrows first, so boxes sit on top.
        let mut edge_n = 0;
        let mut finishing: Vec<u32> = vec![];
        for s in &self.project.scenes {
            let from = rect_of(s.map_pos);
            let mut out: Vec<(String, u32, Color32)> = vec![];
            let mut finishes = 0;
            for i in &s.interactions {
                for (label, j) in i.kind.jumps() {
                    match j {
                        Jump::Scene(t) => out.push((label, t, i.kind.colour())),
                        Jump::Finish => finishes += 1,
                        _ => {}
                    }
                }
            }
            match s.at_end {
                Jump::Scene(t) => out.push(("at end".into(), t, Color32::from_gray(170))),
                Jump::Finish | Jump::Continue => finishes += 1,
                _ => {}
            }
            for (k, (label, target, col)) in out.iter().enumerate() {
                let Some(ts) = self.project.scene(*target) else { continue };
                let to = rect_of(ts.map_pos);
                let spread = (k as f32 - (out.len() as f32 - 1.0) / 2.0) * 12.0;
                let (a, b) = if to.center().x >= from.center().x {
                    (from.right_center() + vec2(0.0, spread), to.left_center() + vec2(0.0, spread * 0.5))
                } else {
                    (from.left_center() + vec2(0.0, spread), to.right_center() + vec2(0.0, spread * 0.5))
                };
                let dir = if b.x >= a.x { 1.0 } else { -1.0 };
                let bend = ((b.x - a.x).abs() * 0.5).max(60.0) * dir;
                let pts = [a, a + vec2(bend, 0.0), b - vec2(bend, 0.0), b];
                painter.add(egui::epaint::CubicBezierShape::from_points_stroke(pts, false, Color32::TRANSPARENT, Stroke::new(2.0, *col)));
                let back = vec2(-10.0 * dir, 0.0);
                painter.add(egui::Shape::convex_polygon(vec![b, b + back + vec2(0.0, -5.0), b + back + vec2(0.0, 5.0)], *col, Stroke::NONE));
                let mid = pos2((a.x + b.x) / 2.0, (a.y + b.y) / 2.0 + if edge_n % 2 == 0 { -8.0 } else { 8.0 });
                let galley = painter.layout_no_wrap(trim(label, 22), FontId::proportional(sz(11.0)), Color32::WHITE);
                let lr = Rect::from_center_size(mid, galley.size() + vec2(8.0, 4.0));
                painter.rect_filled(lr, 4.0, col.gamma_multiply(0.55));
                painter.galley(lr.min + vec2(4.0, 2.0), galley, Color32::WHITE);
                edge_n += 1;
            }
            if finishes > 0 {
                finishing.push(s.id);
            }
        }

        // Boxes.
        let mut drag: Option<(u32, egui::Vec2)> = None;
        let mut open: Option<u32> = None;
        let mut click: Option<u32> = None;
        for s in &self.project.scenes {
            let r = rect_of(s.map_pos);
            let resp = ui.interact(r, ui.scope_id().with(("node", s.id)), Sense::click_and_drag());
            let selected = self.sel_scene == s.id;
            painter.rect_filled(r, 10.0, Color32::from_rgb(40, 44, 56));
            // A small preview of the clip.
            let thumb = Rect::from_min_size(r.min + vec2(8.0, 8.0), vec2(80.0, 45.0));
            paint_sample(&painter.with_clip_rect(thumb), thumb, s, s.duration * 0.3);
            painter.rect_stroke(r, 10.0, Stroke::new(if selected { 2.5 } else { 1.0 }, if selected { Color32::from_rgb(255, 200, 70) } else { Color32::from_gray(90) }), StrokeKind::Inside);
            painter.text(pos2(thumb.right() + 8.0, r.top() + sz(10.0)), Align2::LEFT_TOP, trim(&s.name, 14), FontId::proportional(sz(13.0)), Color32::WHITE);
            painter.text(
                pos2(thumb.right() + 8.0, r.top() + sz(30.0)),
                Align2::LEFT_TOP,
                format!("{} · {} items", fmt_time(s.duration), s.interactions.len()),
                FontId::proportional(sz(11.0)),
                Color32::from_gray(170),
            );
            let mut tags = vec![];
            if s.video.is_some() {
                tags.push("video file");
            }
            if finishing.contains(&s.id) {
                tags.push("can finish here");
            }
            if !tags.is_empty() {
                painter.text(pos2(thumb.right() + 8.0, r.top() + sz(48.0)), Align2::LEFT_TOP, tags.join(" · "), FontId::proportional(sz(10.0)), Color32::from_gray(150));
            }
            if s.id == self.project.start_scene {
                let b = Rect::from_min_size(r.left_top() + vec2(8.0, -10.0), vec2(44.0, 16.0));
                painter.rect_filled(b, 8.0, Color32::from_rgb(90, 200, 120));
                painter.text(b.center(), Align2::CENTER_CENTER, "START", FontId::proportional(sz(10.0)), Color32::BLACK);
            }
            if resp.dragged() {
                drag = Some((s.id, resp.drag_delta()));
            }
            if resp.double_clicked() {
                open = Some(s.id);
            } else if resp.clicked() || resp.drag_started() {
                click = Some(s.id);
            }
        }
        if let Some((id, d)) = drag {
            if let Some(s) = self.project.scene_mut(id) {
                s.map_pos[0] += d.x;
                s.map_pos[1] += d.y;
                self.dirty = true;
            }
        } else if resp.dragged() {
            self.map_offset += resp.drag_delta();
        }
        if let Some(id) = click {
            self.sel_scene = id;
        }
        if let Some(id) = open {
            self.sel_scene = id;
            self.sel_item = None;
            self.edit_time = 0.0;
            self.tab = Tab::Edit;
        }
    }

    /// Lay scenes out in columns by how many steps they are from the start.
    fn tidy_map(&mut self) {
        use std::collections::{HashMap, VecDeque};
        let mut depth: HashMap<u32, usize> = HashMap::new();
        let mut q = VecDeque::new();
        depth.insert(self.project.start_scene, 0);
        q.push_back(self.project.start_scene);
        while let Some(id) = q.pop_front() {
            let d = depth[&id];
            if let Some(s) = self.project.scene(id) {
                let mut targets: Vec<u32> = s
                    .interactions
                    .iter()
                    .flat_map(|i| i.kind.jumps())
                    .filter_map(|(_, j)| if let Jump::Scene(t) = j { Some(t) } else { None })
                    .collect();
                if let Jump::Scene(t) = s.at_end {
                    targets.push(t);
                }
                for t in targets {
                    if !depth.contains_key(&t) {
                        depth.insert(t, d + 1);
                        q.push_back(t);
                    }
                }
            }
        }
        let max_d = depth.values().copied().max().unwrap_or(0);
        let mut rows: HashMap<usize, usize> = HashMap::new();
        for s in &mut self.project.scenes {
            let d = *depth.get(&s.id).unwrap_or(&(max_d + 1));
            let r = rows.entry(d).or_insert(0);
            s.map_pos = [40.0 + d as f32 * 280.0, 40.0 + *r as f32 * 120.0];
            *r += 1;
        }
        self.map_offset = vec2(20.0, 20.0);
        self.dirty = true;
    }

    // ------------------------------------------------------------------ results

    fn results_tab(&mut self, ui: &mut Ui) {
        let mut all: Vec<&Session> = self.history.iter().collect();
        if self.player.session.has_activity() {
            all.push(&self.player.session);
        }
        ui.horizontal(|ui| {
            ui.label(RichText::new("Results").strong().size(sz(16.0)));
            ui.label(RichText::new("Every viewing is recorded: answers, choices, clicks, score and completion.").weak());
        });
        if all.is_empty() {
            ui.add_space(30.0);
            ui.vertical_centered(|ui| {
                ui.label(RichText::new("No viewings yet. Watch the lesson and come back here.").size(sz(16.0)).weak());
            });
            return;
        }
        let mut status = None;
        ui.horizontal(|ui| {
            if ui.button("Export table (CSV)…").clicked() {
                if let Some(path) = rfd::FileDialog::new().add_filter("CSV", &["csv"]).set_file_name("results.csv").save_file() {
                    status = Some(match std::fs::write(&path, session::to_csv(&all)) {
                        Ok(()) => "Results exported".to_string(),
                        Err(e) => format!("Couldn't export: {e}"),
                    });
                }
            }
            if ui.button("Export learning records (JSON)…").clicked() {
                if let Some(path) = rfd::FileDialog::new().add_filter("JSON", &["json"]).set_file_name("statements.json").save_file() {
                    status = Some(match std::fs::write(&path, session::to_statements(&all)) {
                        Ok(()) => "Learning records exported".to_string(),
                        Err(e) => format!("Couldn't export: {e}"),
                    });
                }
            }
        });
        ui.add_space(6.0);
        let mut selected = self.selected_session.min(all.len() - 1);

        ui.columns(2, |cols| {
            let ui = &mut cols[0];
            ui.label(RichText::new("Viewings").strong());
            egui::Grid::new("sessions").striped(true).num_columns(6).show(ui, |ui| {
                for h in ["Viewer", "Started", "Score", "Result", "Finished", "Time"] {
                    ui.label(RichText::new(h).strong());
                }
                ui.end_row();
                for (n, s) in all.iter().enumerate() {
                    if ui.add(egui::Button::selectable(selected == n, &s.learner)).clicked() {
                        selected = n;
                    }
                    ui.label(s.started_at.replace('T', " ").trim_end_matches('Z').to_string());
                    ui.label(if s.max_score > 0 { format!("{}/{} ({}%)", s.score, s.max_score, s.percent()) } else { "–".into() });
                    let (t, c) = if s.max_score == 0 {
                        ("–", Color32::GRAY)
                    } else if s.passed() {
                        ("passed", Color32::from_rgb(110, 210, 130))
                    } else {
                        ("not passed", Color32::from_rgb(240, 130, 110))
                    };
                    ui.label(RichText::new(t).color(c));
                    ui.label(if s.completed { "yes" } else { "no" });
                    ui.label(fmt_time(s.elapsed() as f32));
                    ui.end_row();
                }
            });
            ui.add_space(10.0);
            ui.label(RichText::new("Questions across all viewings").strong());
            let mut stats: Vec<(String, u32, u32, bool)> = vec![];
            for s in &all {
                for e in s.events.iter().filter(|e| e.verb == "answered") {
                    let p = match stats.iter().position(|r| r.0 == e.object) {
                        Some(p) => p,
                        None => {
                            stats.push((e.object.clone(), 0, 0, e.success.is_some()));
                            stats.len() - 1
                        }
                    };
                    stats[p].1 += 1;
                    if e.success == Some(true) {
                        stats[p].2 += 1;
                    }
                }
            }
            if stats.is_empty() {
                ui.label(RichText::new("No answers yet.").weak());
            }
            egui::Grid::new("qstats").striped(true).num_columns(3).show(ui, |ui| {
                for (q, total, right, scored) in &stats {
                    ui.label(trim(q, 48));
                    ui.label(format!("{total} answer(s)"));
                    ui.label(if *scored { format!("{right} right") } else { "open answer".into() });
                    ui.end_row();
                }
            });

            let ui = &mut cols[1];
            let s = all[selected];
            ui.label(RichText::new(format!("What {} did", s.learner)).strong());
            ui.label(RichText::new(format!("Path: {}", s.path.join("  →  "))).small());
            egui::ScrollArea::vertical().id_salt("events").show(ui, |ui| {
                egui::Grid::new("events").striped(true).num_columns(4).show(ui, |ui| {
                    for h in ["At", "Action", "Item", "Answer / detail"] {
                        ui.label(RichText::new(h).strong());
                    }
                    ui.end_row();
                    for e in &s.events {
                        ui.label(RichText::new(format!("{} · {}", fmt_time(e.at as f32), fmt_time(e.video_time))).monospace().small())
                            .on_hover_text(format!("{:.1}s into the viewing, {} into the scene \"{}\"", e.at, fmt_time(e.video_time), e.scene));
                        ui.label(&e.verb);
                        ui.label(trim(&e.object, 36));
                        let mut d = RichText::new(trim(&e.detail, 40));
                        match e.success {
                            Some(true) => d = d.color(Color32::from_rgb(110, 210, 130)),
                            Some(false) => d = d.color(Color32::from_rgb(240, 130, 110)),
                            None => {}
                        }
                        ui.label(d);
                        ui.end_row();
                    }
                });
            });
        });
        self.selected_session = selected;
        if let Some(s) = status {
            self.status = s;
        }
    }
}

/// Open the web version in the default browser, served from this computer so YouTube videos play.
fn open_web_version(ctx: &egui::Context) -> Result<(), String> {
    let url = crate::web_server::url()?;
    ctx.open_url(egui::OpenUrl::new_tab(url));
    Ok(())
}

fn trim(s: &str, n: usize) -> String {
    if s.chars().count() > n {
        format!("{}…", s.chars().take(n).collect::<String>())
    } else {
        s.to_string()
    }
}

/// Pick where something leads: keep playing, a time, a scene, or finish.
fn jump_ui(ui: &mut Ui, salt: impl std::hash::Hash + std::fmt::Debug, jump: &mut Jump, scenes: &[(u32, String)], duration: f32) -> bool {
    let mut changed = false;
    let id = ui.scope_id().with(("jump", salt));
    ui.horizontal(|ui| {
        let first_scene = scenes.first().map(|s| s.0).unwrap_or(0);
        egui::ComboBox::from_id_salt(id).width(120.0).selected_text(jump.kind_name()).show_ui(ui, |ui| {
            for option in [Jump::Continue, Jump::Time(0.0), Jump::Scene(first_scene), Jump::Finish] {
                let same = std::mem::discriminant(&option) == std::mem::discriminant(jump);
                if ui.selectable_label(same, option.kind_name()).clicked() && !same {
                    *jump = option;
                    changed = true;
                }
            }
        });
        match jump {
            Jump::Time(t) => {
                changed |= ui.add(egui::DragValue::new(t).range(0.0..=duration).speed(0.1).custom_formatter(|v, _| fmt_time(v as f32))).changed();
            }
            Jump::Scene(sid) => {
                let name = scenes.iter().find(|s| s.0 == *sid).map(|s| s.1.clone()).unwrap_or_else(|| "(pick a scene)".into());
                egui::ComboBox::from_id_salt(id.with("scene")).width(140.0).selected_text(trim(&name, 18)).show_ui(ui, |ui| {
                    for (s_id, s_name) in scenes {
                        changed |= ui.selectable_value(sid, *s_id, s_name).changed();
                    }
                });
            }
            _ => {}
        }
    });
    changed
}

impl eframe::App for App {
    fn ui(&mut self, ui: &mut Ui, _frame: &mut eframe::Frame) {
        let ctx = ui.ctx().clone();
        self.handle_screenshot(&ctx);
        if ctx.input(|i| i.modifiers.command && i.key_pressed(Key::S)) {
            self.save(false);
        }
        egui::Panel::top("top").show(ui, |ui| self.top_bar(ui));
        match self.tab {
            Tab::Watch => self.watch_tab(ui),
            Tab::Edit => self.edit_tab(ui),
            Tab::Map => {
                egui::CentralPanel::default().show(ui, |ui| self.map_tab(ui));
            }
            Tab::Results => {
                egui::CentralPanel::default().show(ui, |ui| self.results_tab(ui));
            }
        }
    }
}
