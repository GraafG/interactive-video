// ClipQuest web: make and play interactive videos in any browser, phones included.
// Lessons use the same file format as the desktop app. Everything is kept in this
// browser (lessons and results in local storage, picked video files in IndexedDB).
"use strict";

// ---------------------------------------------------------------- helpers

const $ = (sel, root = document) => root.querySelector(sel);

function h(tag, attrs, ...children) {
  const el = document.createElement(tag);
  for (const [k, v] of Object.entries(attrs || {})) {
    if (v === null || v === undefined || v === false) continue;
    if (k.startsWith("on")) el.addEventListener(k.slice(2), v);
    else if (k === "style" && typeof v === "object") Object.assign(el.style, v);
    else if (k === "class") el.className = v;
    else if (k === "html") el.innerHTML = v;
    else if (v === true) el.setAttribute(k, "");
    else el.setAttribute(k, v);
  }
  for (const c of children.flat()) {
    if (c === null || c === undefined || c === false) continue;
    el.append(c instanceof Node ? c : document.createTextNode(String(c)));
  }
  return el;
}

const ICONS = {
  play: '<svg viewBox="0 0 24 24"><path style="fill:currentColor;stroke:none" d="M8 5v14l11-7z"/></svg>',
  pause: '<svg viewBox="0 0 24 24"><path style="fill:currentColor;stroke:none" d="M7 5h4v14H7zM13 5h4v14h-4z"/></svg>',
  back: '<svg viewBox="0 0 24 24"><path d="M3 12a9 9 0 1 0 3-6.7"/><path d="M3 4v5h5"/></svg>',
  full: '<svg viewBox="0 0 24 24"><path d="M4 9V4h5M20 9V4h-5M4 15v5h5M20 15v5h-5"/></svg>',
  watch: '<svg viewBox="0 0 24 24"><rect x="3" y="5" width="18" height="14" rx="3"/><path d="M10 9l5 3-5 3z"/></svg>',
  edit: '<svg viewBox="0 0 24 24"><path d="M4 20h4L19 9l-4-4L4 16z"/><path d="M14 6l4 4"/></svg>',
  map: '<svg viewBox="0 0 24 24"><rect x="2" y="9" width="6" height="6" rx="1.5"/><rect x="16" y="3" width="6" height="6" rx="1.5"/><rect x="16" y="15" width="6" height="6" rx="1.5"/><path d="M8 12h4m0 0V6h4m-4 6v6h4"/></svg>',
  results: '<svg viewBox="0 0 24 24"><path d="M4 20V10M10 20V4M16 20v-7M22 20H2"/></svg>',
};
const icon = (name) => h("span", { html: ICONS[name], style: { display: "contents" } });

function fmtTime(t) {
  t = Math.max(0, Math.floor(t || 0));
  return `${Math.floor(t / 60)}:${String(t % 60).padStart(2, "0")}`;
}
const clamp = (v, a, b) => Math.min(b, Math.max(a, v));
const trim = (s, n) => (s.length > n ? s.slice(0, n) + "…" : s);

let toastTimer = 0;
function toast(msg) {
  const t = $("#toast");
  t.textContent = msg;
  t.classList.add("show");
  clearTimeout(toastTimer);
  toastTimer = setTimeout(() => t.classList.remove("show"), 2200);
}

const isWide = () => window.matchMedia("(min-width: 900px)").matches;

// ---------------------------------------------------------------- the lesson format

const kindType = (k) => Object.keys(k)[0];
const kd = (k) => k[kindType(k)];
const jumpType = (j) => (typeof j === "string" ? j : Object.keys(j)[0]);
const jumpValue = (j) => (typeof j === "string" ? null : j[jumpType(j)]);

const TYPE_NAMES = {
  Info: "Text card", Question: "Quiz question", OpenQuestion: "Open question", Hotspot: "Hotspot",
  Choice: "Choice (branch)", PauseScreen: "Pause screen", ScoreCheck: "Score check", Link: "Link button",
};
const COLOURS = {
  Info: "#5aaaff", Question: "#ffbe3c", OpenQuestion: "#f08c3c", Hotspot: "#50dca0",
  Choice: "#c878ff", PauseScreen: "#b4b4be", ScoreCheck: "#ff6e8c", Link: "#6ed2f0",
};
const BLOCKS = new Set(["Question", "OpenQuestion", "Choice", "PauseScreen"]);
const JUMP_NAMES = { Continue: "Keep playing", Time: "Jump to time", Scene: "Go to scene", Finish: "Finish" };

function summary(k) {
  const d = kd(k);
  const t = kindType(k);
  const s = t === "Info" ? d.title : t === "Hotspot" || t === "Link" ? d.label : t === "PauseScreen" ? d.text
    : t === "ScoreCheck" ? `Score at least ${d.min_percent}%?` : d.prompt;
  return trim(s || "", 40);
}

function jumpsOf(k) {
  const d = kd(k);
  switch (kindType(k)) {
    case "Question": return [["right", d.on_correct], ["wrong", d.on_wrong]];
    case "Hotspot": return [[d.label, d.jump]];
    case "Choice": return d.options.map((o) => [o.text, o.jump]);
    case "ScoreCheck": return [["pass", d.pass], ["fail", d.fail]];
    default: return [];
  }
}

const maxPoints = (p) => p.scenes.flatMap((s) => s.interactions).reduce((n, i) => n + (kindType(i.kind) === "Question" ? kd(i.kind).points : 0), 0);
const sceneName = (p, id) => sceneOf(p, id)?.name ?? "(missing scene)";
/** Where a scene starts inside its video (a scene can be one part of a longer video). */
const clipStart = (s) => (s && s.video ? s.clip_start || 0 : 0);

// ---------------------------------------------------------------- storage

const store = {
  get(key, fallback) {
    try { const v = localStorage.getItem(key); return v ? JSON.parse(v) : fallback; } catch { return fallback; }
  },
  set(key, value) {
    try { localStorage.setItem(key, JSON.stringify(value)); } catch { toast("This browser's storage is full"); }
  },
};

let dbPromise = null;
function db() {
  if (!dbPromise) {
    dbPromise = new Promise((resolve, reject) => {
      const req = indexedDB.open("clipquest", 1);
      req.onupgradeneeded = () => req.result.createObjectStore("videos");
      req.onsuccess = () => resolve(req.result);
      req.onerror = () => reject(req.error);
    });
  }
  return dbPromise;
}
async function putVideo(key, blob) {
  const d = await db();
  return new Promise((res, rej) => {
    const tx = d.transaction("videos", "readwrite");
    tx.objectStore("videos").put(blob, key);
    tx.oncomplete = res; tx.onerror = () => rej(tx.error);
  });
}
async function getVideo(key) {
  const d = await db();
  return new Promise((res) => {
    const req = d.transaction("videos").objectStore("videos").get(key);
    req.onsuccess = () => res(req.result || null);
    req.onerror = () => res(null);
  });
}

/** The video id of a YouTube address, or null. */
function ytId(src) {
  const m = /(?:youtube(?:-nocookie)?\.com\/(?:watch\?(?:.*&)?v=|embed\/|shorts\/|live\/)|youtu\.be\/)([\w-]{11})/.exec(src || "");
  return m ? m[1] : null;
}

let ytApi = null;
/** YouTube's embedded-player script, loaded once. */
function loadYouTube() {
  if (!ytApi) {
    ytApi = new Promise((resolve, reject) => {
      if (window.YT?.Player) return resolve(window.YT);
      window.onYouTubeIframeAPIReady = () => resolve(window.YT);
      const s = document.createElement("script");
      s.src = "https://www.youtube.com/iframe_api";
      s.onerror = () => { ytApi = null; reject(new Error("offline")); };
      document.head.append(s);
    });
  }
  return ytApi;
}

// One set of controls for both kinds of media: a <video> element or a YouTube player.
const media = {
  time: (m) => (m.kind === "yt" ? m.yt.getCurrentTime?.() ?? 0 : m.el.currentTime),
  seek(m, t) {
    if (m.kind === "yt") m.yt.seekTo?.(t, true);
    else m.el.currentTime = t;
  },
  paused: (m) => (m.kind === "yt" ? ![1, 3].includes(m.yt.getPlayerState?.()) : m.el.paused),
  ended: (m) => (m.kind === "yt" ? m.yt.getPlayerState?.() === 0 : m.el.ended),
  play(m) {
    if (m.kind === "yt") {
      const now = performance.now();
      if (now - (m.lastPlay || 0) < 600) return; // the player takes a moment to start
      m.lastPlay = now;
      m.yt.mute?.();
      m.yt.playVideo?.();
    } else m.el.play().catch(() => {});
  },
  pause: (m) => (m.kind === "yt" ? m.yt.pauseVideo?.() : m.el.pause()),
  rate(m, r) {
    if (m.kind === "yt") { if (m.rate !== r) { m.rate = r; m.yt.setPlaybackRate?.(r); } }
    else m.el.playbackRate = r;
  },
};

const urlCache = new Map();
/** A playable address for a scene's video, or null for the sample clip (and YouTube, which has its own player). */
async function videoUrl(src) {
  if (!src || ytId(src)) return null;
  if (urlCache.has(src)) return urlCache.get(src);
  let url = null;
  if (src.startsWith("idb:")) {
    const blob = await getVideo(src);
    url = blob ? URL.createObjectURL(blob) : null;
  } else if (/^(https?:|blob:|data:)/i.test(src)) {
    url = src;
  }
  urlCache.set(src, url);
  return url;
}

// ---------------------------------------------------------------- the sample clip

const LOOK_COLOURS = [
  ["#144678", "#2896be", "#a0e6ff"], ["#782850", "#f08246", "#ffdc8c"], ["#144628", "#469650", "#c8f0a0"],
  ["#0c0e28", "#32326e", "#aaaaff"], ["#3c3c42", "#9696a0", "#faf0dc"], ["#82328c", "#f078aa", "#ffe678"],
];

