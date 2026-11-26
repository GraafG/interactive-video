# Interactive video discovery plan

## Goal

Understand the available platforms, interaction patterns and technical approaches, then choose a practical starting point. Keep the first pass broad: do not assume that education, marketing or branching entertainment is the intended use case.

This is a research plan, not approval to purchase software, create accounts or implement a platform.

## Starting evidence

The [hihaho platform directory](https://hihaho.com/blog/interactive-video-software-tools-platforms/) contains 32 entries. Its displayed publication date is February 20, 2024, but its comparison metrics are from May 2023. It explicitly acknowledges vendor bias. Use it as a discovery index, not a current ranking; employee counts, backlinks and website traffic are not evidence of product suitability.

Three primary-source spot checks establish useful starting tracks:

| Track | Starting source | What the source describes |
| --- | --- | --- |
| Learning and reusable content | [H5P Interactive Video](https://h5p.org/interactive-video) | In-video questions, adaptive jumps, bookmarks and publishing-system integrations. Investigate self-hosted H5P separately from the hosted H5P.com service. |
| Branching experiences | [Stornaway](https://www.stornaway.io/) | A story-map editor for personalized routes through multimedia. |
| Business overlays and measurement | [Mindstamp](https://mindstamp.com/) | Questions, hotspots, forms, branching and viewer-level reporting. |

These are vendor-described capabilities, not hands-on findings or an endorsement. Sources accessed November 26, 2026.

## 1. Map the landscape

Extract every directory entry and follow its vendor link. Give each product one or more provisional categories:

- Learning: quizzes, feedback, assessment and LMS reporting.
- Branching stories: scene choices, conditions, state and multiple endings.
- Marketing and commerce: hotspots, calls to action, forms and product links.
- Enterprise video: hosting, collaboration, integrations and managed production.
- Developer-controlled: open-source authoring, self-hosting, portable content and custom players.

The seed inventory is: Blue Billywig; Brightcove (formerly Hapyak); Cinema8; ClickView; DilogR; Dot.vu; EDpuzzle; Engajer; Exaltive; GuidingTube; H5P; Hihaho; Ivory; Klynt; Kumullus; Luma1; Mindstamp; Nearpod; PlayFilm; Playposit; Rapt Media; Smartzer; Spott; Stornaway; ThingLink; Verse; Viddler; VidVersity; Viewbix; Vimeo (formerly Wirewax); Vudoo; Wootag.

For each entry verify whether it is active, renamed, acquired, discontinued or uncertain. Do not treat an unavailable website as proof of discontinuation. Preserve the original name and record any successor with evidence.

**Output:** a dated landscape table covering all 32 entries, including explicit unknowns.

## 2. Verify current offerings and fill gaps

Use official product pages, documentation, pricing and live examples first. Use independent reviews for usability signals, not as proof of features. Search beyond the article for current competitors and open-source alternatives; add only candidates that offer a distinct capability or deployment model.

Record the following consistently:

| Area | Evidence to collect |
| --- | --- |
| Authoring | Timeline versus node graph, templates, conditions, variables, collaboration and localization |
| Viewer experience | Hotspots, quizzes, branching, captions, keyboard access, mobile and fullscreen behavior |
| Publishing | Hosting requirements, embedding, supported video sources and export portability |
| Integrations | APIs, SDKs, webhooks; SCORM, xAPI and LTI where relevant |
| Analytics | Clicks, choices, completion, answers, event export and consent behavior |
| Ownership and privacy | Content ownership, data location, retention, deletion and vendor lock-in |
| Cost | Currency, billing period, included usage, viewer/storage limits, feature tiers and overages |

Label each finding **documented**, **demonstrated**, **hands-on tested** or **unknown**. Attach a URL and access date. Distinguish marketing claims from verified behavior.

**Output:** an evidence-backed comparison, rather than a feature-count ranking.

### Live demo catalog

For each of the 32 providers, find two or three distinct public viewer-facing examples where available. Prefer examples that cover different interaction patterns, such as a branching story, a quiz and a shoppable hotspot.

Record the actual demo title, exact URL, provider association, interaction pattern, access date and verification level. A page embedding an interactive viewer counts; a demo-booking form, screenshot or promotional trailer does not. Label a showcase collection as a collection, not as several individual demos.

Use the interactive browser when direct fetching cannot inspect JavaScript-rendered players. Where possible, play the video and make a harmless choice to confirm the interaction rather than merely checking that the page loads. Do not submit viewer forms or purchase anything.

All demo videos must remain muted. Establish player or browser/tab muting before playback, including embedded players and autoplay on navigation. If muting cannot be verified, skip playback and record the limitation.

Use explicit labels: **interaction observed**, **viewer loaded**, **embed unverified**, **login-gated**, **broken** or **archival**. If two public demos cannot be found, record the smaller number or the gap and the attempted sources. Link successor demos separately for acquired products; do not imply they demonstrate the former product.

**Output:** a provider-by-provider live demo appendix with honest availability and verification gaps.

## 3. Choose a representative shortlist

Begin with H5P plus Lumi, Stornaway, and Mindstamp, hihaho or Ivory to cover three different approaches. Also consider VidVersity/Kumullus for workplace learning, ClickView/Nearpod for classrooms, Verse for narratives, Smartzer/Spott/Vudoo/Wootag for commerce, and Blue Billywig/Brightcove for enterprise hosting. These are comparison tracks, not a ranking.

Reduce to three candidates only after identifying the intended audience, main interaction, publishing environment, budget and need for self-hosting. Apply must-have requirements before scoring convenience or polish.

**Output:** three candidates with reasons for inclusion, exclusions and unresolved questions.

## 4. Review the technical stack

Review every provider at documentation level first, then perform deeper public-player inspection on the shortlist. Include the relationship between the authoring tool, exported experience and viewer runtime: they may use different stacks.

Do not infer a vendor's private backend from its marketing website. Record **vendor-documented**, **browser-observed**, **public-source verified**, **inferred** or **unknown** beside every technical claim, with the source and access date. A React marketing page or CDN hostname does not prove the video editor's framework or server architecture.

| Layer | Review questions and evidence |
| --- | --- |
| Authoring | Browser or desktop editor? Timeline or graph? How are branches, conditions, variables, versions and translations represented? Inspect official documentation and public formats. |
| Viewer runtime | HTML5 player, iframe or web component? What player library or runtime is identifiable from public pages? Inspect the actual demo embed, public scripts and visible behavior rather than the marketing shell. |
| Video delivery | MP4, HLS or DASH? Who hosts and transcodes? How do captions, adaptive quality, caching, preloading and transitions between clips work? Examine public demo requests without collecting credentials or personal data. |
| Interaction model | Timed overlays versus scene branching; graph/state representation, answer feedback, seeking, replay and resume behavior. Distinguish navigation within one clip from switching clips. |
| Content portability | Export format, downloadable project data, self-hosted playback, offline use and separation of media from interaction definitions. Check licenses and whether an export preserves interactivity rather than only a flattened video. |
| Integration | Public APIs/SDKs, embed events, webhooks, identity requirements, LMS integrations and SCORM/xAPI/LTI support. Check relevant versions and plan-tier restrictions. |
| Measurement and privacy | Documented events and export, cookies/third-party requests, consent dependencies, retention and data location. Anonymous demo behavior does not establish enterprise privacy guarantees. |
| Operational fit | Browser/device support, keyboard/caption behavior, mobile fullscreen, load/branch latency, failure behavior, documented limits, maintainability and support. Test identical conditions on shortlisted players. |

For open-source options, inspect public repositories, licenses, release activity, extension mechanisms and deployment requirements. For proprietary SaaS, leave undocumented backend and database choices unknown and prioritize integration contracts, exportability and observed runtime behavior.

Compare three starting architectures without prematurely selecting one:

1. Hosted editor and hosted player: fastest setup, with vendor-dependent hosting and exports.
2. Portable/self-hosted content: evaluate H5P/Lumi and relevant public player/export options; confirm reporting and hosting requirements separately.
3. Custom authoring/player: consider only after identifying a concrete requirement existing tools cannot satisfy. Then assess a graph-based content model, HTML5 delivery, interaction-state management and event reporting before choosing frameworks or infrastructure.

**Outputs:** an evidence-labeled technical matrix, a compact authoring-to-playback architecture sketch for each shortlisted approach, and a build-versus-buy recommendation with portability and operating-cost tradeoffs.

**Boundary:** this is a technical review plan, not authorization to deploy, create accounts, execute downloaded installers, access private systems or copy proprietary source code.

## 5. Plan a comparable hands-on test

Use the same small set of original or licensed clips for every candidate. The test experience should contain:

1. A choice leading to two different paths.
2. A timed hotspot and a question with feedback.
3. A return point or replay option.
4. Captions and a mobile-friendly embedded player.
5. A record of choices, answers and completion, where supported.

For each tool measure authoring time, supported versus missing interactions, mobile/keyboard usability, playback transitions, analytics usefulness and content/data export. Distinguish a jump within one video from branching between clips. Mark unsupported features rather than silently changing the test.

Account creation, paid trials and use of non-public footage require a separate decision before testing.

**Output:** a side-by-side test report with limitations and evidence.

## 6. Recommend how to get started

Produce a short recommendation answering:

- Which approach fits the selected use case, and why?
- What is the simplest first prototype?
- What would it cost at a stated usage level?
- What technical or portability risks remain?
- Should the starting point be hosted software, self-hosted authoring or a custom player?

Recommend custom development only when tested existing options fail important requirements, not simply because a repository is available.

**Finish condition:** the landscape is accounted for, public demo coverage and gaps are recorded for all 32 entries, three approaches are compared technically and practically, and the next prototype has a clear scope and justified tool choice. Unresolved vendor questions remain visible.

## Suggested effort

Budget approximately one to two working weeks for the expanded scope: landscape and demo discovery, offering verification and shortlisting, then technical review, comparative trials and recommendation. This is an estimate, not a scheduled commitment; gated demos and trial access may extend it.

**Current baseline:** the initial investigation and demo appendix account for all 32 directory entries, with explicit uncertainty for inaccessible or changed offerings. Next, verify the remaining embeds and use comparable experiences to guide the technical comparison before choosing a winner.

## Appendix: public demo discovery

Access date: **November 26, 2026**. All 32 providers were researched. Two or three distinct examples are listed where found; unavailable examples and unverified players are not presented as working live demos.

Verification: **O** = interaction or playback-related behavior observed; **L** = actual viewer loaded, without a completed interaction test; **U** = identified example/embed, live behavior unverified; **A** = archival/broken; **G** = explicit account gate. Read the detailed demo catalog for exact observations, access restrictions, player identifiers and sources.

After the mute instruction, deliberately started playback was muted. Cross-origin players without established mute control were not deliberately started. These external URLs do not guarantee muted playback when opened independently: mute the player or browser tab before playing, or skip playback if silence cannot be established.

### Providers with individual examples or identifiable embedded projects

| Provider | Individual examples and verification |
| --- | --- |
| Blue Billywig | [Swedish Red Cross](https://www.bluebillywig.com/customers/swedish-red-cross/) **U**; [Nespresso](https://www.bluebillywig.com/customers/nespresso/) **U**; [Luma Delikatessen](https://www.bluebillywig.com/customers/luma-delikatessen/) **U**. Official case pages invite interactive viewing; behavior not tested. |
| Brightcove / HapYak | [Under Armour](https://studio.support.brightcove.com/interactivity/examples/under-armour.html) **U**; [AIG](https://studio.support.brightcove.com/interactivity/examples/aig.html) **U**; [Tableau](https://studio.support.brightcove.com/interactivity/examples/tableau.html) **U**. Shopping, form and chapter examples respectively; no forms submitted. |
| Cinema8 | [Make Your Own Coffee](https://cinema8.com/video/zO1LdBJK) **L**; [Zara shoppable proof of concept](https://cinema8.com/video/mJddVWJZ) **U**; [Venice 360 experience](https://cinema8.com/video/6JM8ZgXZ) **U**. Zara example is vendor-imagined, not evidence of a customer deployment. |
| ClickView | [Nouns](https://www.clickview.net/au/primary/videos/8416974/nouns) **U**; [Verbs](https://www.clickview.net/au/primary/videos/8417845/verbs) **U**; [What Is Binary?](https://www.clickview.net/au/primary/videos/26619390/what-is-binary) **U**. Official interactive-learning resources; anonymous playback unresolved, not proven login-gated. |
| Dot.vu | [Branching Video](https://live.dot.vu/p/_249/_59c8bc892b5c0f884fac40bf/) **U**; [Beautiful Kitchenware / Designs from Recycled Materials](https://dot.vu/p/_249/_6227282954fc8052883f39d2) **L**; [Video Quiz Contest](https://live.dot.vu/p/_249/_59536c5d2b5c0ff47e8b45bf/) **U**. Do not enter the contest or submit personal data. |
| Edpuzzle | [Biology Lab Safety](https://edpuzzle.com/media/6723eaeb1d35447e49b45a40) **L**; [Place Value](https://edpuzzle.com/media/65411901e7876e4024ec0823) **L**; [Chemistry Lab Safety](https://edpuzzle.com/media/6723ebb61b7162c9aaaf3a9d) **U**. The first two are genuine anonymous previews; answers/progress are not saved. |
| Engajer | [Alive & Free](https://player.engajer.com/alive-and-free-review-campaign) **O**; [Engajer's own presentation](https://player.engajer.com/engajer-engajer-for-the-website) **U**; [Premier Tennis / 2014 Necker Cup](https://player.engajer.com/premier-tennis) **U**. Topic selection changed the video source in Alive & Free. |
| H5P | [Smoothie, first carousel example](https://h5p.org/interactive-video) **O**; [Video Based Game / Animal sound game](https://h5p.org/interactive-video#example=27611) **L**. These are two distinct projects on one collection page. Lumi is an optional authoring route, not another demo format. |
| hihaho | [Interactive Training Scenario](https://hihaho.com/showcase/interactive-training-scenario/) **U**; [Interactive Orchestra](https://player.hihaho.com/embed/969377a0-8e25-4d17-8548-64592e609b0b) **L**; [Cooking Tutorial](https://hihaho.com/showcase/cooking-tutorial/) **U**. Orchestra loaded labeled instrument/member navigation. |
| Ivory | [Big Green Egg Sessions](https://projects.ivorystudio.net/embed/projects/5c7c36e41df6e31b150cd709) **U**; [Holiday Planner](https://projects.ivorystudio.net/embed/projects/0d372b01c3ca3452a491d737?iv_dnt=1) **U**; [Resuscitation: You Decide!](https://projects.ivorystudio.net/embed/projects/ps7GGIZfiOgY) **U**. Exact official-case embeds identified; a working interactive viewer was not established. |
| Klynt | [Hot Town](https://project.klynt.net/v3/demos/DEMO_NYC_MAP/) **U**; [Klynt 3 menu demos](https://project.klynt.net/v3/demos/DEMO_MENUS_KLYNT/) **U**. Related [Wine Pairing infographic](https://project.klynt.net/v3/demos/infographics_wine/) **L** is excluded from the video count: no video element was found. Menu variants are one project, not four demos. |
| Mindstamp | [Electric Vehicle Discovery](https://embed.mindstamp.com/e/phjVtLoMpmbW) **L**; [Video Quiz](https://embed.mindstamp.com/embed/MDcyKxFEZwFZ) **U**; [Choose Your Own Journey](https://embed.mindstamp.com/e/uZUZtyiWJgBL) **U**. No completed quiz, certificate or branching outcome tested. |
| Spott | [Spott Gallery](https://spott.ai/spott-gallery/) contains distinct loaded video embeds **L**. Three identifiers: `37c8e750-d437-4ed9-bf7c-fdfc98b1a259`, `1c65b485-c11f-4680-a2a6-8787a2eccc32`, `64360417-eef8-4f60-ab3c-ce70c642fd77`. Titles/standalone links were not exposed; do not invent names or count the collection itself as three demos. |
| Stornaway | [Homepage Demo](https://player.stornaway.io/watch/cee237d8) **L**; [Interactive Video Demo](https://player.stornaway.io/watch/c8b979b1) **U**; [Think it Through](https://www.doseofknowledge.com/educators/interactive/) **U**. Additional customer lead: [Wrong Side of the Road](https://drinkdriving.drinkiq.com/) **U**. |
| ThingLink | [Turf, 360 video resource](https://www.thinglink.com/video/1500157031665893378) **U**; [Hach multimeter training scenario](https://www.thinglink.com/coursecard/2015087388581167972) **U**, mixed media rather than verified video-first. Two or three pure live video examples were not established. |
| Verse | [Exploring Creativity and Imagination: Ten Minute Challenge](https://www.verse.com/stories/57409-exploring-creativity-and-imagination-ten-minute-challenge/?autoplay=0) **U**; [DAY ZERO](https://www.verse.com/stories/2086-day-zero/?autoplay=0) **U**; [Voters Make The Call](https://www.verse.com/stories/6451-voters-make-the-call/?autoplay=0) **U**. Exact story/player metadata read; no browser interaction tested. |
| VidVersity | [The Right to Disconnect sample](https://benchmark.vidversity.com/share/683503d7e7a89/vidversity/the-right-to-disconnect-website-sample) **L**; [Victorian curriculum English module lead](https://benchmark.vidversity.com/share/69c60f9d68aa0/victorian-curriculum-and-assessment-authority/understanding-the-victorian-curriculum-f-10-version-20-english-module-primary-curriculum-leader) **U**, name derived from indexed URL, not an observed viewer title. Only the first is a verified named viewer. |
| Vimeo / Wirewax | [Interactive Product Walkthrough](https://player.vimeo.com/video/823111497?autoplay=0&muted=1) **L**; [Hortology Quiz](https://player.vimeo.com/video/701698166?h=7011f1d9b0&autoplay=0&muted=1) **U**; [Dyson Live Demo](https://player.vimeo.com/video/639523249?autoplay=0&muted=1) **U**. All three expose interactive configuration/IDs, distinguishing them from ordinary recordings; only the walkthrough was browser-loaded. |
| Vudoo | [Travel Runway](https://flight-centre.vudoo.io/embed/70194114973) **U**; [Bottlemart Wine/Champagne Finder](https://pernod-ricard.vudoo.io/watch/22849952978) **U**; [Vogue x Dyson](https://newscorp-shoppable.vudoo.io/embed/15505032735) **U**. Actual video embeds from the official gallery, not display-ad preview cards. |
| Wootag | [Dimples Meaty Spaghetti](https://www.wootag.com/videos/embed/v2/483627-DCE8-5B9B41C89E-0070-4C1E?autoplay=0) **O**; [Promise / Xaxis Hongkong example](https://www.wootag.com/videos/share/v2/838107-A3B0-44AE691415-8E8F-D5B9?autoplay=0) **L**. First playback ended with replay/tag overlays; second loaded application tags. No commerce/application links clicked. |

### Providers without a verified current public individual demo

| Provider | Findings and discovery leads, not substitutes for live demos |
| --- | --- |
| DilogR | [Original site](https://dilogr.com/) redirects to domain parking. No public demo or authoritative successor found. |
| Exaltive | [Main site](https://exaltive.com/) and [indexed production login](https://prod.exaltive.com/Login.aspx) failed access; browser reads timed out. The login URL is not a verified functioning gate. |
| GuidingTube | [Customer cases](https://guidingtube.com/nl/services/klantcases/) describe PersVeilig, Talent in Huis and Laurens examples. [PersVeilig training lead](https://persveilig.nl/artikelen/interactieve-training) returns 403; exact player/provider association unresolved. No public player identified on Talent in Huis; Laurens distribution appears private. |
| Kumullus | [Show Kase access instructions](https://help.kumullus.com/en/article/how-to-access-show-kase-the-kumullus-example-modules-platform-15xzwab/) establish an account-dependent example collection **G**, not individual public demos. No registration performed. |
| Luma1 | [Demo page](https://www.luma1.com/demo) is a personalized meeting booking, not a public viewer. No individual playable sample found. |
| Nearpod | [Preview-a-video library route](https://nearpod.com/t/search?t=2&s=4&f=5) is a genuine discovery lead **U**, but no named anonymous viewer confirmed. An application shell alone does not prove a login gate. |
| PlayFilm | [Demo Telco](https://support.playfilm.tv/demo4) and [legacy demo route](https://support.playfilm.tv/demo) return 404 **A**. Indexed historical categories do not justify guessing new demo URLs. |
| PlayPosit / WeVideo | [Current interactivity page](https://www.wevideo.com/interactivity) and [product tour](https://www.wevideo.com/interactivity/wevideo-product-tour) describe the product, but no public live bulb confirmed. [Preview-sharing documentation](https://knowledge.playposit.com/article/222-sharing-and-copying-bulbs) is a future discovery route; other vendors' examples in WeVideo articles are not PlayPosit demos. |
| Rapt Media / Kaltura | Historical [Rapt customer source](https://dzkafjmecfpv4.cloudfront.net/customers/) maps Deloitte, CAEL's The Undo Button and Samsung Galaxy S6 to original Rapt players, whose hostname fails **A**. Successor [Kaltura Deloitte example](https://corp.kaltura.com/resources/videos/interactive-video-example-recruiting-for-deloitte/) **U** and [Dead Lonely](https://videos.kaltura.com/media/Aardman+Studios+-+Dead+Lonely/1_988lthlw/8955351) **U** need interactive-versus-recording verification; they do not prove the old players work. |
| Smartzer | [Swarovski](https://casestudies.smartzer.com/case-study/swarovski), [Priceline](https://casestudies.smartzer.com/case-study/priceline), [MINI](https://casestudies.smartzer.com/case-study/mini) and [Montblanc](https://casestudies.smartzer.com/case-study/montblanc) are case leads, not verified live players. Browser inspection found no player/link on Swarovski. |
| Viddler | [Current site](https://viddler.com/) describes hosting and analytics; no public interactive demo established. Upload API examples do not qualify. |
| Viewbix | [Main domain](https://www.viewbix.com/) and [corporate domain](https://corp.viewbix.com/) fail resolution; browser reads timed out. No public live demo found. |

### Use the demos in the technical review

Begin runtime inspection with viewers actually loaded: H5P, Cinema8 coffee, Dot.vu kitchenware, Edpuzzle, Engajer Alive & Free, hihaho Orchestra, Mindstamp's electric-vehicle example, Spott's gallery embeds, Stornaway, VidVersity's sample, Vimeo's walkthrough and Wootag. Choose comparable interaction patterns before drawing conclusions.

For every shortlisted demo, record its exact viewer URL, whether it is an iframe or inline player, observed interaction model, public delivery format, event/configuration evidence, fullscreen/mobile behavior and export documentation. Keep public runtime observations separate from claims about the editor or backend. Detailed technical inspection is **planned, not completed**.

The remaining identified embeds are a verification backlog, not assumed working examples. Where access requires an account or provider contact, record that decision for later rather than creating accounts or sending inquiries automatically.

## Appendix: official feature and technical source pages

Access date: **November 26, 2026**. This directory accounts for all 32 vendors. Unless a note states otherwise, the linked sources were directly read; Edpuzzle's getting-started page and Kaltura's path page were browser-read. These are vendor capability statements, not independently tested features or proof of a working demo.

Dedicated product/features pages are preferred. A homepage is retained only when useful capability content was found there or no better current route was established. Successor products, general platform features and historical material are labeled separately. The detailed official-source directory preserves page titles, access evidence and compatibility notes.

| Provider | Official feature/product pages | Complementary technical sources or limitations |
| --- | --- | --- |
| Blue Billywig | [Interactive video](https://www.bluebillywig.com/interactive-video/) | [HTML5 video player](https://www.bluebillywig.com/html5-video-player/): branding, delivery and accessibility; native/OTT claims do not automatically apply to every interactive feature. |
| Brightcove / HapYak | [Interactivity](https://www.brightcove.com/products/interactivity) | [Getting started with video interactivity](https://studio.support.brightcove.com/interactivity/getting-started/getting-started-video-interactivity.html): distinguishes former-HapYak module behavior from native Studio Interactivity. |
| Cinema8 | [Interactive video](https://cinema8.com/interactive-video) | [SCORM and LTI](https://cinema8.com/scorm-lti); [Video analytics](https://cinema8.com/video-analytics). |
| ClickView | [Interactive videos](https://www.clickvieweducation.com/en-au/product/interactive-videos) | Classroom video quizzes and learning insights; viewing and authoring access need separate evaluation. |
| DilogR | **No current official feature page verified.** | [Original domain](https://dilogr.com/) redirects to domain parking; historical third-party lists are not current vendor specifications. |
| Dot.vu | [Interactive video](https://dot.vu/interactive-experiences/interactive-video); [Platform features](https://dot.vu/features) | [Integrations](https://dot.vu/integrations). Separate video features from general interactive-content editor capabilities. |
| Edpuzzle | [Getting started](https://edpuzzle.com/getting-started), browser-read | [LMS integration documentation](https://support.edpuzzle.com/hc/en-us/sections/360006699191-LMS-Integrations). A separate current feature-page URL was not established; homepage metadata alone is not a complete specification. |
| Engajer | [The Engajer Platform](https://engajer.com/platform/) | Creation tree, sharing, analytics, lead ranking, CRM and unlockable content documented together; [homepage](https://engajer.com/) is a complementary positioning source. |
| Exaltive | **No current official feature page verified.** | [Main domain](https://exaltive.com/) fails access; indexed login is not a feature specification. |
| GuidingTube | [Interactive video](https://guidingtube.com/services/selection-buttons/) | [Video management/analytics](https://guidingtube.com/services/videomanagement/); [Multichannel publishing](https://guidingtube.com/services/multichannel-2/). |
| H5P | [Interactive Video content type](https://h5p.org/interactive-video) | [Exact xAPI coverage](https://h5p.org/node/617/xapi-coverage). Optional [Lumi authoring/export route](https://lumi.education/en/) is distinct from H5P itself and hosted H5P.com. |
| hihaho | [Features](https://hihaho.com/features/); [Interaction types](https://hihaho.com/interactions/) | [SCORM package guide](https://support.hihaho.com/en/articles/5694732-exporting-your-video-as-a-scorm-package); [xAPI guide](https://support.hihaho.com/en/articles/5695482-xapi-tin-can-api-guide). |
| Ivory | [Feature directory](https://ivoryvideo.com/software/); [Editor](https://ivoryvideo.com/software/video-editing/) | [Player/publishing](https://ivoryvideo.com/software/video-player/); [Analytics](https://ivoryvideo.com/software/video-analytics/). |
| Klynt | [Features](https://www.klynt.net/features/) | [Self-hosting tutorial](https://www.klynt.net/course/publish/), historical guidance. HTML5 player, customization and publishing are documented; contemporary OS/plugin compatibility remains untested. |
| Kumullus | [Current product landing page](https://kumullus.com/) | Substantial authoring, branching, AI, multilingual and SCORM capability sections; no separate dedicated feature URL established. Demo-collection registration is a different access question. |
| Luma1 | [Software](https://www.luma1.com/software) | Dedicated workforce-readiness, delivery, assessment, certification and analytics source; [homepage](https://www.luma1.com/) gives current compliance positioning. |
| Mindstamp | [Features and capabilities](https://mindstamp.com/features) | [Reporting and integrations help](https://mindstamp.com/help/reporting): reports, CSV fields, identity, webhooks and setup routes. Native, custom and intermediary integrations must be distinguished. |
| Nearpod | [Interactive video](https://nearpod.com/interactive-video) | Embedded assessment, learning insights and own/library videos; do not import unrelated slides/gamification features into the video specification. |
| PlayFilm | [Current service homepage](https://playfilm.tv/) | Interactive-video/AI lead-to-sales offering. No current granular self-service editor specification verified. |
| PlayPosit / WeVideo | [Current WeVideo interactivity](https://www.wevideo.com/interactivity) | [Migration notice](https://go.playposit.com/) confirms successor identity. The old analytics route redirects to the same interactivity page, not a distinct additional source. |
| Rapt Media / Kaltura | **Successor:** [Kaltura Interactive Video Paths](https://corp.kaltura.com/developer-suite/interactive-video-paths/), browser-read | [Kaltura interactive video/editing tools](https://corp.kaltura.com/developer-suite/interactive-video-editing-tools/), broader product material with 2023 metadata. Original Rapt domain failed; successor features do not establish legacy feature parity. |
| Smartzer | [Current product landing page](https://www.smartzer.com/) | [Official documentation](https://docs.smartzer.com/); [SDK](https://docs.smartzer.com/sdk/): JavaScript library, web components, iframe rendering and player events. |
| Spott | [How Spott works](https://spott.ai/how-spott-works/) | [Interactive-content capabilities](https://spott.ai/how-spott-works/make-all-of-your-content-interactive/): video layers, product tracking, branching and player integrations. Separate video from images/PDFs and confirm current integration compatibility. |
| Stornaway | [Features](https://www.stornaway.io/features/) | [SCORM export](https://www.stornaway.io/scorm-export/); [xAPI export](https://www.stornaway.io/xapi-tin-can-export/). Completion/progress configuration and plan requirements need explicit checking. |
| ThingLink | [Creation platform](https://www.thinglink.com/creation) | [Learning/delivery platform](https://www.thinglink.com/learning). Mixed-media/spatial features are not all video-only. |
| Verse | [Homepage feature sections](https://www.verse.com/) | Story editor, hotspots, decision points, embedding, analytics and API positioning. No separate textual API/feature specification verified; a linked editor-guide video was not separately inspected. |
| Viddler | [Current hosting feature source](https://viddler.com/) | Uploads, embedding and viewing analytics only; **no current interactive-authoring feature page verified**. |
| VidVersity | [Current product overview](https://vidversity.com/) | [Interactive video for your LMS](https://vidversity.com/lms-power-up); [All-in-one training platform](https://vidversity.com/all-in-one-solution). Lightweight LMS packages and hosted portal are separate offerings. |
| Viewbix | **No current official feature page verified.** | [Main domain](https://www.viewbix.com/) and [corporate domain](https://corp.viewbix.com/) fail resolution. Third-party historical descriptions cannot fill the current specification gap. |
| Vimeo / Wirewax | [Current Vimeo interactive video](https://vimeo.com/features/interactive-video) | [Video analytics](https://vimeo.com/features/video-analytics), broader platform source. Treat these as current Vimeo capabilities, not unchanged legacy Wirewax specifications. |
| Vudoo | [Solutions](https://vudoo.com/solutions/) | Checkout/cart approaches, CTAs/forms, interactive storytelling and measurement; [homepage](https://www.vudoo.com/) covers distribution across media. Separate video from display/CTV and checkout from simple cart links. |
| Wootag | [Interactive video player/features](https://www.wootag.com/products/player/) | Interaction categories, editing, publishing and engagement/conversion analytics documented together. Broad social/player compatibility remains a claim pending tests. |

### Concrete questions for the technical review

- **SCORM does not automatically mean offline portability.** hihaho's guide requires an internet connection to hihaho.com; VidVersity describes hosted video with a lightweight LMS package. Test whether media and interactions remain usable without the vendor.
- **Learning events need exact semantics.** H5P documents constituent xAPI triggers; Stornaway requires author-set progress/completion and documents a separate pass/fail limitation in its xAPI guide. Compare events and LMS interpretation, not merely a yes/no integration checkbox.
- **Player and product variants matter.** Brightcove documents different support for former-HapYak module versus native Studio interactivity, including in-page/native-player limitations. Identify the exact proposed workflow before claiming support.
- **An integration logo is not an implementation contract.** Use Smartzer's SDK and Mindstamp's reporting/setup documentation to distinguish actual APIs/events, native connectors, intermediary services and custom work.

These source findings refine the planned review; no complete stack audit, integration test, feature scorecard or implementation has been performed.
