//! The interactive-video project: scenes (clips), timed interactions, chapters,
//! captions and where each choice leads. Saved as plain JSON so the interaction
//! definitions stay separate from the media files they point to.

use serde::{Deserialize, Serialize};

/// Where the viewer goes after an interaction or at the end of a scene.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum Jump {
    /// Keep playing from where the video is.
    Continue,
    /// Jump to a time (seconds) in the same scene.
    Time(f32),
    /// Switch to another scene (clip), from its start.
    Scene(u32),
    /// End the experience and show the summary.
    Finish,
}

impl Jump {
    pub fn kind_name(&self) -> &'static str {
        match self {
            Jump::Continue => "Keep playing",
            Jump::Time(_) => "Jump to time",
            Jump::Scene(_) => "Go to scene",
            Jump::Finish => "Finish",
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AnswerOption {
    pub text: String,
    pub correct: bool,
    #[serde(default)]
    pub feedback: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ChoiceOption {
    pub text: String,
    pub jump: Jump,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum Kind {
    /// A text card shown over the video for a while.
    Info { title: String, body: String },
    /// Multiple choice with feedback and points; the answer can decide where to go next.
    Question {
        prompt: String,
        options: Vec<AnswerOption>,
        points: u32,
        allow_retry: bool,
        on_correct: Jump,
        on_wrong: Jump,
    },
    /// A free-text answer that is recorded but not scored.
    OpenQuestion { prompt: String },
    /// A clickable area on the picture (normalised x, y, w, h).
    Hotspot {
        rect: [f32; 4],
        label: String,
        info: String,
        jump: Jump,
    },
    /// Branching: buttons that each lead somewhere.
    Choice { prompt: String, options: Vec<ChoiceOption> },
    /// Stops the video with a message until the viewer continues.
    PauseScreen { text: String },
    /// Conditional logic: compares the score so far with a threshold.
    ScoreCheck { min_percent: u32, pass: Jump, fail: Jump },
    /// A call-to-action button that opens a web page.
    Link { label: String, url: String },
}

impl Kind {
    pub fn type_name(&self) -> &'static str {
        match self {
            Kind::Info { .. } => "Text card",
            Kind::Question { .. } => "Quiz question",
            Kind::OpenQuestion { .. } => "Open question",
            Kind::Hotspot { .. } => "Hotspot",
            Kind::Choice { .. } => "Choice (branch)",
            Kind::PauseScreen { .. } => "Pause screen",
            Kind::ScoreCheck { .. } => "Score check",
            Kind::Link { .. } => "Link button",
        }
    }

    pub fn summary(&self) -> String {
        let s = match self {
            Kind::Info { title, .. } => title.clone(),
            Kind::Question { prompt, .. } => prompt.clone(),
            Kind::OpenQuestion { prompt } => prompt.clone(),
            Kind::Hotspot { label, .. } => label.clone(),
            Kind::Choice { prompt, .. } => prompt.clone(),
            Kind::PauseScreen { text } => text.clone(),
            Kind::ScoreCheck { min_percent, .. } => format!("Score at least {min_percent}%?"),
            Kind::Link { label, .. } => label.clone(),
        };
        if s.chars().count() > 40 {
            format!("{}…", s.chars().take(40).collect::<String>())
        } else {
            s
        }
    }

    /// Interactions that stop the video until the viewer acts.
    pub fn always_blocks(&self) -> bool {
        matches!(
            self,
            Kind::Question { .. } | Kind::OpenQuestion { .. } | Kind::Choice { .. } | Kind::PauseScreen { .. }
        )
    }

    pub fn max_points(&self) -> u32 {
        match self {
            Kind::Question { points, .. } => *points,
            _ => 0,
        }
    }

    pub fn colour(&self) -> egui::Color32 {
        use egui::Color32;
        match self {
            Kind::Info { .. } => Color32::from_rgb(90, 170, 255),
            Kind::Question { .. } => Color32::from_rgb(255, 190, 60),
            Kind::OpenQuestion { .. } => Color32::from_rgb(240, 140, 60),
            Kind::Hotspot { .. } => Color32::from_rgb(80, 220, 160),
            Kind::Choice { .. } => Color32::from_rgb(200, 120, 255),
            Kind::PauseScreen { .. } => Color32::from_rgb(180, 180, 190),
            Kind::ScoreCheck { .. } => Color32::from_rgb(255, 110, 140),
            Kind::Link { .. } => Color32::from_rgb(110, 210, 240),
        }
    }

    /// Every place this interaction can send the viewer.
    pub fn jumps(&self) -> Vec<(String, Jump)> {
        match self {
            Kind::Question { on_correct, on_wrong, .. } => vec![
                ("right".into(), on_correct.clone()),
                ("wrong".into(), on_wrong.clone()),
            ],
            Kind::Hotspot { label, jump, .. } => vec![(label.clone(), jump.clone())],
            Kind::Choice { options, .. } => options.iter().map(|o| (o.text.clone(), o.jump.clone())).collect(),
            Kind::ScoreCheck { pass, fail, .. } => vec![("pass".into(), pass.clone()), ("fail".into(), fail.clone())],
            _ => vec![],
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Interaction {
    pub id: u32,
    /// When it appears, seconds into the scene.
    pub time: f32,
    /// How long cards and hotspots stay visible.
    pub show_for: f32,
    /// Also pause the video for cards and hotspots.
    pub pause: bool,
    pub kind: Kind,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Chapter {
    pub time: f32,
    pub title: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Caption {
    pub start: f32,
    pub end: f32,
    pub text: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Scene {
    pub id: u32,
    pub name: String,
    /// A video file on this computer; without one a built-in sample clip plays.
    pub video: Option<String>,
    pub duration: f32,
    /// Colour theme of the sample clip.
    pub look: usize,
    pub interactions: Vec<Interaction>,
    pub chapters: Vec<Chapter>,
    pub captions: Vec<Caption>,
    pub at_end: Jump,
    pub map_pos: [f32; 2],
    /// Where the scene starts inside its video, so one long video can be cut into scenes.
    #[serde(default)]
    pub clip_start: f32,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Project {
    pub title: String,
    pub description: String,
    pub scenes: Vec<Scene>,
    pub start_scene: u32,
    pub pass_percent: u32,
    /// Viewers can't skip ahead past what they have watched.
    pub prevent_skipping: bool,
    pub next_id: u32,
}

pub const LOOKS: [&str; 6] = ["Ocean", "Sunset", "Forest", "Night", "Studio", "Candy"];

impl Project {
    pub fn new_blank() -> Self {
        let mut p = Project {
            title: "Untitled lesson".into(),
            description: String::new(),
            scenes: vec![],
            start_scene: 1,
            pass_percent: 60,
            prevent_skipping: false,
            next_id: 1,
        };
        let id = p.add_scene("Scene 1", 40.0, 0, [80.0, 120.0]);
        p.start_scene = id;
        p
    }

    pub fn alloc_id(&mut self) -> u32 {
        let id = self.next_id;
        self.next_id += 1;
        id
    }

    pub fn add_scene(&mut self, name: &str, duration: f32, look: usize, pos: [f32; 2]) -> u32 {
        let id = self.alloc_id();
        self.scenes.push(Scene {
            id,
            name: name.into(),
            video: None,
            duration,
            look,
            interactions: vec![],
            chapters: vec![],
            captions: vec![],
            at_end: Jump::Finish,
            map_pos: pos,
            clip_start: 0.0,
        });
        id
    }

    pub fn scene(&self, id: u32) -> Option<&Scene> {
        self.scenes.iter().find(|s| s.id == id)
    }

    pub fn scene_mut(&mut self, id: u32) -> Option<&mut Scene> {
        self.scenes.iter_mut().find(|s| s.id == id)
    }

    pub fn scene_name(&self, id: u32) -> String {
        self.scene(id).map(|s| s.name.clone()).unwrap_or_else(|| "(missing scene)".into())
    }

    pub fn add_interaction(&mut self, scene: u32, time: f32, kind: Kind) -> u32 {
        let id = self.alloc_id();
        let show_for = match kind {
            Kind::Hotspot { .. } => 6.0,
            Kind::Link { .. } => 8.0,
            _ => 5.0,
        };
        if let Some(s) = self.scene_mut(scene) {
            s.interactions.push(Interaction { id, time, show_for, pause: false, kind });
            s.interactions.sort_by(|a, b| a.time.total_cmp(&b.time));
        }
        id
    }

    /// Points available across every scene (used for the pass mark).
    pub fn max_points(&self) -> u32 {
        self.scenes
            .iter()
            .flat_map(|s| s.interactions.iter())
            .map(|i| i.kind.max_points())
            .sum()
    }

    pub fn to_json(&self) -> String {
        serde_json::to_string_pretty(self).unwrap_or_default()
    }

    pub fn from_json(text: &str) -> Result<Self, String> {
        serde_json::from_str(text).map_err(|e| e.to_string())
    }
}

// ---------------------------------------------------------------------------
// Example projects, modelled on the interaction patterns in the research notes:
// in-video quiz lessons, timed overlays with hotspots, branching stories and
// product tours with links. All copy is original sample text.
// ---------------------------------------------------------------------------

fn mc(prompt: &str, opts: &[(&str, bool, &str)], points: u32) -> Kind {
    Kind::Question {
        prompt: prompt.into(),
        options: opts
            .iter()
            .map(|(t, c, f)| AnswerOption { text: (*t).into(), correct: *c, feedback: (*f).into() })
            .collect(),
        points,
        allow_retry: false,
        on_correct: Jump::Continue,
        on_wrong: Jump::Continue,
    }
}

fn cap(start: f32, end: f32, text: &str) -> Caption {
    Caption { start, end, text: text.into() }
}

fn chap(time: f32, title: &str) -> Chapter {
    Chapter { time, title: title.into() }
}

pub fn example_names() -> [&'static str; 6] {
    ["Lab safety quiz", "Smoothie recipe", "Choose your path", "Video demo: AMD and gamers", "Story map demo: AMD routes", "Product tour"]
}

pub fn example(index: usize) -> Project {
    match index {
        1 => example_smoothie(),
        2 => example_branching(),
        3 => example_youtube(),
        4 => example_routes(),
        5 => example_tour(),
        _ => example_lab_safety(),
    }
}

fn example_lab_safety() -> Project {
    let mut p = Project::new_blank();
    p.scenes.clear();
    p.title = "Lab safety quiz".into();
    p.description = "A short science lesson with questions along the way. Score 60% or more to pass; otherwise a review clip plays.".into();
    p.pass_percent = 60;
    p.prevent_skipping = true;
    let lesson = p.add_scene("Lesson: staying safe", 72.0, 0, [60.0, 140.0]);
    let review = p.add_scene("Review: the key rules", 30.0, 1, [380.0, 40.0]);
    let done = p.add_scene("Well done!", 14.0, 2, [380.0, 250.0]);
    p.start_scene = lesson;

    p.add_interaction(lesson, 4.0, Kind::Info {
        title: "In this lesson".into(),
        body: "Protective gear, handling liquids and what to do after a spill.".into(),
    });
    let q1 = p.add_interaction(lesson, 16.0, mc(
        "What should you wear before handling any liquid in the lab?",
        &[
            ("Safety goggles", true, "Right: eyes come first."),
            ("Sunglasses", false, "Sunglasses don't seal around the eyes."),
            ("Nothing special", false, "Splashes happen when you least expect them."),
        ],
        10,
    ));
    let _ = q1;
    p.add_interaction(lesson, 30.0, mc(
        "Where do you pour broken glass?",
        &[
            ("The ordinary bin", false, "Someone could get cut."),
            ("The marked glass container", true, "Correct."),
            ("Down the sink", false, "That blocks drains and hurts people."),
        ],
        10,
    ));
    p.add_interaction(lesson, 44.0, mc(
        "Someone spills a liquid. What comes first?",
        &[
            ("Tell the teacher", true, "Yes: they know how to clean it safely."),
            ("Wipe it with your sleeve", false, "Never touch an unknown spill."),
            ("Ignore it", false, "Others might slip or touch it."),
        ],
        10,
    ));
    p.add_interaction(lesson, 58.0, Kind::OpenQuestion {
        prompt: "In your own words: why do we tie long hair back in the lab?".into(),
    });
    p.add_interaction(lesson, 71.0, Kind::ScoreCheck { min_percent: 60, pass: Jump::Scene(done), fail: Jump::Scene(review) });
    if let Some(s) = p.scene_mut(lesson) {
        s.chapters = vec![chap(0.0, "Welcome"), chap(12.0, "Protective gear"), chap(26.0, "Glass and sharp things"), chap(40.0, "Spills"), chap(55.0, "Wrap-up")];
        s.captions = vec![
            cap(0.5, 4.0, "Welcome to the lab! Today: how to stay safe."),
            cap(8.0, 15.0, "First, let's look at what you wear."),
            cap(20.0, 29.0, "Glass can break. Sharp things need their own place."),
            cap(34.0, 43.0, "Accidents happen. What matters is what you do next."),
            cap(48.0, 57.0, "Let's put it all together."),
            cap(62.0, 70.0, "Let's see how you did."),
        ];
        s.at_end = Jump::Finish;
    }
    p.add_interaction(review, 3.0, Kind::Info { title: "Remember".into(), body: "Goggles on · glass in the marked box · tell an adult about spills.".into() });
    p.add_interaction(review, 20.0, Kind::Choice {
        prompt: "Ready to try the lesson again?".into(),
        options: vec![
            ChoiceOption { text: "Watch the lesson again".into(), jump: Jump::Scene(lesson) },
            ChoiceOption { text: "Finish for now".into(), jump: Jump::Finish },
        ],
    });
    if let Some(s) = p.scene_mut(review) {
        s.captions = vec![cap(1.0, 12.0, "Let's go over the three rules once more.")];
        s.at_end = Jump::Finish;
    }
    if let Some(s) = p.scene_mut(done) {
        s.captions = vec![cap(1.0, 10.0, "Great work: you know how to stay safe in the lab.")];
        s.at_end = Jump::Finish;
    }
    p
}

fn example_smoothie() -> Project {
    let mut p = Project::new_blank();
    p.scenes.clear();
    p.title = "Smoothie recipe".into();
    p.description = "A timed cooking video with an ingredients card, clickable hotspots, bookmarks and a quick check.".into();
    p.prevent_skipping = false;
    let s = p.add_scene("Make a berry smoothie", 60.0, 5, [100.0, 140.0]);
    p.start_scene = s;
    p.add_interaction(s, 9.0, Kind::Info {
        title: "Ingredients list".into(),
        body: "1 banana · a handful of berries · 1 cup of yoghurt · a splash of milk · honey to taste".into(),
    });
    let h1 = p.add_interaction(s, 18.0, Kind::Hotspot {
        rect: [0.12, 0.45, 0.22, 0.3],
        label: "Banana".into(),
        info: "Riper bananas make the smoothie sweeter.".into(),
        jump: Jump::Continue,
    });
    let h2 = p.add_interaction(s, 18.0, Kind::Hotspot {
        rect: [0.62, 0.40, 0.24, 0.32],
        label: "Berries".into(),
        info: "Frozen berries work too and make it colder.".into(),
        jump: Jump::Continue,
    });
    for id in [h1, h2] {
        if let Some(i) = p.scene_mut(s).and_then(|sc| sc.interactions.iter_mut().find(|i| i.id == id)) {
            i.show_for = 10.0;
        }
    }
    p.add_interaction(s, 34.0, mc(
        "Which ingredient makes the smoothie creamy?",
        &[
            ("Yoghurt", true, "Yes, yoghurt adds creaminess."),
            ("Honey", false, "Honey adds sweetness."),
            ("Berries", false, "Berries add flavour and colour."),
        ],
        5,
    ));
    p.add_interaction(s, 44.0, Kind::PauseScreen { text: "Your turn: blend everything for 30 seconds, then press Continue.".into() });
    p.add_interaction(s, 52.0, mc(
        "True or false: you should fill the blender to the very top.",
        &[("True", false, "Leave room, or it can overflow."), ("False", true, "Right, leave some room.")],
        5,
    ));
    if let Some(sc) = p.scene_mut(s) {
        sc.chapters = vec![chap(0.0, "Intro"), chap(8.0, "Ingredients"), chap(25.0, "Blending"), chap(50.0, "Serving")];
        sc.captions = vec![
            cap(1.0, 7.0, "Hi! Let's make a quick berry smoothie."),
            cap(10.0, 17.0, "Here's everything you need."),
            cap(19.0, 27.0, "Tap the ingredients to learn more."),
            cap(28.0, 33.0, "Into the blender it goes."),
            cap(46.0, 51.0, "Pour it into a glass and enjoy."),
        ];
    }
    p
}

fn example_branching() -> Project {
    let mut p = Project::new_blank();
    p.scenes.clear();
    p.title = "Choose your path".into();
    p.description = "A branching story: the viewer's choices decide which clip plays next and which ending they reach.".into();
    let intro = p.add_scene("The new job", 24.0, 3, [40.0, 160.0]);
    let help = p.add_scene("You help a colleague", 26.0, 2, [320.0, 50.0]);
    let alone = p.add_scene("You work alone", 26.0, 1, [320.0, 270.0]);
    let good = p.add_scene("Ending: team player", 12.0, 0, [600.0, 60.0]);
    let mixed = p.add_scene("Ending: lesson learned", 12.0, 4, [600.0, 260.0]);
    p.start_scene = intro;
    p.add_interaction(intro, 20.0, Kind::Choice {
        prompt: "A colleague looks stuck on a task. What do you do?".into(),
        options: vec![
            ChoiceOption { text: "Offer to help".into(), jump: Jump::Scene(help) },
            ChoiceOption { text: "Focus on my own work".into(), jump: Jump::Scene(alone) },
        ],
    });
    p.add_interaction(help, 18.0, Kind::Choice {
        prompt: "Your manager asks who solved the problem.".into(),
        options: vec![
            ChoiceOption { text: "\"We did it together\"".into(), jump: Jump::Scene(good) },
            ChoiceOption { text: "\"I did\"".into(), jump: Jump::Scene(mixed) },
        ],
    });
    p.add_interaction(alone, 10.0, Kind::Info { title: "Meanwhile…".into(), body: "The deadline is getting close for the whole team.".into() });
    p.add_interaction(alone, 20.0, Kind::Choice {
        prompt: "The team misses the deadline. What now?".into(),
        options: vec![
            ChoiceOption { text: "Go back and offer help".into(), jump: Jump::Scene(help) },
            ChoiceOption { text: "Carry on".into(), jump: Jump::Scene(mixed) },
        ],
    });
    for (id, text) in [
        (intro, "It's your first week. The office is busy."),
        (help, "You sit down together and work through it."),
        (alone, "You finish your own list early."),
        (good, "The team trusts you. Great start!"),
        (mixed, "Every choice teaches something. Try another path?"),
    ] {
        if let Some(s) = p.scene_mut(id) {
            s.captions = vec![cap(1.0, 9.0, text)];
            s.at_end = Jump::Finish;
        }
    }
    if let Some(s) = p.scene_mut(mixed) {
        s.interactions.clear();
    }
    let retry = p.alloc_id();
    if let Some(s) = p.scene_mut(mixed) {
        s.interactions.push(Interaction {
            id: retry,
            time: 10.0,
            show_for: 5.0,
            pause: false,
            kind: Kind::Choice {
                prompt: "Want to try a different path?".into(),
                options: vec![
                    ChoiceOption { text: "Start again".into(), jump: Jump::Scene(intro) },
                    ChoiceOption { text: "Finish".into(), jump: Jump::Finish },
                ],
            },
        });
    }
    p
}

/// A real YouTube video (Gamers Nexus) turned into a lesson. Chapters follow the
/// video's own timestamps; questions are based on its description and chapter list.
/// It plays in the web version; here a sample clip stands in.
fn example_youtube() -> Project {
    let mut p = Project::new_blank();
    p.scenes.clear();
    p.title = "Video demo: AMD and gamers".into();
    p.description = "A Gamers Nexus video (YouTube) turned into a lesson: chapters, a few checks along the way and a shortcut to the conclusion. Questions are based on the video's own description and chapter list.".into();
    p.pass_percent = 60;
    let s = p.add_scene("AMD's message to gamers", 1788.0, 3, [100.0, 140.0]);
    p.start_scene = s;
    let yt = "https://www.youtube.com/watch?v=qo9NYpDJkSo";
    if let Some(sc) = p.scene_mut(s) {
        sc.video = Some(yt.into());
    }
    let info = p.add_interaction(s, 2.0, Kind::Info {
        title: "About this video".into(),
        body: "How AMD talks to gamers while calling itself a data-center-first company. Use the chapters to jump around; questions pop up at the end of some chapters.".into(),
    });
    let link = p.add_interaction(s, 3.0, Kind::Link { label: "Watch on YouTube".into(), url: yt.into() });
    p.add_interaction(s, 222.0, mc(
        "What does the video say AMD recently called itself?",
        &[
            ("A data-center-first company", true, "Right: that's the starting point of the video."),
            ("A gaming-only company", false, "The video points out the opposite."),
            ("A phone maker", false, "Not this one."),
        ],
        10,
    ));
    p.add_interaction(s, 568.0, Kind::OpenQuestion {
        prompt: "AMD's posts told gamers it has always been on their side. Having heard this part, how convincing do you find that message, and why?".into(),
    });
    p.add_interaction(s, 1075.0, mc(
        "Which part of the market does the video say AMD now puts first?",
        &[
            ("Data centers", true, "Yes: the chapters keep coming back to this."),
            ("Game consoles", false, "Consoles aren't the focus here."),
            ("Budget graphics cards", false, "The chapter list points to high-end products instead."),
        ],
        10,
    ));
    p.add_interaction(s, 1176.0, Kind::Choice {
        prompt: "Next up: AMD's price increases. Keep watching, or jump straight to the conclusion?".into(),
        options: vec![
            ChoiceOption { text: "Keep watching".into(), jump: Jump::Continue },
            ChoiceOption { text: "Jump to the conclusion".into(), jump: Jump::Time(1317.0) },
        ],
    });
    p.add_interaction(s, 1315.0, mc(
        "Which topic comes right before the conclusion?",
        &[
            ("AMD price increases", true, "Correct."),
            ("Low risk of smuggling", false, "That one comes early in the video."),
            ("High-end products only", false, "That's a bit earlier."),
        ],
        10,
    ));
    if let Some(sc) = p.scene_mut(s) {
        for i in &mut sc.interactions {
            if i.id == info {
                i.show_for = 12.0;
            }
            if i.id == link {
                i.show_for = 20.0;
            }
        }
        sc.chapters = vec![
            chap(0.0, "AMD a data center company"),
            chap(226.0, "Low risk of smuggling"),
            chap(446.0, "AMD marketing finds foot"),
            chap(572.0, "AMD reduces consumer focus"),
            chap(905.0, "High-end products only"),
            chap(1078.0, "AMD loves data centers"),
            chap(1180.0, "AMD price increases"),
            chap(1317.0, "Conclusion"),
        ];
    }
    p
}

/// The same video cut into scenes by its own chapter timestamps and arranged as
/// routes the viewer picks from a hub. Uses every kind of interaction.
fn example_routes() -> Project {
    let yt = "https://www.youtube.com/watch?v=qo9NYpDJkSo";
    let mut p = Project::new_blank();
    p.scenes.clear();
    p.title = "Story map demo: AMD routes".into();
    p.description = "The Gamers Nexus video cut into routes. Pick what to explore from the hub, come back for another route, then watch the conclusion. A score of 40% or more unlocks the 'ready to discuss' ending. Questions are based on the video's description and chapter titles.".into();
    p.pass_percent = 40;
    let part = |p: &mut Project, name: &str, start: f32, end: f32, pos: [f32; 2]| {
        let id = p.add_scene(name, end - start, 3, pos);
        if let Some(s) = p.scene_mut(id) {
            s.video = Some(yt.into());
            s.clip_start = start;
        }
        id
    };
    let start = part(&mut p, "Start: AMD, a data center company", 0.0, 226.0, [40.0, 230.0]);
    let hub = p.add_scene("Hub: choose a route", 12.0, 3, [320.0, 230.0]);
    let mkt = part(&mut p, "Route A · Marketing finds its foot", 446.0, 572.0, [600.0, 40.0]);
    let focus = part(&mut p, "Route A · Less focus on consumers", 572.0, 905.0, [880.0, 40.0]);
    let high = part(&mut p, "Route B · High-end products only", 905.0, 1078.0, [600.0, 230.0]);
    let price = part(&mut p, "Route B · Price increases", 1180.0, 1317.0, [880.0, 230.0]);
    let dc = part(&mut p, "Route C · AMD loves data centers", 1078.0, 1180.0, [600.0, 420.0]);
    let smug = part(&mut p, "Route C · Low risk of smuggling", 226.0, 446.0, [880.0, 420.0]);
    let concl = part(&mut p, "Conclusion", 1317.0, 1788.0, [320.0, 560.0]);
    let ready = p.add_scene("Ending: ready to discuss", 16.0, 2, [40.0, 720.0]);
    let recap = p.add_scene("Ending: quick recap", 30.0, 1, [600.0, 720.0]);
    p.start_scene = start;
    let hub_choice = |prompt: &str| Kind::Choice {
        prompt: prompt.into(),
        options: vec![
            ChoiceOption { text: "Route A: marketing vs. reality".into(), jump: Jump::Scene(mkt) },
            ChoiceOption { text: "Route B: products and prices".into(), jump: Jump::Scene(high) },
            ChoiceOption { text: "Route C: data centers and exports".into(), jump: Jump::Scene(dc) },
            ChoiceOption { text: "Go to the conclusion".into(), jump: Jump::Scene(concl) },
        ],
    };
    fn show(p: &mut Project, scene: u32, id: u32, secs: f32) {
        if let Some(i) = p.scene_mut(scene).and_then(|s| s.interactions.iter_mut().find(|i| i.id == id)) {
            i.show_for = secs;
        }
    }
    fn info(title: &str, body: &str) -> Kind {
        Kind::Info { title: title.into(), body: body.into() }
    }

    // Start
    let i = p.add_interaction(start, 2.0, info("How this demo works", "The video is cut into routes. At the end of each part you choose where to go; the story map shows every path. Questions add to your score."));
    show(&mut p, start, i, 12.0);
    let i = p.add_interaction(start, 6.0, Kind::Hotspot {
        rect: [0.03, 0.06, 0.32, 0.24],
        label: "Who's this?".into(),
        info: "Steve Burke, host and writer at Gamers Nexus (from the video's credits).".into(),
        jump: Jump::Continue,
    });
    show(&mut p, start, i, 20.0);
    let i = p.add_interaction(start, 6.0, Kind::Link { label: "Video on YouTube".into(), url: yt.into() });
    show(&mut p, start, i, 20.0);
    let mut q1 = mc(
        "What does AMD now call itself, according to the video?",
        &[
            ("A data-center-first company", true, "Right: everything else in the video builds on this."),
            ("A gaming company first", false, "Not quite: let's rewatch the last bit."),
            ("A chip importer", false, "Not quite: let's rewatch the last bit."),
        ],
        10,
    );
    if let Kind::Question { on_wrong, .. } = &mut q1 {
        *on_wrong = Jump::Time(150.0);
    }
    p.add_interaction(start, 200.0, q1);

    // The hub every route comes back to
    let i = p.add_interaction(hub, 1.0, info("Pick a route", "Each route is two parts of the video. Come back here afterwards for another one, or go to the conclusion when you're done."));
    show(&mut p, hub, i, 9.0);
    p.add_interaction(hub, 10.0, hub_choice("Where do you want to go?"));

    // Route A
    p.add_interaction(mkt, 2.0, Kind::PauseScreen {
        text: "Before this part: think of a time a brand said it has always been on your side. Did you believe it? Press Continue when you're ready.".into(),
    });
    p.add_interaction(mkt, 118.0, Kind::OpenQuestion {
        prompt: "In your own words: what is the mixed message the video says AMD sends to gamers?".into(),
    });
    let i = p.add_interaction(focus, 3.0, info("Watch for", "How AMD's 'data-center-first' line compares with what it tells gamers."));
    show(&mut p, focus, i, 10.0);
    let mut qa = mc(
        "Which customers does the video say AMD is putting first now?",
        &[("Data centers", true, "Yes."), ("Gamers", false, "That's the message the video questions."), ("Console makers", false, "Not the focus here.")],
        10,
    );
    if let Kind::Question { allow_retry, .. } = &mut qa {
        *allow_retry = true;
    }
    p.add_interaction(focus, 325.0, qa);

    // Route B
    let i = p.add_interaction(high, 5.0, Kind::Hotspot {
        rect: [0.58, 0.52, 0.36, 0.38],
        label: "High-end only?".into(),
        info: "This part looks at AMD concentrating on its most expensive products.".into(),
        jump: Jump::Continue,
    });
    show(&mut p, high, i, 25.0);
    p.add_interaction(high, 168.0, mc(
        "The chapter is called 'High-end products only'. What does that suggest for budget buyers?",
        &[
            ("Fewer new low-cost options", true, "That's the worry the chapter raises."),
            ("Free upgrades", false, "Unfortunately not."),
            ("Nothing changes", false, "The chapter suggests otherwise."),
        ],
        10,
    ));
    let i = p.add_interaction(price, 3.0, info("Price check", "Note the price changes mentioned; you'll be asked what you think."));
    show(&mut p, price, i, 10.0);
    p.add_interaction(price, 128.0, Kind::OpenQuestion { prompt: "Would these price increases change what you'd buy next? Why or why not?".into() });

    // Route C
    p.add_interaction(dc, 95.0, mc(
        "What does this chapter's title say AMD loves?",
        &[("Data centers", true, "Correct."), ("Gamers", false, "Listen again to the title."), ("Consoles", false, "Not this time.")],
        10,
    ));
    let i = p.add_interaction(smug, 3.0, info("Context", "This part covers the risk of chips being smuggled abroad; the chapter title calls it low."));
    show(&mut p, smug, i, 10.0);
    p.add_interaction(smug, 210.0, mc(
        "How does the chapter title describe the risk of smuggling?",
        &[("Low", true, "Right."), ("High", false, "The title says the opposite."), ("It isn't mentioned", false, "It's the whole chapter.")],
        10,
    ));

    // Conclusion and endings
    let i = p.add_interaction(concl, 3.0, info("Last part", "The wrap-up. At the end, your score picks your ending."));
    show(&mut p, concl, i, 10.0);
    p.add_interaction(concl, 450.0, mc(
        "Which chapter comes right before the conclusion?",
        &[("AMD price increases", true, "Correct."), ("Low risk of smuggling", false, "That one is early in the video."), ("High-end products only", false, "That's earlier.")],
        10,
    ));
    p.add_interaction(concl, 469.0, Kind::ScoreCheck { min_percent: 40, pass: Jump::Scene(ready), fail: Jump::Scene(recap) });
    let i = p.add_interaction(ready, 1.0, info("You're ready to discuss", "You got most checks right. Bring your open answers to the discussion."));
    show(&mut p, ready, i, 14.0);
    let i = p.add_interaction(ready, 4.0, Kind::Link { label: "Watch the full video".into(), url: yt.into() });
    show(&mut p, ready, i, 12.0);
    let i = p.add_interaction(recap, 1.0, info("Quick recap", "Data center company → smuggling risk → marketing → less consumer focus → high-end only → loves data centers → price increases → conclusion."));
    show(&mut p, recap, i, 20.0);
    p.add_interaction(recap, 26.0, Kind::Choice {
        prompt: "What now?".into(),
        options: vec![
            ChoiceOption { text: "Try another route".into(), jump: Jump::Scene(hub) },
            ChoiceOption { text: "Finish".into(), jump: Jump::Finish },
        ],
    });
    for (id, next) in [(start, hub), (mkt, focus), (high, price), (dc, smug), (focus, hub), (price, hub), (smug, hub)] {
        if let Some(s) = p.scene_mut(id) {
            s.at_end = Jump::Scene(next);
        }
    }
    if let Some(s) = p.scene_mut(hub) {
        s.captions = vec![cap(0.5, 9.0, "Three routes through the video. Which one first?")];
    }
    if let Some(s) = p.scene_mut(ready) {
        s.captions = vec![cap(1.0, 10.0, "Nice work: you explored the video and passed the checks.")];
    }
    if let Some(s) = p.scene_mut(recap) {
        s.captions = vec![cap(1.0, 12.0, "Here's the whole video in one line. Try another route?")];
    }
    p
}

fn example_tour() -> Project {
    let mut p = Project::new_blank();
    p.scenes.clear();
    p.title = "Product tour".into();
    p.description = "A product walkthrough with hotspots that open details or jump to a feature, plus a link button.".into();
    let s = p.add_scene("Meet the lamp", 48.0, 4, [100.0, 140.0]);
    p.start_scene = s;
    let hs = [
        (6.0, [0.18, 0.25, 0.2, 0.3], "Dimmer", "Turn the ring to set the brightness.", Jump::Time(22.0)),
        (6.0, [0.58, 0.5, 0.22, 0.28], "Base", "A weighted base so it never tips over.", Jump::Continue),
    ];
    for (t, rect, label, info, jump) in hs {
        let id = p.add_interaction(s, t, Kind::Hotspot { rect, label: label.into(), info: info.into(), jump });
        if let Some(i) = p.scene_mut(s).and_then(|sc| sc.interactions.iter_mut().find(|i| i.id == id)) {
            i.show_for = 12.0;
        }
    }
    p.add_interaction(s, 30.0, Kind::Choice {
        prompt: "What would you like to see next?".into(),
        options: vec![
            ChoiceOption { text: "Colours".into(), jump: Jump::Time(32.0) },
            ChoiceOption { text: "Setup in 1 minute".into(), jump: Jump::Time(40.0) },
        ],
    });
    p.add_interaction(s, 42.0, Kind::Link { label: "Read more".into(), url: "https://example.com".into() });
    if let Some(sc) = p.scene_mut(s) {
        sc.chapters = vec![chap(0.0, "Overview"), chap(20.0, "Dimmer"), chap(32.0, "Colours"), chap(40.0, "Setup")];
        sc.captions = vec![
            cap(1.0, 5.0, "Here's a quick tour."),
            cap(7.0, 16.0, "Click the glowing spots to explore."),
            cap(22.0, 28.0, "The dimmer ring sets the mood."),
            cap(32.0, 38.0, "Three colours to choose from."),
            cap(40.0, 46.0, "Plug it in and you're done."),
        ];
    }
    p
}
