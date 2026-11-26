# interactive-video

## ClipQuest: an interactive video tool

**Try it: https://graafg.github.io/interactive-video/**

Built from the research below: timed quiz questions, open questions, choices that
branch to other scenes, hotspots, text cards, pause screens, score checks, link
buttons, captions, chapters and a story map, plus a Results view with CSV and
learning-record exports. Examples include a branching "story map" demo built on a
YouTube video.

- [`web/`](web/): the web app. Static, with no backend, and works on phones. Deployed to
  GitHub Pages automatically on every push to `main`. See [web/README.md](web/README.md).
- [`desktop/`](desktop/): the same tool as a desktop program (Rust + egui).
  `cargo build` inside `desktop/`. Its *File → Open the web version* serves `web/`
  locally. Plays local video files through ffmpeg when it is installed. The
  `rust-toolchain.toml` pins the GNU toolchain used on the machine it was built on;
  remove it to build with your default toolchain.

### CI/CD

- **CI** (`.github/workflows/ci.yml`) runs on every pull request and push to `main`:
  the web app's code must parse, every example lesson must hold together
  (`node tests/check-examples.js`), and the desktop program must build.
- **CD** (`.github/workflows/pages.yml`) publishes `web/` to GitHub Pages when it
  changes on `main`, after the same web checks pass. Each release is stamped with a
  version (shown in the app's menu) so browsers load the new files straight away.

Lessons and results stay in each viewer's own browser; lessons move between devices
and between the two apps as `.json` files.

## Research

- [Discovery and technical-review plan](docs/research/interactive-video-discovery-plan.md)
- [Initial investigation of all 32 providers](docs/research/interactive-video-first-pass.md)
- [Demo catalog, observed behavior and gaps](docs/research/interactive-video-demo-catalog.md)
- [Official feature and documentation sources](docs/research/official-feature-source-directory.md)

Keep all browser videos muted. Before playback, verify player or browser/tab
muting; skip playback if silence cannot be established.
