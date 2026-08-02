use clap::{Parser, Subcommand};
use std::process::{Command as ShellCommand, Stdio};

mod mon;

const VERSION: &str = "0.1.0";
const IMAGE_NAME: &str = "kalilinux/kali-rolling";
const CONTAINER_NAME: &str = "docklab";

/// docklab - A CLI tool to create, monitor, and destroy a Kali Linux Docker lab
#[derive(Parser)]
#[command(
    name = "lab",
    disable_help_flag = true,
    disable_version_flag = true,
    about = "Test anything or do any cybersecurity operation via Docklab"
)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,

    /// Show version info
    #[arg(long = "version", global = true)]
    version: bool,

    /// Enable verbose mode
    #[arg(short = 'V', long = "verbose", global = true)]
    verbose: bool,

    /// Show help section with bio
    #[arg(short = 'h', long = "help", global = true)]
    help: bool,

    /// Show an intro and guidelines for this tool
    #[arg(long = "guide", global = true)]
    guide: bool,
}

#[derive(Subcommand)]
enum Commands {
    /// Pull the Kali image and run it as a container
    Create,
    /// Stop and fully remove the container and image from this device
    Close,
    /// Show whether the lab is running and give full container/image details
    Mon,
    /// Reconnect to the existing container
    Reconnect,
}

fn main() {
    let cli = Cli::parse();

    // Flags take priority over subcommands, matching the requested behavior
    // for `lab -h`, `lab -version`, `lab -V`, `lab -guide`.
    if cli.help {
        print_help();
        return;
    }
    if cli.version {
        print_version();
        return;
    }
    if cli.guide {
        print_guide();
        return;
    }

    let verbose = cli.verbose;

    match cli.command {
        Some(Commands::Create) => cmd_create(verbose),
        Some(Commands::Close) => cmd_close(verbose),
        Some(Commands::Mon) => mon::run(verbose),
        Some(Commands::Reconnect) => cmd_reconnect(verbose),
        None => {
            if verbose {
                println!("[verbose] No subcommand given, showing help.");
            }
            print_help();
        }
    }
}

fn run(cmd: &str, args: &[&str], post_cmd: &str, verbose: bool) -> bool {
    let combined;
    let mut full_args: Vec<&str> = args.to_vec();
    if !post_cmd.is_empty() {
        combined = format!("{}; exec bash", post_cmd);
        full_args.push("-c");
        full_args.push(&combined);
    }

    if verbose {
        println!("[verbose] Running: {} {}", cmd, full_args.join(" "));
    }
    match ShellCommand::new(cmd)
        .args(&full_args)
        .stdin(Stdio::inherit())
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .status()
    {
        Ok(status) => {
            if verbose {
                println!("[verbose] Exit status: {}", status);
            }
            status.success()
        }
        Err(e) => {
            eprintln!("Failed to execute '{}': {}", cmd, e);
            false
        }
    }
}

