use iced::widget::{button, column};

use crate::AppEvent;

use super::{workspace::WorkspacePage, Page};

#[derive(Debug, Clone, PartialEq)]
pub enum ProjectEvent {}
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
    fn show(&self) -> iced::Element<AppEvent> {
        column![
            iced::widget::text("Project Page"),
            //button("Switch to Workspace Page")
            //    .on_press(AppEvent::GoTo(Page::Workspace(WorkspacePage::default())))
        ]
        .into()
    }
}
