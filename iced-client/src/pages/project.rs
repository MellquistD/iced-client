use iced::widget::{button, column, row, Button, Text};

use crate::AppEvent;

use super::{workspace::WorkspacePage, Page};

#[derive(Debug, Clone, PartialEq)]
pub enum ProjectPageEvent {}

impl From<ProjectPageEvent> for AppEvent {
    fn from(value: ProjectPageEvent) -> Self {
        AppEvent::Project(value)
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProjectPage {
    title: String,
    value: usize,
}

impl Default for ProjectPage {
    fn default() -> Self {
        Self {
            title: "Project".into(),
            value: 0,
        }
    }
}

impl Page for ProjectPage {
    fn title(&self) -> &str {
        self.title.as_str()
    }

    fn run_event(&mut self, app_event: AppEvent) {
        let AppEvent::Project(event) = app_event else {
            return;
        };

        match event {
            _ => (),
        }
    }

    fn show(&self) -> iced::Element<AppEvent> {
        column![
            row![
                Button::new("Switch to Workspace Page")
                    .on_press(AppEvent::GoTo("Workspace".into())),
                Button::new("Switch to Component Page")
                    .on_press(AppEvent::GoTo("Component".into())),
            ],
            Text::new(self.value)
        ]
        .into()
    }
}
