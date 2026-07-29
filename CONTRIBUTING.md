# Contributing to docklab

Thanks for considering a contribution. This doc covers dev setup, the
actual project layout, testing, and the PR process.

## Development setup

You need:

- **Rust** (2021 edition — install via [rustup](https://rustup.rs))
- **Docker**, running and reachable from your user (`docker ps` should work
  without `sudo`) — `lab` shells out to the `docker` CLI directly, so it's
  needed to exercise the commands you're changing

```bash
git clone https://github.com/MrEchoFi/docklab.git
cd docklab
cargo build
cargo test
```

`cargo build` gets you a debug binary at `target/debug/lab` to iterate
against:

```bash
cargo run -- create
cargo run -- mon
```

## Project structure

This is a small, single-crate project — there's no `commands/`, `docker/`,
or `catalog/` module yet:

```
src/
  main.rs        CLI definition (clap), command dispatch, and the
                  create/close/reconnect logic — everything shells out to
                  the `docker` binary via std::process::Command
  mon/            The `lab mon` TUI (ratatui + crossterm)
    mod.rs        Terminal setup/teardown and the event loop
    app.rs        App state: active tab, overlays, tick-based refresh
    data.rs       Runs `docker ps` / `docker stats` / `docker inspect` /
                  `docker system df` / `docker top` and formats the output
    ui.rs         Renders the tabs/overlays with ratatui widgets

labs/             CVE lab reference material (see below) — not currently
                  read or launched by anything in src/
```

**Rule of thumb for where a change goes:** CLI-facing changes (new flags,
new subcommands, changes to `create`/`close`/`reconnect`) go in
`main.rs`. Anything about the live dashboard — a new tab, a new `docker`
command surfaced, a new keybinding — goes in `src/mon/`: wire the data
fetch in `data.rs`, the state in `app.rs`, and the rendering in `ui.rs`.

## About `labs/`

`labs/` contains three example CVE lab definitions
(`CVE-2011-2523`, `CVE-2014-6271`, `CVE-2021-44228`), each with a
`metadata.yaml` (id, title, difficulty, description, briefing, hints,
ports, `allow_net`) and a `docker/Dockerfile` that reproduces the
vulnerability hermetically (see the existing three for the pattern — e.g.
`labs/CVE-2011-2523/docker/vsftpd_2342_backdoor.py` reproduces the
backdoor's observable behavior in a small script rather than depending on
a 2011-era binary from an unmaintained mirror).

**Nothing in `src/` currently parses `metadata.yaml` or launches these
labs** — there's no `lab catalog` or `lab start <cve-id>` command. Building
that integration (reading `labs/*/metadata.yaml`, adding subcommands to
`main.rs` to build/run/tear down a lab's Dockerfile) is the highest-value
contribution this project can receive right now. If you want to add a new
CVE lab, follow the existing `metadata.yaml` shape for forward
compatibility even though it isn't consumed yet, and mention in your PR
that it's uncatalogued content pending CLI integration. If you want to
build the catalog/launch feature itself, open an issue or discussion first
to coordinate the CLI surface before investing in a large PR.

Until that integration exists, a lab is only usable manually:

```bash
docker build -t <name> labs/<CVE-ID>/docker
docker run --rm -it -p <port>:<port> <name>
```

## Testing approach

```bash
cargo test
```

- All current tests are plain unit tests in `src/mon/data.rs`
  (`#[cfg(test)] mod tests`), covering how `render_result` maps raw
  `docker` error output to user-facing messages. They run with no Docker
  daemon needed.
- There's no Docker-dependent integration test suite yet, and no
  auto-skip-if-no-daemon helper — if you add tests that need a live
  `docker` daemon, either gate them behind a check similar to what you'd
  want `render_result` to handle, or note in the PR that they require
  Docker locally.
- There's no automated end-to-end test of the `create` → `reconnect` →
  `close` lifecycle (it needs a real attached TTY). If you touch
  `cmd_create`, `cmd_reconnect`, or `cmd_close` in `main.rs`, manually run
  through that lifecycle before opening the PR and say so in the PR
  description.

Add tests alongside the code you change, in the same file's `#[cfg(test)]
mod tests` block — that's the existing convention in `src/mon/data.rs`.

## Commit style

Keep subject lines short, imperative, and capitalized, e.g.:

```
Add lab mon TUI dashboard
Fix docker pull error handling in cmd_create
Add CVE-2021-44228 lab Dockerfile
```

`Add <feature>`, `Fix <bug>`, `Add lab <cve-id>`. Keep the subject line
under ~70 characters; use the body for the "why" if it's not obvious from
the subject.

## Pull request process

1. Fork or branch, make your change, `cargo test` and `cargo build --release`
   both pass.
2. Run `cargo fmt` and check `cargo clippy` doesn't introduce new warnings.
3. Open a PR describing **what** changed and **why**. For a new lab,
   include confirmation you exploited it yourself locally and an honest
   difficulty placement in `metadata.yaml`.
4. A maintainer reviews, may ask for changes, and merges once green.

Questions before you start? Open a discussion/issue — see
[COMMUNITY.md](COMMUNITY.md) for how the community operates day to day.
