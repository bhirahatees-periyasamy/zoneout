# zoneout

Block AI coding assistants (and any other distracting sites) on macOS for a set duration. When the timer ends, everything is automatically unblocked.

Default blocked domains: `claude.ai`, `anthropic.com`, `chatgpt.com`, `openai.com`, `copilot.microsoft.com`

---

## Requirements

- macOS
- Rust (via [rustup](https://rustup.rs))

---

## Installation

```bash
# 1. Clone the repo
git clone https://github.com/bhirahatees-periyasamy/anti-cheating.git
cd anti-cheating

# 2. Build
cargo build --release

# 3. Install the binary system-wide
sudo cp target/release/zoneout /usr/local/bin/zoneout
```

---

## Usage

> `enable` and `disable` require `sudo` because they modify `/etc/hosts`.  
> `add`, `remove`, `list`, and `status` do not require `sudo`.

### Start a session

```bash
sudo zoneout enable --time 01:00:00   # 1 hour (HH:MM:SS)
sudo zoneout enable -h 1              # 1 hour
sudo zoneout enable -m 30             # 30 minutes
sudo zoneout enable -h 1 -m 30        # 1 hour 30 minutes
```

### Block extra domains for one session only

```bash
sudo zoneout enable -h 1 -d youtube.com
sudo zoneout enable -m 30 -d youtube.com -d twitter.com
```

The `-d` domains are active for that session only and are not saved to your list.

### Stop a session

```bash
sudo zoneout disable
```

Immediately removes all blocks and kills the background timer.

### Watch the countdown

```bash
zoneout status
```

Shows a live ticking clock in your terminal:

```
Focus active — ends at 16:30:00
Press Ctrl+C to exit status view.

  00h 42m 17s remaining
```

Ctrl+C exits the view only — the session keeps running in the background.

---

## Managing your domain list

Custom domains are saved to `~/.zoneout/domains.json` and merged with the defaults every time you enable a session.

```bash
# Add a domain permanently
zoneout add youtube.com

# Remove a domain
zoneout remove youtube.com

# Remove all custom domains
zoneout remove --all

# Show all blocked domains (defaults + custom)
zoneout list
```

---

## How it works

1. **Enable** — appends a marked block to `/etc/hosts` redirecting all blocked domains to `127.0.0.1`, then flushes the DNS cache (`dscacheutil` + `mDNSResponder`).
2. **Background timer** — a detached daemon process runs silently and removes the block when the time is up. Closing your terminal does not cancel it.
3. **Notifications** — macOS notification banners fire on enable, every minute as a reminder, and when the session ends.
4. **Disable** — removes the hosts block, kills the daemon, and clears the session state.

---

## Uninstall

```bash
# Remove the binary
sudo rm /usr/local/bin/zoneout

# Remove session state and custom domain list
rm -rf ~/.focus ~/.zoneout
```

If you uninstall while a session is active, manually remove the lines between `# BEGIN FOCUS BLOCK` and `# END FOCUS BLOCK` in `/etc/hosts`.
