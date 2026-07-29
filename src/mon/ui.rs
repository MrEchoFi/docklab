use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Clear, Paragraph, Tabs, Wrap},
    Frame,
};

use crate::{CONTAINER_NAME, IMAGE_NAME};

use super::app::{App, Overlay, Tab};
use super::data::{self, RunState};

const HELP_TEXT: &str = "\
Keybindings:
  1-5      Switch tabs (Overview / Stats / Disk Usage / Network / Processes)
  r        Force refresh the active tab
  h, ?     Toggle this help overlay
  g        Toggle the guide overlay
  q, Esc   Quit (Esc closes an overlay first if one is open)
";

const GUIDE_TEXT: &str = "\
DockLAB quick start:
  1. lab create   -> pull the Kali image, start the container, drop into bash
  2. lab mon      -> this screen: live status, stats, disk usage, network, processes
  3. lab close    -> stop and remove the container and image
  4. ./reconnect.sh -> reconnect to the existing container

Overview, Stats, and Processes refresh automatically every 2s.
Disk Usage and Network only refresh when you switch to them or press 'r',
since they change less often and cost more to query.
";

pub fn draw(frame: &mut Frame, app: &App) {
    let size = frame.area();
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(3), Constraint::Length(3), Constraint::Min(0)])
        .split(size);

    draw_status_bar(frame, chunks[0], app);
    draw_tab_bar(frame, chunks[1], app);
    draw_content(frame, chunks[2], app);

    if let Some(overlay) = app.overlay {
        draw_overlay(frame, size, overlay);
    }
}

fn draw_status_bar(frame: &mut Frame, area: Rect, app: &App) {
    let (label, color) = match app.overview.running {
        RunState::Running => ("RUNNING", Color::Green),
        RunState::Stopped => ("STOPPED", Color::Yellow),
        RunState::NotCreated => ("NOT CREATED", Color::Red),
    };

    let line = Line::from(vec![
        Span::styled(" DockLAB ", Style::default().add_modifier(Modifier::BOLD)),
        Span::raw(format!("container={CONTAINER_NAME} image={IMAGE_NAME}  ")),
        Span::styled(
            format!(" {label} "),
            Style::default().fg(Color::Black).bg(color).add_modifier(Modifier::BOLD),
        ),
        Span::raw("   ('h' for help, 'g' for guide, 'q' to quit)"),
    ]);

    let block = Block::default().borders(Borders::ALL).title(" lab mon ");
    frame.render_widget(Paragraph::new(line).block(block), area);
}

fn draw_tab_bar(frame: &mut Frame, area: Rect, app: &App) {
    let titles: Vec<Line> = Tab::ALL
        .iter()
        .enumerate()
        .map(|(i, t)| Line::from(format!("[{}] {}", i + 1, t.title())))
        .collect();

    let tabs = Tabs::new(titles)
        .select(app.active_tab.index())
        .block(Block::default().borders(Borders::ALL))
        .highlight_style(Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD))
        .divider(" ");

    frame.render_widget(tabs, area);
}

fn draw_content(frame: &mut Frame, area: Rect, app: &App) {
    let body = match app.active_tab {
        Tab::Overview => {
            let container = if app.overview.container_info.lines().count() <= 1 {
                format!("No container named '{CONTAINER_NAME}' found.")
            } else {
                app.overview.container_info.clone()
            };
            let image = if app.overview.image_info.lines().count() <= 1 {
                format!("No image '{IMAGE_NAME}' found.")
            } else {
                app.overview.image_info.clone()
            };
            format!("-- Container --\n{container}\n\n-- Image --\n{image}")
        }
        Tab::Stats => data::render_result(&app.stats, "docker stats"),
        Tab::Disk => app
            .disk_usage
            .as_ref()
            .map(|r| data::render_result(r, "docker system df -v"))
            .unwrap_or_else(|| "Loading...".to_string()),
        Tab::Network => app
            .network
            .as_ref()
            .map(|r| data::render_result(r, "docker inspect (network)"))
            .unwrap_or_else(|| "Loading...".to_string()),
        Tab::Processes => data::render_result(&app.processes, "docker top"),
    };

    let paragraph = Paragraph::new(body)
        .block(Block::default().borders(Borders::ALL).title(format!(" {} ", app.active_tab.title())))
        .wrap(Wrap { trim: false });

    frame.render_widget(paragraph, area);
}

fn draw_overlay(frame: &mut Frame, area: Rect, overlay: Overlay) {
    let popup = centered_rect(70, 60, area);
    let (title, text) = match overlay {
        Overlay::Help => ("Help", HELP_TEXT),
        Overlay::Guide => ("Guide", GUIDE_TEXT),
    };

    let paragraph = Paragraph::new(text)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title(format!(" {title} (Esc to close) ")),
        )
        .wrap(Wrap { trim: false });

    frame.render_widget(Clear, popup);
    frame.render_widget(paragraph, popup);
}

fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    let vertical = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage((100 - percent_y) / 2),
            Constraint::Percentage(percent_y),
            Constraint::Percentage((100 - percent_y) / 2),
        ])
        .split(r);

    Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage((100 - percent_x) / 2),
            Constraint::Percentage(percent_x),
            Constraint::Percentage((100 - percent_x) / 2),
        ])
        .split(vertical[1])[1]
}
