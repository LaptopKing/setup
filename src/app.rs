use projects::Projects;
use tui::widgets::ListState;

pub mod projects;

pub enum Window {
    Projects,
    ProjectDetails,
}

pub struct App {
    pub projects: Projects,

    pub state: ListState,
    pub state_index: u16,
    pub window: Window,
}

impl App {
    pub fn new() -> Self {
        Self {
            projects: Projects::new(),

            state: ListState::default(),
            state_index: 0,

            window: Window::Projects,
        }
    }

    pub fn next(&mut self) {
        if self.projects.projects.len() <= usize::from(self.state_index + 1) {
            return;
        }

        self.state_index += 1;
        self.state.select(Some(usize::from(self.state_index)));
    }

    pub fn previous(&mut self) {
        if self.state_index == 0 {
            return;
        }

        self.state_index -= 1;
        self.state.select(Some(usize::from(self.state_index)));
    }
}
