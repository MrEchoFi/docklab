use std::time::{Duration, Instant};

use super::data::{self, OverviewData};

const LIVE_REFRESH_INTERVAL: Duration = Duration::from_secs(2);

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Tab {
    Overview,
    Stats,
    Disk,
    Network,
    Processes,
}

impl Tab {
    pub const ALL: [Tab; 5] = [Tab::Overview, Tab::Stats, Tab::Disk, Tab::Network, Tab::Processes];

    pub fn title(&self) -> &'static str {
        match self {
            Tab::Overview => "Overview",
            Tab::Stats => "Stats",
            Tab::Disk => "Disk Usage",
            Tab::Network => "Network",
            Tab::Processes => "Processes",
        }
    }

    pub fn index(&self) -> usize {
        Tab::ALL.iter().position(|t| t == self).unwrap()
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Overlay {
    Help,
    Guide,
}

pub struct App {
    pub active_tab: Tab,
    pub overlay: Option<Overlay>,
    pub should_quit: bool,
    pub overview: OverviewData,
    pub stats: Result<String, String>,
    pub disk_usage: Option<Result<String, String>>,
    pub network: Option<Result<String, String>>,
    pub processes: Result<String, String>,
    last_live_refresh: Instant,
}

impl App {
    pub fn new() -> Self {
        let mut app = App {
            active_tab: Tab::Overview,
            overlay: None,
            should_quit: false,
            overview: data::fetch_overview(),
            stats: data::fetch_stats(),
            disk_usage: None,
            network: None,
            processes: data::fetch_processes(),
            last_live_refresh: Instant::now(),
        };
        app.ensure_tab_loaded(Tab::Overview);
        app
    }

    pub fn set_tab(&mut self, tab: Tab) {
        self.active_tab = tab;
        self.ensure_tab_loaded(tab);
    }

    fn ensure_tab_loaded(&mut self, tab: Tab) {
        match tab {
            Tab::Disk if self.disk_usage.is_none() => {
                self.disk_usage = Some(data::fetch_disk_usage());
            }
            Tab::Network if self.network.is_none() => {
                self.network = Some(data::fetch_network());
            }
            _ => {}
        }
    }

    pub fn force_refresh_active(&mut self) {
        match self.active_tab {
            Tab::Overview => self.overview = data::fetch_overview(),
            Tab::Stats => self.stats = data::fetch_stats(),
            Tab::Disk => self.disk_usage = Some(data::fetch_disk_usage()),
            Tab::Network => self.network = Some(data::fetch_network()),
            Tab::Processes => self.processes = data::fetch_processes(),
        }
    }

    /// Overview, Stats, and Processes are cheap enough to poll on a timer.
    /// Disk Usage and Network only refresh on tab switch or explicit 'r'.
    pub fn on_tick(&mut self) {
        if self.last_live_refresh.elapsed() >= LIVE_REFRESH_INTERVAL {
            self.overview = data::fetch_overview();
            self.stats = data::fetch_stats();
            self.processes = data::fetch_processes();
            self.last_live_refresh = Instant::now();
        }
    }
}
