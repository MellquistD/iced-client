use iced::{
    advanced::{graphics::core::Element, widget},
    widget::{button, column},
};

use crate::AppEvent;

use super::{project::ProjectPage, Page};

enum InternalEvent {
    IncrementValue,
    DecrementValue,
}

#[derive(Default, Debug, Clone, PartialEq, Eq)]
pub struct WorkspacePage {
    value: usize,
}

impl WorkspacePage {
    pub fn show(&self) -> iced::Element<AppEvent> {
        column![
            iced::widget::text("Workspace"),
            button("Switch to Project Page")
                .on_press(AppEvent::GoTo(Page::Project(ProjectPage::default())))
        ]
        .into()
    }
}