function drawSample(cv, scene, time, withText = true) {
  const dpr = window.devicePixelRatio || 1;
  const w = cv.clientWidth, hgt = cv.clientHeight;
  if (!w || !hgt) return;
  if (cv.width !== Math.round(w * dpr) || cv.height !== Math.round(hgt * dpr)) {
    cv.width = Math.round(w * dpr); cv.height = Math.round(hgt * dpr);
  }
  const c = cv.getContext("2d");
  c.setTransform(dpr, 0, 0, dpr, 0, 0);
  const [top, bottom, accent] = LOOK_COLOURS[(scene.look || 0) % LOOK_COLOURS.length];
  const g = c.createLinearGradient(0, 0, 0, hgt);
  g.addColorStop(0, top); g.addColorStop(1, bottom);
  c.fillStyle = g; c.fillRect(0, 0, w, hgt);
  for (let i = 0; i < 7; i++) {
    const x = w * (0.1 + 0.8 * (0.5 + 0.5 * Math.sin(time * 0.13 * (1 + i * 0.21) + i * 1.7)));
    const y = hgt * (0.15 + 0.55 * (0.5 + 0.5 * Math.cos(time * 0.09 * (1 + i * 0.17) + i * 2.3)));
    c.globalAlpha = 0.18 + 0.05 * (i % 3);
    c.fillStyle = accent;
    c.beginPath(); c.arc(x, y, hgt * (0.05 + 0.03 * Math.abs(Math.sin(i * 0.9))), 0, Math.PI * 2); c.fill();
  }
  c.globalAlpha = 1;
  c.fillStyle = "rgba(0,0,0,.28)";
  c.beginPath(); c.moveTo(0, hgt);
  for (let s = 0; s <= 40; s++) {
    const fx = s / 40;
    c.lineTo(fx * w, hgt * (0.78 + 0.04 * Math.sin(fx * 9 + time * 0.4) + 0.02 * Math.sin(fx * 23 - time * 0.7)));
  }
  c.lineTo(w, hgt); c.fill();
  const frac = scene.duration > 0 ? clamp(time / scene.duration, 0, 1) : 0;
  c.globalAlpha = 0.85; c.fillStyle = accent;
  c.beginPath(); c.arc(w * (0.08 + 0.84 * frac), hgt * 0.3, hgt * 0.07, 0, Math.PI * 2); c.fill();
  c.globalAlpha = 1;
  if (withText && w > 200) {
    c.fillStyle = "rgba(255,255,255,.6)";
    c.textAlign = "center"; c.textBaseline = "middle";
    c.font = `${clamp(hgt * 0.075, 13, 40)}px system-ui, sans-serif`;
    c.fillText(scene.name, w / 2, hgt * 0.44, w * 0.9);
    c.font = "12px system-ui, sans-serif"; c.textAlign = "right"; c.textBaseline = "top";
    c.fillStyle = "rgba(255,255,255,.5)";
    c.fillText("sample clip", w - 10, 10);
  }
}

// ---------------------------------------------------------------- a viewing (results)

function newSession(learner, project) {
  return {
    learner, project: project.title, started_at: new Date().toISOString().slice(0, 19) + "Z",
    started_ms: Date.now(), ended_after: null, events: [], score: 0, max_score: maxPoints(project),
    pass_percent: project.pass_percent, answered: [], completed: false, path: [],
  };
}
const percent = (s) => (s.max_score === 0 ? 100 : Math.round((s.score * 100) / s.max_score));
const passed = (s) => percent(s) >= s.pass_percent;
const elapsed = (s) => s.ended_after ?? (Date.now() - s.started_ms) / 1000;

// ---------------------------------------------------------------- the player

class Player {
  constructor(project, learner) {
    this.p = project;
    this.scene = project.start_scene;
    this.time = 0;
    this.playing = false;
    this.started = false;
    this.finished = false;
    this.blocking = null;
    this.fired = new Set();
    this.dismissed = new Set();
    this.maxWatched = 0;
    this.pick = null;
    this.checked = null;
    this.openText = "";
    this.popup = null;
    this.speed = 1;
    this.captions = true;
    this.session = newSession(learner, project);
    this.media = null; // { el, ready } while a real video plays
  }

  get sceneObj() { return sceneOf(this.p, this.scene); }

  log(verb, object, detail = "", success = null, points = 0) {
    this.session.events.push({
      at: (Date.now() - this.session.started_ms) / 1000, scene: sceneName(this.p, this.scene),
      video_time: Math.round(this.time * 10) / 10, verb, object, detail, success, points,
    });
    saveResults();
  }

  start() {
    this.started = true;
    this.playing = true;
    this.session.learner = state.learner;
    this.session.path.push(sceneName(this.p, this.scene));
    this.log("started", this.p.title);
    this.log("entered scene", sceneName(this.p, this.scene));
  }

  resetCard() { this.pick = null; this.checked = null; this.openText = ""; this.popup = null; }

  seek(t) {
    const s = this.sceneObj;
    if (!s) return;
    t = clamp(t, 0, s.duration);
    if (this.p.prevent_skipping) t = Math.min(t, this.maxWatched);
    this.time = t;
    this.fired = new Set(s.interactions.filter((i) => i.time < t - 0.01).map((i) => i.id));
    this.dismissed.clear();
    this.popup = null;
    if (this.media?.ready) media.seek(this.media, clipStart(s) + t);
  }

  go(jump) {
    this.blocking = null;
    this.resetCard();
    const type = jumpType(jump);
    if (type === "Continue") this.playing = true;
    else if (type === "Time") {
      const t = jumpValue(jump);
      this.maxWatched = Math.max(this.maxWatched, t);
      this.seek(t);
      this.playing = true;
      this.log("jumped", fmtTime(t));
    } else if (type === "Scene") {
      const id = jumpValue(jump);
      if (!sceneOf(this.p, id)) return this.finish();
      this.scene = id;
      this.time = 0;
      this.maxWatched = 0;
      this.fired.clear();
      this.dismissed.clear();
      this.playing = true;
      const name = sceneName(this.p, id);
      this.session.path.push(name);
      this.log("entered scene", name);
    } else this.finish();
  }

  finish() {
    this.finished = true;
    this.playing = false;
    const s = this.session;
    this.log("completed", this.p.title, `score ${s.score}/${s.max_score} (${percent(s)}%)`, passed(s), s.score);
    if (!s.completed) { s.completed = true; s.ended_after = (Date.now() - s.started_ms) / 1000; }
    saveResults();
  }

  tick(dt) {
    if (!this.started || this.finished || this.blocking !== null || !this.playing) return;
    const s = this.sceneObj;
    if (!s) return;
    const fromVideo = this.media?.ready;
    let newT = fromVideo ? media.time(this.media) - clipStart(s) : this.time + dt * this.speed;
    // Until the video has caught up with a jump (or while it stalls), keep our own clock
    // and ask it again now and then, so a scene never ends early or runs backwards.
    if (fromVideo && Math.abs(newT - this.time) > 2) {
      newT = this.time + (media.paused(this.media) ? 0 : dt * this.speed);
      const now = performance.now();
      if (now - (this.media.lastSync || 0) > 1200) { this.media.lastSync = now; media.seek(this.media, clipStart(s) + newT); }
    }
    newT = Math.min(newT, s.duration);
    for (const i of s.interactions) {
      if (this.fired.has(i.id) || i.time > newT) continue;
      this.fired.add(i.id);
      this.time = Math.max(i.time, this.time);
      const t = kindType(i.kind);
      if (t === "ScoreCheck") {
        const d = kd(i.kind);
        const ok = percent(this.session) >= d.min_percent;
        this.log("score checked", `at least ${d.min_percent}%`, `${percent(this.session)}%`, ok);
        this.go(ok ? d.pass : d.fail);
        return;
      }
      if (BLOCKS.has(t) || i.pause) {
        this.blocking = i.id;
        this.playing = false;
        this.resetCard();
        if (fromVideo) { media.pause(this.media); media.seek(this.media, clipStart(s) + i.time); }
        this.log("reached", summary(i.kind), TYPE_NAMES[t]);
        return;
      }
    }
    this.time = newT;
    this.maxWatched = Math.max(this.maxWatched, newT);
    const ended = fromVideo && media.ended(this.media);
    if (newT >= s.duration - 0.02 || ended) {
      this.log("watched to the end", s.name);
      const end = jumpType(s.at_end) === "Continue" ? "Finish" : s.at_end;
      this.go(end);
    }
  }

  // The answer to the blocking question, checked.
  checkAnswer(inter) {
    const d = kd(inter.kind);
    const o = d.options[this.pick];
    const ok = !!o?.correct;
    this.checked = ok;
    const first = !this.session.answered.includes(inter.id);
    if (first) this.session.answered.push(inter.id);
    const gained = ok && first ? d.points : 0;
    this.session.score += gained;
    this.log("answered", d.prompt, o?.text ?? "", ok, gained);
  }
}

// ---------------------------------------------------------------- app state

const state = {
  project: null,
  tab: "watch",
  learner: store.get("cq.learner", "Guest"),
  history: store.get("cq.results", []),
  player: null,
  selScene: null,
  selItem: null,
  editTime: 0,
  selSession: 0,
  mapOffset: { x: 20, y: 20 },
  mapZoom: null,
};

function saveProject() { store.set("cq.project", state.project); renderTitle(); }
function saveResults() {
  const list = state.history.slice();
  if (state.player?.session.events.length) list.push(state.player.session);
  store.set("cq.results", list.slice(-200));
}

function archiveSession() {
  const s = state.player?.session;
  if (s && s.events.length) state.history.push(s);
  store.set("cq.results", state.history.slice(-200));
}

function newPlayer() {
  archiveSession();
  state.player = new Player(state.project, state.learner);
}

function loadProject(p) {
  archiveSession();
  state.project = p;
  state.selScene = p.start_scene;
  state.selItem = null;
  state.editTime = 0;
  state.mapZoom = null;
  state.player = new Player(p, state.learner);
  saveProject();
}

function setTab(tab) {
  state.tab = tab;
  if (tab !== "watch") state.player.playing = false;
  render();
  window.scrollTo(0, 0);
}

// ---------------------------------------------------------------- navigation and menu

const TABS = [["watch", "Watch"], ["edit", "Edit"], ["map", "Story map"], ["results", "Results"]];

function renderNav() {
  for (const host of [$("#tabs-top"), $("#tabs-bottom")]) {
    host.replaceChildren(...TABS.map(([id, name]) =>
      h("button", { class: "nav-btn" + (state.tab === id ? " active" : ""), "aria-current": state.tab === id ? "page" : null, onclick: () => setTab(id) },
        icon(id), h("span", {}, name))));
  }
}

function renderTitle() { $("#lesson-title").textContent = state.project.title; }

function textSize(size) {
  document.documentElement.classList.remove("ts-normal", "ts-xl");
  if (size !== "large") document.documentElement.classList.add("ts-" + size);
  store.set("cq.textsize", size);
}

