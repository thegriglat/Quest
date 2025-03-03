pub mod actions;
pub mod configs;
pub mod events;
pub mod file_handler;
pub mod widget;

use configs::Configs;
use serde::{Deserialize, Serialize};
use std::{error::Error, io::Stdout};
use tui::{backend::CrosstermBackend, style::Style, Frame, Terminal};

pub type DynResult = Result<(), Box<dyn Error>>;
pub type CrossTerminal = Terminal<CrosstermBackend<Stdout>>;
pub type TerminalFrame<'a> = Frame<'a, CrosstermBackend<Stdout>>;

/// Represent a task
#[derive(Serialize, Deserialize, Clone)]
pub struct Quest {
    pub title: String,
    pub completed: bool,
}

impl Quest {
    pub fn new(title: String) -> Self {
        Self {
            title,
            completed: false,
        }
    }
}

/// Represent a list of tasks
#[derive(Serialize, Deserialize, Default)]
pub struct QuestList {
    pub quests: Vec<Quest>,
}

impl QuestList {
    pub fn new(quests: &[Quest]) -> Self {
        Self {
            quests: quests.to_vec(),
        }
    }
}

/// Possible Input field states
pub enum InputMode {
    /// Browsing quests
    Normal,
    /// Adding a new quest
    Adding,
}

/// Application state
pub struct App {
    /// New quest input value
    pub input: String,
    /// Current input mode
    pub input_mode: InputMode,
    /// List of all quests
    pub quests: Vec<Quest>,
    /// Should be true when application wants to exit
    pub should_exit: bool,
    /// Application Configs
    pub configs: Configs,
}

impl App {
    pub fn new(quests: &[Quest], configs: Configs) -> Self {
        Self {
            quests: quests.to_vec(),
            input: String::new(),
            input_mode: InputMode::Normal,
            should_exit: false,
            configs,
        }
    }

    pub fn default_style(&self) -> Style {
        Style::default()
            .fg(self.configs.colors.foreground)
            .bg(self.configs.colors.background)
    }

    pub fn selection_style(&self) -> Style {
        self.default_style()
            .fg(self.configs.colors.selection_fg)
            .bg(self.configs.colors.selection_bg)
    }

    pub fn delete_quest(&mut self, index: usize) {
        if index < self.quests.len() {
            self.quests.remove(index);
        }
    }

    pub fn add_quest(&mut self, quest: Quest) {
        self.quests.push(quest);
    }
}
