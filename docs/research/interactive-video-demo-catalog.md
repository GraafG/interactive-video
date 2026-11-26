# Interactive video demo catalog

Access date for every observation below: **26 November 2026**.

Scope: all 32 providers in the [hihaho platform directory](https://hihaho.com/blog/interactive-video-software-tools-platforms/). This is a demo-discovery attachment for the project plan, not a completed technology-stack review. Lumi is included only as an optional H5P authoring route.

## How to use this catalog

Start with H5P, Engajer, Edpuzzle, hihaho, Wootag, VidVersity and Vimeo for public viewers actually loaded during this investigation. Several other vendors provide well-documented individual embeds, but those are explicitly marked unverified. Do not treat every link below as a working, fully tested demo.

Verification labels:

| Label | Meaning |
| --- | --- |
| **interacted and observed behavior** | An actual viewer action or timed interaction produced an observable result. The description states exactly what was observed. |
| **loaded interactive viewer** | Browser loaded the player and its interaction UI/configuration; no complete end-to-end interaction test is claimed. |
| **embed not verified** | Named example and/or actual embed identified from a source, but its live interactive behavior was not established. An HTTP response alone is insufficient. |
| **login-gated** | Explicit account requirement established by the source or viewer. |
| **broken** | The tested URL failed in this investigation; this is not a conclusion about the company. |
| **archival** | Historical source/example, not evidence of a current functioning product. Can be combined with broken. |

Unless stated otherwise, "public" means the page or viewer could be opened without creating an account; it does not guarantee every activity or saved result is available anonymously. Interaction patterns are vendor-described unless the evidence column explicitly says they were observed.

**Mute requirement:** After the user's request, accessible HTML5 video/audio elements were set to `muted=true`, volume zero, and normally paused. Deliberately started playback was muted. The browser canvas did not expose tab-wide audio muting; cross-origin players whose mute state could not be established were not deliberately started. A Stornaway playback check preceded the mute request; that player was subsequently muted and paused. This catalog does not claim that external links will open muted on another device.

No accounts were created, forms submitted, products purchased, executables downloaded, or repositories modified. Transient media/session tokens are deliberately omitted. Collections are source indexes, not several demos; separate entries sharing a collection URL are distinguished only when distinct embedded players were observed.

## 1. Blue Billywig

Official sources: [interactive video](https://www.bluebillywig.com/interactive-video/) and [example collection](https://www.bluebillywig.com/knowledge-hub/interactive-video-examples/).

| Individual example | Pattern and access | Verification |
| --- | --- | --- |
| [Swedish Red Cross](https://www.bluebillywig.com/customers/swedish-red-cross/) | First-aid scenarios and multiple-choice decisions. Public official case page explicitly invites viewers to chapter 3 or the full Swedish experience. | **embed not verified**; case/demo invitation read, embedded experience not played. |
| [Nespresso](https://www.bluebillywig.com/customers/nespresso/) | Coffee-machine troubleshooting and navigation. Dutch starting experience; official case describes English Aguila 220/440 examples. | **embed not verified**; actual Blue Billywig script identified: `https://bluebillywig.bbvms.com/p/blue_billywig_cases/p/2881.js`. [HTML player wrapper](https://bluebillywig.bbvms.com/p/blue_billywig_cases/p/2881.html) also responds. |
| [Luma Delikatessen](https://www.bluebillywig.com/customers/luma-delikatessen/) | German shoppable carousel with product selection/checkout links. Public case page includes an in-action invitation. | **embed not verified**; no checkout action taken. |

These are three separate official customer examples, not three copies of the collection.

## 2. Brightcove, formerly HapYak

Official [example collection](https://studio.support.brightcove.com/interactivity/examples/index.html). The separate [legacy HapYak demo documentation](https://support.brightcove.com/shared/s-brightcove-interactivity/platform/demos/) should not be confused with current viewer verification.

| Individual example | Pattern and access | Verification |
| --- | --- | --- |
| [Under Armour](https://studio.support.brightcove.com/interactivity/examples/under-armour.html) | Add-to-cart interaction around 00:03; public support example. | **embed not verified**; actual [Brightcove player with interactivity project ID](https://players.brightcove.net/1752604059001/UivhMH9oq_default/index.html?videoId=1742506883763014353&interactivityProjectId=646f8dcd54410b001c0417ea) extracted from the page. |
| [AIG](https://studio.support.brightcove.com/interactivity/examples/aig.html) | Embedded form around 00:10. | **embed not verified**; page read, form not submitted. |
| [Tableau](https://studio.support.brightcove.com/interactivity/examples/tableau.html) | Chapter navigation around 00:02. | **embed not verified**; public named example, viewer behavior not tested. |

## 3. Cinema8

Official [showcase collection](https://cinema8.com/showcase), hydrated and read in the browser.

| Individual example | Pattern and access | Verification |
| --- | --- | --- |
| [Interactive video guide to making coffee](https://cinema8.com/showcase/how-to-make-coffee) / player title **Make Your Own Coffee** ([direct viewer](https://cinema8.com/video/zO1LdBJK)) | Coffee selection and guided ingredients/steps. Public. | **loaded interactive viewer**; direct viewer loaded media, duration 924.84 seconds, explicitly muted and paused. Choices were not exercised. |
| [Interactive video for ecommerce brands](https://cinema8.com/showcase/zara-shoppable-video) ([direct viewer](https://cinema8.com/video/mJddVWJZ)) | Clothing hotspots/product overlays. | **embed not verified**; exact iframe extracted. Explicitly a Cinema8-imagined Zara proof of concept, **not** evidence of a Zara customer deployment. |
| [Discover Venice with 360° Interactive Video](https://cinema8.com/showcase/discover-venice-with-360-interactive-video) ([direct viewer](https://cinema8.com/video/6JM8ZgXZ)) | 360 navigation, maps, information boxes and hotel videos. | **embed not verified**; exact iframe extracted, live behavior not exercised. |

The three direct viewer shells respond; only coffee received a browser load check.

## 4. ClickView

Official [interactive-video collection](https://www.clickvieweducation.com/en-au/product/interactive-videos) explicitly labels these resources interactive.

| Individual example | Pattern and access | Verification |
| --- | --- | --- |
| **Miniclips: Grammar - Nouns** ([Nouns](https://www.clickview.net/au/primary/videos/8416974/nouns)) | Educational video with embedded assessment; public metadata. | **embed not verified**; named page and app shell read. |
| **Miniclips: Grammar - Verbs** ([Verbs](https://www.clickview.net/au/primary/videos/8417845/verbs)) | Grammar learning/assessment. | **embed not verified**; named page and app shell read. |
| **Miniclips: Algo's Coding Adventures - What Is Binary?** ([resource](https://www.clickview.net/au/primary/videos/26619390/what-is-binary)) | Coding lesson with interactive assessment. | **embed not verified**; linked by the official interactive collection. |

The first two pages expose an unauthenticated application state. That alone does **not** establish whether an anonymous playable preview is available or login-gated; viewer access remains unresolved.

## 5. DilogR

**No public live demo found in this investigation.**

Attempted [original vendor URL](https://dilogr.com/) redirects to [manyclicks.com](https://manyclicks.com/), with a `/lander` redirect. The coordinator's earlier app-browser observation identified a parked domain; the current HTTP read corroborated the redirect. **broken** as an interactive-demo destination. No inference of company shutdown or permanent loss of all historical examples is warranted.

## 6. Dot.vu

Official [interactive-video collection](https://dot.vu/marketplace/collection/interactive-video), browser loaded. The following are distinct embedded examples, not three counts of the collection.

| Individual example | Pattern and access | Verification |
| --- | --- | --- |
| [Branching Video](https://dot.vu/marketplace/collection/interactive-video/template/branching-video) ([stable embedded viewer](https://live.dot.vu/p/_249/_59c8bc892b5c0f884fac40bf/)) | Branching decision journey. Public preview modal. | **embed not verified**; modal and exact iframe observed; cross-origin player not started because muting was not established. |
| **Beautiful Kitchenware / Designs from Recycled Materials** ([embedded viewer](https://dot.vu/p/_249/_6227282954fc8052883f39d2)) | Company/product presentation and information hotspots; opening text includes "START THE INTERACTIVE VIDEO". | **loaded interactive viewer**; same-origin embedded media loaded, muted and paused; hotspots not exercised. |
| [Video Quiz Contest](https://dot.vu/marketplace/collection/interactive-video/template/video-quiz-contest) ([stable viewer](https://live.dot.vu/p/_249/_59536c5d2b5c0ff47e8b45bf/)) | Watch a promotional video, answer a question, then enter a contest. | **embed not verified**; exact modal title, description and iframe observed. Do not submit personal data or enter the contest. |

The **Gated Interactive Company Presentation** modal was also observed, but is not counted as an ungated public demo. Stable iframe URLs above omit temporary `_sst` tokens.

## 7. Edpuzzle

Source: official [Edpuzzle Originals lesson article](https://blog.edpuzzle.com/free-resources/back-to-school-lessons-edpuzzle-originals/).

| Individual example | Pattern and access | Verification |
| --- | --- | --- |
| [Biology Lab Safety](https://edpuzzle.com/media/6723eaeb1d35447e49b45a40) | Public anonymous preview, 05:41. Multiple-choice events at 01:41, 02:23, 02:47, 03:13 and 04:08; open-ended at 04:44 and 05:26. | **loaded interactive viewer**; browser explicitly says "You were shared a preview link. Answers and progress won't be saved." Muted/paused; no answers submitted. |
| [Place Value](https://edpuzzle.com/media/65411901e7876e4024ec0823) | Public anonymous preview, 03:15. Multiple-choice at 02:31, 02:43 and 02:54; open-ended at 03:04. | **loaded interactive viewer**; same no-save notice observed. Muted/paused. |
| [Chemistry Lab Safety](https://edpuzzle.com/media/6723ebb61b7162c9aaaf3a9d) | Official named science lesson. | **embed not verified**; found through the official lesson source; viewer not browser checked. |

The visible Log in button does not prevent the observed previews. Account requirements for copying lessons are separate from anonymous viewing.

## 8. Engajer

Official [showcase](https://engajer.com/showcase/); direct viewers use `player.engajer.com`.

| Individual example | Pattern and access | Verification |
| --- | --- | --- |
| [Alive & Free](https://player.engajer.com/alive-and-free-review-campaign) | Topic navigation including INTRO, ABOUT ALIVE & FREE, programs and interviews. Public. | **interacted and observed behavior**; with video muted, selecting ABOUT ALIVE & FREE changed the actual video source from the intro to a different topic. Player was then paused. |
| [Engajer's engajer](https://player.engajer.com/engajer-engajer-for-the-website) | Vendor's own conversational/topic-led presentation. | **embed not verified**; exact official-homepage link found through search. |
| [Premier Tennis / 2014 Necker Cup](https://player.engajer.com/premier-tennis) | Topic-led tennis-event interviews, including founders/host/players. | **embed not verified**; direct player shell read and indexed topics found. |

Do not confuse these with the unrelated VideoEngager product.

## 9. Exaltive

**No public live demo found in this investigation.**

Attempted [exaltive.com](https://exaltive.com/) and [indexed production login](https://prod.exaltive.com/Login.aspx) failed transport access; earlier coordinator browser reads timed out. **broken** for the tested destinations. The indexed Login.aspx is not a verified accessible login gate and does not establish current functionality or shutdown.

## 10. GuidingTube

**No verified public live player found.** The official [customer cases](https://guidingtube.com/nl/services/klantcases/) are substantive, but descriptions and screenshots are not playable demos.

| Lead, not counted as a verified demo | Evidence and gap |
| --- | --- |
| **Eerste hulp bij online bedreigingen**, PersVeilig | GuidingTube attributes an interactive advice video to this customer. [PersVeilig's interactive training page](https://persveilig.nl/artikelen/interactieve-training) is indexed as a 25-minute training, but current fetch returns 403. **embed not verified**; the exact training's relationship to the earlier case and its viewer remain unconfirmed. |
| **Talent in Huis** | Official case says an interactive recruitment video was on the [customer homepage](https://www.talentinhuis.nl/). Current homepage read yielded ordinary site content, not an identified interactive player. **embed not verified**. |
| **Laurens avatar coach** | Official case describes employee distribution through intranet/care systems and mobile channels. No public anonymous example URL identified; not counted. |

## 11. H5P, with optional Lumi authoring

Official [Interactive Video examples](https://h5p.org/interactive-video): one carousel containing two distinct projects.

| Individual example | Pattern and access | Verification |
| --- | --- | --- |
| **Smoothie** ([first example](https://h5p.org/interactive-video), example ID 617) | Timed information overlays and quiz interactions. Public, no viewing account required. | **interacted and observed behavior**; media was muted before playback, a timed "Ingredients list" interaction appeared around nine seconds, and clicking it changed the interaction UI. No completed quiz is claimed. |
| **Video Based Game**, iframe title **Animal sound game** ([second example](https://h5p.org/interactive-video#example=27611)) | Game-like branching/adaptive interactive video; timeline 06:58. | **loaded interactive viewer**; second carousel viewer loaded. Playback not deliberately started because nested media mute control was not established. |

Optional authoring route: [Lumi](https://lumi.education/en/) offers desktop/cloud H5P authoring and describes HTML/SCORM export. This is **not** a 33rd provider entry or a separate interaction format. No executable was downloaded.

## 12. hihaho

Official [examples collection](https://hihaho.com/examples/).

| Individual example | Pattern and access | Verification |
| --- | --- | --- |
| [Interactive Training Scenario](https://hihaho.com/showcase/interactive-training-scenario/) | Trainify firefighter emergency choices, pause screens and hover elements. Public showcase. | **embed not verified**; official example description read, viewer not exercised. |
| [Interactive Orchestra video](https://hihaho.com/showcase/interactive-orchestra-video/) ([direct viewer](https://player.hihaho.com/embed/969377a0-8e25-4d17-8548-64592e609b0b)) | Rotterdams Philharmonisch Orkest instrument/member menu and contents navigation; Dutch. | **loaded interactive viewer**; browser loaded actual media and numerous labeled navigation choices, including concert/home choices. Muted/paused; no branch choice claimed. |
| [Cooking Tutorial](https://hihaho.com/showcase/cooking-tutorial/) | Preference branching, recipes, ingredients and an email-list feature. Public showcase. | **embed not verified**; no form submission. |

## 13. Ivory

Official [case collection](https://ivoryvideo.com/cases/). Actual project embeds were extracted from the case HTML, rather than treating case text as a functioning viewer.

| Individual example | Pattern and access | Verification |
| --- | --- | --- |
| [Big Green Egg Sessions](https://ivoryvideo.com/cases/big-green-egg-sessions/) ([direct project](https://projects.ivorystudio.net/embed/projects/5c7c36e41df6e31b150cd709)) | Cooking/product session exploration; public embedded case. | **embed not verified**; exact embed identified, player not browser tested. |
| [Holiday Planner](https://ivoryvideo.com/cases/holiday-planner/) ([direct project](https://projects.ivorystudio.net/embed/projects/0d372b01c3ca3452a491d737?iv_dnt=1)) | Traveo holiday-planning choices. | **embed not verified**; direct browser page title "Traveo_v7 - Video" loaded, but no media or interaction controls were exposed to inspection. This is insufficient to claim a functioning interactive viewer. |
| [Resuscitation: You Decide!](https://ivoryvideo.com/cases/resuscitation/) ([direct project](https://projects.ivorystudio.net/embed/projects/ps7GGIZfiOgY)) | Decision-based resuscitation scenario. | **embed not verified**; exact project iframe identified. |

## 14. Klynt

Official [demo collection](https://www.klynt.net/demo/) links separate HTML5 projects.

| Individual project | Pattern and access | Verification |
| --- | --- | --- |
| [Klynt New Demo: "Hot Town"](https://project.klynt.net/v3/demos/DEMO_NYC_MAP/) | Map/annotated New York journey; public project. | **embed not verified**; actual HTML5 player shell read. |
| [Discover Klynt 3 New Demos](https://project.klynt.net/v3/demos/DEMO_MENUS_KLYNT/) | Vertical/grid/horizontal/slider menu examples. | **embed not verified**; one project with several menu variants, **not four separate demos**. Media playback untested. |
| [Discover Klynt New Demo: "Wine Pairing"](https://project.klynt.net/v3/demos/infographics_wine/) | Wine/food interactive infographic. | **loaded interactive viewer**, but **related non-video example, not counted as a video demo**: browser showed labeled wine categories and share/mute/fullscreen controls, with no HTML5 video element found. |

Do not claim Klynt "died with Flash": the inspected project shells load the HTML5 Klynt player. Earlier coordinator evidence also found 3.5.007 installers on its live installation page. A later [quick-install](https://www.klynt.net/quick-install/) retry timed out; that timeout does not overturn the earlier evidence. No installers were downloaded.

## 15. Kumullus

**No anonymous public individual demos verified.**

The official [Show Kase instructions](https://help.kumullus.com/en/article/how-to-access-show-kase-the-kumullus-example-modules-platform-15xzwab/) require registration, then adding the example organization in K.Air. [Show Kase destination](https://air.kumullus.com/#/login/live/show%20kase/E_f8dK9Y0): **login-gated collection**, not two or three individual demos. Source says French examples and an English playlist are available. No account created.

## 16. Luma1

**No public playable individual demo found.**

The current [vendor homepage](https://www.luma1.com/) focuses on worker readiness, inductions and compliance; [demo page](https://www.luma1.com/demo) offers a personalized 20-minute booking, not an anonymous live viewer. A [LearningZen product article](https://www.learningzen.com/elearning/luma1-transforming-your-videos-into-engaging-tutorials/) describes interactive tutorials but does not establish a public playable sample. Do not substitute Luma AI or Blue Billywig's Luma Delikatessen.

## 17. Mindstamp

Official [example collection](https://mindstamp.com/examples) provides separate named example pages and actual embed URLs.

| Individual example | Pattern and access | Verification |
| --- | --- | --- |
| [Electric Vehicle Discovery](https://mindstamp.com/examples/electric-vehicle-discovery) ([direct viewer](https://embed.mindstamp.com/e/phjVtLoMpmbW)); player title **Showcase: How Electric Are You - PL4** | Lexus lifestyle questions, score/conditional hybrid, plug-in or electric outcome. | **loaded interactive viewer**; browser showed a Play control and media duration about 207 seconds. Muted/paused. A Play click did not establish advancing playback, so no interaction outcome is claimed. |
| [Video quiz](https://mindstamp.com/examples/video-quiz) ([direct viewer](https://embed.mindstamp.com/embed/MDcyKxFEZwFZ)) | In-video questions and graded quiz/certificate features. | **embed not verified**; exact `VideoObject.embedUrl` read. |
| [Choose your own journey](https://mindstamp.com/examples/choose-your-own-journey) ([direct viewer](https://embed.mindstamp.com/e/uZUZtyiWJgBL)) | Branching story with conditional buttons. | **embed not verified**; exact embedded viewer identified. |

Named pages are publicly accessible; anonymous completion/certificate requirements remain untested.

## 18. Nearpod

**No named public playable individual video verified.**

Official [interactive video page](https://nearpod.com/interactive-video) describes embedded assessment. The [library page](https://nearpod.com/nearpod-library) has a real **Preview a video** link to [filtered library search](https://nearpod.com/t/search?t=2&s=4&f=5), not a sales booking form. This destination returns a JavaScript application shell: **embed not verified** as a collection/access route, not an individual demo. Anonymous preview availability was not established; do not label it login-gated solely because it is an app shell.

## 19. PlayFilm

**No current public live demo verified.**

Search indexes the official **Demo Telco / EJEMPLO TELCO** at [support.playfilm.tv/demo4](https://support.playfilm.tv/demo4), describing click-to-answer interactions and links to training, health and energy examples. The current URL returns 404: **archival / broken**. [support.playfilm.tv/demo](https://support.playfilm.tv/demo) also returns 404.

The [current UGC page](https://playfilm.tv/ugc/) is not an interactive viewer. Do not invent demo1/demo2/demo3 URL mappings from indexed example categories.

## 20. PlayPosit, now WeVideo

**No exact public interactive bulb verified in this investigation.**

[go.playposit.com](https://go.playposit.com/) explicitly states PlayPosit is now WeVideo. Current [WeVideo interactivity](https://www.wevideo.com/interactivity) describes in-video questions and includes a product introduction, but no named anonymous live bulb was established. The [product tour](https://www.wevideo.com/interactivity/playposit-product-tour) redirects to [WeVideo product tour](https://www.wevideo.com/interactivity/wevideo-product-tour); the inspected content was feature descriptions/screenshots, not a confirmed interactive lesson.

[Sharing and Copying Bulbs](https://knowledge.playposit.com/article/222-sharing-and-copying-bulbs) documents preview links and public privacy settings; [University of Arizona's premade-bulb guidance](https://help.intech.arizona.edu/article/474-locate-copy-edit-premade-bulbs) establishes a resource-discovery route inside instructor accounts, not a specific public demo.

The [WeVideo interactive-example article](https://www.wevideo.com/blog/interactive-video-examples) includes **other vendors'** experiences. Its Verse, Mindstamp, Storyline and SBS examples must not be attributed to PlayPosit.

## 21. Rapt Media, Kaltura successor

Historical official [customer showcase mirror](https://dzkafjmecfpv4.cloudfront.net/customers/) remains readable and maps these exact names to Rapt.

| Historical individual example | Pattern | Verification |
| --- | --- | --- |
| [Will You Fit into Deloitte](https://cdn1.raptmedia.com/projects/j2Nu6adZ/embed?playsinline=true&autoplay=false&controls=overlay) | Recruiting choices/values scenarios. | **archival / broken**; direct player hostname failed resolution. |
| [The Undo Button](https://cdn1.raptmedia.com/projects/KZPZEf93/embed?playsinline=true&autoplay=false&controls=overlay), CAEL | Decision-led educational narrative. | **archival / broken**; same hostname failure. |
| [Galaxy S6 Camera Demo](https://cdn1.raptmedia.com/projects/uPk12V8q/embed?playsinline=true&autoplay=false&controls=overlay), Samsung | Product feature exploration. | **archival / broken**; same hostname failure. |

These are not counted as current live demos. Kaltura acquired Rapt in 2018. Successor [Interactive Video Example: Recruiting for Deloitte](https://corp.kaltura.com/resources/videos/interactive-video-example-recruiting-for-deloitte/) has a readable official description: **embed not verified**, not proof that the old Rapt player works. Another successor lead, [Aardman Studios - Dead Lonely](https://videos.kaltura.com/media/Aardman+Studios+-+Dead+Lonely/1_988lthlw/8955351), requires distinguishing a functioning interactive version from a conventional recording.

## 22. Smartzer

**No public live interactive player verified.**

The [current case-study collection](https://casestudies.smartzer.com/) hydrates in the browser. The individual [Swarovski Shoppable Campaigns](https://casestudies.smartzer.com/case-study/swarovski) page was inspected: substantive case text, but no iframe or direct live-demo link found. [Priceline](https://casestudies.smartzer.com/case-study/priceline), [MINI](https://casestudies.smartzer.com/case-study/mini) and [Montblanc](https://casestudies.smartzer.com/case-study/montblanc) are additional case leads, not three verified demos.

The [homepage](https://www.smartzer.com/) and [documentation](https://docs.smartzer.com/) establish a current product and describe actual players. They do not supply a confirmed anonymously playable example in this investigation. Do not substitute demo bookings or case-study recordings.

## 23. Spott

Official [Spott Gallery](https://spott.ai/spott-gallery/) actually loaded four **different video embeds**. The page did not expose individual video titles, so the identifiers below are evidence locators, not invented titles. This is stronger than merely finding a collection, but weaker than obtaining named, shareable standalone demonstrations.

| Distinct embedded individual, title unavailable | Pattern and access | Verification |
| --- | --- | --- |
| Gallery video `37c8e750-d437-4ed9-bf7c-fdfc98b1a259`, duration 16 seconds | Spott interactive video player; public in collection. | **loaded interactive viewer**; `data-video-id` and real HTML5 media observed. Muted/paused; hotspot behavior not tested. |
| Gallery video `1c65b485-c11f-4680-a2a6-8787a2eccc32`, duration 59 seconds | Player includes visible **get early access** overlay. | **loaded interactive viewer**; exact ID, media and overlay observed. Muted/paused; no lead form submitted. |
| Gallery video `64360417-eef8-4f60-ab3c-ce70c642fd77`, duration 11.633333 seconds | Spott interactive video embed; public in collection. | **loaded interactive viewer**; exact ID and media observed. Muted/paused; no interaction outcome claimed. |

A fourth embed, `f909c25c-bb51-435c-8dcd-52f96cd16979`, duration 15 seconds, explicitly had `data-autoplay=true`, `data-muted=true`, looping and tracked-tags-on-pause configuration. It was also explicitly muted and paused.

The official [Top 10 interactive video examples](https://spott.ai/insights/top-10-interactive-video-examples/) names **Spott x JBC**, **Polestar x Spott**, and **GCN x Spott**, but those names could not reliably be mapped to the observed gallery IDs. A subsequent direct raw-page request returned 403. Do not attribute unrelated Cinema8/Eko/Smartzer/Netflix examples from that mixed article to Spott.

## 24. Stornaway

Official [examples collection](https://www.stornaway.io/examples/).

| Individual example | Pattern and access | Verification |
| --- | --- | --- |
| [Stornaway Homepage Demo](https://player.stornaway.io/watch/cee237d8) | Vendor feature/use-case/pricing paths. Public direct player. | **loaded interactive viewer**; actual media playback established before the mute request. Subsequently all inspected media elements were muted and paused. No branch choice outcome is claimed. |
| [Interactive Video Demo](https://player.stornaway.io/watch/c8b979b1) | Vendor's named interactive demo. | **embed not verified**; indexed exact player title/link, not browser played. |
| **Think it Through**, Discovery Education/CVS ([educator experience](https://www.doseofknowledge.com/educators/interactive/)) | Educational decision scenarios. | **embed not verified**; linked as a customer experience by the official collection. |

Additional separate customer lead: Diageo/We Are Tilt [Wrong Side of the Road](https://drinkdriving.drinkiq.com/), decision-based drink-driving education; **embed not verified**. Console films and YouTube trailers were not counted as browser demos.

## 25. ThingLink

Official [example collection](https://www.thinglink.com/explore/examples) and [demos](https://www.thinglink.com/demos). Important: much of this library is interactive images, 360 images or mixed-media courses, not video.

| Candidate | Pattern and access | Verification |
| --- | --- | --- |
| **Mindful Materials - Public Education 360 video resource**, actual video-page title **Turf** ([direct video](https://www.thinglink.com/video/1500157031665893378)) | 360 video with tagged information; public metadata credits Lisa Jackson. | **embed not verified**; actual video page identified and title read, 360/tag behavior untested. |
| **Hach multimeter device training scenario with step-by-step user guide and knowledge test** ([scenario](https://www.thinglink.com/view/scenario/2015087388581167972), redirects to [course card](https://www.thinglink.com/coursecard/2015087388581167972)) | Mixed video/image/3D training and assessment. | **embed not verified**; application shell only. Mixed-media candidate, not proven video-first. |

Related but **not counted as video**: [Pae Ringa Toi - Tā moko](https://www.thinglink.com/card/1773862200193057254), described in the collection as a traditional Maori tattooing learning resource; inspected starting scene was an image with tags. Two or three pure live video demonstrations were not established.

## 26. Verse

Official [watch collection](https://www.verse.com/watch/) exposes actual individual story/player links.

| Individual example | Pattern and access | Verification |
| --- | --- | --- |
| [Exploring Creativity and Imagination: Ten Minute Challenge](https://www.verse.com/stories/57409-exploring-creativity-and-imagination-ten-minute-challenge/?autoplay=0), British Council | Decision-led teaching scenarios; also documented in WeVideo's example article. | **embed not verified**; actual player shell and exact metadata read, no browser interaction. |
| [DAY ZERO](https://www.verse.com/stories/2086-day-zero/?autoplay=0), Bloomberg Media Studios / Optum Health collection context | Chapter/hotspot narrative. | **embed not verified**; individual player and metadata read. |
| [Voters Make The Call](https://www.verse.com/stories/6451-voters-make-the-call/?autoplay=0), McClatchy Video Lab | Explore voicemail stories from 69 voters. | **embed not verified**; public story/player shell read. Historical 2016 editorial content, not evidence that the current product is discontinued. |

No observed login gate. `autoplay=0` is exposed in provider embed metadata and retained for safe viewing; it does not establish mute state.

## 27. Viddler

**No current public interactive-video demo found.**

The [live homepage](https://viddler.com/) currently says **Viddler | Simple Video Hosting**, offering uploads, unlisted links, embed codes and view/watch-time analytics. No interactive authoring or viewer demo was established. API upload examples found in search are not viewer-facing interactive examples. This is a current-evidence gap, not a shutdown claim.

## 28. VidVersity

Official [homepage](https://vidversity.com/) links one genuine sample.

| Individual example | Pattern and access | Verification |
| --- | --- | --- |
| [The right to disconnect website sample](https://benchmark.vidversity.com/share/683503d7e7a89/vidversity/the-right-to-disconnect-website-sample) | Chapter navigation, transcript and progress. Browser showed **Why does it matter?** 00:57, **What is the right to disconnect?** 07:02, **Case studies** 02:34. | **loaded interactive viewer**; public course hydrated, first chapter media duration 57.023999 seconds, explicitly muted and paused. No quiz/certificate submission. |
| **Understanding the Victorian Curriculum F-10 Version 2.0 - English module, primary curriculum leader** ([indexed course](https://benchmark.vidversity.com/share/69c60f9d68aa0/victorian-curriculum-and-assessment-authority/understanding-the-victorian-curriculum-f-10-version-20-english-module-primary-curriculum-leader)) | Course/assessment application shell; Victorian Curriculum and Assessment Authority path. | **embed not verified**; search and direct response identify the route, but exact rendered course title and viewer access were not confirmed. Name above is normalized from the indexed URL, **not an observed viewer title**. |

Only the first is a verified named live viewer. No third qualifying public example found; instruction recordings do not fill that gap.

## 29. Viewbix

**No public live demo found.**

[viewbix.com](https://www.viewbix.com/) and [corp.viewbix.com](https://corp.viewbix.com/) failed hostname resolution; earlier coordinator browser reads timed out. **broken** tested destinations. Historical directory descriptions of interactive calls to action do not establish a current working demo or company shutdown.

## 30. Vimeo, formerly Wirewax

Current context: [Vimeo interactive video](https://vimeo.com/features/interactive-video). These examples were identified from the [B2W example article](https://www.b2w.tv/blog/best-interactive-video-examples/), then the **actual Vimeo player configuration** was checked. Unlike an ordinary recording merely titled "interactive", all three returned non-null `interactive` fragments/markers and an `interactive_id`.

| Individual example | Pattern and access | Verification |
| --- | --- | --- |
| [Vimeo - Interactive Product Walkthrough](https://player.vimeo.com/video/823111497?autoplay=0&muted=1) | Product exploration with eight configured fragments and markers. Public embed. | **loaded interactive viewer**; browser loaded media, duration about 413.667 seconds, and muted state. Configuration exposes `interactive_id=8282566`. No hotspot/branch outcome tested. |
| [Hortology Quiz // WIREWAX Interactive Video](https://player.vimeo.com/video/701698166?h=7011f1d9b0&autoplay=0&muted=1) | Interactive plant-selection quiz. Public embed response. | **embed not verified**; exact title and non-null interactive configuration, `interactive_id=8263700`, observed in player response. Browser quiz behavior not tested. |
| [Dyson Live Demo](https://player.vimeo.com/video/639523249?autoplay=0&muted=1) | Interactive product demonstration. Public embed response. | **embed not verified**; exact title and non-null interactive configuration, `interactive_id=8237038`, observed. Full hotspot behavior not tested. |

The shared B2W article also embeds ordinary recordings; those were not automatically counted. [Non-stop dogwear](https://vimeo.com/customers/nonstop-dogwear) is a descriptive customer case, not an additional verified live viewer. Query parameters request no autoplay and muted playback; the browser mute state was independently checked for the walkthrough.

## 31. Vudoo

Official [customer gallery](https://admin.vudoo.com/customer-gallery/) and [page 2](https://admin.vudoo.com/customer-gallery/?pageNumber=2) provide actual video iframes, distinct from the gallery's interactive **display-ad** preview cards.

| Individual video, exact player title | Pattern and access | Verification |
| --- | --- | --- |
| [Travel Runway](https://flight-centre.vudoo.io/embed/70194114973), gallery **Flight Centre | Interactive Video** | Interactive travel video. | **embed not verified**; exact gallery iframe extracted; original `vudoo.io/embed/70194114973` redirects to the Flight Centre viewer and title. |
| [Bottlemart Wine/Champagne Finder](https://pernod-ricard.vudoo.io/watch/22849952978), gallery **Pernod Ricard | Interactive Video** | Product-selection journey; page invites finding a perfect product pick. | **embed not verified**; direct viewer and exact heading read, choices not exercised. |
| [Vogue x Dyson](https://newscorp-shoppable.vudoo.io/embed/15505032735), gallery **Dyson | Interactive Video** | Interactive/shoppable product video. | **embed not verified**; exact gallery iframe and player title read. |

Public player shells respond; browser playback/muting and commerce actions were not exercised. Kmart/Priceline/Kia ad-center previews were found but **not counted as video demos**, since they may be interactive display ads. No purchase or lead form submission.

## 32. Wootag

Official [explore Wootagged videos](https://www.wootag.com/explore-wootagged-videos/) includes two separate actual player URLs. Original modal links request autoplay; catalog URLs change that to `autoplay=0`.

| Individual video, exact player title | Pattern and access | Verification |
| --- | --- | --- |
| [Dimples Meaty Spaghetti](https://www.wootag.com/videos/embed/v2/483627-DCE8-5B9B41C89E-0070-4C1E?autoplay=0), gallery **Unilever / Mindshare Philippines** | Recipe/shop tags and cooking-level selection. Public, duration 15.18585 seconds. | **interacted and observed behavior**; video was muted before Play, playback reached its end, and replay/share plus labeled recipe/buy/selection overlays appeared. No outbound commerce click or form submission. |
| [邦民 #3 貸款All Clear 篇](https://www.wootag.com/videos/share/v2/838107-A3B0-44AE691415-8E8F-D5B9?autoplay=0), gallery **Promise / Xaxis Hongkong** | Loan-promotion/application tags in Chinese. Public, duration 26.052993 seconds. | **loaded interactive viewer**; real media and labeled application tags observed, explicitly muted and paused. Application links were not clicked and no financial/personal data entered. |

Only two distinct public examples verified; no third is invented from repeated gallery cards. The separate [Wootag Gallery](https://www.wootag.com/wootag-gallery/) is a JavaScript lead, with indexed email-gate text; no email submitted and it is not counted as another demo.

## Planning implications and unresolved gaps

Use the verification labels in the plan rather than flattening these results into "all demos work". The catalog covers every provider, but some have fewer than two or three qualifying experiences.

- **No verified current public interactive video:** DilogR, Exaltive, GuidingTube, Kumullus, Luma1, Nearpod, PlayFilm, PlayPosit/WeVideo, Rapt's original player, Smartzer, Viddler and Viewbix.
- **Limited or qualified examples:** ClickView access unresolved; Ivory embeds not demonstrated functioning; Klynt's Wine Pairing is non-video; ThingLink mixes video and image/course formats; Spott's loaded videos lack exposed titles; VidVersity has only one verified named viewer.
- **Strong next checks among identified embeds:** Blue Billywig's three official customer experiences, Brightcove's examples, remaining Cinema8/H5P/hihaho/Mindstamp viewers, Verse's stories, Vudoo's three video embeds, and Vimeo's configured interactive examples.

These gaps do not imply defunct companies. Retain the broken/archive/access notes, distinguish historical ownership from present capabilities, and keep all future browser viewing muted. The coordinator owns integration into the discovery plan and the separate technology-stack-review plan.