function openMenu(open) {
  $("#menu").hidden = !open;
  $("#menu-backdrop").hidden = !open;
  if (!open) return;
  const size = store.get("cq.textsize", "large");
  const close = () => openMenu(false);
  $("#menu").replaceChildren(
    h("h2", {}, "ClipQuest"),
    h("button", { class: "btn", onclick: () => { loadProject(blankProject()); close(); setTab("edit"); } }, "New lesson"),
    h("button", { class: "btn", onclick: () => { close(); $("#open-file").click(); } }, "Open a lesson file…"),
    h("button", { class: "btn", onclick: () => { close(); download(`${state.project.title}.json`, JSON.stringify(state.project, null, 2), "application/json"); } }, "Save lesson as a file"),
    h("h3", {}, "Examples"),
    ...EXAMPLES.map(([name, make]) => h("button", { class: "btn", onclick: () => { loadProject(make()); close(); setTab("watch"); } }, name)),
    h("h3", {}, "Text and button size"),
    h("div", { class: "seg" }, ...[["normal", "Normal"], ["large", "Large"], ["xl", "Extra large"]].map(([id, name]) =>
      h("button", { class: "btn" + (size === id ? " on" : ""), onclick: () => { textSize(id); openMenu(true); render(); } }, name))),
    h("h3", {}, "About"),
    h("p", { class: "muted small" }, "Lessons, results and chosen videos stay in this browser. Videos always play muted."),
    h("p", { class: "muted small" }, "ClipQuest is a static web app: no server or account needed. Put this folder on any web host (https) and it works on phones too. YouTube videos need the page to be opened from a web address, not from a file."),
  );
}

function download(name, text, type) {
  const a = h("a", { href: URL.createObjectURL(new Blob([text], { type })), download: name });
  document.body.append(a);
  a.click();
  setTimeout(() => { URL.revokeObjectURL(a.href); a.remove(); }, 1000);
}

// ---------------------------------------------------------------- render switch

let frameJob = null; // per-frame updater of the current view

function render() {
  renderNav();
  renderTitle();
  frameJob = null;
  const main = $("#main");
  main.replaceChildren();
  if (state.tab === "watch") renderWatch(main);
  else if (state.tab === "edit") renderEdit(main);
  else if (state.tab === "map") renderMap(main);
  else renderResults(main);
}

let lastFrame = performance.now();
function loop(now) {
  const dt = Math.min(0.1, (now - lastFrame) / 1000);
  lastFrame = now;
  if (frameJob) frameJob(dt);
  requestAnimationFrame(loop);
}

// ---------------------------------------------------------------- Watch

function renderWatch(main) {
  const pl = state.player;
  const canvas = h("canvas", { "aria-hidden": "true" });
  const video = h("video", { muted: true, playsinline: true, preload: "auto", "webkit-playsinline": true });
  video.muted = true;
  video.defaultMuted = true;
  const overlay = h("div", { class: "overlay" });
  const caption = h("div", { class: "caption", hidden: true });
  const ytHost = h("div", { class: "yt-host", hidden: true });
  const playerBox = h("div", { class: "player" }, canvas, video, ytHost, caption, overlay);
  playerBox.addEventListener("click", (e) => {
    if (e.target === overlay || e.target === canvas || e.target === video || e.target === ytHost) {
      if (pl.started && !pl.finished && pl.blocking === null) { pl.playing = !pl.playing; refresh(); }
    }
  });

  // seek bar
  const fill = h("div", { class: "fill" });
  const watched = h("div", { class: "watched" });
  const thumb = h("div", { class: "thumb" });
  const marks = h("div");
  const seek = h("div", { class: "seek", role: "slider", "aria-label": "Position", tabindex: "0" }, h("div", { class: "track" }), watched, fill, marks, thumb);
  const seekTo = (e) => {
    const s = pl.sceneObj;
    if (!s || !pl.started || pl.finished || pl.blocking !== null) return;
    const r = seek.getBoundingClientRect();
    pl.seek(clamp((e.clientX - r.left) / r.width, 0, 1) * s.duration);
    refresh();
  };
  seek.addEventListener("pointerdown", (e) => { seek.setPointerCapture(e.pointerId); seekTo(e); });
  seek.addEventListener("pointermove", (e) => { if (seek.hasPointerCapture(e.pointerId)) seekTo(e); });

  const playBtn = h("button", { class: "play-btn", "aria-label": "Play or pause", onclick: () => {
    if (!pl.started) pl.start(); else pl.playing = !pl.playing;
    refresh();
  } });
  const timeLabel = h("span", { class: "time" });
  const speed = h("select", { "aria-label": "Speed", onchange: (e) => { pl.speed = +e.target.value; } },
    ...[0.5, 0.75, 1, 1.5, 2].map((v) => h("option", { value: v, selected: v === pl.speed }, `${v}×`)));
  const cc = h("button", { class: "btn" + (pl.captions ? " on" : ""), "aria-pressed": String(pl.captions), onclick: () => {
    pl.captions = !pl.captions; cc.classList.toggle("on", pl.captions); cc.setAttribute("aria-pressed", String(pl.captions));
  } }, "CC");
  const back = h("button", { class: "icon-btn", "aria-label": "Back 10 seconds", onclick: () => {
    if (pl.blocking === null && pl.started) { pl.seek(pl.time - 10); refresh(); }
  } }, icon("back"));
  const full = h("button", { class: "icon-btn", "aria-label": "Full screen", onclick: () => {
    if (document.fullscreenElement) document.exitFullscreen();
    else (playerBox.requestFullscreen?.() ?? video.webkitEnterFullscreen?.());
  } }, icon("full"));
  const controls = h("div", { class: "controls" }, seek, h("div", { class: "ctrl-row" }, playBtn, back, timeLabel, speed, cc, full));

  const below = h("div", { class: "below-cards" });
  const side = h("aside", { class: "side" });
  main.append(h("div", { class: "page" }, h("div", { class: "watch" }, h("div", { class: "stack" }, playerBox, controls, below), side)));

  // keyboard
  const onKey = (e) => {
    if (state.tab !== "watch" || /INPUT|TEXTAREA|SELECT/.test(document.activeElement?.tagName)) return;
    const free = pl.started && !pl.finished && pl.blocking === null;
    if (e.key === " " && free) { pl.playing = !pl.playing; e.preventDefault(); refresh(); }
    if ((e.key === "ArrowLeft" || e.key === "ArrowRight") && free) { pl.seek(pl.time + (e.key === "ArrowLeft" ? -5 : 5)); refresh(); }
  };
  document.onkeydown = onKey;

  // media for the current scene
  let mediaScene = null;
  let mediaSrc = undefined;
  async function syncMedia() {
    const s = pl.sceneObj;
    if (!s || (mediaScene === s.id && mediaSrc === s.video)) return;
    const sameVideo = mediaSrc === s.video && pl.media;
    mediaScene = s.id;
    mediaSrc = s.video;
    // Scenes cut from the same video share one player: just move to the new part.
    if (sameVideo) {
      if (pl.media.ready) media.seek(pl.media, clipStart(s) + pl.time);
      return;
    }
    pl.media = null;
    video.hidden = true;
    canvas.hidden = false;
    ytHost.replaceChildren();
    ytHost.hidden = true;
    const yt = ytId(s.video);
    if (yt) {
      // YouTube: its own embedded player, always muted, with our controls and cards on top.
      let YT;
      try { YT = await loadYouTube(); } catch { toast("YouTube can't be reached; showing a sample clip"); return; }
      if (mediaScene !== s.id) return;
      const slot = h("div");
      // The video's own picture shows until the player is ready (and if it never is).
      ytHost.replaceChildren(h("img", { class: "yt-thumb", alt: "", src: `https://i.ytimg.com/vi/${yt}/hqdefault.jpg` }), slot);
      ytHost.hidden = false;
      canvas.hidden = true;
      const m = { kind: "yt", yt: null, ready: false };
      pl.media = m;
      const explain = () => {
        if (m.ready || pl.media !== m || ytHost.querySelector(".yt-note")) return;
        // Only explain once the lesson is playing; before that the start screen is up.
        if (!pl.started) { setTimeout(explain, 3000); return; }
        const fromFile = location.protocol === "file:";
        ytHost.append(h("div", { class: "yt-note" },
          fromFile
            ? "YouTube only plays when this page is opened from a web address. In the ClipQuest desktop app use File → “Open the web version in your browser”, or put these files online."
            : "YouTube didn't load. Check the internet connection, or watch it on YouTube.",
          h("a", { class: "btn", href: `https://www.youtube.com/watch?v=${yt}`, target: "_blank", rel: "noopener" }, "Watch on YouTube ↗")));
      };
      setTimeout(explain, 12000);
      m.yt = new YT.Player(slot, {
        videoId: yt, width: "100%", height: "100%",
        playerVars: { controls: 0, playsinline: 1, mute: 1, rel: 0, disablekb: 1, fs: 0, iv_load_policy: 3, cc_load_policy: 0, start: Math.floor(clipStart(s) + pl.time) },
        events: {
          onReady: (e) => { e.target.mute(); m.ready = true; canvas.hidden = true; e.target.seekTo(clipStart(pl.sceneObj) + pl.time, true); },
          onError: () => { if (pl.media === m) pl.media = null; explain(); toast("YouTube won't play this video here"); },
        },
      });
      return;
    }
    const url = await videoUrl(s.video);
    if (!url || mediaScene !== s.id) return;
    video.src = url;
    video.hidden = false;
    const m = { kind: "video", el: video, ready: false };
    pl.media = m;
    video.onloadeddata = () => {
      m.ready = true;
      canvas.hidden = true;
      video.currentTime = clipStart(pl.sceneObj) + pl.time;
    };
    video.onerror = () => { pl.media = null; video.hidden = true; canvas.hidden = false; toast("Couldn't play this video; showing a sample clip"); };
  }

  let key = "";
  function refresh() { key = ""; }

  function overlayKey(s) {
    const visible = s.interactions.filter((i) => pl.time >= i.time && pl.time <= i.time + i.show_for && !pl.dismissed.has(i.id)).map((i) => i.id);
    return [pl.scene, pl.started, pl.finished, pl.blocking, pl.pick, pl.checked, pl.popup?.[0], visible.join(","), isWide(), pl.session.score, pl.session.events.length, pl.playing].join("|");
  }

  frameJob = (dt) => {
    syncMedia();
    pl.tick(dt);
    const s = pl.sceneObj;
    if (!s) return;
    // keep the video element in step with the player
    if (pl.media?.ready) {
      const m = pl.media;
      media.rate(m, pl.speed);
      if (pl.playing && media.paused(m) && !media.ended(m)) media.play(m);
      if (!pl.playing && !media.paused(m)) media.pause(m);
      const want = clipStart(s) + pl.time;
      if (!pl.playing && Math.abs(media.time(m) - want) > (m.kind === "yt" ? 1 : 0.3)) media.seek(m, want);
    } else {
      drawSample(canvas, s, pl.time);
    }
    const frac = s.duration > 0 ? clamp(pl.time / s.duration, 0, 1) : 0;
    fill.style.width = `${frac * 100}%`;
    thumb.style.left = `${frac * 100}%`;
    watched.style.width = pl.p.prevent_skipping ? `${clamp(pl.maxWatched / s.duration, 0, 1) * 100}%` : "0";
    timeLabel.textContent = `${fmtTime(pl.time)} / ${fmtTime(s.duration)}`;
    seek.setAttribute("aria-valuenow", Math.round(pl.time));
    const c = pl.captions && s.captions.find((c) => pl.time >= c.start && pl.time <= c.end);
    caption.hidden = !c;
    if (c && caption.textContent !== c.text) caption.textContent = c.text;

    const k = overlayKey(s);
    if (k === key) return;
    key = k;
    playBtn.innerHTML = pl.playing ? ICONS.pause : ICONS.play;
    playBtn.disabled = pl.started && (pl.finished || pl.blocking !== null);
    marks.replaceChildren(
      ...s.chapters.map((c) => h("div", { class: "chap", style: { left: `${(c.time / s.duration) * 100}%` } })),
      ...s.interactions.map((i) => h("div", { class: "mark", title: `${TYPE_NAMES[kindType(i.kind)]} · ${summary(i.kind)}`, style: { left: `${clamp(i.time / s.duration, 0, 1) * 100}%`, background: COLOURS[kindType(i.kind)] } })),
    );
    buildOverlay(s);
    renderSide(side, refresh);
  };

  function buildOverlay(s) {
    const items = [];
    const infos = [];
    playerBox.classList.toggle("dim", false);
    for (const i of s.interactions) {
      const t = kindType(i.kind), d = kd(i.kind);
      const shown = pl.started && !pl.dismissed.has(i.id) && ((pl.time >= i.time && pl.time <= i.time + i.show_for) || pl.blocking === i.id);
      if (!shown) continue;
      if (t === "Info") {
        infos.push(h("div", { class: "info-card", style: { borderColor: COLOURS.Info } },
          h("div", { class: "t" }, d.title),
          h("button", { class: "x", "aria-label": "Close", onclick: () => { pl.dismissed.add(i.id); pl.log("closed", d.title); refresh(); } }, "✕"),
          h("div", { class: "b" }, d.body)));
      } else if (t === "Hotspot") {
        const [x, y, w, hh] = d.rect;
        items.push(h("button", {
          class: "hotspot", "aria-label": d.label,
          style: { left: `${x * 100}%`, top: `${y * 100}%`, width: `${w * 100}%`, height: `${hh * 100}%`, borderColor: COLOURS.Hotspot },
          onclick: (e) => {
            e.stopPropagation();
            pl.log("clicked hotspot", d.label);
            if (d.info) pl.popup = [d.label, d.info];
            if (jumpType(d.jump) !== "Continue") pl.go(d.jump);
            refresh();
          },
        }, h("span", { class: "chip", style: { background: COLOURS.Hotspot } }, d.label)));
      } else if (t === "Link") {
        items.push(h("a", { class: "btn primary link-btn", href: d.url, target: "_blank", rel: "noopener", style: { background: COLOURS.Link }, onclick: () => pl.log("opened link", d.label, d.url) }, `${d.label} ↗`));
      }
    }
    let popupEl = null;
    if (pl.popup) {
      popupEl = h("div", { class: "info-card light" },
        h("div", { class: "t" }, pl.popup[0]),
        h("button", { class: "x", "aria-label": "Close", onclick: () => { pl.popup = null; refresh(); } }, "✕"),
        h("div", { class: "b" }, pl.popup[1]));
    }
    // On phones the cards go under the video so they don't hide it.
    if (isWide()) {
      if (infos.length) items.push(h("div", { class: "info-stack" }, ...infos));
      if (popupEl) items.push(h("div", { class: "popup-host" }, popupEl));
      below.replaceChildren();
    } else {
      below.replaceChildren(...infos, ...(popupEl ? [popupEl] : []));
    }

    if (pl.blocking !== null) {
      const inter = s.interactions.find((i) => i.id === pl.blocking);
      const t = inter && kindType(inter.kind);
      if (inter && !BLOCKS.has(t)) {
        items.push(h("button", { class: "btn primary big continue-btn", onclick: () => { pl.go("Continue"); refresh(); } }, "Continue →"));
      } else if (inter) {
        playerBox.classList.add("dim");
        items.push(questionCard(inter, refresh));
      }
    }
    if (!pl.started) {
      items.push(h("div", { class: "start-screen" },
        h("div", { class: "title" }, pl.p.title),
        pl.p.description ? h("div", { class: "desc" }, pl.p.description) : null,
        h("button", { class: "btn primary big", onclick: (e) => { e.stopPropagation(); pl.start(); refresh(); } }, icon("play"), "Start"),
        h("div", { class: "small muted" }, "Videos play muted.")));
      if (!isWide() && pl.p.description) below.append(h("p", { class: "muted" }, pl.p.description));
    }
    if (pl.finished) items.push(summaryCard());
    overlay.replaceChildren(...items);
  }

  function summaryCard() {
    const s = pl.session;
    return h("div", { class: "card summary", style: { "--kind": "#ffc846" } },
      h("h2", {}, "Finished!"),
      s.max_score > 0 ? h("p", { style: { fontSize: "1.2rem", margin: ".2rem 0" } }, `Score ${s.score} / ${s.max_score} (${percent(s)}%)`) : null,
      s.max_score > 0 ? h("p", { style: { color: passed(s) ? "var(--good)" : "var(--bad)", fontWeight: 700, margin: ".2rem 0" } }, passed(s) ? "Passed" : "Not passed yet") : null,
      h("p", { class: "muted" }, `${s.answered.length} question(s) answered`),
      h("p", { class: "muted small" }, `Your path: ${s.path.join(" → ")}`),
      h("div", { class: "actions" },
        h("button", { class: "btn primary", onclick: () => { newPlayer(); state.player.start(); render(); } }, "Watch again"),
        h("button", { class: "btn", onclick: () => setTab("results") }, "See results")));
  }
}

