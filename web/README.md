# ClipQuest web: how it runs

**No backend needed.** This folder is a complete static web app: four files
(`index.html`, `style.css`, `examples.js`, `app.js`), no build step, no server code,
no database, no accounts.

## Putting it online

Upload this folder to any static host, and it works on phones and computers:

- GitHub Pages, Netlify, Cloudflare Pages, Vercel (static), an S3 or Azure storage
  bucket, or any ordinary web server (Apache, nginx, IIS).
- It needs **https** (or `http://localhost`). Opening `index.html` straight from
  disk works for everything except YouTube videos, because YouTube's player
  refuses pages that have no web address. On this computer, the desktop app's
  *File → Open the web version in your browser* serves it locally for that.

## Where things are kept

Everything stays **in the viewer's own browser** on their own device:

| What | Where |
| --- | --- |
| The lesson being edited | the browser's local storage |
| Results (answers, choices, scores) | the browser's local storage |
| Video files picked from the device | the browser's IndexedDB |
| YouTube and `.mp4` web addresses | streamed from where they live |

To move a lesson to another device, use *Menu → Save lesson as a file*, then
*Open a lesson file…* there. Results are exported from the Results tab as a table
(CSV) or as learning-record statements (JSON).

## What a backend would add (not needed to run it)

Only add a server if you need these:

- **Results from many viewers in one place.** Today each viewer's results stay
  on their device. A small endpoint (or a learning-record store, an LRS) that
  receives the exported statements would collect them centrally.
- **Sharing lessons by link** instead of by file, with editing rights.
- **Hosting your own uploaded videos.** Today video files picked from a device
  stay on that device; for others to see them, put them on a video host and paste
  the address.

## Outside services it uses

- YouTube's embedded player (`youtube.com/iframe_api`) and YouTube thumbnails
  (`i.ytimg.com`), only for lessons with YouTube videos. Sound starts off (browsers only
  start videos by themselves when muted); the speaker button and volume slider under the
  video turn it on, and the choice is remembered.
- Nothing else: no analytics, no fonts or scripts from other sites.
