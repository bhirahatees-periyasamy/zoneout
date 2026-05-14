# zoneout

Block AI coding assistants on macOS for a set duration. When the timer ends, everything is automatically unblocked.

Blocks: `claude.ai`, `anthropic.com`, `chatgpt.com`, `openai.com`, `copilot.microsoft.com`

---

## Requirements

- macOS
- Rust (via [rustup](https://rustup.rs))

---

## Installation

```bash
# 1. Clone the repo
git clone <your-repo-url>
cd anti-cheating

# 2. Build
cargo build --release

# 3. Install the binary system-wide
sudo cp target/release/zoneout /usr/local/bin/zoneout
```

---

## Usage

All `enable` and `disable` commands require `sudo` because they modify `/etc/hosts`.

```bash
# Enable for 1 hour using HH:MM:SS format
sudo zoneout enable --time 01:00:00

# Enable using flags
sudo zoneout enable -h 1          # 1 hour
sudo zoneout enable -m 30         # 30 minutes
sudo zoneout enable -h 1 -m 30    # 1 hour 30 minutes

# Disable immediately (also kills the background timer)
sudo zoneout disable

# Watch a live countdown (does NOT end the session — Ctrl+C to quit view)
zoneout status
```

---

## How it works

1. **Enable** — appends a marked block to `/etc/hosts` that redirects the blocked domains to `127.0.0.1`, then flushes the DNS cache.
2. **Background timer** — a detached daemon process runs silently in the background and removes the block when the time is up. Closing your terminal does not cancel it.
3. **Notifications** — macOS notification banners fire on enable, every minute as a reminder, and when the session ends.
4. **Disable** — removes the hosts block, kills the daemon, and clears the session state.

---

## Status view

`zoneout status` shows a live ticking clock in your terminal:

```
Focus active — ends at 16:30:00
Press Ctrl+C to exit status view.

  00h 42m 17s remaining
```

The countdown updates every second. Pressing Ctrl+C exits the view only — the session and blocking continue running.

---

## Uninstall

```bash
# Remove the binary
sudo rm /usr/local/bin/zoneout

# Remove session state (if any)
rm -rf ~/.focus
```

If you uninstall while a session is active, manually clean up `/etc/hosts` by removing the lines between `# BEGIN FOCUS BLOCK` and `# END FOCUS BLOCK`.