/** The card for a question, choice, open question or pause screen. */
function questionCard(inter, refresh) {
  const pl = state.player;
  const t = kindType(inter.kind), d = kd(inter.kind);
  const col = COLOURS[t];
  const card = h("div", { class: "card", role: "dialog", "aria-label": TYPE_NAMES[t], style: { "--kind": col } },
    h("div", { class: "kind" }, TYPE_NAMES[t]));
  const next = (jump) => { pl.go(jump); refresh(); };
  if (t === "Question") {
    card.append(h("div", { class: "prompt" }, d.prompt));
    card.append(h("div", { class: "answers", role: "radiogroup" }, ...d.options.map((o, n) => {
      let cls = "answer";
      if (pl.pick === n) cls += " picked";
      if (pl.checked !== null && o.correct) cls += " right";
      else if (pl.checked !== null && pl.pick === n) cls += " wrong";
      return h("button", { class: cls, role: "radio", "aria-checked": String(pl.pick === n), disabled: pl.checked !== null, onclick: () => { pl.pick = n; refresh(); } },
        h("span", { class: "dot" }), h("span", {}, o.text));
    })));
    if (pl.checked === null) {
      card.append(h("div", { class: "actions" },
        h("button", { class: "btn primary", disabled: pl.pick === null, onclick: () => { pl.checkAnswer(inter); refresh(); } }, "Check answer")));
    } else {
      const fb = d.options[pl.pick]?.feedback;
      card.append(h("div", { class: "feedback " + (pl.checked ? "good" : "bad") }, pl.checked ? "Correct!" : "Not quite."));
      if (fb) card.append(h("div", {}, fb));
      card.append(h("div", { class: "actions" },
        !pl.checked && d.allow_retry ? h("button", { class: "btn", onclick: () => { pl.checked = null; pl.pick = null; refresh(); } }, "Try again") : null,
        h("button", { class: "btn primary", onclick: () => next(pl.checked ? d.on_correct : d.on_wrong) }, "Continue →")));
    }
  } else if (t === "OpenQuestion") {
    const ta = h("textarea", { placeholder: "Type your answer…", rows: "3", oninput: (e) => { pl.openText = e.target.value; submit.disabled = !pl.openText.trim(); } });
    ta.value = pl.openText;
    const submit = h("button", { class: "btn primary", disabled: !pl.openText.trim(), onclick: () => {
      if (!pl.session.answered.includes(inter.id)) pl.session.answered.push(inter.id);
      pl.log("answered", d.prompt, pl.openText.trim());
      next("Continue");
    } }, "Submit");
    card.append(h("div", { class: "prompt" }, d.prompt), ta,
      h("div", { class: "actions" }, h("button", { class: "btn", onclick: () => { pl.log("skipped", d.prompt); next("Continue"); } }, "Skip"), submit));
  } else if (t === "Choice") {
    card.append(h("div", { class: "prompt" }, d.prompt),
      h("div", { class: "answers" }, ...d.options.map((o) => h("button", { class: "choice-btn", style: { background: col }, onclick: () => { pl.log("chose", d.prompt, o.text); next(o.jump); } }, o.text))));
  } else if (t === "PauseScreen") {
    card.append(h("div", { class: "prompt" }, d.text), h("div", { class: "actions" }, h("button", { class: "btn primary", onclick: () => next("Continue") }, "Continue →")));
  }
  return card;
}

function renderSide(side, refresh) {
  const pl = state.player;
  const s = pl.session;
  const scene = pl.sceneObj;
  const nameInput = h("input", { type: "text", value: state.learner, "aria-label": "Viewer name", oninput: (e) => {
    state.learner = e.target.value; store.set("cq.learner", state.learner);
    if (!pl.started) s.learner = state.learner;
  } });
  const current = scene ? scene.chapters.reduce((acc, c, n) => (c.time <= pl.time + 0.01 ? n : acc), -1) : -1;
  side.replaceChildren(...[
    h("h3", {}, "Viewer"), nameInput,
    h("h3", {}, "Progress"),
    h("div", { class: "stat" }, h("span", { class: "muted" }, "Scene"), h("span", {}, sceneName(pl.p, pl.scene))),
    s.max_score > 0 ? h("div", { class: "stat" }, h("span", { class: "muted" }, "Score"), h("span", {}, `${s.score} / ${s.max_score} (${percent(s)}%)`)) : null,
    s.max_score > 0 ? h("div", { class: "stat" }, h("span", { class: "muted" }, "Pass mark"), h("span", {}, `${pl.p.pass_percent}%`)) : null,
    s.max_score > 0 ? h("div", { class: "bar" }, h("div", { style: { width: `${percent(s)}%` } })) : null,
    s.path.length ? h("div", { class: "small muted" }, "Path: ", s.path.join(" → ")) : null,
    h("h3", {}, "Chapters"),
    scene && scene.chapters.length ? null : h("div", { class: "muted small" }, "No chapters in this scene"),
    ...(scene ? scene.chapters.map((c, n) => {
      const locked = pl.p.prevent_skipping && c.time > pl.maxWatched + 0.01;
      return h("button", { class: "chapter-btn" + (n === current ? " current" : ""), disabled: locked || pl.blocking !== null, onclick: () => {
        if (!pl.started) pl.start();
        pl.seek(c.time); pl.playing = true; refresh();
      } }, h("span", { class: "tm" }, fmtTime(c.time)), h("span", {}, c.title + (locked ? " (not reached)" : "")));
    }) : []),
    h("h3", {}, "In this scene"),
    ...(scene ? scene.interactions.map((i) => h("div", { class: "dotline" + (i.time < pl.time ? " done" : "") },
      h("i", { style: { background: COLOURS[kindType(i.kind)] } }), h("span", {}, `${fmtTime(i.time)} ${TYPE_NAMES[kindType(i.kind)]}`))) : []),
    pl.p.prevent_skipping ? h("p", { class: "small muted" }, "Skipping ahead is turned off for this lesson.") : null,
    h("button", { class: "btn", onclick: () => { newPlayer(); render(); } }, "Restart lesson"),
  ].filter(Boolean));
}