fn cmd_create(verbose: bool) {
    println!("==> Pulling image: {}", IMAGE_NAME);
    if !run("docker", &["pull", IMAGE_NAME], "", verbose) {
        eprintln!("docker pull failed. Aborting.");
        return;
    }

    println!(r"________                 __   .____       _____ __________ 
\______ \   ____   ____ |  | _|    |     /  _  \\______   \
 |    |  \ /  _ \_/ ___\|  |/ /    |    /  /_\  \|    |  _/
 |    `   (  <_> )  \___|    <|    |___/    |    \    |   \
/_______  /\____/ \___  >__|_ \_______ \____|__  /______  /
        \/            \/     \/       \/       \/       \/ BY github/MrEchoFi");
        
    println!("==> Starting DockLab: {}", CONTAINER_NAME);
    println!("(You will be dropped into an interactive bash shell inside Kali.)");
    let ok = run(
        "docker",
        &[
            "run",
            "-it",
            "--name",
            CONTAINER_NAME,
            IMAGE_NAME,
            "bash",
        ],
        r#"apt update -y && apt upgrade -y && apt install -y curl wget nmap nano git net-tools lynis && echo "PS1='┌──(DockLAB<~>\h)-[\w]\n└─# '" >> ~/.bashrc"#,
        verbose,
    );

    if ok {
        println!("==> Lab session ended.");
    } else {
        eprintln!(
            "docker run failed. If a container named '{}' already exists, try 'lab close' first.",
            CONTAINER_NAME
        );
    }
}

fn cmd_close(verbose: bool) {
    println!("==> Stopping container (if running): {}", CONTAINER_NAME);
    run("docker", &["stop", CONTAINER_NAME], "", verbose);

    println!("==> Removing container: {}", CONTAINER_NAME);
    run("docker", &["rm", "-f", CONTAINER_NAME], "", verbose);

    println!("==> Removing image: {}", IMAGE_NAME);
    let removed = run("docker", &["rmi", "-f", IMAGE_NAME], "", verbose);

    if removed {
        println!("==> Lab fully removed. No Kali container or image remain on this device.");
    } else {
        eprintln!(
            "==> Image removal reported an issue. Run 'lab mon' to check current state, \
             or 'docker images' manually."
        );
    }
}

//For reconnect to the docklab
fn cmd_reconnect(verbose: bool) {
    //let _reconnect = Command::new("docker").args(["start","-ai", CONTAINER_NAME]).status().expect("failed to reconnect!!! SORRY!!!~~~");

    if !run("docker", &["start", "-ai", CONTAINER_NAME], "", verbose) {
        panic!("failed to reconnect!!! SORRY!!!~~~");
    }
}



fn print_version() {
    println!("lab {}", VERSION);
}

fn print_help() {
    println!(
        r#"________                 __   .____       _____ __________ 
\______ \   ____   ____ |  | _|    |     /  _  \\______   \
 |    |  \ /  _ \_/ ___\|  |/ /    |    /  /_\  \|    |  _/
 |    `   (  <_> )  \___|    <|    |___/    |    \    |   \
/_______  /\____/ \___  >__|_ \_______ \____|__  /______  /
        \/            \/     \/       \/       \/       \/  By MrEchoFi
        
        DockLAB {version} — DockLAB

BIO:
 DockLAB ~ Disposable, isolated terminals for practicing IT stuffs, CVE PoCs and pentesting — safely.
  'DockLAB' is a small CLI wrapper around Docker that makes it effortless to spin
  up a disposable Kali Linux environment, check on it, and tear it down again
  without leaving any trace on your host system.

USAGE:
  lab <COMMAND> [FLAGS]

COMMANDS:
  create        Pull the Kali image and run it as an interactive container
  close         Stop and fully remove the container and image from this device
  mon           Open a live TUI dashboard: status, stats, disk usage, network, processes
  reconnect     Reconnect to the existing container

FLAGS:
  --version     Show version info
  -V, --verbose Enable verbose mode (prints every underlying docker command)
  -h, --help    Show this help section
  --guide       Show an intro and usage guidelines for this tool

EXAMPLES:
  lab create
  lab mon
  lab close
  lab create -V
"#,
        version = VERSION
    );
}

fn print_guide() {
    println!(
        r#"________                 __   .____       _____ __________ 
\______ \   ____   ____ |  | _|    |     /  _  \\______   \
 |    |  \ /  _ \_/ ___\|  |/ /    |    /  /_\  \|    |  _/
 |    `   (  <_> )  \___|    <|    |___/    |    \    |   \
/_______  /\____/ \___  >__|_ \_______ \____|__  /______  /
        \/            \/     \/       \/       \/       \/  By MrEchoFi
==================================
 DockLAB — QUICK START GUIDE
==================================

WHAT THIS IS:
  DockLAB ~Disposable, isolated terminals for practicing IT stuffs, CVE PoCs and pentesting — safely.
  'lab' manages a single Kali Linux Docker container for you, called '{container}',
  built from the '{image}' image. It's meant for quick, disposable
  security-tooling practice — not a permanent VM replacement.

TYPICAL WORKFLOW:
  1. lab create
     -> Pulls the Kali image (if not already present) and drops you into
        an interactive bash shell inside the container.
     -> Install whatever tools you need with apt, e.g.:
          apt update && apt install -y nmap curl git lynis

  2. Do your work inside the container.
     -> Type 'exit' when done. The container stops but is NOT deleted,
        so your installed tools remain if you reconnect with:
          docker start -ai {container}

  3. lab mon
     -> Anytime, open a live TUI dashboard covering container/image status,
        resource stats, disk usage, network settings, and running processes.
        Press '1'-'5' to switch tabs, 'r' to refresh, 'h' for help, 'g' for
        this guide, 'q' to quit.

  4. lab close
     -> Stops the container, removes it, and removes the Kali image
        entirely. After this, nothing related to the lab remains on
        your machine — you'd start fresh with 'lab create' next time.
  5. Want to Re-connect without close the lab? Use:
         'lab reconnect' 

NOTES:
  - Add -V / --verbose to any command to see the exact docker commands
    being run under the hood.
  - Requires Docker installed and the Docker daemon running.
  - Container isolation is not a full security boundary the way a VM
    is — treat this as a convenience tool, not a hard sandbox.
"#,
        container = CONTAINER_NAME,
        image = IMAGE_NAME
    );
}
