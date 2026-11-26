// Builds every example lesson in web/examples.js and checks that it holds together:
// a valid start scene, unique ids, every jump leads to a real scene or time, and
// every interaction, chapter and caption sits inside its scene. Run: node tests/check-examples.js
"use strict";
const fs = require("fs");
const path = require("path");
const vm = require("vm");

const file = path.join(__dirname, "..", "web", "examples.js");
const context = {};
vm.createContext(context);
vm.runInContext(fs.readFileSync(file, "utf8") + "\nthis.EXAMPLES = EXAMPLES;", context, { filename: file });

const KINDS = ["Info", "Question", "OpenQuestion", "Hotspot", "Choice", "PauseScreen", "ScoreCheck", "Link"];
let problems = 0;
const fail = (lesson, msg) => { problems++; console.error(`✗ ${lesson}: ${msg}`); };

for (const [name, make] of context.EXAMPLES) {
  const p = make();
  const ids = new Set();
  const sceneIds = new Set(p.scenes.map((s) => s.id));
  if (!p.scenes.length) fail(name, "has no scenes");
  if (!sceneIds.has(p.start_scene)) fail(name, `start scene ${p.start_scene} doesn't exist`);
  const checkJump = (where, j, duration) => {
    if (j === "Continue" || j === "Finish") return;
    if (j && typeof j === "object" && "Scene" in j) { if (!sceneIds.has(j.Scene)) fail(name, `${where} jumps to missing scene ${j.Scene}`); return; }
    if (j && typeof j === "object" && "Time" in j) { if (!(j.Time >= 0 && j.Time <= duration)) fail(name, `${where} jumps to ${j.Time}s, outside the scene`); return; }
    fail(name, `${where} has an unknown jump ${JSON.stringify(j)}`);
  };
  for (const s of p.scenes) {
    for (const id of [s.id, ...s.interactions.map((i) => i.id)]) {
      if (ids.has(id)) fail(name, `id ${id} is used twice`);
      if (id >= p.next_id) fail(name, `id ${id} isn't below next_id ${p.next_id}`);
      ids.add(id);
    }
    if (!(s.duration > 0)) fail(name, `scene "${s.name}" has no length`);
    checkJump(`end of "${s.name}"`, s.at_end, s.duration);
    for (const i of s.interactions) {
      const kind = Object.keys(i.kind)[0];
      const d = i.kind[kind];
      const where = `"${s.name}" ${kind} at ${i.time}s`;
      if (!KINDS.includes(kind)) fail(name, `${where}: unknown kind`);
      if (!(i.time >= 0 && i.time <= s.duration)) fail(name, `${where} is outside the scene (${s.duration}s)`);
      if (kind === "Question") {
        if (!d.options.some((o) => o.correct)) fail(name, `${where} has no right answer`);
        checkJump(`${where} (right)`, d.on_correct, s.duration);
        checkJump(`${where} (wrong)`, d.on_wrong, s.duration);
      }
      if (kind === "Choice") {
        if (d.options.length < 2) fail(name, `${where} offers fewer than two options`);
        d.options.forEach((o) => checkJump(`${where} "${o.text}"`, o.jump, s.duration));
      }
      if (kind === "Hotspot") {
        const [x, y, w, h] = d.rect;
        if (x < 0 || y < 0 || x + w > 1.0001 || y + h > 1.0001) fail(name, `${where} sticks out of the picture`);
        checkJump(where, d.jump, s.duration);
      }
      if (kind === "ScoreCheck") { checkJump(`${where} (pass)`, d.pass, s.duration); checkJump(`${where} (fail)`, d.fail, s.duration); }
      if (kind === "Link" && !/^https?:\/\//.test(d.url)) fail(name, `${where} has a bad web address`);
    }
    for (const c of s.chapters) if (!(c.time >= 0 && c.time <= s.duration)) fail(name, `chapter "${c.title}" is outside "${s.name}"`);
    for (const c of s.captions) if (!(c.start <= c.end && c.start >= 0)) fail(name, `caption "${c.text}" has bad times`);
  }
  console.log(`✓ ${name}: ${p.scenes.length} scene(s), ${p.scenes.reduce((n, s) => n + s.interactions.length, 0)} interaction(s)`);
}

if (problems) {
  console.error(`\n${problems} problem(s) found`);
  process.exit(1);
}
console.log("\nAll example lessons hold together.");