// ---------------------------------------------------------------- Edit

function selectedScene() { return sceneOf(state.project, state.selScene) || state.project.scenes[0]; }
function selectedItem() { return selectedScene()?.interactions.find((i) => i.id === state.selItem) || null; }

function newKind(type) {
  switch (type) {
    case "Question": return { Question: { prompt: "New question", options: [{ text: "Right answer", correct: true, feedback: "Well done." }, { text: "Wrong answer", correct: false, feedback: "Have another look." }], points: 10, allow_retry: true, on_correct: "Continue", on_wrong: "Continue" } };
    case "Choice": return { Choice: { prompt: "What next?", options: [{ text: "Option A", jump: "Continue" }, { text: "Option B", jump: "Continue" }] } };
    case "Hotspot": return { Hotspot: { rect: [0.4, 0.35, 0.2, 0.25], label: "Look here", info: "More about this.", jump: "Continue" } };
    case "Info": return { Info: { title: "Did you know?", body: "Some extra detail." } };
    case "OpenQuestion": return { OpenQuestion: { prompt: "What do you think?" } };
    case "PauseScreen": return { PauseScreen: { text: "Take a moment, then continue." } };
    case "ScoreCheck": return { ScoreCheck: { min_percent: state.project.pass_percent, pass: "Continue", fail: "Continue" } };
    default: return { Link: { label: "Learn more", url: "https://example.com" } };
  }
}

