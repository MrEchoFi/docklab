# Installation

`docklab` needs two things on any platform: **Docker** (the `docker` CLI,
reachable and logged in as your user) and the **`lab` binary**. Pick your
OS below.

> **Honesty note on prebuilt binaries:** `dist/lab-linux-x86_64` is built
> natively on Linux as part of producing this repo's release artifacts,
> and smoke-tested (`lab --version`, `lab --help`) in that same
> environment — it's the one binary here that's actually been run.
> `dist/lab-windows-x86_64.exe` is **cross-compiled** from Linux
> (`x86_64-pc-windows-gnu` + `mingw-w64`) and confirmed to be a valid
> PE32+ executable, but has not been run against a real Docker Desktop for
> Windows install. `dist/lab-macos-x86_64` and `dist/lab-macos-arm64` are
> **cross-compiled** from Linux using [Zig](https://ziglang.org) as the
> linker/C toolchain via [`cargo-zigbuild`](https://github.com/rust-cross/cargo-zigbuild)
> (there's no Apple hardware or Xcode SDK in this environment) and
> confirmed to be valid Mach-O x86_64 / arm64 executables, but neither has
> been run on a real Mac. If you hit anything unexpected on Windows or
> macOS, please report it — see [SECURITY.md](SECURITY.md) or
> [CONTRIBUTING.md](CONTRIBUTING.md). Building from source with Cargo
> (Option B/C below) avoids all of this by compiling natively on your own
> machine.

---

## First clone the REPO

```bash

git clone https://github.com/MrEchoFi/docklab.git
cd docklab

```


## Linux

### 1. Install Docker

```bash
# Debian/Ubuntu
curl -fsSL https://get.docker.com | sh
sudo usermod -aG docker "$USER"   # log out/in (or `newgrp docker`) so it takes effect

# Fedora
sudo dnf install -y docker
sudo systemctl enable --now docker
sudo usermod -aG docker "$USER"

# Arch
sudo pacman -S docker
sudo systemctl enable --now docker
sudo usermod -aG docker "$USER"
```

Confirm it works **without `sudo`** before moving on:

```bash
docker ps
```

If that fails with a permissions error, your group membership change
hasn't taken effect yet — fully log out and back in.

### 2. Install `lab`

**Option A — prebuilt binary (fastest):**

```bash
chmod +x dist/lab-linux-x86_64
sudo cp dist/lab-linux-x86_64 /usr/local/bin/lab
```

(See [Verifying a download](#verifying-a-download) below to check the
checksum first.)

**Option B — via Cargo, from source:**

```bash
# Rust toolchain, if you don't have one:
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

cargo install --path .
```

This installs to `~/.cargo/bin/lab` — make sure that directory is on your
`PATH` (rustup's installer offers to do this for you).

**Option C — build it yourself:**

```bash
cargo build --release
sudo cp target/release/lab /usr/local/bin/lab
```

### 3. Verify

```bash
lab --version   # lab 0.1.0
lab create      # pulls kalilinux/kali-rolling and drops you into a shell —
                 # confirms Docker + lab both work end to end
```

---

## Windows

### 1. Install Docker

Install **Docker Desktop for Windows** with the **WSL2 backend** (the
default and recommended option):
[docker.com/products/docker-desktop](https://www.docker.com/products/docker-desktop/).
During setup, ensure "Use the WSL 2 based engine" is checked.

Confirm it works from PowerShell:

```powershell
docker ps
```

### 2. Install `lab`

**Option A — prebuilt binary:**

```powershell
# From a clone of this repo:
Copy-Item dist\lab-windows-x86_64.exe C:\Windows\System32\lab.exe
# Or somewhere already on PATH, e.g. a personal C:\bin\ you've added to PATH.
```

(See [Verifying a download](#verifying-a-download) below to check the
checksum first — recommended given the cross-compile note above.)

**Option B — via Cargo, from source:**

Install Rust via [rustup-init.exe](https://rustup.rs) (this also installs
the MSVC build tools prompt if needed), then from a clone of this repo:

```powershell
cargo install --path .
```

This installs to `%USERPROFILE%\.cargo\bin\lab.exe` — rustup adds this to
`PATH` automatically.

**Option C — build it yourself:**

```powershell
cargo build --release
# binary at target\release\lab.exe
```

### 3. Verify

```powershell
lab --version
lab create
```

`lab` calls the `docker` executable directly (it does not talk to the
Docker Engine API), so all it needs is `docker` on `PATH` and Docker
Desktop running.

---

## macOS

### 1. Install Docker

Install **Docker Desktop for Mac** (Apple Silicon or Intel, matching your
Mac): [docker.com/products/docker-desktop](https://www.docker.com/products/docker-desktop/).

Confirm:

```bash
docker ps
```

### 2. Install `lab`

**Option A — prebuilt binary:**

```bash
# Apple Silicon (M1/M2/M3/M4):
chmod +x dist/lab-macos-arm64
sudo cp dist/lab-macos-arm64 /usr/local/bin/lab

# Intel:
chmod +x dist/lab-macos-x86_64
sudo cp dist/lab-macos-x86_64 /usr/local/bin/lab
```

These are cross-compiled and, per the honesty note above, not yet run on
real Apple hardware — verify the checksum first (see
[Verifying a download](#verifying-a-download)), and if macOS Gatekeeper
blocks the unsigned binary on first run, either allow it via **System
Settings → Privacy & Security**, or use Option B/C to build natively
instead.

**Option B — via Cargo, from source:**

```bash
# Rust toolchain, if you don't have one:
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

cargo install --path .
```

**Option C — build it yourself:**

```bash
cargo build --release
sudo cp target/release/lab /usr/local/bin/lab
```

Both Intel and Apple Silicon Macs work — Cargo builds for whichever
architecture you're running on, and a native build sidesteps the
cross-compile caveats entirely.

### 3. Verify

```bash
lab --version
lab create
```

---

## Verifying a download

`dist/SHA256SUMS` contains the checksum for each binary in this repo.
Verify before you run anything you didn't build yourself:

```bash
# Linux/macOS
cd dist && sha256sum -c SHA256SUMS
# on macOS if sha256sum isn't available: shasum -a 256 -c SHA256SUMS
```

```powershell
# Windows PowerShell
cd dist
Get-FileHash lab-windows-x86_64.exe -Algorithm SHA256
# Compare the Hash value against the matching line in SHA256SUMS
```

Expected output for a clean checksum check (Linux/macOS): each file prints
`: OK`.

---

## Uninstalling

```bash
lab close                          # full teardown: stops + removes the
                                    # docklab container and image
sudo rm /usr/local/bin/lab         # or wherever you installed it
                                    # (cargo install --path . → cargo uninstall docklab)
```

On Windows, delete `lab.exe` from wherever you placed it (or run
`cargo uninstall docklab` if you used `cargo install`).

---

## Troubleshooting

| Symptom | Likely cause | Fix |
|---|---|---|
| `Failed to execute 'docker': ...` / `docker pull failed` | Docker isn't running, or the `docker` binary isn't on `PATH` | Start Docker Desktop / `systemctl start docker`; confirm `docker ps` works in the same shell you're running `lab` from |
| `docker pull failed` with a permissions error on Linux | Your user isn't in the `docker` group, or the group change hasn't taken effect yet | `sudo usermod -aG docker "$USER"`, then fully log out and back in |
| `lab: command not found` after `cargo install` | `~/.cargo/bin` (or `%USERPROFILE%\.cargo\bin` on Windows) not on `PATH` | Add it, or restart your shell — rustup usually does this automatically on install |
| `docker run failed. If a container named 'docklab' already exists, try 'lab close' first.` | A `docklab` container from a previous session is still around | Run `lab close`, or `lab reconnect` if you want to reuse it instead |
| `permission denied` running the prebuilt Linux binary | Missing execute bit | `chmod +x dist/lab-linux-x86_64` |
| macOS: "cannot be opened because the developer cannot be verified" | Unsigned, cross-compiled binary — Gatekeeper's default stance on any unsigned binary, not `docklab`-specific | Verify the [checksum](#verifying-a-download) matches, then allow it via **System Settings → Privacy & Security → Open Anyway**, or build from source with Option B/C |
| Windows: antivirus flags/quarantines `lab.exe` | Unsigned binary from a small OSS project — common false-positive pattern for any unsigned cross-compiled `.exe` | Verify the [checksum](#verifying-a-download) matches, then allow it, or build from source yourself with Option B/C above |

For anything not covered here, see [CONTRIBUTING.md](CONTRIBUTING.md) for
how to file an issue, or [SECURITY.md](SECURITY.md) if it looks like an
isolation-breaking bug rather than a setup issue.
