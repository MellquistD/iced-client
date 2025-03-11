use iced::{
    advanced::{graphics::core::Element, widget},
    widget::{button, column},
};

use crate::Command;

use super::{project::ProjectPage, Page};

#[derive(Default, Debug, Clone, PartialEq, Eq)]
pub struct WorkspacePage;

impl WorkspacePage {
    pub fn show(&self) -> iced::Element<Command> {
        column![
            iced::widget::text("Workspace"),
            button("Switch to Project Page")
                .on_press(Command::GoTo(Page::Project(ProjectPage::default())))
        ]
        .into()
    }
}
