<div align="center">

# docklab

**Disposable, isolated terminals for practicing IT operations, CVE proof-of-concepts, and penetration testing — safely.**

Built on Docker, so nothing you do inside a session can permanently touch your host machine.

[![License: MIT](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE.md)
[![Rust](https://img.shields.io/badge/rust-2021%20edition-orange.svg)](Cargo.toml)
[![Platforms](https://img.shields.io/badge/platforms-Linux%20%7C%20macOS%20%7C%20Windows-lightgrey.svg)](INSTALLATION.md)

[Installation](INSTALLATION.md) ·
[Tutorial](TUTORIAL.md) ·
[Contributing](CONTRIBUTING.md) ·
[Security](SECURITY.md) ·
[Community](COMMUNITY.md)

</div>

---

## What is this?

`docklab` (binary name `lab`) replaces the heavyweight "spin up a VM to
safely run this exploit" workflow with a single command. The project is
organized around three ideas:

1. **An attacker terminal** — a containerized, disposable shell you drop
   into to practice IT operations, build and run CVE PoCs, or just get a
   throwaway Linux box, with nothing you do inside it able to reach your
   host filesystem.
2. **A library of vulnerable target labs** — real, per-CVE Dockerfiles you
   build and run to practice specific, historical vulnerabilities hands-on,
   instead of only reading about them.
3. **Isolation as the default, not a manual step** — no host bind-mounts,
   ever. Anything a session creates (installed tools, cloned PoCs, exploit
   artifacts) lives only in that container's writable layer and disappears
   the moment you tear the session down.

See [SOLVING_REAL_WORLD_PROBLEMS.md](SOLVING_REAL_WORLD_PROBLEMS.md) for
the motivation behind each of these in more depth.

### What's implemented today

`docklab` is early (`0.1.0`). Idea #1 is what the CLI actually does right
now — it wraps the `docker` command so you don't have to remember the
incantation for "give me a disposable Kali Linux shell":

- `lab create` pulls the public `kalilinux/kali-rolling` image and drops
  you straight into an interactive shell inside a container named
  `docklab`, with a handful of common pentest/IT tools (`curl`, `wget`,
  `nmap`, `nano`, `git`, `net-tools`, `lynis`) installed automatically on
  first run.
- `lab reconnect` re-attaches to that same container later — anything you
  installed or created inside it is still there.
- `lab mon` opens a live TUI dashboard: container status, resource stats,
  disk usage, network settings, and running processes.
- `lab close` tears it all down — stops the container, removes it, and
  removes the image — leaving nothing behind.

Idea #2 exists today as **content, not yet a CLI feature**: `labs/` ships
three real, hermetically-built CVE Dockerfiles (see
[Bundled CVE lab content](#bundled-cve-lab-content) below) that you build
and run by hand with `docker`. Wiring them into `lab catalog` and
`lab start <cve-id>` is the next milestone — see
[CONTRIBUTING.md](CONTRIBUTING.md#about-labs).

Idea #3 holds for what exists today — no bind-mounts, full teardown via
`lab close` — but doesn't yet include the deeper guarantees (per-session
network isolation, capability drops, resource limits) a mature version of
this tool should have. In short: `docklab` is a fast, honest way to get a
disposable shell today, and is working toward being a hardened lab
platform — it isn't one yet. See [SECURITY.md](SECURITY.md) for the exact
threat model this version provides.

## Quick start

```bash
lab create     # pulls kalilinux/kali-rolling, builds you a shell, drops you in
# ...do your work inside the container...
exit           # leaves the container stopped (not deleted)

lab reconnect  # jump back into the same container later
lab mon        # open the live TUI dashboard in another terminal
lab close      # stop + remove the container and image entirely
```

Full walkthrough: [TUTORIAL.md](TUTORIAL.md).

## Install

Three ways to get `lab` running, in order of speed:

```bash
# 1. Prebuilt binary, already in this repo at dist/ (fastest, no Rust needed)
chmod +x dist/lab-linux-x86_64
sudo cp dist/lab-linux-x86_64 /usr/local/bin/lab

# 2. Install from source via Cargo
git clone https://github.com/MrEchoFi/docklab.git
cd docklab
cargo install --path .     # installs `lab` to ~/.cargo/bin

# 3. Build it yourself
cargo build --release      # binary lands at target/release/lab
```

`dist/` ships binaries for Linux, Windows, and macOS (Intel + Apple
Silicon), plus a `SHA256SUMS` file to verify against — see
[INSTALLATION.md](INSTALLATION.md#verifying-a-download) for the checksum
command per OS.

You need Docker installed and reachable via the `docker` CLI (`docker ps`
should work without `sudo`) — `lab` shells out to `docker`, it does not
talk to the Docker daemon directly.

Full platform-by-platform instructions, including how each `dist/`
binary was built and how well-verified it is on its target OS:
**[INSTALLATION.md](INSTALLATION.md)**.

## Command overview

| Command | What it does |
|---|---|
| `lab create` | Pull `kalilinux/kali-rolling` and start it as an interactive container named `docklab`, installing a base toolset on first run |
| `lab close` | Stop and force-remove the `docklab` container, then remove the `kalilinux/kali-rolling` image |
| `lab mon` | Open a live TUI dashboard: overview, resource stats, disk usage, network settings, running processes |
| `lab reconnect` | Re-attach to the existing `docklab` container (`docker start -ai`) |

| Flag | What it does |
|---|---|
| `--version` | Print `lab <version>` |
| `-V`, `--verbose` | Print every underlying `docker` command before running it |
| `-h`, `--help` | Print the command/flag reference |
| `--guide` | Print a short walkthrough of the typical workflow |

Flags are recognized on any invocation (`lab --version`, `lab -h`, etc.)
and take priority over subcommands.

### `lab mon` keys

| Key | Action |
|---|---|
| `1`–`5` | Switch between Overview / Stats / Disk Usage / Network / Processes tabs |
| `r` | Force-refresh the active tab |
| `h` or `?` | Toggle the help overlay |
| `g` | Toggle the quick-start guide overlay |
| `q` or `Esc` | Quit (or close an open overlay) |

## Bundled CVE lab content

`labs/` in this repo contains reference material for three historical
CVEs — `CVE-2011-2523` (vsftpd 2.3.4 backdoor), `CVE-2014-6271`
(Shellshock), and `CVE-2021-44228` (Log4Shell) — each with a `metadata.yaml`
briefing/hints and a self-contained `docker/Dockerfile` that reproduces the
vulnerability. **These are not yet wired into the `lab` CLI**: there is no
`lab catalog` or `lab start <cve-id>` command today. To use one, build and
run it directly with `docker`, e.g.:

```bash
docker build -t docklab-vsftpd-backdoor labs/CVE-2011-2523/docker
docker run --rm -it -p 2121:21 -p 6200:6200 docklab-vsftpd-backdoor
```

then attack it from a `lab create` shell on the same Docker network, or
however you'd like to wire it up. Integrating these into the CLI directly
(`lab catalog`, `lab start <cve-id>`) is the most-wanted next feature — see
[CONTRIBUTING.md](CONTRIBUTING.md#about-labs).

## Documentation map

| Doc | For |
|---|---|
| [TUTORIAL.md](TUTORIAL.md) | First-time users — the `lab` workflow end to end |
| [INSTALLATION.md](INSTALLATION.md) | Linux / macOS / Windows setup from source |
| [SOLVING_REAL_WORLD_PROBLEMS.md](SOLVING_REAL_WORLD_PROBLEMS.md) | Why this exists, what it replaces |
| [CONTRIBUTING.md](CONTRIBUTING.md) | Dev setup, project layout, PR process |
| [SECURITY.md](SECURITY.md) | Reporting vulnerabilities, responsible use |
| [COMMUNITY.md](COMMUNITY.md) | Code of conduct, ethics, how to engage |

## License

MIT — see [LICENSE.md](LICENSE.md). The bundled CVE labs under `labs/`
intentionally contain historical vulnerabilities on purpose; see the
license file's scope note and [SECURITY.md](SECURITY.md) for how that's
handled.
