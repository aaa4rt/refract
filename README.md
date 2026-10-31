<p align="center"><img src="app-icon.png" width="128" alt="Refract icon"></p>

<h1 align="center">Refract</h1>
<p align="center">A tiny desktop remote for Spotify with a liquid glass look.<br>
Windows · macOS · Linux — a few MB, no Electron.</p>

---

## Features

- Shows what's playing, with album art that tints the whole window
- Play / pause, skip, seek, shuffle, repeat, volume
- Search songs and play them instantly
- Switch which device plays (phone, laptop, speakers, TV…)
- **Liquid glass** theme, plus a plain dark theme (half-circle button)
- Keyboard: <kbd>Space</kbd> play/pause, <kbd>←</kbd>/<kbd>→</kbd> previous/next

Refract is a **remote**: Spotify itself plays the music, on any of your devices,
and Refract controls it. That's why it stays so small.

## Download

Grab the latest version from the [**Releases**](../../releases) page:

| Your computer | File |
|---|---|
| Windows | `Glass.Player_x.x.x_x64-setup.exe` |
| Mac with Apple chip (M1 or newer) | `Glass.Player_x.x.x_aarch64.dmg` |
| Intel Mac | `Glass.Player_x.x.x_x64.dmg` |
| Linux | `.AppImage` / `.deb` / `.rpm` |

**First launch warnings:** the app isn't code-signed, so your computer will be cautious:

- **Windows:** “Windows protected your PC” → click **More info → Run anyway**.
- **Mac:** if it says the app “can't be opened” or “is damaged”, open **System Settings → Privacy & Security** and click **Open Anyway**.
  If that button doesn't appear, run this once in Terminal:
  `xattr -cr "/Applications/Refract.app"`

## One-time Spotify setup (about 2 minutes)

Spotify requires every app to have its own “Client ID”. Because of Spotify's rules,
**each person makes their own** (free, no coding) instead of sharing one:

1. Go to **[developer.spotify.com/dashboard](https://developer.spotify.com/dashboard)** and log in.
   *(Spotify requires the account that creates the app to have **Premium**.)*
2. Click **Create app**. Use any name and description.
3. For **Redirect URI**, enter exactly:
   ```
   http://127.0.0.1:43821/
   ```
4. Under “Which API/SDKs are you planning to use?”, tick **Web API**. Save.
5. Copy the **Client ID**, paste it into Refract, and click **Log in with Spotify**.

That's it. You stay logged in after that.

## Build it yourself

You need [Node.js](https://nodejs.org) and [Rust](https://rustup.rs).
On Linux you also need the [Tauri system packages](https://v2.tauri.app/start/prerequisites/).

```bash
npm install
npm run dev      # run it
npm run build    # make an installer for your computer
```

The whole player is one file, `src/index.html`. Edit it and re-run.

You can also just open the player in a browser: run
`python3 -m http.server 43821 --bind 127.0.0.1` inside `src/` and visit `http://127.0.0.1:43821/`.

## Releasing a new version (for maintainers)

1. Change `version` in `src-tauri/tauri.conf.json` (and `package.json`).
2. Push a tag: `git tag v0.2.0 && git push origin v0.2.0`
3. GitHub Actions builds Windows, Mac and Linux installers and makes a **draft release**.
   Check it on the Releases page and click **Publish**.

If the action fails with “Resource not accessible by integration”, go to
**Settings → Actions → General → Workflow permissions** and choose **Read and write permissions**.

## How it works

- Built with [Tauri](https://tauri.app). It uses the computer's built-in web view instead of bundling a browser.
- Login uses Spotify's PKCE flow, so there's no client secret to leak.
  Your login stays on your computer.
- The app serves its page from `http://127.0.0.1:43821`, because Spotify only accepts
  loopback addresses for desktop login redirects.
- **“Play here”** (making the app itself the speaker) uses Spotify's Web Playback SDK. It needs full Premium and
  works in regular browsers, but may not work inside the desktop app. Remote control always works.

## Disclaimer

Not affiliated with or endorsed by Spotify. Spotify is a trademark of Spotify AB.

## License

MIT