function renderEdit(main) {
  const p = state.project;
  if (!sceneOf(p, state.selScene)) state.selScene = p.scenes[0]?.id;
  const scene = selectedScene();
  if (!scene) { main.append(h("div", { class: "page empty" }, "No scenes.")); return; }
  state.editTime = clamp(state.editTime, 0, scene.duration);

  const lessonPanel = h("section", { class: "panel stack" });
  const center = h("section", { class: "stack" });
  const inspector = h("section", { class: "panel stack" });
  main.append(h("div", { class: "page" }, h("div", { class: "edit" }, lessonPanel, center, inspector)));
  const rerender = () => { saveProject(); render(); };

  // ----- lesson and scenes
  const field = (label, el) => h("label", { class: "field" }, label, el);
  const text = (obj, prop, multi) => {
    const el = h(multi ? "textarea" : "input", { type: "text", oninput: (e) => { obj[prop] = e.target.value; saveProject(); } });
    el.value = obj[prop] ?? "";
    return el;
  };
  const num = (obj, prop, min, max, step, after) => {
    const el = h("input", { type: "number", min, max, step, inputmode: "decimal", onchange: (e) => { obj[prop] = clamp(+e.target.value || 0, min, max); saveProject(); after?.(); } });
    el.value = obj[prop] ?? 0;
    return el;
  };
  const sceneSelect = h("select", { "aria-label": "Scene", onchange: (e) => { state.selScene = +e.target.value; state.selItem = null; state.editTime = 0; render(); } },
    ...p.scenes.map((s) => h("option", { value: s.id, selected: s.id === scene.id }, `${s.id === p.start_scene ? "Start · " : ""}${s.name} (${s.interactions.length})`)));
  lessonPanel.append(
    h("h2", {}, "Lesson"),
    field("Title", text(p, "title")),
    field("Description (shown at the start)", text(p, "description", true)),
    field(`Pass mark: ${p.pass_percent}%`, h("input", { type: "range", min: 0, max: 100, value: p.pass_percent, oninput: (e) => { p.pass_percent = +e.target.value; e.target.parentElement.firstChild.textContent = `Pass mark: ${p.pass_percent}%`; saveProject(); } })),
    h("label", { class: "check" }, h("input", { type: "checkbox", checked: p.prevent_skipping, onchange: (e) => { p.prevent_skipping = e.target.checked; saveProject(); } }), "Don't let viewers skip ahead"),
    h("div", { class: "small muted" }, `${maxPoints(p)} points in total`),
    h("h3", {}, "Scene"),
    h("div", { class: "scene-pick" }, sceneSelect),
    h("div", { class: "row" },
      h("button", { class: "btn", onclick: () => {
        const n = p.scenes.length;
        state.selScene = addScene(p, `Scene ${n + 1}`, 30, n % LOOKS.length, [60 + (n % 3) * 260, 40 + Math.floor(n / 3) * 120]);
        state.selItem = null; rerender();
      } }, "+ Scene"),
      h("button", { class: "btn", onclick: () => {
        const copy = JSON.parse(JSON.stringify(scene));
        copy.id = allocId(p); copy.name += " copy"; copy.map_pos = [copy.map_pos[0], copy.map_pos[1] + 90];
        copy.interactions.forEach((i) => { i.id = allocId(p); });
        p.scenes.push(copy); state.selScene = copy.id; rerender();
      } }, "Duplicate"),
      h("button", { class: "btn danger", disabled: p.scenes.length < 2, onclick: () => {
        if (!confirm(`Delete the scene "${scene.name}"?`)) return;
        p.scenes = p.scenes.filter((s) => s.id !== scene.id);
        if (p.start_scene === scene.id) p.start_scene = p.scenes[0].id;
        state.selScene = p.scenes[0].id; state.selItem = null; rerender();
      } }, "Delete"),
      h("button", { class: "btn", disabled: scene.id === p.start_scene, onclick: () => { p.start_scene = scene.id; rerender(); } }, "Make start")),
    h("button", { class: "btn primary", onclick: () => { newPlayer(); setTab("watch"); } }, icon("play"), "Preview"),
  );

  // ----- preview with the playhead
  const canvas = h("canvas");
  const video = h("video", { muted: true, playsinline: true, preload: "auto", hidden: true });
  video.muted = true;
  const overlay = h("div", { class: "overlay" });
  const box = h("div", { class: "player" }, canvas, video, overlay);
  let previewPlaying = false;
  const playBtn = h("button", { class: "play-btn", "aria-label": "Play the preview", onclick: () => { previewPlaying = !previewPlaying; playBtn.innerHTML = previewPlaying ? ICONS.pause : ICONS.play; } });
  playBtn.innerHTML = ICONS.play;
  const scrub = h("input", { type: "range", min: 0, max: scene.duration, step: 0.1, value: state.editTime, "aria-label": "Playhead", oninput: (e) => { state.editTime = +e.target.value; previewPlaying = false; playBtn.innerHTML = ICONS.play; update(); } });
  const timeLabel = h("span", { class: "muted" });
  const addLabel = h("strong", {});
  const chips = h("div", { class: "add-chips" }, addLabel,
    ...["Question", "Choice", "Hotspot", "Info", "OpenQuestion", "PauseScreen", "ScoreCheck", "Link"].map((t) =>
      h("button", { class: "chip-btn", style: { background: COLOURS[t] }, onclick: () => {
        state.selItem = addInteraction(p, scene.id, Math.round(state.editTime * 10) / 10, newKind(t));
        rerender();
      } }, TYPE_NAMES[t].replace(" (branch)", ""))),
    h("button", { class: "btn", onclick: () => { scene.chapters.push(chap(state.editTime, "New chapter")); scene.chapters.sort((a, b) => a.time - b.time); state.selItem = null; rerender(); } }, "+ Chapter"),
    h("button", { class: "btn", onclick: () => { scene.captions.push(cap(state.editTime, Math.min(scene.duration, state.editTime + 4), "New caption")); scene.captions.sort((a, b) => a.start - b.start); state.selItem = null; rerender(); } }, "+ Caption"));
  const timeline = h("div", { class: "timeline" });
  center.append(box, h("div", { class: "preview-time" }, playBtn, scrub, timeLabel), h("div", { class: "panel stack" }, chips), h("div", { class: "timeline-wrap" }, timeline));

  // A YouTube video shows its picture here; it plays in Watch.
  const ytPreview = ytId(scene.video);
  if (ytPreview) {
    canvas.hidden = true;
    box.prepend(h("img", { class: "yt-thumb", alt: "", src: `https://i.ytimg.com/vi/${ytPreview}/hqdefault.jpg`, onerror: (e) => { e.target.remove(); canvas.hidden = false; } }));
    box.append(h("span", { class: "yt-badge" }, "YouTube video · plays in Watch"));
  }
  videoUrl(scene.video).then((url) => {
    if (!url) return;
    video.src = url; video.hidden = false;
    video.onloadeddata = () => { canvas.hidden = true; video.currentTime = clipStart(scene) + state.editTime; };
  });

  function drawOverlay() {
    const t = state.editTime;
    const items = [];
    for (const i of scene.interactions) {
      const type = kindType(i.kind), d = kd(i.kind);
      const sel = state.selItem === i.id;
      const visible = t >= i.time && t <= i.time + Math.max(i.show_for, 0.5);
      if (type === "Hotspot" && (visible || sel)) items.push(hotspotEditor(i, sel));
      else if (sel || (visible && type !== "Hotspot")) items.push(h("div", { class: "tag", style: { background: COLOURS[type] } }, `${TYPE_NAMES[type]} · ${summary(i.kind)}`));
    }
    const c = scene.captions.find((c) => t >= c.start && t <= c.end);
    if (c) items.push(h("div", { class: "caption" }, c.text));
    overlay.replaceChildren(...items);
  }

  function hotspotEditor(i, sel) {
    const d = kd(i.kind);
    const place = (el) => Object.assign(el.style, { left: `${d.rect[0] * 100}%`, top: `${d.rect[1] * 100}%`, width: `${d.rect[2] * 100}%`, height: `${d.rect[3] * 100}%`, borderColor: COLOURS.Hotspot });
    const handle = h("div", { class: "handle", "aria-label": "Resize" });
    const el = h("div", { class: "hotspot" + (sel ? " sel" : "") }, h("span", { class: "chip", style: { background: COLOURS.Hotspot } }, d.label), sel ? handle : null);
    place(el);
    const drag = (target, resize) => {
      target.addEventListener("pointerdown", (e) => {
        e.stopPropagation(); e.preventDefault();
        if (state.selItem !== i.id) { state.selItem = i.id; renderInspector(); drawOverlay(); return; }
        target.setPointerCapture(e.pointerId);
        const r = box.getBoundingClientRect();
        const start = [e.clientX, e.clientY], orig = d.rect.slice();
        const move = (ev) => {
          const dx = (ev.clientX - start[0]) / r.width, dy = (ev.clientY - start[1]) / r.height;
          if (resize) { d.rect[2] = clamp(orig[2] + dx, 0.03, 1 - d.rect[0]); d.rect[3] = clamp(orig[3] + dy, 0.03, 1 - d.rect[1]); }
          else { d.rect[0] = clamp(orig[0] + dx, 0, 1 - d.rect[2]); d.rect[1] = clamp(orig[1] + dy, 0, 1 - d.rect[3]); }
          place(el);
        };
        const up = () => { target.removeEventListener("pointermove", move); target.removeEventListener("pointerup", up); saveProject(); renderInspector(); };
        target.addEventListener("pointermove", move);
        target.addEventListener("pointerup", up);
      });
    };
    drag(el, false);
    drag(handle, true);
    return el;
  }

  const PX = 14; // timeline pixels per second, before fitting
  function drawTimeline() {
    const wrapW = timeline.parentElement.clientWidth || 600;
    const pxs = Math.max(PX, (wrapW - 20) / scene.duration);
    const width = scene.duration * pxs + 20;
    const x = (t) => 10 + t * pxs;
    timeline.style.width = `${width}px`;
    const kids = [];
    const lanes = [["", 0], ["Interactions", 1.7], ["Chapters", 5], ["Captions", 7.2]];
    for (const [name, top] of lanes.slice(1)) kids.push(h("div", { class: "lane", style: { top: `${top}rem` } }, h("span", { class: "lane-name" }, name)));
    const step = scene.duration > 240 ? 30 : scene.duration > 90 ? 10 : 5;
    for (let t = 0; t <= scene.duration + 0.01; t += step) kids.push(h("div", { class: "tick", style: { left: `${x(t)}px` } }, fmtTime(t)));
    for (const i of scene.interactions) {
      const type = kindType(i.kind);
      const timed = !BLOCKS.has(type) && type !== "ScoreCheck";
      const m = h("div", { class: "tmark" + (state.selItem === i.id ? " sel" : ""), title: `${TYPE_NAMES[type]} at ${fmtTime(i.time)}: ${summary(i.kind)}`,
        style: { left: `${x(i.time) - 10}px`, top: "2.8rem", width: timed ? `${Math.max(24, i.show_for * pxs)}px` : "24px", background: COLOURS[type] } });
      m.addEventListener("pointerdown", (e) => {
        e.preventDefault();
        m.setPointerCapture(e.pointerId);
        const startX = e.clientX, orig = i.time;
        let moved = false;
        state.selItem = i.id; state.editTime = i.time;
        const move = (ev) => {
          if (Math.abs(ev.clientX - startX) > 6) moved = true;
          if (!moved) return;
          i.time = Math.round(clamp(orig + (ev.clientX - startX) / pxs, 0, scene.duration) * 10) / 10;
          state.editTime = i.time;
          m.style.left = `${x(i.time) - 10}px`;
          update(false);
        };
        const up = () => {
          m.removeEventListener("pointermove", move); m.removeEventListener("pointerup", up);
          scene.interactions.sort((a, b) => a.time - b.time);
          saveProject(); update(); renderInspector();
        };
        m.addEventListener("pointermove", move);
        m.addEventListener("pointerup", up);
      });
      kids.push(m);
    }
    for (const c of scene.chapters) kids.push(h("div", { class: "tchap", style: { left: `${x(c.time)}px`, top: "5.7rem" } }, c.title));
    for (const c of scene.captions) kids.push(h("div", { class: "tcap", style: { left: `${x(c.start)}px`, width: `${Math.max(3, (c.end - c.start) * pxs)}px`, top: "7.9rem" } }));
    kids.push(h("div", { class: "playhead", style: { left: `${x(state.editTime)}px` } }));
    timeline.replaceChildren(...kids);
    timeline.onclick = (e) => {
      if (e.target !== timeline && !e.target.classList.contains("lane") && !e.target.classList.contains("tick")) return;
      const r = timeline.getBoundingClientRect();
      state.editTime = clamp((e.clientX - r.left - 10) / pxs, 0, scene.duration);
      update();
    };
  }

  function update(full = true) {
    scrub.value = state.editTime;
    timeLabel.textContent = `${fmtTime(state.editTime)} / ${fmtTime(scene.duration)}`;
    addLabel.textContent = `Add at ${fmtTime(state.editTime)}:`;
    if (!video.hidden && video.readyState >= 2 && Math.abs(video.currentTime - clipStart(scene) - state.editTime) > 0.15) video.currentTime = clipStart(scene) + state.editTime;
    drawOverlay();
    if (full) drawTimeline();
    else timeline.querySelector(".playhead")?.style.setProperty("left", `${10 + state.editTime * Math.max(PX, ((timeline.parentElement.clientWidth || 600) - 20) / scene.duration)}px`);
  }

  frameJob = (dt) => {
    if (previewPlaying) {
      state.editTime += dt;
      if (state.editTime >= scene.duration) { state.editTime = scene.duration; previewPlaying = false; playBtn.innerHTML = ICONS.play; }
      update(false);
    }
    if (canvas.hidden === false) drawSample(canvas, scene, state.editTime);
  };

  // ----- inspector
  function renderInspector() {
    inspector.replaceChildren(...(selectedItem() ? itemInspector(selectedItem()) : sceneInspector()).filter(Boolean));
  }

  function jumpEditor(obj, prop) {
    const j = obj[prop];
    const type = jumpType(j);
    const kind = h("select", { onchange: (e) => {
      const v = e.target.value;
      obj[prop] = v === "Time" ? { Time: 0 } : v === "Scene" ? { Scene: p.scenes[0].id } : v;
      saveProject(); renderInspector(); drawOverlay();
    } }, ...Object.entries(JUMP_NAMES).map(([k, n]) => h("option", { value: k, selected: k === type }, n)));
    let extra = null;
    if (type === "Time") extra = num(j, "Time", 0, scene.duration, 0.1);
    if (type === "Scene") extra = h("select", { onchange: (e) => { j.Scene = +e.target.value; saveProject(); } },
      ...p.scenes.map((s) => h("option", { value: s.id, selected: s.id === j.Scene }, s.name)));
    return h("div", { class: "inline" }, kind, extra || h("span"));
  }

  function sceneInspector() {
    const fileBtn = h("button", { class: "btn", onclick: () => {
      const input = $("#video-file");
      input.value = "";
      input.onchange = async () => {
        const f = input.files[0];
        if (!f) return;
        const key = `idb:${scene.id}:${Date.now()}:${f.name}`;
        try { await putVideo(key, f); } catch { toast("Couldn't keep that video in this browser"); return; }
        scene.video = key;
        const url = await videoUrl(key);
        const probe = h("video", { preload: "metadata", muted: true });
        probe.onloadedmetadata = () => { if (isFinite(probe.duration)) scene.duration = Math.round(probe.duration * 10) / 10; rerender(); };
        probe.onerror = () => rerender();
        probe.src = url;
        toast("Video added");
      };
      input.click();
    } }, "Choose a video…");
    const urlInput = h("input", { type: "url", placeholder: "…or paste a YouTube or .mp4 address", value: scene.video && /^https?:/.test(scene.video) ? scene.video : "" });
    const videoName = !scene.video ? "Sample clip (no video)" : ytId(scene.video) ? `YouTube video ${ytId(scene.video)} (set its length below)` : scene.video.startsWith("idb:") ? scene.video.split(":").slice(3).join(":") : /^https?:/.test(scene.video) ? scene.video : "A file on another computer (not available here)";
    const list = (arr, render, onAdd, addLabel) => h("div", { class: "stack" }, ...arr.map(render), h("button", { class: "btn", onclick: onAdd }, addLabel));
    return [
      h("h2", {}, "Scene"),
      field("Name", text(scene, "name")),
      h("h3", {}, "Video"),
      h("div", { class: "small" }, videoName),
      h("div", { class: "row" }, fileBtn,
        scene.video ? h("button", { class: "btn", onclick: () => { scene.video = null; rerender(); } }, "Remove") : null),
      h("div", { class: "row" }, urlInput, h("button", { class: "btn", onclick: () => { if (urlInput.value.trim()) { scene.video = urlInput.value.trim(); rerender(); } } }, "Use")),
      scene.video ? field("Starts at (seconds into the video) — a scene can be one part of a longer video", num(scene, "clip_start", 0, 36000, 1, rerender)) : null,
      field("Length (seconds)", num(scene, "duration", 2, 7200, 0.5, rerender)),
      !scene.video ? field("Sample look", h("select", { onchange: (e) => { scene.look = +e.target.value; saveProject(); } },
        ...LOOKS.map((l, n) => h("option", { value: n, selected: n === scene.look }, l)))) : null,
      h("h3", {}, "When the scene ends"),
      jumpEditor(scene, "at_end"),
      h("h3", {}, "Chapters"),
      list(scene.chapters, (c, n) => h("div", { class: "opt" },
        h("div", { class: "inline" }, num(c, "time", 0, 7200, 0.5, () => drawTimeline()), h("button", { class: "btn danger", onclick: () => { scene.chapters.splice(n, 1); rerender(); } }, "Remove")),
        text(c, "title")), () => { scene.chapters.push(chap(state.editTime, "New chapter")); rerender(); }, `+ Chapter at ${fmtTime(state.editTime)}`),
      h("h3", {}, "Captions"),
      list(scene.captions, (c, n) => h("div", { class: "opt" },
        h("div", { class: "inline" }, num(c, "start", 0, 7200, 0.1, () => drawTimeline()), num(c, "end", 0, 7200, 0.1, () => drawTimeline())),
        text(c, "text"),
        h("button", { class: "btn danger", onclick: () => { scene.captions.splice(n, 1); rerender(); } }, "Remove")),
        () => { scene.captions.push(cap(state.editTime, state.editTime + 4, "New caption")); rerender(); }, `+ Caption at ${fmtTime(state.editTime)}`),
      h("h3", {}, "Interactions"),
      scene.interactions.length ? null : h("p", { class: "muted small" }, "Use the coloured buttons to add one at the playhead."),
      h("div", { class: "item-list" }, ...scene.interactions.map((i) => h("button", { onclick: () => { state.selItem = i.id; state.editTime = i.time; renderInspector(); update(); } },
        h("i", { style: { width: ".75rem", height: ".75rem", borderRadius: "50%", background: COLOURS[kindType(i.kind)], flex: "none" } }),
        `${fmtTime(i.time)} ${TYPE_NAMES[kindType(i.kind)]} · ${summary(i.kind)}`))),
    ];
  }

  function itemInspector(it) {
    const type = kindType(it.kind), d = kd(it.kind);
    const out = [
      h("div", { class: "row" },
        h("button", { class: "btn", onclick: () => { state.selItem = null; renderInspector(); drawOverlay(); drawTimeline(); } }, "← Scene"),
        h("span", { class: "kind-title", style: { color: COLOURS[type] } }, TYPE_NAMES[type])),
      field("Appears at (seconds)", num(it, "time", 0, scene.duration, 0.1, () => { scene.interactions.sort((a, b) => a.time - b.time); state.editTime = it.time; update(); })),
    ];
    if (!BLOCKS.has(type) && type !== "ScoreCheck") {
      out.push(field("Visible for (seconds)", num(it, "show_for", 0.5, 600, 0.5, () => drawTimeline())));
      out.push(h("label", { class: "check" }, h("input", { type: "checkbox", checked: it.pause, onchange: (e) => { it.pause = e.target.checked; saveProject(); } }), "Pause the video while it shows"));
    } else if (type !== "ScoreCheck") out.push(h("p", { class: "small muted" }, "The video pauses until the viewer responds."));
    const live = () => { saveProject(); drawOverlay(); };
    const tx = (obj, prop, multi) => { const el = text(obj, prop, multi); el.addEventListener("input", live); return el; };
    switch (type) {
      case "Info":
        out.push(field("Title", tx(d, "title")), field("Text", tx(d, "body", true)));
        break;
      case "Question":
        out.push(field("Question", tx(d, "prompt", true)), h("h3", {}, "Answers (tick the right ones)"),
          ...d.options.map((o, n) => h("div", { class: "opt" },
            h("label", { class: "check" }, h("input", { type: "checkbox", checked: o.correct, onchange: (e) => { o.correct = e.target.checked; saveProject(); } }), "Right answer"),
            tx(o, "text"),
            h("input", { type: "text", placeholder: "Feedback for this answer", value: o.feedback || "", oninput: (e) => { o.feedback = e.target.value; saveProject(); } }),
            h("button", { class: "btn danger", onclick: () => { d.options.splice(n, 1); saveProject(); renderInspector(); } }, "Remove answer"))),
          h("button", { class: "btn", onclick: () => { d.options.push({ text: "Another answer", correct: false, feedback: "" }); saveProject(); renderInspector(); } }, "+ Answer"),
          field("Points", num(d, "points", 0, 1000, 1)),
          h("label", { class: "check" }, h("input", { type: "checkbox", checked: d.allow_retry, onchange: (e) => { d.allow_retry = e.target.checked; saveProject(); } }), "Allow another try after a wrong answer"),
          h("h3", {}, "After a right answer"), jumpEditor(d, "on_correct"),
          h("h3", {}, "After a wrong answer"), jumpEditor(d, "on_wrong"));
        break;
      case "OpenQuestion":
        out.push(field("Question", tx(d, "prompt", true)), h("p", { class: "small muted" }, "Answers are recorded in Results, not scored."));
        break;
      case "Hotspot":
        out.push(field("Label", tx(d, "label")), field("Shown when tapped", tx(d, "info", true)),
          h("h3", {}, "Tapping it also"), jumpEditor(d, "jump"),
          h("p", { class: "small muted" }, "Drag the hotspot on the picture to move it; drag its round handle to resize."));
        break;
      case "Choice":
        out.push(field("Question shown to the viewer", tx(d, "prompt", true)), h("h3", {}, "Buttons and where each leads"),
          ...d.options.map((o, n) => h("div", { class: "opt" }, tx(o, "text"), jumpEditor(o, "jump"),
            h("button", { class: "btn danger", onclick: () => { d.options.splice(n, 1); saveProject(); renderInspector(); } }, "Remove button"))),
          h("button", { class: "btn", onclick: () => { d.options.push({ text: "Another option", jump: "Continue" }); saveProject(); renderInspector(); } }, "+ Button"));
        break;
      case "PauseScreen":
        out.push(field("Message", tx(d, "text", true)));
        break;
      case "ScoreCheck":
        out.push(h("p", { class: "small muted" }, `Checks the viewer's score so far (lesson pass mark is ${p.pass_percent}%).`),
          field("At least (%)", num(d, "min_percent", 0, 100, 1)),
          h("h3", {}, "If reached"), jumpEditor(d, "pass"), h("h3", {}, "If not"), jumpEditor(d, "fail"));
        break;
      case "Link":
        out.push(field("Button text", tx(d, "label")), field("Web address", tx(d, "url")));
        break;
    }
    out.push(h("div", { class: "row" },
      h("button", { class: "btn", onclick: () => {
        const copy = JSON.parse(JSON.stringify(it));
        copy.id = allocId(p); copy.time = Math.min(scene.duration, it.time + 2);
        scene.interactions.push(copy); scene.interactions.sort((a, b) => a.time - b.time);
        state.selItem = copy.id; rerender();
      } }, "Duplicate"),
      h("button", { class: "btn danger", onclick: () => { scene.interactions = scene.interactions.filter((x) => x.id !== it.id); state.selItem = null; rerender(); } }, "Delete")));
    return out;
  }

  renderInspector();
  requestAnimationFrame(() => update());
}

