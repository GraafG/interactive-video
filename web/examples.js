// Example lessons, in the same file format as the desktop app, so lessons move
// freely between the two. Modelled on the interaction patterns in the research:
// in-video quizzes, timed cards with hotspots, branching stories, product tours.
"use strict";

const LOOKS = ["Ocean", "Sunset", "Forest", "Night", "Studio", "Candy"];

function blankProject() {
  const p = { title: "Untitled lesson", description: "", scenes: [], start_scene: 1, pass_percent: 60, prevent_skipping: false, next_id: 1 };
  p.start_scene = addScene(p, "Scene 1", 40, 0, [80, 120]);
  return p;
}

function allocId(p) { return p.next_id++; }

function addScene(p, name, duration, look, pos) {
  const id = allocId(p);
  p.scenes.push({ id, name, video: null, duration, look, interactions: [], chapters: [], captions: [], at_end: "Finish", map_pos: pos });
  return id;
}

function addInteraction(p, sceneId, time, kind, showFor) {
  const id = allocId(p);
  const t = Object.keys(kind)[0];
  const s = p.scenes.find((s) => s.id === sceneId);
  s.interactions.push({ id, time, show_for: showFor ?? (t === "Hotspot" ? 6 : t === "Link" ? 8 : 5), pause: false, kind });
  s.interactions.sort((a, b) => a.time - b.time);
  return id;
}

function mc(prompt, opts, points) {
  return { Question: { prompt, options: opts.map(([text, correct, feedback]) => ({ text, correct, feedback })), points, allow_retry: false, on_correct: "Continue", on_wrong: "Continue" } };
}
const cap = (start, end, text) => ({ start, end, text });
const chap = (time, title) => ({ time, title });
const sceneOf = (p, id) => p.scenes.find((s) => s.id === id);

