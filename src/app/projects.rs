use crate::utils::config_loader::{projects_config_loader, Project};
use tui::{
    style::Style,
    widgets::{Block, Borders, List, ListItem, ListState},
};

pub struct Projects {
    pub projects: Vec<Project>,
}

impl Projects {
    pub fn new() -> Self {
        let projects: Vec<Project> = projects_config_loader();

        return Self { projects };
    }

    pub fn projects_as_widget_list(&self) -> List {
        let projects: Vec<ListItem> = self
            .projects
            .iter()
            .map(|project| ListItem::new(project.name.clone()))
            .collect();

        List::new(projects)
            .block(Block::default().borders(Borders::ALL).title("Projects"))
            .highlight_style(Style::default().bg(tui::style::Color::Magenta))
    }
}