// ---------------------------------------------------------------- Story map

function renderMap(main) {
  const p = state.project;
  const layer = h("div", { class: "layer" });
  const svg = document.createElementNS("http://www.w3.org/2000/svg", "svg");
  const map = h("div", { class: "map" }, layer);
  layer.append(svg);
  const page = h("div", { class: "page stack" },
    h("div", { class: "row" },
      h("h2", { style: { margin: 0, marginRight: "auto" } }, "Story map"),
      h("button", { class: "btn", onclick: () => {
        const n = p.scenes.length;
        state.selScene = addScene(p, `Scene ${n + 1}`, 30, n % LOOKS.length, [100 - state.mapOffset.x, 300 - state.mapOffset.y]);
        saveProject(); render();
      } }, "+ Scene"),
      h("button", { class: "btn", onclick: () => { tidyMap(); state.mapZoom = null; render(); } }, "Tidy up"),
      h("button", { class: "btn", "aria-label": "Zoom out", onclick: () => zoomBy(1 / 1.25) }, "−"),
      h("button", { class: "btn", "aria-label": "Zoom in", onclick: () => zoomBy(1.25) }, "+"),
      h("button", { class: "btn", onclick: () => { state.mapZoom = null; fit(); } }, "Fit"),
      h("button", { class: "btn primary", onclick: () => { state.selItem = null; state.editTime = 0; setTab("edit"); } }, icon("edit"), "Edit selected")),
    h("p", { class: "muted small", style: { margin: 0 } }, "Each box is a scene. Arrows show where choices, answers and scene endings lead. Drag boxes to arrange them; drag the background to move around."),
    map);
  main.append(page);

  const nodes = new Map();
  for (const s of p.scenes) {
    const cv = h("canvas");
    const finishes = s.interactions.some((i) => jumpsOf(i.kind).some(([, j]) => jumpType(j) === "Finish")) || ["Finish", "Continue"].includes(jumpType(s.at_end));
    const el = h("div", { class: "node" + (state.selScene === s.id ? " sel" : "") }, cv,
      h("div", {}, h("div", { class: "nm" }, s.name), h("div", { class: "meta" }, s.video ? `video ${fmtTime(clipStart(s))}–${fmtTime(clipStart(s) + s.duration)}` : `${fmtTime(s.duration)} sample clip`, ` · ${s.interactions.length} items`), finishes ? h("div", { class: "meta" }, "can finish here") : null),
      s.id === p.start_scene ? h("span", { class: "start" }, "START") : null);
    el.style.left = `${s.map_pos[0]}px`;
    el.style.top = `${s.map_pos[1]}px`;
    layer.append(el);
    nodes.set(s.id, el);
    requestAnimationFrame(() => drawSample(cv, s, s.duration * 0.3, false));
    el.addEventListener("pointerdown", (e) => {
      e.stopPropagation();
      el.setPointerCapture(e.pointerId);
      const start = [e.clientX, e.clientY], orig = s.map_pos.slice();
      state.selScene = s.id;
      nodes.forEach((n, id) => n.classList.toggle("sel", id === s.id));
      const move = (ev) => {
        const z = state.mapZoom || 1;
        s.map_pos = [orig[0] + (ev.clientX - start[0]) / z, orig[1] + (ev.clientY - start[1]) / z];
        el.style.left = `${s.map_pos[0]}px`; el.style.top = `${s.map_pos[1]}px`;
        drawEdges();
      };
      const up = () => { el.removeEventListener("pointermove", move); el.removeEventListener("pointerup", up); saveProject(); };
      el.addEventListener("pointermove", move);
      el.addEventListener("pointerup", up);
    });
    el.addEventListener("dblclick", () => { state.selScene = s.id; state.selItem = null; state.editTime = 0; setTab("edit"); });
  }

  const place = () => { layer.style.transform = `translate(${state.mapOffset.x}px, ${state.mapOffset.y}px) scale(${state.mapZoom || 1})`; };
  // Fit the whole story on screen (phones see all of it at once).
  function fit() {
    const w = Math.max(...[...nodes.values()].map((n) => n.offsetLeft + n.offsetWidth), 1) + 20;
    const hh = Math.max(...[...nodes.values()].map((n) => n.offsetTop + n.offsetHeight), 1) + 30;
    state.mapZoom = clamp(Math.min(map.clientWidth / w, map.clientHeight / hh), 0.35, 1);
    state.mapOffset = { x: 10, y: 20 };
    place();
  }
  function zoomBy(f) {
    state.mapZoom = clamp((state.mapZoom || 1) * f, 0.3, 2);
    place();
  }
  if (state.mapZoom == null) requestAnimationFrame(fit);
  place();
  map.addEventListener("pointerdown", (e) => {
    map.setPointerCapture(e.pointerId);
    const start = [e.clientX, e.clientY], orig = { ...state.mapOffset };
    const move = (ev) => { state.mapOffset = { x: orig.x + ev.clientX - start[0], y: orig.y + ev.clientY - start[1] }; place(); };
    const up = () => { map.removeEventListener("pointermove", move); map.removeEventListener("pointerup", up); };
    map.addEventListener("pointermove", move);
    map.addEventListener("pointerup", up);
  });

  function drawEdges() {
    const ns = "http://www.w3.org/2000/svg";
    svg.replaceChildren();
    let n = 0;
    for (const s of p.scenes) {
      const from = nodes.get(s.id);
      const out = [];
      for (const i of s.interactions) for (const [label, j] of jumpsOf(i.kind)) if (jumpType(j) === "Scene") out.push([label, jumpValue(j), COLOURS[kindType(i.kind)]]);
      if (jumpType(s.at_end) === "Scene") out.push(["at end", jumpValue(s.at_end), "#aaa"]);
      out.forEach(([label, target, col], k) => {
        const to = nodes.get(target);
        if (!to || !from) return;
        const fw = from.offsetWidth, fh = from.offsetHeight, th = to.offsetHeight;
        const fx = from.offsetLeft, fy = from.offsetTop, tx = to.offsetLeft, ty = to.offsetTop;
        const spread = (k - (out.length - 1) / 2) * 12;
        const forward = tx >= fx;
        const a = [forward ? fx + fw : fx, fy + fh / 2 + spread];
        const b = [forward ? tx : tx + to.offsetWidth, ty + th / 2 + spread / 2];
        const dir = b[0] >= a[0] ? 1 : -1;
        const bend = Math.max(60, Math.abs(b[0] - a[0]) / 2) * dir;
        const path = document.createElementNS(ns, "path");
        path.setAttribute("d", `M${a[0]},${a[1]} C${a[0] + bend},${a[1]} ${b[0] - bend},${b[1]} ${b[0]},${b[1]}`);
        path.setAttribute("stroke", col); path.setAttribute("stroke-width", "2.5"); path.setAttribute("fill", "none");
        const head = document.createElementNS(ns, "path");
        head.setAttribute("d", `M${b[0]},${b[1]} l${-10 * dir},-6 l0,12 z`);
        head.setAttribute("fill", col);
        const mx = (a[0] + b[0]) / 2, my = (a[1] + b[1]) / 2 + (n % 2 ? 10 : -10);
        const txt = document.createElementNS(ns, "text");
        txt.setAttribute("x", mx); txt.setAttribute("y", my); txt.setAttribute("text-anchor", "middle"); txt.setAttribute("class", "edge-label");
        txt.textContent = trim(label, 22);
        const bg = document.createElementNS(ns, "rect");
        const w = Math.min(22, label.length) * 6.6 + 12;
        bg.setAttribute("x", mx - w / 2); bg.setAttribute("y", my - 13); bg.setAttribute("width", w); bg.setAttribute("height", 18);
        bg.setAttribute("rx", 5); bg.setAttribute("fill", col); bg.setAttribute("opacity", ".55");
        svg.append(path, head, bg, txt);
        n++;
      });
    }
  }
  requestAnimationFrame(drawEdges);
}