const EXAMPLES = [
  ["Lab safety quiz", () => {
    const p = blankProject(); p.scenes = [];
    p.title = "Lab safety quiz";
    p.description = "A short science lesson with questions along the way. Score 60% or more to pass; otherwise a review clip plays.";
    p.prevent_skipping = true;
    const lesson = addScene(p, "Lesson: staying safe", 72, 0, [60, 140]);
    const review = addScene(p, "Review: the key rules", 30, 1, [380, 40]);
    const done = addScene(p, "Well done!", 14, 2, [380, 250]);
    p.start_scene = lesson;
    addInteraction(p, lesson, 4, { Info: { title: "In this lesson", body: "Protective gear, handling liquids and what to do after a spill." } });
    addInteraction(p, lesson, 16, mc("What should you wear before handling any liquid in the lab?", [["Safety goggles", true, "Right: eyes come first."], ["Sunglasses", false, "Sunglasses don't seal around the eyes."], ["Nothing special", false, "Splashes happen when you least expect them."]], 10));
    addInteraction(p, lesson, 30, mc("Where do you put broken glass?", [["The ordinary bin", false, "Someone could get cut."], ["The marked glass container", true, "Correct."], ["Down the sink", false, "That blocks drains and hurts people."]], 10));
    addInteraction(p, lesson, 44, mc("Someone spills a liquid. What comes first?", [["Tell the teacher", true, "Yes: they know how to clean it safely."], ["Wipe it with your sleeve", false, "Never touch an unknown spill."], ["Ignore it", false, "Others might slip or touch it."]], 10));
    addInteraction(p, lesson, 58, { OpenQuestion: { prompt: "In your own words: why do we tie long hair back in the lab?" } });
    addInteraction(p, lesson, 71, { ScoreCheck: { min_percent: 60, pass: { Scene: done }, fail: { Scene: review } } });
    Object.assign(sceneOf(p, lesson), {
      chapters: [chap(0, "Welcome"), chap(12, "Protective gear"), chap(26, "Glass and sharp things"), chap(40, "Spills"), chap(55, "Wrap-up")],
      captions: [cap(0.5, 4, "Welcome to the lab! Today: how to stay safe."), cap(8, 15, "First, let's look at what you wear."), cap(20, 29, "Glass can break. Sharp things need their own place."), cap(34, 43, "Accidents happen. What matters is what you do next."), cap(48, 57, "Let's put it all together."), cap(62, 70, "Let's see how you did.")],
    });
    addInteraction(p, review, 3, { Info: { title: "Remember", body: "Goggles on · glass in the marked box · tell an adult about spills." } });
    addInteraction(p, review, 20, { Choice: { prompt: "Ready to try the lesson again?", options: [{ text: "Watch the lesson again", jump: { Scene: lesson } }, { text: "Finish for now", jump: "Finish" }] } });
    sceneOf(p, review).captions = [cap(1, 12, "Let's go over the three rules once more.")];
    sceneOf(p, done).captions = [cap(1, 10, "Great work: you know how to stay safe in the lab.")];
    return p;
  }],
  ["Smoothie recipe", () => {
    const p = blankProject(); p.scenes = [];
    p.title = "Smoothie recipe";
    p.description = "A timed cooking video with an ingredients card, tappable hotspots, bookmarks and a quick check.";
    const s = addScene(p, "Make a berry smoothie", 60, 5, [100, 140]);
    p.start_scene = s;
    addInteraction(p, s, 9, { Info: { title: "Ingredients list", body: "1 banana · a handful of berries · 1 cup of yoghurt · a splash of milk · honey to taste" } });
    addInteraction(p, s, 18, { Hotspot: { rect: [0.12, 0.45, 0.22, 0.3], label: "Banana", info: "Riper bananas make the smoothie sweeter.", jump: "Continue" } }, 10);
    addInteraction(p, s, 18, { Hotspot: { rect: [0.62, 0.4, 0.24, 0.32], label: "Berries", info: "Frozen berries work too and make it colder.", jump: "Continue" } }, 10);
    addInteraction(p, s, 34, mc("Which ingredient makes the smoothie creamy?", [["Yoghurt", true, "Yes, yoghurt adds creaminess."], ["Honey", false, "Honey adds sweetness."], ["Berries", false, "Berries add flavour and colour."]], 5));
    addInteraction(p, s, 44, { PauseScreen: { text: "Your turn: blend everything for 30 seconds, then press Continue." } });
    addInteraction(p, s, 52, mc("True or false: you should fill the blender to the very top.", [["True", false, "Leave room, or it can overflow."], ["False", true, "Right, leave some room."]], 5));
    Object.assign(sceneOf(p, s), {
      chapters: [chap(0, "Intro"), chap(8, "Ingredients"), chap(25, "Blending"), chap(50, "Serving")],
      captions: [cap(1, 7, "Hi! Let's make a quick berry smoothie."), cap(10, 17, "Here's everything you need."), cap(19, 27, "Tap the ingredients to learn more."), cap(28, 33, "Into the blender it goes."), cap(46, 51, "Pour it into a glass and enjoy.")],
    });
    return p;
  }],
  ["Choose your path", () => {
    const p = blankProject(); p.scenes = [];
    p.title = "Choose your path";
    p.description = "A branching story: the viewer's choices decide which clip plays next and which ending they reach.";
    const intro = addScene(p, "The new job", 24, 3, [40, 160]);
    const help = addScene(p, "You help a colleague", 26, 2, [320, 50]);
    const alone = addScene(p, "You work alone", 26, 1, [320, 270]);
    const good = addScene(p, "Ending: team player", 12, 0, [600, 60]);
    const mixed = addScene(p, "Ending: lesson learned", 12, 4, [600, 260]);
    p.start_scene = intro;
    addInteraction(p, intro, 20, { Choice: { prompt: "A colleague looks stuck on a task. What do you do?", options: [{ text: "Offer to help", jump: { Scene: help } }, { text: "Focus on my own work", jump: { Scene: alone } }] } });
    addInteraction(p, help, 18, { Choice: { prompt: "Your manager asks who solved the problem.", options: [{ text: "\"We did it together\"", jump: { Scene: good } }, { text: "\"I did\"", jump: { Scene: mixed } }] } });
    addInteraction(p, alone, 10, { Info: { title: "Meanwhile…", body: "The deadline is getting close for the whole team." } });
    addInteraction(p, alone, 20, { Choice: { prompt: "The team misses the deadline. What now?", options: [{ text: "Go back and offer help", jump: { Scene: help } }, { text: "Carry on", jump: { Scene: mixed } }] } });
    addInteraction(p, mixed, 10, { Choice: { prompt: "Want to try a different path?", options: [{ text: "Start again", jump: { Scene: intro } }, { text: "Finish", jump: "Finish" }] } });
    [[intro, "It's your first week. The office is busy."], [help, "You sit down together and work through it."], [alone, "You finish your own list early."], [good, "The team trusts you. Great start!"], [mixed, "Every choice teaches something. Try another path?"]]
      .forEach(([id, text]) => { sceneOf(p, id).captions = [cap(1, 9, text)]; });
    return p;
  }],
  ["Video demo: AMD and gamers", () => {
    const p = blankProject(); p.scenes = [];
    p.title = "Video demo: AMD and gamers";
    p.description = "A Gamers Nexus video (YouTube) turned into a lesson: chapters, a few checks along the way and a shortcut to the conclusion. Questions are based on the video's own description and chapter list.";
    p.pass_percent = 60;
    const s = addScene(p, "AMD's message to gamers", 1788, 3, [100, 140]);
    p.start_scene = s;
    sceneOf(p, s).video = "https://www.youtube.com/watch?v=qo9NYpDJkSo";
    addInteraction(p, s, 2, { Info: { title: "About this video", body: "How AMD talks to gamers while calling itself a data-center-first company. Use the chapters to jump around; questions pop up at the end of some chapters." } }, 12);
    addInteraction(p, s, 3, { Link: { label: "Watch on YouTube", url: "https://www.youtube.com/watch?v=qo9NYpDJkSo" } }, 20);
    addInteraction(p, s, 222, mc("What does the video say AMD recently called itself?", [["A data-center-first company", true, "Right: that's the starting point of the video."], ["A gaming-only company", false, "The video points out the opposite."], ["A phone maker", false, "Not this one."]], 10));
    addInteraction(p, s, 568, { OpenQuestion: { prompt: "AMD's posts told gamers it has always been on their side. Having heard this part, how convincing do you find that message, and why?" } });
    addInteraction(p, s, 1075, mc("Which part of the market does the video say AMD now puts first?", [["Data centers", true, "Yes: the chapters keep coming back to this."], ["Game consoles", false, "Consoles aren't the focus here."], ["Budget graphics cards", false, "The chapter list points to high-end products instead."]], 10));
    addInteraction(p, s, 1176, { Choice: { prompt: "Next up: AMD's price increases. Keep watching, or jump straight to the conclusion?", options: [{ text: "Keep watching", jump: "Continue" }, { text: "Jump to the conclusion", jump: { Time: 1317 } }] } });
    addInteraction(p, s, 1315, mc("Which topic comes right before the conclusion?", [["AMD price increases", true, "Correct."], ["Low risk of smuggling", false, "That one comes early in the video."], ["High-end products only", false, "That's a bit earlier."]], 10));
    Object.assign(sceneOf(p, s), {
      chapters: [chap(0, "AMD a data center company"), chap(226, "Low risk of smuggling"), chap(446, "AMD marketing finds foot"), chap(572, "AMD reduces consumer focus"), chap(905, "High-end products only"), chap(1078, "AMD loves data centers"), chap(1180, "AMD price increases"), chap(1317, "Conclusion")],
    });
    return p;
  }],
  ["Story map demo: AMD routes", () => {
    // One long video cut into scenes by its own chapter timestamps, arranged as routes
    // the viewer picks from a hub. Uses every kind of interaction.
    const p = blankProject(); p.scenes = [];
    const yt = "https://www.youtube.com/watch?v=qo9NYpDJkSo";
    p.title = "Story map demo: AMD routes";
    p.description = "The Gamers Nexus video cut into routes. Pick what to explore from the hub, come back for another route, then watch the conclusion. A score of 40% or more unlocks the 'ready to discuss' ending. Questions are based on the video's description and chapter titles.";
    p.pass_percent = 40;
    const part = (name, start, end, pos) => {
      const id = addScene(p, name, end - start, 3, pos);
      Object.assign(sceneOf(p, id), { video: yt, clip_start: start });
      return id;
    };
    const start = part("Start: AMD, a data center company", 0, 226, [40, 230]);
    const hub = addScene(p, "Hub: choose a route", 12, 3, [320, 230]);
    const mkt = part("Route A · Marketing finds its foot", 446, 572, [600, 40]);
    const focus = part("Route A · Less focus on consumers", 572, 905, [880, 40]);
    const high = part("Route B · High-end products only", 905, 1078, [600, 230]);
    const price = part("Route B · Price increases", 1180, 1317, [880, 230]);
    const dc = part("Route C · AMD loves data centers", 1078, 1180, [600, 420]);
    const smug = part("Route C · Low risk of smuggling", 226, 446, [880, 420]);
    const concl = part("Conclusion", 1317, 1788, [320, 560]);
    const ready = addScene(p, "Ending: ready to discuss", 16, 2, [40, 720]);
    const recap = addScene(p, "Ending: quick recap", 30, 1, [600, 720]);
    p.start_scene = start;
    const hubChoice = (prompt) => ({ Choice: { prompt, options: [
      { text: "Route A: marketing vs. reality", jump: { Scene: mkt } },
      { text: "Route B: products and prices", jump: { Scene: high } },
      { text: "Route C: data centers and exports", jump: { Scene: dc } },
      { text: "Go to the conclusion", jump: { Scene: concl } },
    ] } });

    // Start
    addInteraction(p, start, 2, { Info: { title: "How this demo works", body: "The video is cut into routes. At the end of each part you choose where to go; the story map shows every path. Questions add to your score." } }, 12);
    addInteraction(p, start, 6, { Hotspot: { rect: [0.03, 0.06, 0.32, 0.24], label: "Who's this?", info: "Steve Burke, host and writer at Gamers Nexus (from the video's credits).", jump: "Continue" } }, 20);
    addInteraction(p, start, 6, { Link: { label: "Video on YouTube", url: yt } }, 20);
    const q1 = mc("What does AMD now call itself, according to the video?", [["A data-center-first company", true, "Right: everything else in the video builds on this."], ["A gaming company first", false, "Not quite: let's rewatch the last bit."], ["A chip importer", false, "Not quite: let's rewatch the last bit."]], 10);
    q1.Question.on_wrong = { Time: 150 };
    addInteraction(p, start, 200, q1);
    sceneOf(p, start).at_end = { Scene: hub };

    // The hub every route comes back to
    addInteraction(p, hub, 1, { Info: { title: "Pick a route", body: "Each route is two parts of the video. Come back here afterwards for another one, or go to the conclusion when you're done." } }, 9);
    addInteraction(p, hub, 10, hubChoice("Where do you want to go?"));
    sceneOf(p, hub).captions = [cap(0.5, 9, "Three routes through the video. Which one first?")];

    // Route A
    addInteraction(p, mkt, 2, { PauseScreen: { text: "Before this part: think of a time a brand said it has always been on your side. Did you believe it? Press Continue when you're ready." } });
    addInteraction(p, mkt, 118, { OpenQuestion: { prompt: "In your own words: what is the mixed message the video says AMD sends to gamers?" } });
    sceneOf(p, mkt).at_end = { Scene: focus };
    addInteraction(p, focus, 3, { Info: { title: "Watch for", body: "How AMD's 'data-center-first' line compares with what it tells gamers." } }, 10);
    const qa = mc("Which customers does the video say AMD is putting first now?", [["Data centers", true, "Yes."], ["Gamers", false, "That's the message the video questions."], ["Console makers", false, "Not the focus here."]], 10);
    qa.Question.allow_retry = true;
    addInteraction(p, focus, 325, qa);
    sceneOf(p, focus).at_end = { Scene: hub };

    // Route B
    addInteraction(p, high, 5, { Hotspot: { rect: [0.58, 0.52, 0.36, 0.38], label: "High-end only?", info: "This part looks at AMD concentrating on its most expensive products.", jump: "Continue" } }, 25);
    addInteraction(p, high, 168, mc("The chapter is called 'High-end products only'. What does that suggest for budget buyers?", [["Fewer new low-cost options", true, "That's the worry the chapter raises."], ["Free upgrades", false, "Unfortunately not."], ["Nothing changes", false, "The chapter suggests otherwise."]], 10));
    sceneOf(p, high).at_end = { Scene: price };
    addInteraction(p, price, 3, { Info: { title: "Price check", body: "Note the price changes mentioned; you'll be asked what you think." } }, 10);
    addInteraction(p, price, 128, { OpenQuestion: { prompt: "Would these price increases change what you'd buy next? Why or why not?" } });
    sceneOf(p, price).at_end = { Scene: hub };

    // Route C
    addInteraction(p, dc, 95, mc("What does this chapter's title say AMD loves?", [["Data centers", true, "Correct."], ["Gamers", false, "Listen again to the title."], ["Consoles", false, "Not this time."]], 10));
    sceneOf(p, dc).at_end = { Scene: smug };
    addInteraction(p, smug, 3, { Info: { title: "Context", body: "This part covers the risk of chips being smuggled abroad; the chapter title calls it low." } }, 10);
    addInteraction(p, smug, 210, mc("How does the chapter title describe the risk of smuggling?", [["Low", true, "Right."], ["High", false, "The title says the opposite."], ["It isn't mentioned", false, "It's the whole chapter."]], 10));
    sceneOf(p, smug).at_end = { Scene: hub };

    // Conclusion and endings
    addInteraction(p, concl, 3, { Info: { title: "Last part", body: "The wrap-up. At the end, your score picks your ending." } }, 10);
    addInteraction(p, concl, 450, mc("Which chapter comes right before the conclusion?", [["AMD price increases", true, "Correct."], ["Low risk of smuggling", false, "That one is early in the video."], ["High-end products only", false, "That's earlier."]], 10));
    addInteraction(p, concl, 469, { ScoreCheck: { min_percent: 40, pass: { Scene: ready }, fail: { Scene: recap } } });
    addInteraction(p, ready, 1, { Info: { title: "You're ready to discuss", body: "You got most checks right. Bring your open answers to the discussion." } }, 14);
    addInteraction(p, ready, 4, { Link: { label: "Watch the full video", url: yt } }, 12);
    addInteraction(p, recap, 1, { Info: { title: "Quick recap", body: "Data center company → smuggling risk → marketing → less consumer focus → high-end only → loves data centers → price increases → conclusion." } }, 20);
    addInteraction(p, recap, 26, { Choice: { prompt: "What now?", options: [{ text: "Try another route", jump: { Scene: hub } }, { text: "Finish", jump: "Finish" }] } });
    sceneOf(p, ready).captions = [cap(1, 10, "Nice work: you explored the video and passed the checks.")];
    sceneOf(p, recap).captions = [cap(1, 12, "Here's the whole video in one line. Try another route?")];
    return p;
  }],
  ["Product tour", () => {
    const p = blankProject(); p.scenes = [];
    p.title = "Product tour";
    p.description = "A product walkthrough with hotspots that open details or jump to a feature, plus a link button.";
    const s = addScene(p, "Meet the lamp", 48, 4, [100, 140]);
    p.start_scene = s;
    addInteraction(p, s, 6, { Hotspot: { rect: [0.18, 0.25, 0.2, 0.3], label: "Dimmer", info: "Turn the ring to set the brightness.", jump: { Time: 22 } } }, 12);
    addInteraction(p, s, 6, { Hotspot: { rect: [0.58, 0.5, 0.22, 0.28], label: "Base", info: "A weighted base so it never tips over.", jump: "Continue" } }, 12);
    addInteraction(p, s, 30, { Choice: { prompt: "What would you like to see next?", options: [{ text: "Colours", jump: { Time: 32 } }, { text: "Setup in 1 minute", jump: { Time: 40 } }] } });
    addInteraction(p, s, 42, { Link: { label: "Read more", url: "https://example.com" } });
    Object.assign(sceneOf(p, s), {
      chapters: [chap(0, "Overview"), chap(20, "Dimmer"), chap(32, "Colours"), chap(40, "Setup")],
      captions: [cap(1, 5, "Here's a quick tour."), cap(7, 16, "Tap the glowing spots to explore."), cap(22, 28, "The dimmer ring sets the mood."), cap(32, 38, "Three colours to choose from."), cap(40, 46, "Plug it in and you're done.")],
    });
    return p;
  }],
];
