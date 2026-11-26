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

/** A scene that plays one part (start to end, in seconds) of a longer video. */
function videoPart(p, url, name, start, end, pos) {
  const id = addScene(p, name, end - start, 3, pos);
  Object.assign(sceneOf(p, id), { video: url, clip_start: start });
  return id;
}

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
  ["Smoothie recipes", () => {
    // A real cooking video (Toasty Apron on YouTube) cut by its own timestamps: the
    // formula first, then a menu to pick which recipes to watch. Amounts come from the
    // video's description.
    const p = blankProject(); p.scenes = [];
    const yt = "https://www.youtube.com/watch?v=9QBVaQdCrZA";
    p.title = "Smoothie recipes";
    p.description = "A real cooking video turned into a lesson: learn the 5-step smoothie formula, then pick the recipes you want to see. Ingredient lists come from the video's description.";
    p.pass_percent = 60;
    const intro = videoPart(p, yt, "Intro and the 5-step formula", 0, 213, [40, 200]);
    const menu = addScene(p, "Pick a recipe", 12, 5, [320, 200]);
    const recipes = [
      ["Apple kiwi green smoothie", 213, 258, "Almond milk · 1 green apple · 2 kiwis · spinach · half an avocado · half a frozen banana · ginger · hemp seeds (520 calories for 2 glasses)"],
      ["Berries and beets smoothie", 258, 302, "Almond milk · frozen mixed berries · frozen pineapple · beets · carrots · frozen banana · honey · chia seeds (308 calories for 2 glasses)"],
      ["Tropical breeze smoothie", 302, 348, "Light coconut milk · mango · pineapple · peach · carrots · zucchini or cucumber · half a frozen banana · lemon · vanilla · flax seeds (393 calories for 2 glasses)"],
      ["Strawberry watermelon smoothie", 348, 386, "Unsweetened almond milk · frozen watermelon · frozen strawberries · half a frozen banana · Greek yogurt · lemon · vanilla · hemp seeds (238 calories for 2 glasses)"],
      ["Chocolate banana smoothie", 386, 420, "Unsweetened almond milk · 2 large frozen bananas · spinach · cocoa powder · honey · chia seeds (398 calories for 2 glasses)"],
    ].map(([name, start, end, ingredients], n) => {
      const id = videoPart(p, yt, name, start, end, [600 + (n % 2) * 280, 20 + n * 90]);
      addInteraction(p, id, 1, { Info: { title: "Ingredients", body: ingredients } }, Math.min(25, end - start - 4));
      sceneOf(p, id).at_end = { Scene: menu };
      return id;
    });
    const outro = videoPart(p, yt, "Outro", 420, 448, [320, 420]);
    p.start_scene = intro;

    addInteraction(p, intro, 2, { Info: { title: "What you'll learn", body: "A simple 5-step formula for healthy smoothies, then five recipes. You pick which recipes to watch." } }, 10);
    addInteraction(p, intro, 6, { Link: { label: "Video on YouTube", url: yt } }, 15);
    addInteraction(p, intro, 52, { Info: { title: "The 5-step formula", body: "1 Liquid (1 cup / 250 ml) · 2 Fruits (1.5 cups) · 3 Veggies (1 cup) · 4 Thickener (optional, 1–2 tbsp) · 5 Boosters (flavour and nutrition)" } }, 60);
    const q1 = mc("How much liquid does the formula start with?", [["1 cup (250 ml)", true, "Right: step 1."], ["3 cups", false, "That would make it very runny."], ["No liquid at all", false, "Liquid is step 1."]], 10);
    q1.Question.allow_retry = true;
    addInteraction(p, intro, 172, q1);
    addInteraction(p, intro, 176, mc("Which step of the formula is optional?", [["The thickener", true, "Yes: 1–2 tbsp, if you like it thick."], ["The liquid", false, "Every smoothie needs liquid."], ["The fruit", false, "Fruit is step 2."]], 10));
    Object.assign(sceneOf(p, intro), {
      chapters: [chap(0, "Intro"), chap(49, "My 5-step smoothie formula"), chap(178, "Freezer storage and batch prep")],
      at_end: { Scene: menu },
    });

    addInteraction(p, menu, 1, { Info: { title: "Pick a recipe", body: "Watch as many as you like; each one brings you back here." } }, 9);
    addInteraction(p, menu, 10, { Choice: { prompt: "Which smoothie do you want to see?", options: [
      ...recipes.map((id) => ({ text: sceneOf(p, id).name.replace(" smoothie", ""), jump: { Scene: id } })),
      { text: "I'm done: show the outro", jump: { Scene: outro } },
    ] } });
    sceneOf(p, menu).captions = [cap(0.5, 9, "Five smoothies. Which one first?")];

    const [green, berries, tropical, strawberry, chocolate] = recipes;
    addInteraction(p, green, 38, mc("What makes the green smoothie creamy?", [["Avocado and frozen banana", true, "Both are thickeners."], ["Ice cubes", false, "Not in this recipe."], ["Extra apple", false, "Apple is a fruit, not the thickener here."]], 10));
    addInteraction(p, berries, 36, { OpenQuestion: { prompt: "Beets and carrots in a smoothie: would you try it? Why or why not?" } });
    addInteraction(p, tropical, 40, mc("Which liquid does the tropical smoothie use?", [["Light coconut milk", true, "Right."], ["Orange juice", false, "Not this time."], ["Water", false, "It uses coconut milk."]], 10));
    addInteraction(p, strawberry, 3, { PauseScreen: { text: "Got frozen fruit ready? This one uses frozen watermelon and strawberries. Press Continue to watch." } });
    addInteraction(p, strawberry, 32, mc("Besides banana, which thickener goes in this smoothie?", [["Greek yogurt", true, "Yes: one tablespoon."], ["Oats", false, "Not in this recipe."], ["Peanut butter", false, "Not this one."]], 10));
    addInteraction(p, chocolate, 4, { Hotspot: { rect: [0.66, 0.05, 0.3, 0.2], label: "Calories?", info: "398 calories for 2 glasses, according to the video's description.", jump: "Continue" } }, 20);

    addInteraction(p, outro, 2, { Info: { title: "Your turn", body: "Pick a liquid, fruits, veggies, a thickener and a booster, and make your own." } }, 20);
    return p;
  }],  ["Choose your path", () => {
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
  ["GPU black market: investigation map", () => {
    // A 3.5-hour Gamers Nexus film cut by its own timestamps into an investigation:
    // an opening, a hub, routes through its parts, and a final part whose score picks
    // the ending. Questions are based on the film's description and chapter titles.
    const p = blankProject(); p.scenes = [];
    const yt = "https://www.youtube.com/watch?v=1H3xQaf7BFI";
    const timeline = "https://gamersnexus.net/gpus-news/timeline-gpu-export-controls-nvidia-gpu-bans-ai-gpu-black-market";
    p.title = "GPU black market: investigation map";
    p.description = "Gamers Nexus' film on the AI GPU black market, as an investigation you steer: start with the opening, then pick which part of the trail to follow from the map. Questions are based on the film's description and chapter titles.";
    p.pass_percent = 40;
    const open = videoPart(p, yt, "The black market (opening)", 0, 706, [40, 300]);
    const hub = addScene(p, "Investigation map", 14, 3, [320, 300]);
    const bg1 = videoPart(p, yt, "Background · Why the ban, timeline", 706, 1292, [600, 20]);
    const bg2 = videoPart(p, yt, "Background · Revenue deal, counting GPUs", 1292, 2019, [880, 20]);
    const hk1 = videoPart(p, yt, "Part 1 · Hong Kong: demand and suppliers", 2019, 2898, [600, 160]);
    const hk2 = videoPart(p, yt, "Part 1 · GPU rich and poor, the military", 2898, 4188, [880, 160]);
    const hk3 = videoPart(p, yt, "Part 1 · Around the ban, the HK market", 4188, 5789, [1160, 160]);
    const cn = videoPart(p, yt, "Parts 2–3 · China and Shenzhen", 5789, 6994, [600, 300]);
    const fence = videoPart(p, yt, "Parts 4–5 · The fence and the warehouse", 6994, 8837, [600, 440]);
    const mod = videoPart(p, yt, "Part 6 · Mod shop and a custom AI GPU", 8837, 10593, [880, 440]);
    const fac = videoPart(p, yt, "Parts 7–9 · Factory, Taiwan, a smuggler", 10593, 11111, [600, 580]);
    const legal = videoPart(p, yt, "Final · Legality and corruption", 11111, 12565, [320, 580]);
    const closed = addScene(p, "Ending: case closed", 14, 2, [40, 760]);
    const review = addScene(p, "Ending: review the evidence", 20, 1, [600, 760]);
    p.start_scene = open;
    const back = (id) => { sceneOf(p, id).at_end = { Scene: hub }; };

    // Opening
    addInteraction(p, open, 3, { Info: { title: "About this film", body: "An investigation into how export-controlled AI GPUs reach China: smugglers, middlemen, fences and governments. It's long; this lesson lets you follow one trail at a time." } }, 12);
    addInteraction(p, open, 3, { Link: { label: "Full film on YouTube", url: yt } }, 15);
    addInteraction(p, open, 690, mc("Whose AI GPUs is this black market mainly about?", [["NVIDIA's", true, "Right: it's in the title."], ["AMD's", false, "AMD comes up, but the title names NVIDIA."], ["Intel's", false, "Not the main subject."]], 10));
    sceneOf(p, open).chapters = [chap(0, "The NVIDIA AI GPU black market"), chap(366, "We need your help"), chap(461, "A big adventure"), chap(610, "Ignored by the US")];
    back(open);

    // The hub
    addInteraction(p, hub, 1, { Info: { title: "Pick a trail", body: "Each trail plays part of the film and brings you back here. Finish with the final part when you're ready." } }, 11);
    addInteraction(p, hub, 12, { Choice: { prompt: "Which trail do you want to follow?", options: [
      { text: "Background: why GPUs are banned", jump: { Scene: bg1 } },
      { text: "Part 1: Hong Kong", jump: { Scene: hk1 } },
      { text: "Parts 2–3: mainland China", jump: { Scene: cn } },
      { text: "Parts 4–6: fence, warehouse, mod shop", jump: { Scene: fence } },
      { text: "Parts 7–9: factory, Taiwan, smuggler", jump: { Scene: fac } },
      { text: "Final: legality and corruption", jump: { Scene: legal } },
    ] } });
    sceneOf(p, hub).captions = [cap(0.5, 11, "Where does the trail lead next?")];

    // Background
    addInteraction(p, bg1, 10, { Link: { label: "Read the full timeline", url: timeline } }, 20);
    addInteraction(p, bg1, 570, mc("Whose export rules make it illegal to ship these GPUs to China without a license?", [["The US government's", true, "Right: licenses are rarely granted."], ["China's", false, "The restrictions come from the US."], ["Taiwan's", false, "Taiwan isn't the one imposing them."]], 10));
    sceneOf(p, bg1).chapters = [chap(0, "Background: why the ban"), chap(258, "Timeline")];
    sceneOf(p, bg1).at_end = { Scene: bg2 };
    addInteraction(p, bg2, 265, mc("What did NVIDIA and AMD negotiate with the US government?", [["A cut of sales of two GPU models", true, "Yes: to unblock over $6 billion in lost revenue."], ["A total ban on all GPUs", false, "The opposite: a way to keep selling."], ["Free GPUs for schools", false, "Not this deal."]], 10));
    addInteraction(p, bg2, 700, { OpenQuestion: { prompt: "Should a government take a cut of sales it otherwise bans? Why or why not?" } });
    sceneOf(p, bg2).chapters = [chap(0, "H20 revenue share with the US"), chap(269, "Calculating banned GPUs"), chap(479, "Our informants"), chap(615, "The smuggling pipeline")];
    back(bg2);

    // Part 1: Hong Kong
    addInteraction(p, hk1, 2, { PauseScreen: { text: "Part 1 is the longest trail: three scenes, about an hour. Use the chapters to skip around. Press Continue to start." } });
    sceneOf(p, hk1).chapters = [chap(0, "Hong Kong demand drivers"), chap(575, "How suppliers get the GPUs")];
    sceneOf(p, hk1).at_end = { Scene: hk2 };
    addInteraction(p, hk2, 1270, mc("Besides the military, which Chinese company's GPUs does this part cover?", [["Huawei", true, "Right: it's in the chapter title."], ["Xiaomi", false, "Not this one."], ["Lenovo", false, "Not this one."]], 10));
    sceneOf(p, hk2).chapters = [chap(0, "GPU rich and GPU poor"), chap(481, "Datacenter with banned GPUs"), chap(1081, "Chinese military and Huawei GPUs")];
    sceneOf(p, hk2).at_end = { Scene: hk3 };
    addInteraction(p, hk3, 1580, { OpenQuestion: { prompt: "What surprised you most about how the ban gets around, or about the Hong Kong GPU market?" } });
    sceneOf(p, hk3).chapters = [chap(0, "How China circumvents the ban"), chap(582, "GPU market in Hong Kong"), chap(1371, "Wiring money to China")];
    back(hk3);

    // Parts 2–3
    addInteraction(p, cn, 1190, mc("According to the chapter titles, which GPUs are unwanted by Shenzhen middlemen?", [["AMD and Intel GPUs", true, "Right."], ["NVIDIA GPUs", false, "Those are exactly what they want."], ["All GPUs", false, "Only some brands."]], 10));
    sceneOf(p, cn).chapters = [chap(0, "China smuggling process"), chap(417, "Shenzhen GPU middlemen"), chap(833, "AMD and Intel GPUs unwanted")];
    back(cn);

    // Parts 4–6
    addInteraction(p, fence, 3, { Info: { title: "Words to know", body: "A fence sells on goods that were obtained illegally; a fixer arranges deals and supplies for others." } }, 15);
    addInteraction(p, fence, 1820, mc("What does a 'fence' do in a black market?", [["Sells on illegally obtained goods", true, "Right."], ["Builds the GPUs", false, "That's the factory."], ["Writes the export rules", false, "That's the government."]], 10));
    sceneOf(p, fence).chapters = [chap(0, "The GPU fence"), chap(567, "Finding the GPUs"), chap(1118, "The fixer IC supplier"), chap(1478, "GPU warehouse")];
    sceneOf(p, fence).at_end = { Scene: mod };
    addInteraction(p, mod, 20, { Hotspot: { rect: [0.66, 0.05, 0.3, 0.2], label: "VRAM?", info: "Video memory on a graphics card. AI models need a lot of it, so mod shops add more.", jump: "Continue" } }, 30);
    addInteraction(p, mod, 1740, mc("What does the mod shop upgrade, according to the chapter title?", [["VRAM (video memory)", true, "Right: VRAM upgrades and repairs."], ["Only the fans", false, "More than that."], ["Power cables", false, "Not this one."]], 10));
    sceneOf(p, mod).chapters = [chap(0, "GPU mod shop for VRAM and repair"), chap(455, "Build a custom AI GPU")];
    back(mod);

    // Parts 7–9
    addInteraction(p, fac, 500, mc("Why could the film show the Yeston factory?", [["Yeston isn't involved in the black market", true, "Right, and the process is the same for all cards."], ["It's NVIDIA's own factory", false, "It makes cards for the Yeston brand."], ["It makes the banned GPUs", false, "It doesn't make the banned high-end GPUs."]], 10));
    sceneOf(p, fac).chapters = [chap(0, "Factory"), chap(268, "Taiwan and Singapore"), chap(333, "Smuggler")];
    back(fac);

    // Final part and endings
    addInteraction(p, legal, 3, { Info: { title: "The last part", body: "Who is breaking which rules, and who profits? At the end, your score picks your ending." } }, 12);
    addInteraction(p, legal, 1290, { OpenQuestion: { prompt: "Who do you think carries the most responsibility for this black market, and why?" } });
    addInteraction(p, legal, 1450, { ScoreCheck: { min_percent: 40, pass: { Scene: closed }, fail: { Scene: review } } });
    sceneOf(p, legal).chapters = [chap(0, "Legality of buying and selling"), chap(174, "Corruption: NVIDIA and governments"), chap(1300, "Signoff")];
    addInteraction(p, closed, 1, { Info: { title: "Case closed", body: "You followed the evidence and got most checks right. Bring your open answers to the discussion." } }, 12);
    addInteraction(p, closed, 3, { Link: { label: "Read the full timeline", url: timeline } }, 11);
    addInteraction(p, review, 1, { Info: { title: "Review the evidence", body: "A few checks went wrong. Follow another trail and try again." } }, 16);
    addInteraction(p, review, 17, { Choice: { prompt: "What now?", options: [{ text: "Back to the investigation map", jump: { Scene: hub } }, { text: "Finish", jump: "Finish" }] } });
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