function tidyMap() {
  const p = state.project;
  const depth = new Map([[p.start_scene, 0]]);
  const q = [p.start_scene];
  while (q.length) {
    const id = q.shift();
    const s = sceneOf(p, id);
    if (!s) continue;
    const targets = s.interactions.flatMap((i) => jumpsOf(i.kind)).filter(([, j]) => jumpType(j) === "Scene").map(([, j]) => jumpValue(j));
    if (jumpType(s.at_end) === "Scene") targets.push(jumpValue(s.at_end));
    for (const t of targets) if (!depth.has(t)) { depth.set(t, depth.get(id) + 1); q.push(t); }
  }
  const maxD = Math.max(0, ...depth.values());
  const rows = new Map();
  const narrow = !isWide();
  for (const s of p.scenes) {
    const d = depth.get(s.id) ?? maxD + 1;
    const r = rows.get(d) ?? 0;
    rows.set(d, r + 1);
    // On phones lay the story out top to bottom.
    s.map_pos = narrow ? [20 + r * 240, 30 + d * 140] : [40 + d * 280, 40 + r * 120];
  }
  state.mapOffset = { x: 10, y: 10 };
  saveProject();
}

// ---------------------------------------------------------------- Results

function renderResults(main) {
  const all = state.history.slice();
  if (state.player.session.events.length) all.push(state.player.session);
  const page = h("div", { class: "page stack" });
  main.append(page);
  page.append(h("div", { class: "row" },
    h("h2", { style: { margin: 0, marginRight: "auto" } }, "Results"),
    h("button", { class: "btn", disabled: !all.length, onclick: () => download("results.csv", toCsv(all), "text/csv") }, "Export table"),
    h("button", { class: "btn", disabled: !all.length, onclick: () => download("statements.json", toStatements(all), "application/json") }, "Export learning records"),
    h("button", { class: "btn danger", disabled: !state.history.length, onclick: () => {
      if (!confirm("Delete all earlier viewings?")) return;
      state.history = []; saveResults(); render();
    } }, "Clear")));
  page.append(h("p", { class: "muted small", style: { margin: 0 } }, "Every viewing is recorded: answers, choices, taps, score and whether it was finished."));
  if (!all.length) { page.append(h("div", { class: "empty" }, "No viewings yet. Watch a lesson and come back here.")); return; }
  state.selSession = clamp(state.selSession, 0, all.length - 1);
  const sel = all[state.selSession];

  const stats = [];
  for (const s of all) for (const e of s.events.filter((e) => e.verb === "answered")) {
    let row = stats.find((r) => r.q === e.object);
    if (!row) stats.push((row = { q: e.object, total: 0, right: 0, scored: e.success !== null }));
    row.total++;
    if (e.success === true) row.right++;
  }

  page.append(h("div", { class: "results" },
    h("section", { class: "panel stack" },
      h("h3", { style: { marginTop: 0 } }, "Viewings"),
      ...all.map((s, n) => h("button", { class: "viewing" + (n === state.selSession ? " sel" : ""), onclick: () => { state.selSession = n; render(); } },
        h("strong", {}, s.learner || "Guest"),
        s.max_score > 0 ? h("span", { class: "pill " + (passed(s) ? "good" : "bad") }, passed(s) ? "passed" : "not passed") : h("span", { class: "pill" }, s.completed ? "finished" : "in progress"),
        h("span", { class: "small muted" }, `${s.project} · ${s.started_at.replace("T", " ").replace("Z", "")}`),
        h("span", { class: "small" }, s.max_score > 0 ? `${s.score}/${s.max_score} (${percent(s)}%)` : fmtTime(elapsed(s))))).reverse(),
      h("h3", {}, "Questions across all viewings"),
      stats.length ? null : h("p", { class: "muted" }, "No answers yet."),
      ...stats.map((r) => h("div", { class: "qstat" }, h("span", {}, trim(r.q, 70)), h("span", { class: "muted" }, r.scored ? `${r.right}/${r.total} right` : `${r.total} open answer(s)`)))),
    h("section", { class: "panel" },
      h("h3", { style: { marginTop: 0 } }, `What ${sel.learner || "Guest"} did`),
      h("p", { class: "small muted" }, `Path: ${sel.path.join(" → ")}`),
      ...sel.events.map((e) => h("div", { class: "event" },
        h("span", { class: "when" }, fmtTime(e.at)),
        h("span", { class: "what" }, `${e.verb} · ${trim(e.object, 60)}`),
        e.detail ? h("span", { class: "detail " + (e.success === true ? "good" : e.success === false ? "bad" : "") }, e.detail) : null)))));
}

function csvCell(s) { s = String(s ?? ""); return /[",\n]/.test(s) ? `"${s.replace(/"/g, '""')}"` : s; }
function toCsv(sessions) {
  const rows = ["viewer,lesson,started,seconds_in,scene,video_time,action,item,detail,correct,points"];
  for (const s of sessions) for (const e of s.events) {
    rows.push([s.learner, s.project, s.started_at, e.at.toFixed(1), e.scene, e.video_time, e.verb, e.object, e.detail, e.success === null ? "" : e.success ? "yes" : "no", e.points].map(csvCell).join(","));
  }
  return rows.join("\n") + "\n";
}
function toStatements(sessions) {
  const out = [];
  for (const s of sessions) {
    const actor = { name: s.learner, objectType: "Agent" };
    for (const e of s.events) {
      const result = {};
      if (e.success !== null) { result.success = e.success; result.score = { raw: e.points }; }
      if (e.detail) result.response = e.detail;
      out.push({ actor, verb: { display: { "en-US": e.verb } }, object: { definition: { name: { "en-US": e.object } }, extensions: { scene: e.scene, videoTime: e.video_time } }, result, context: { lesson: s.project, sessionStarted: s.started_at, secondsIn: Math.round(e.at * 10) / 10 } });
    }
    out.push({ actor, verb: { display: { "en-US": s.completed ? "completed" : "suspended" } }, object: { definition: { name: { "en-US": s.project } } }, result: { completion: s.completed, success: passed(s), score: { raw: s.score, max: s.max_score, scaled: percent(s) / 100 } } });
  }
  return JSON.stringify(out, null, 2);
}

// ---------------------------------------------------------------- start

function boot() {
  textSize(store.get("cq.textsize", "large"));
  const saved = store.get("cq.project", null);
  // First visit: open the story-map demo (the branching YouTube lesson).
  const firstDemo = EXAMPLES.find(([name]) => name.startsWith("Story map demo")) || EXAMPLES[0];
  state.project = saved && Array.isArray(saved.scenes) && saved.scenes.length ? saved : firstDemo[1]();
  state.selScene = state.project.start_scene;
  state.player = new Player(state.project, state.learner);

  // Address options, e.g. #example=1&tab=edit&at=15.9 (used for checks and links).
  const opts = new URLSearchParams(location.hash.slice(1));
  if (opts.has("example")) loadProject(EXAMPLES[clamp(+opts.get("example") || 0, 0, EXAMPLES.length - 1)][1]());
  if (opts.has("tab") && TABS.some(([id]) => id === opts.get("tab"))) state.tab = opts.get("tab");
  if (opts.has("at")) {
    const t = +opts.get("at");
    state.player.start();
    state.player.maxWatched = t;
    state.player.seek(t);
    state.editTime = t;
    const it = selectedScene().interactions.find((i) => i.time >= t);
    if (state.tab === "edit" && it) { state.selItem = it.id; state.editTime = it.time; }
  }

  $("#menu-btn").onclick = () => openMenu($("#menu").hidden);
  $("#menu-backdrop").onclick = () => openMenu(false);
  $("#open-file").onchange = async (e) => {
    const f = e.target.files[0];
    if (!f) return;
    try {
      const p = JSON.parse(await f.text());
      if (!Array.isArray(p.scenes) || !p.scenes.length) throw new Error();
      loadProject(p);
      toast("Lesson opened");
      setTab("watch");
    } catch { toast("That file isn't a lesson"); }
    e.target.value = "";
  };
  let lastWide = isWide();
  window.addEventListener("resize", () => { if (isWide() !== lastWide) { lastWide = isWide(); render(); } });
  window.addEventListener("beforeunload", saveResults);
  render();
  requestAnimationFrame(loop);
}

boot();
