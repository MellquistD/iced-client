use iced::widget::{button, column};

use crate::Command;

use super::{workspace::WorkspacePage, Page};

#[derive(Default, Debug, Clone, PartialEq, Eq)]
pub struct ProjectPage;

impl ProjectPage {
    pub fn show(&self) -> iced::Element<Command> {
        column![
            iced::widget::text("Project Page"),
            button("Switch to Workspace Page")
                .on_press(Command::GoTo(Page::Workspace(WorkspacePage::default())))
        ]
        .into()
    }
}
