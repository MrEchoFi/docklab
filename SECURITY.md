# Security Policy

`docklab` is a security-testing convenience tool: it wraps `docker` to
give you a disposable Kali Linux terminal, and it bundles Dockerfiles for
historical, intentionally-vulnerable services (under `labs/`) to practice
against. That makes "what counts as a security bug here" different from a
typical project, so read the scope section below before reporting
anything — and read [README.md](README.md#whats-implemented-today) first
if you haven't, since several isolation properties described in earlier
drafts of this project's docs are **not implemented yet**. This document
reflects the actual `0.1.0` codebase, not a future version of it.

## What isolation `docklab` actually provides today

Be precise about this before reporting or relying on anything:

- **No host bind-mounts.** `lab create` runs `docker run -it --name
  docklab kalilinux/kali-rolling bash` with no `-v`/`--mount` flag — see
  `cmd_create` in `src/main.rs`. Nothing you do inside the container can
  read or write your host filesystem through a mount.
- **No dynamic, user-controlled arguments reach `docker`.** The CLI takes
  no flags for a custom image, container name, extra `docker run` flags,
  or a URL to fetch — every `docker` invocation in `src/main.rs` and
  `src/mon/data.rs` uses fixed, hardcoded arguments (plus the constant
  `docklab` container name and `kalilinux/kali-rolling` image name). There
  is currently no code path where attacker-controlled input (a malicious
  repo URL, a crafted filename) flows into a shell command.

## What isolation `docklab` does *not* provide yet

These are known, documented gaps — not vulnerabilities to report:

- **No network isolation.** `lab create` does not create or attach to an
  internal-only Docker network; the container runs on Docker's default
  bridge, which can reach your LAN and the internet like any other
  container you'd `docker run` by hand.
- **No capability drops or `--privileged` guarantee analysis.** The
  container runs with whatever Docker's own defaults are — `docklab`
  doesn't add `--cap-drop=ALL` or otherwise harden the container beyond
  "no bind-mounts."
- **No resource limits.** No CPU, memory, or pids limit is applied — a
  runaway process inside the container can still pressure your host.
- **Single container only.** There's no session/catalog model, so there's
  also no resource-labeling or "only touch what docklab created" logic to
  audit — `docklab` only ever touches the one container named `docklab`
  and the one image `kalilinux/kali-rolling`.

If you're looking for the plan to close these gaps, see
[SOLVING_REAL_WORLD_PROBLEMS.md](SOLVING_REAL_WORLD_PROBLEMS.md) and
[CONTRIBUTING.md](CONTRIBUTING.md).

## Scope

### In scope — please report these

Anything that breaks the guarantee `docklab` *does* claim today (no
bind-mounts, no attacker-controlled input reaching a shell command), or
any memory-safety/logic bug in the Rust CLI itself. Concretely:

- Any way for the `docklab` container to read from or write to the **host
  filesystem** without a bind-mount being added deliberately (e.g. via a
  Docker feature being misused, a symlink trick, or similar).
- Any way user input (env vars, a crafted `PATH`, arguments to `lab`)
  changes what command actually gets executed by `run()` in `src/main.rs`
  in an attacker-controlled way (command injection).
- A `docker` binary earlier in `PATH` being silently preferred over the
  real one in a way that isn't the normal, expected shell `PATH` lookup
  behavior any CLI tool is subject to (i.e. a `docklab`-specific bug, not
  "the user has a malicious `PATH`," which is a general shell hygiene
  issue outside this project's control).
- Panics, crashes, or undefined behavior in `src/main.rs` or `src/mon/`
  triggerable by normal `docker` output the TUI parses (e.g. `docker
  stats`, `docker inspect`, `docker top` output).
- Supply-chain issues in this repo's own dependencies (`Cargo.toml`/
  `Cargo.lock`) — a compromised crate, a dependency confusion risk, etc.

### Out of scope — expected, not a vulnerability

- **The absence of network isolation, capability drops, and resource
  limits** described above — these are roadmap items, not bugs. Feel free
  to open a feature request/discussion instead (see
  [COMMUNITY.md](COMMUNITY.md)).
- **The vulnerabilities inside the bundled lab containers themselves.**
  `labs/CVE-2011-2523/docker/vsftpd_2342_backdoor.py`, the Shellshock
  interpreter in `labs/CVE-2014-6271/docker/vulnerable_shell.c`, and the
  log4j-core 2.14.1 build in `labs/CVE-2021-44228/` are *supposed* to be
  exploitable exactly as their CVE describes. Reporting "the vsftpd lab
  has a backdoor" is expected behavior, not a finding.
- **`kalilinux/kali-rolling` running as `root` inside its own
  container.** This matches how Kali/pentest tooling is normally used; it
  isn't a `docklab`-specific hardening gap on top of what's already listed
  above.
- Resource exhaustion from running heavy tools yourself inside the
  container on purpose — that's the "no resource limits" gap above, not a
  new finding.

If you're unsure which bucket something falls in, report it anyway and
say you're unsure — see below.

## Reporting a vulnerability

**Do not open a public GitHub issue for an in-scope finding.** Email:

**tanjibisham888@gmail.com** — subject line `[docklab security] <short summary>`

Include:

1. What you did (the exact `lab` command(s) and/or `docker` command(s)
   run).
2. What you expected to happen (per the "What isolation `docklab`
   actually provides today" section above).
3. What actually happened, with concrete evidence — output, a file that
   appeared on the host, a `docker inspect` showing something unexpected,
   etc.
4. Your `lab --version` output and OS/Docker version.
5. Whether you're comfortable being credited by name/handle in a fix
   changelog (optional).

### What to expect

- **Acknowledgement:** within 5 days.
- **Triage:** confirmed in-scope findings get a severity assessment
  within 10 days.
- **Fix timeline:** no fixed SLA (this is a small, unfunded open-source
  project) but isolation-breaking bugs are treated as the highest priority
  class of issue in this repo, ahead of new features.
- **Disclosure:** please give a reasonable window (90 days is a sane
  default) before any public disclosure. We'll coordinate a disclosure
  date with you once a fix ships.

## Supported versions

`docklab` is pre-1.0 (currently `0.1.0`). Only the latest release on the
default branch receives security fixes — there is no long-term-support
branch at this stage.

| Version | Supported |
|---|---|
| `0.1.x` (latest) | ✅ |
| anything older | ❌ (upgrade first) |

## Responsible use of this tool

`docklab` exists so people can practice offensive security techniques
against real, historical CVEs, and get a disposable terminal quickly. That
only holds up if you use it responsibly, especially given the gaps above:

- Run PoCs and exploits **only** against systems you own or have explicit
  written authorization to test — never against systems you don't.
- Because there's **no network isolation yet**, don't assume a session is
  cut off from your LAN or the internet — it isn't. Don't run anything
  inside `lab create` that you wouldn't run in an ordinary `docker run`
  container on your machine today.
- Treat the bundled CVE labs as training material, not as a toolkit for
  attacking production systems still running these ancient, unpatched
  versions in the wild — that's a legal problem for you, not a docklab
  problem.

See [COMMUNITY.md](COMMUNITY.md) for the fuller ethics/conduct
expectations.
