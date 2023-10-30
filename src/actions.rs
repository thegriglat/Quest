//! Actions to do after a specific event occurs

use tui::widgets::ListState;

use crate::{App, InputMode, Quest};

pub fn new_quest(app: &mut App, list_state: &mut ListState) {
    app.input_mode = InputMode::Adding;
    list_state.select(None)
}

pub fn exit_app(app: &mut App) {
    app.should_exit = true;
}

pub fn list_up(list_state: &mut ListState) {
    if let Some(index) = list_state.selected() {
        if index > 0 {
            list_state.select(Some(index - 1));
        }
    }
}

pub fn list_down(quests_len: usize, list_state: &mut ListState) {
    if let Some(index) = list_state.selected() {
        if index < quests_len - 1 {
            list_state.select(Some(index + 1));
        }
    }
}

pub fn check_and_uncheck_quest(app: &mut App, list_state: &mut ListState) {
    if let Some(index) = list_state.selected() {
        app.quests[index].completed = !app.quests[index].completed;
    }
}

pub fn delete_quest(app: &mut App, list_state: &mut ListState) {
    if let Some(index) = list_state.selected() {
        app.delete_quest(index);
        if app.quests.is_empty() {
            list_state.select(None);
        } else {
            let new_index = match index {
                0 => 0,
                _ => index - 1,
            };
            list_state.select(Some(new_index));
        }
    }
}

pub fn save_quest(app: &mut App) {
    let new_quest = Quest::new(app.input.drain(..).collect());
    app.add_quest(new_quest)
}

pub fn exit_adding(app: &mut App, list_state: &mut ListState) {
    app.input_mode = InputMode::Normal;
    list_state.select(Some(0));
}

pub fn input_add_char(app: &mut App, c: char) {
    app.input.push(c);
}

pub fn input_del_char(app: &mut App) {
    app.input.pop();
}
