use iced::{
    advanced::widget,
    widget::{button, canvas, checkbox, column, container, row, text},
};

use crate::{AppEvent, DrawingApp};

use super::Page;

#[derive(Default, Debug, Clone, PartialEq, Eq)]
pub struct WorkspacePage {
    value: usize,
}

impl Page for WorkspacePage {
    fn title(&self) -> String {
        "Workspace Page".into()
    }
    fn show(&self) -> iced::Element<AppEvent> {
        column![
            iced::widget::text("Workspace Page"),
            //button("Switch to Project Page")
            //    .on_press(AppEvent::GoTo(Page::Project(ProjectPage::default()))),
            canvas(DrawingApp::default()).width(100.0).height(100.0)
        ]
        .into()
    }
}
