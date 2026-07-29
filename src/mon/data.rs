use std::process::Command as ShellCommand;

use crate::{CONTAINER_NAME, IMAGE_NAME};

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum RunState {
    Running,
    Stopped,
    NotCreated,
}

pub struct OverviewData {
    pub container_info: String,
    pub image_info: String,
    pub running: RunState,
}

fn capture(cmd: &str, args: &[&str]) -> Result<String, String> {
    match ShellCommand::new(cmd).args(args).output() {
        Ok(output) => {
            if output.status.success() {
                Ok(String::from_utf8_lossy(&output.stdout).trim().to_string())
            } else {
                let stderr = String::from_utf8_lossy(&output.stderr).trim().to_string();
                if stderr.is_empty() {
                    Err(format!("{} exited with {}", cmd, output.status))
                } else {
                    Err(stderr)
                }
            }
        }
        Err(e) => Err(format!("failed to execute '{}': {}", cmd, e)),
    }
}

fn capture_shell(script: &str) -> Result<String, String> {
    capture("sh", &["-c", script])
}

pub fn fetch_overview() -> OverviewData {
    let container_info = capture(
        "docker",
        &[
            "ps",
            "-a",
            "--filter",
            &format!("name={}", CONTAINER_NAME),
            "--format",
            "table {{.Names}}\\t{{.Status}}\\t{{.Image}}\\t{{.ID}}",
        ],
    )
    .unwrap_or_default();

    let image_info = capture(
        "docker",
        &[
            "images",
            IMAGE_NAME,
            "--format",
            "table {{.Repository}}\\t{{.Tag}}\\t{{.ID}}\\t{{.Size}}",
        ],
    )
    .unwrap_or_default();

    let running_raw = capture("docker", &["inspect", "-f", "{{.State.Running}}", CONTAINER_NAME])
        .unwrap_or_default();

    let running = if running_raw.trim() == "true" {
        RunState::Running
    } else if !running_raw.trim().is_empty() {
        RunState::Stopped
    } else {
        RunState::NotCreated
    };

    OverviewData {
        container_info,
        image_info,
        running,
    }
}

pub fn fetch_stats() -> Result<String, String> {
    capture("docker", &["stats", "--no-stream", CONTAINER_NAME])
}

pub fn fetch_disk_usage() -> Result<String, String> {
    capture("docker", &["system", "df", "-v"])
}

pub fn fetch_network() -> Result<String, String> {
    capture_shell(&format!(
        "docker inspect {} | grep -A 10 NetworkSettings",
        CONTAINER_NAME
    ))
}

pub fn fetch_processes() -> Result<String, String> {
    capture("docker", &["top", CONTAINER_NAME])
}

/// Turns a raw docker Result into panel text, mapping "container not running"
/// style failures to a friendly hint instead of the raw docker error.
pub fn render_result(res: &Result<String, String>, label: &str) -> String {
    match res {
        Ok(s) if !s.trim().is_empty() => s.clone(),
        Ok(_) => format!("No output from {}.", label),
        Err(e) => {
            let lower = e.to_lowercase();
            if lower.contains("is not running") || lower.contains("no such container") {
                "Container not running \u{2014} run 'lab create' to start it.".to_string()
            } else {
                format!("Error running {}: {}", label, e)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn render_result_maps_not_running_error() {
        let res: Result<String, String> = Err("Error response from daemon: Container abc is not running".to_string());
        assert_eq!(
            render_result(&res, "docker stats"),
            "Container not running \u{2014} run 'lab create' to start it."
        );
    }

    #[test]
    fn render_result_maps_no_such_container_error() {
        let res: Result<String, String> = Err("Error: No such container: kalilab".to_string());
        assert_eq!(
            render_result(&res, "docker top"),
            "Container not running \u{2014} run 'lab create' to start it."
        );
    }

    #[test]
    fn render_result_passes_through_other_errors() {
        let res: Result<String, String> = Err("permission denied".to_string());
        assert_eq!(
            render_result(&res, "docker stats"),
            "Error running docker stats: permission denied"
        );
    }

    #[test]
    fn render_result_passes_through_success() {
        let res: Result<String, String> = Ok("some output".to_string());
        assert_eq!(render_result(&res, "docker top"), "some output");
    }

    #[test]
    fn render_result_flags_empty_success() {
        let res: Result<String, String> = Ok("".to_string());
        assert_eq!(render_result(&res, "docker top"), "No output from docker top.");
    }
}
