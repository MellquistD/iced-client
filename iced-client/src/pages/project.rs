use iced::widget::{button, column};

use crate::AppEvent;

use super::{workspace::WorkspacePage, Page};

#[derive(Debug, Clone, PartialEq)]
pub enum ProjectEvent {}
#[derive(Default, Debug, Clone, PartialEq, Eq)]
pub struct ProjectPage {
    value: usize,
}

impl Page for ProjectPage {
    fn title(&self) -> String {
        "Project Page".into()
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
