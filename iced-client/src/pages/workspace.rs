use super::Page;
use crate::{AppEvent, DrawingApp};
use iced::{
    advanced::widget,
    widget::{button, canvas, checkbox, column, container, row, text, Button, Text},
};

#[derive(Debug, Clone, PartialEq)]
pub enum WorkspacePageEvent {
    IncrementNum,
}

impl From<WorkspacePageEvent> for AppEvent {
    fn from(value: WorkspacePageEvent) -> Self {
        AppEvent::Workspace(value.clone())
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WorkspacePage {
    title: String,
    value: usize,
}

impl Default for WorkspacePage {
    fn default() -> Self {
        Self {
            title: "Workspace".into(),
            value: 0,
        }
    }
}

impl WorkspacePage {
    fn run_increment(&mut self) {
        self.value += 1
    }
}

impl Page for WorkspacePage {
    fn title(&self) -> &str {
        self.title.as_str()
    }

    fn run_event(&mut self, app_event: AppEvent) {
        let AppEvent::Workspace(event) = app_event else {
            return;
        };

        match event {
            WorkspacePageEvent::IncrementNum => self.run_increment(),
        }
    }

    fn show(&self) -> iced::Element<AppEvent> {
        column![
            row![
                button("Switch to Project Page").on_press(AppEvent::GoTo("Project".into())),
                button("Switch to Component Page").on_press(AppEvent::GoTo("Component".into())),
            ],
            row![
                Text::new(self.value),
                Button::new("+").on_press(WorkspacePageEvent::IncrementNum.into())
            ]
        ]
        .into()
    }
}
