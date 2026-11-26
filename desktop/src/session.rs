//! What one viewer did during one viewing: every choice, answer and click,
//! the score and whether they finished. Exported as a table or as
//! learning-record statements (actor / verb / object / result).

use serde::Serialize;
use std::collections::HashSet;
use std::time::{Instant, SystemTime, UNIX_EPOCH};

#[derive(Clone, Debug, Serialize)]
pub struct Event {
    /// Seconds since the viewing started.
    pub at: f64,
    pub scene: String,
    pub video_time: f32,
    pub verb: String,
    pub object: String,
    pub detail: String,
    pub success: Option<bool>,
    pub points: u32,
}

#[derive(Clone, Debug)]
pub struct Session {
    pub learner: String,
    pub project: String,
    pub started_at: String,
    started: Instant,
    pub ended_after: Option<f64>,
    pub events: Vec<Event>,
    pub score: u32,
    pub max_score: u32,
    pub pass_percent: u32,
    pub answered: HashSet<u32>,
    pub completed: bool,
    pub path: Vec<String>,
}

impl Session {
    pub fn new(learner: &str, project: &str, max_score: u32, pass_percent: u32) -> Self {
        Session {
            learner: learner.to_string(),
            project: project.to_string(),
            started_at: now_iso(),
            started: Instant::now(),
            ended_after: None,
            events: vec![],
            score: 0,
            max_score,
            pass_percent,
            answered: HashSet::new(),
            completed: false,
            path: vec![],
        }
    }

    pub fn elapsed(&self) -> f64 {
        self.ended_after.unwrap_or_else(|| self.started.elapsed().as_secs_f64())
    }

    pub fn log(&mut self, scene: &str, video_time: f32, verb: &str, object: &str, detail: &str, success: Option<bool>, points: u32) {
        self.events.push(Event {
            at: self.started.elapsed().as_secs_f64(),
            scene: scene.to_string(),
            video_time,
            verb: verb.to_string(),
            object: object.to_string(),
            detail: detail.to_string(),
            success,
            points,
        });
    }

    pub fn percent(&self) -> u32 {
        if self.max_score == 0 {
            100
        } else {
            (self.score * 100 + self.max_score / 2) / self.max_score
        }
    }

    pub fn passed(&self) -> bool {
        self.percent() >= self.pass_percent
    }

    pub fn questions_answered(&self) -> usize {
        self.answered.len()
    }

    pub fn finish(&mut self) {
        if !self.completed {
            self.completed = true;
            self.ended_after = Some(self.started.elapsed().as_secs_f64());
        }
    }

    pub fn has_activity(&self) -> bool {
        !self.events.is_empty()
    }
}

fn csv_cell(s: &str) -> String {
    if s.contains(',') || s.contains('"') || s.contains('\n') {
        format!("\"{}\"", s.replace('"', "\"\""))
    } else {
        s.to_string()
    }
}

/// Every event of every viewing as a spreadsheet table.
pub fn to_csv(sessions: &[&Session]) -> String {
    let mut out = String::from("viewer,lesson,started,seconds_in,scene,video_time,action,item,detail,correct,points\n");
    for s in sessions {
        for e in &s.events {
            let ok = match e.success {
                Some(true) => "yes",
                Some(false) => "no",
                None => "",
            };
            out.push_str(&format!(
                "{},{},{},{:.1},{},{:.1},{},{},{},{},{}\n",
                csv_cell(&s.learner),
                csv_cell(&s.project),
                s.started_at,
                e.at,
                csv_cell(&e.scene),
                e.video_time,
                csv_cell(&e.verb),
                csv_cell(&e.object),
                csv_cell(&e.detail),
                ok,
                e.points
            ));
        }
    }
    out
}

/// Learning-record statements in the actor / verb / object / result shape LMSs understand.
pub fn to_statements(sessions: &[&Session]) -> String {
    let mut all = vec![];
    for s in sessions {
        let actor = serde_json::json!({ "name": s.learner, "objectType": "Agent" });
        for e in &s.events {
            let mut result = serde_json::Map::new();
            if let Some(ok) = e.success {
                result.insert("success".into(), ok.into());
                result.insert("score".into(), serde_json::json!({ "raw": e.points }));
            }
            if !e.detail.is_empty() {
                result.insert("response".into(), e.detail.clone().into());
            }
            all.push(serde_json::json!({
                "actor": actor,
                "verb": { "display": { "en-US": e.verb } },
                "object": { "definition": { "name": { "en-US": e.object } }, "extensions": { "scene": e.scene, "videoTime": e.video_time } },
                "result": result,
                "context": { "lesson": s.project, "sessionStarted": s.started_at, "secondsIn": (e.at * 10.0).round() / 10.0 }
            }));
        }
        all.push(serde_json::json!({
            "actor": actor,
            "verb": { "display": { "en-US": if s.completed { "completed" } else { "suspended" } } },
            "object": { "definition": { "name": { "en-US": s.project } } },
            "result": { "completion": s.completed, "success": s.passed(), "score": { "raw": s.score, "max": s.max_score, "scaled": s.percent() as f32 / 100.0 } },
        }));
    }
    serde_json::to_string_pretty(&all).unwrap_or_default()
}

pub fn now_iso() -> String {
    let secs = SystemTime::now().duration_since(UNIX_EPOCH).map(|d| d.as_secs() as i64).unwrap_or(0);
    let days = secs.div_euclid(86_400);
    let rem = secs.rem_euclid(86_400);
    // Civil date from days since 1970 (Howard Hinnant's algorithm).
    let z = days + 719_468;
    let era = z.div_euclid(146_097);
    let doe = z - era * 146_097;
    let yoe = (doe - doe / 1460 + doe / 36_524 - doe / 146_096) / 365;
    let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
    let mp = (5 * doy + 2) / 153;
    let d = doy - (153 * mp + 2) / 5 + 1;
    let m = if mp < 10 { mp + 3 } else { mp - 9 };
    let y = yoe + era * 400 + if m <= 2 { 1 } else { 0 };
    format!("{y:04}-{m:02}-{d:02}T{:02}:{:02}:{:02}Z", rem / 3600, rem % 3600 / 60, rem % 60)
}
