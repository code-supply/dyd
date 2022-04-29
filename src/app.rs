use crate::event::Event;
use crate::manifest::Manifest;
use crate::repo::{Log, Repo, RepoStatus};
use crate::ui;

use indexmap::map::IndexMap;
use std::error;
use std::path::PathBuf;
use std::sync::mpsc;
use tui::backend::Backend;
use tui::layout::{Constraint, Direction, Layout};
use tui::terminal::Frame;
use tui::widgets::TableState;

pub type AppResult<T> = std::result::Result<T, Box<dyn error::Error>>;

#[derive(Debug, PartialEq)]
pub enum AppState {
    Init,
    Checking,
}

impl Default for AppState {
    fn default() -> Self {
        AppState::Init
    }
}

/// Selected pane
#[derive(Debug, PartialEq)]
pub enum SelectedPane {
    Diff,
    Repos,
}

impl Default for SelectedPane {
    fn default() -> Self {
        SelectedPane::Repos
    }
}

#[derive(Debug)]
pub struct App {
    pub repos: IndexMap<String, Repo>,
    pub repo_state: TableState,
    pub running: bool,
    pub root_path: PathBuf,
    pub selected_pane: SelectedPane,
    pub selected_repo_state: TableState,
    pub since: chrono::DateTime<chrono::Utc>,
    pub state: AppState,
}

impl From<Manifest> for App {
    fn from(manifest: Manifest) -> Self {
        let repos: IndexMap<String, Repo> = manifest
            .remotes
            .into_iter()
            .map(|(id, remote)| (id, remote.into()))
            .collect();

        let mut repo_state = TableState::default();
        if repos.len() > 0 {
            repo_state.select(Some(0))
        }

        let mut selected_repo_state = TableState::default();
        selected_repo_state.select(Some(0));

        Self {
            repos,
            repo_state,
            selected_repo_state,
            root_path: manifest.root.unwrap(),
            selected_pane: SelectedPane::default(),
            since: manifest.since_datetime.unwrap(),
            running: true,
            state: AppState::default(),
        }
    }
}

impl App {
    pub fn tick(&mut self, sender: mpsc::Sender<Event>) -> AppResult<()> {
        match self.state {
            AppState::Init => {
                self.update(sender)?;
                self.state = AppState::Checking;
            }
            _ => {}
        }
        Ok(())
    }

    pub fn render<B: Backend>(&mut self, frame: &mut Frame<'_, B>) {
        let size = frame.size();
        let layout = Layout::default()
            .direction(Direction::Horizontal)
            .margin(0)
            .constraints([Constraint::Percentage(75), Constraint::Percentage(25)].as_ref())
            .split(size);

        let sidebar = Layout::default()
            .direction(Direction::Vertical)
            .margin(0)
            .constraints([Constraint::Ratio(7, 10), Constraint::Ratio(3, 10)].as_ref())
            .split(layout[1]);

        frame.render_stateful_widget(ui::diff::render(self), layout[0], &mut self.selected_repo_state.clone());
        frame.render_stateful_widget(ui::repos::render(self), sidebar[0], &mut self.repo_state.clone());
        frame.render_widget(ui::help::render(self), sidebar[1]);
    }

    pub fn update(&self, sender: mpsc::Sender<Event>) -> AppResult<()> {
        for (id, repo) in &self.repos {
            repo.update(id.clone(), &self.root_path, sender.clone())?;
        }
        Ok(())
    }

    pub fn update_repo_status(&mut self, id: String, status: RepoStatus) -> AppResult<()> {
        if let Some(repo) = self.repos.get_mut(&id) {
            repo.status = status;
        }
        Ok(())
    }

    pub fn update_repo_logs(&mut self, id: String, logs: Vec<Log>) -> AppResult<()> {
        if let Some(repo) = self.repos.get_mut(&id) {
            repo.logs = logs;
            repo.status = RepoStatus::Finished;
        }
        Ok(())
    }
}
