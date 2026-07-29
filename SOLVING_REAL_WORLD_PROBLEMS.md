# Solving Real-World Problems

Why does `docklab` exist, specifically? This isn't a "why containers are
good" pitch — it's a list of concrete, recurring problems people hit when
practicing IT operations and offensive security, what `docklab` already
does about each one, and what's still on the roadmap. Each problem is
tagged **Solved today** or **Roadmap** so this doc doesn't overstate what
the current `0.1.0` release actually does — see
[README.md](README.md#whats-implemented-today) for the same distinction
applied to the CLI as a whole.

## Problem 1: The VM tax on trying an exploit or a new tool

**Before:** You want to try a CVE PoC, or just want a throwaway Linux box
to install something questionable on. Standard workflow: download a VM
image (or build one), install VirtualBox/VMware, allocate several GB of
disk and RAM, boot it, snapshot it *before* you touch anything so you can
roll back, then finally run the thing you actually wanted to run.
Fifteen-plus minutes of setup before a single useful command.

**After — Solved today:**

```bash
lab create
```

One command. A disposable Kali Linux container, built and dropped into,
in the time it takes Docker to pull an image and start a container —
seconds, not minutes. See [TUTORIAL.md](TUTORIAL.md) for the full
walkthrough.

## Problem 2: "Did that script I just ran do something to my host?"

Running unfamiliar code — someone else's PoC, a script you haven't fully
read, an installer you don't quite trust — carries an obvious question:
what does it actually touch? VMs answer this at the cost of the setup tax
above. A plain `docker run -v .:/work` bind-mount answers it *incorrectly*,
because a bind-mount is a direct path back to your filesystem — exactly
the thing you were trying to avoid.

**docklab's answer — Solved today:** `lab create` never uses a bind-mount.
Look at `cmd_create` in `src/main.rs` — the `docker run` invocation has no
`-v` flag anywhere. Anything you clone, build, or drop onto disk inside
the container lives only in that container's writable layer. Run
`lab close` and every trace — cloned code, build artifacts, whatever it
wrote to `/tmp` — is gone with the container and image.

**Roadmap:** a dedicated `lab poc <github-url>` command that clones a PoC
repo straight into a fresh session for you doesn't exist yet — today you
`git clone` it yourself once you're inside the shell from `lab create`,
which is still bind-mount-free, just one extra manual step.

## Problem 3: "Wait, did that PoC just eat all my laptop's resources?"

Some PoCs are buggy or intentionally misbehave (a fork-bomb demo, a
container-escape attempt that spins). Without limits, a container can
still consume a large share of a host's CPU, memory, or process table.

**Status — Roadmap, not yet solved:** `lab create` currently runs the
container with Docker's defaults — no `--cpus`, `--memory`, or
`--pids-limit` flags are set (see `cmd_create` in `src/main.rs`). A
runaway process inside the container can still pressure your host. Adding
configurable resource limits (likely via a `~/.docklab/config.toml` or
CLI flags) is on the roadmap; until it lands, keep an eye on `lab mon`'s
Stats tab while running anything you don't fully trust, and consider
setting your own `docker update --cpus=... --memory=...` on the `docklab`
container in the meantime.

## Problem 4: Network exposure you didn't mean to have

A common mistake with ad-hoc Docker-based lab setups: the container ends
up on the default bridge network, which *can* reach your LAN and the
internet, and you don't notice until something calls home or scans a
neighbor's machine.

**Status — Roadmap, not yet solved:** `lab create` currently attaches the
`docklab` container to Docker's default bridge network — there is no
internal-only network created for it, and no `--allow-net`-style opt-in
flag. If you need network isolation today, create your own
`docker network create --internal` and reconnect the container to it
manually, or pass `--network none` yourself via a modified `docker run`.
Building this in as the default (an isolated network per session, with
outbound access as an explicit opt-in) is one of the most important items
on the roadmap — see [SECURITY.md](SECURITY.md) for how this gap is
currently scoped as a known limitation rather than a bug to report.

## Problem 5: Cleanup debt — "what is even still running from last month?"

Ad-hoc `docker run` experimentation accumulates: stopped containers,
orphaned networks, half-built images from abandoned attempts. The usual
fix, `docker system prune`, is a blunt instrument — it doesn't distinguish
your lab containers from anything else on that Docker daemon (a database
container for an unrelated project, say).

**docklab's answer — Solved today, for the one thing it manages:**
`docklab` currently manages exactly one named container (`docklab`) and
one image (`kalilinux/kali-rolling`), so there's nothing ambiguous to
clean up:

```bash
lab close    # stops + removes the docklab container, removes the image
```

**Roadmap:** this problem gets more interesting once `docklab` manages
*multiple* sessions (see Problem 7) — at that point, resource labeling
and a real "remove everything docklab created, and only what it created"
command become necessary. That doesn't exist yet because there's only one
resource to track today.

## Problem 6: Onboarding the next person on your team

Getting a new pentester or security-curious engineer set up to practice
exploit techniques usually means walking them through VM software
installation, image sourcing, network configuration, and *then* whatever
the actual lesson was about. Most of the setup time isn't spent learning
the technique.

**Status — partially solved.** `lab create` removes the VM/network-config
tax (Problem 1) for getting *a* terminal up. What it doesn't yet do is
hand someone a graduated, self-contained curriculum: there's no
`lab catalog` / `lab info <cve-id>` / `lab start <cve-id>` flow. Today,
onboarding someone means: `lab create` for the terminal, then walk them
through building and running one of the `labs/*/docker/Dockerfile`s by
hand (see [README.md](README.md#bundled-cve-lab-content)) as the target.
[TUTORIAL.md](TUTORIAL.md) documents exactly that manual path so it's
zero-context-required today, even without the catalog command existing
yet.

## Problem 7: "Where did this vulnerable binary even come from?"

Downloading a pre-built vulnerable image or binary from a random registry
or forum post to practice against is itself a security risk — you're
trusting an unverified third party to hand you a binary and asserting it's
*only* vulnerable in the way you expect.

**docklab's answer — Solved today, for the bundled labs:** every lab
under `labs/` is buildable from source you can read in this repo. None of
the three starter labs pull a historical, possibly-dead upstream artifact
— they reproduce the *exact observable vulnerable behavior* in small,
auditable code you can read end to end in a couple of minutes:
`labs/CVE-2011-2523/docker/vsftpd_2342_backdoor.py` is a small Python
script standing in for the real backdoored binary,
`labs/CVE-2014-6271/docker/vulnerable_shell.c` is a minimal C interpreter
with the real Shellshock parsing bug, and
`labs/CVE-2021-44228/docker/` builds a small Java service against the
genuinely vulnerable `log4j-core 2.14.1`. `docker build` runs each
Dockerfile straight from these files — nothing you run came from an opaque
pre-built layer nobody in this repo can point you to.

## The throughline

Every "Solved today" item above comes down to the same trade: **make the
safe, disposable option also the fast option**, so people don't reach for
a bind-mount or a shared VM out of impatience. Every "Roadmap" item is
the same trade applied to a guarantee `docklab` doesn't provide yet — the
project is explicit about that gap rather than implying it's already
covered. See [README.md](README.md#whats-implemented-today) for the
current state of the CLI, and [CONTRIBUTING.md](CONTRIBUTING.md) if you
want to help close one of these gaps.
