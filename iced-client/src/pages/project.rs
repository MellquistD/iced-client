use iced::widget::{button, column};

use crate::AppEvent;

use super::{workspace::WorkspacePage, Page};

#[derive(Default, Debug, Clone, PartialEq, Eq)]
pub struct ProjectPage {
    value: usize,
}

impl ProjectPage {
    pub fn show(&self) -> iced::Element<AppEvent> {
        column![
            iced::widget::text("Project Page"),
            button("Switch to Workspace Page")
                .on_press(AppEvent::GoTo(Page::Workspace(WorkspacePage::default())))
        ]
        .into()
    }
}
