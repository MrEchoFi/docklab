# Installation

`docklab` needs two things: **Docker** (specifically the `docker` CLI,
reachable and logged in as your user) and the **`lab` binary**, which you
build from source with Cargo.

> **Honesty note:** there are no prebuilt binaries or GitHub Releases for
> `docklab` yet — `dist/` doesn't exist in this repo. Every platform below
> installs the same way: clone the repo and build with Cargo. The Rust
> code itself has no platform-specific logic (it shells out to `docker`
> and uses cross-platform terminal crates), so it should build cleanly
> anywhere Rust and Docker both run — but only the Linux path has been
> exercised in this repository so far. If you hit something unexpected on
> macOS or Windows, please report it — see [CONTRIBUTING.md](CONTRIBUTING.md)
> or [SECURITY.md](SECURITY.md).

---

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

### 2. Install Rust

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

`docklab` targets the 2021 Rust edition — any reasonably recent stable
toolchain (installed via the command above) works.

### 3. Build and install `lab`

```bash
git clone https://github.com/MrEchoFi/docklab.git
cd docklab

cargo install --path .
```

This installs to `~/.cargo/bin/lab` — make sure that directory is on your
`PATH` (rustup's installer offers to do this for you).

If you'd rather control exactly where the binary lands:

```bash
cargo build --release
sudo cp target/release/lab /usr/local/bin/lab
```

### 4. Verify

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

### 2. Install Rust

Install via [rustup-init.exe](https://rustup.rs) (this also prompts to
install the MSVC build tools if needed).

### 3. Build and install `lab`

```powershell
git clone https://github.com/MrEchoFi/docklab.git
cd docklab

cargo install --path .
```

This installs to `%USERPROFILE%\.cargo\bin\lab.exe` — rustup adds this to
`PATH` automatically.

Or, to build without installing:

```powershell
cargo build --release
# binary at target\release\lab.exe
```

### 4. Verify

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

### 2. Install Rust

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

### 3. Build and install `lab`

```bash
git clone https://github.com/MrEchoFi/docklab.git
cd docklab

cargo install --path .
```

or, to control exactly where the binary lands:

```bash
cargo build --release
sudo cp target/release/lab /usr/local/bin/lab
```

Both Intel and Apple Silicon Macs work — Cargo builds for whichever
architecture you're running on.

### 4. Verify

```bash
lab --version
lab create
```

---

## Uninstalling

```bash
lab close                          # removes the docklab container + image
cargo uninstall docklab            # if you installed via `cargo install --path .`
# or, if you copied the binary manually:
sudo rm /usr/local/bin/lab
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

For anything not covered here, see [CONTRIBUTING.md](CONTRIBUTING.md) for
how to file an issue, or [SECURITY.md](SECURITY.md) if it looks like an
isolation-breaking bug rather than a setup issue.
