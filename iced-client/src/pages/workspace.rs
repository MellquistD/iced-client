use iced::{
    advanced::widget,
    widget::{button, canvas, checkbox, column, container, row, text},
};

use crate::{AppEvent, DrawingApp};

use super::Page;

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

impl Page for WorkspacePage {
    fn title(&self) -> &str {
        self.title.as_str()
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
