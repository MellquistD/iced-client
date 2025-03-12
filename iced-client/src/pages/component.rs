use super::Page;
use crate::{AppEvent, ColorExt};
use iced::{
    padding,
    widget::{self, column, container::Style, row, Button, Container, Text},
    Border, Color, Element,
    Length::Fill,
};

#[derive(Debug, Clone, PartialEq)]
pub enum ComponentPageEvent {
    PressButton,
}
impl From<ComponentPageEvent> for AppEvent {
    fn from(value: ComponentPageEvent) -> Self {
        AppEvent::Component(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct ComponentPage {
    title: String,
    num: f32,
    checked: bool,
}

impl Default for ComponentPage {
    fn default() -> Self {
        Self {
            title: "Component".into(),
            num: 0.0,
            checked: false,
        }
    }
}

// EVENTS
impl ComponentPage {
    fn run_pressbutton(&mut self) {
        self.checked = !self.checked
    }
}

impl Page for ComponentPage {
    /*************  ✨ Codeium Command ⭐  *************/
    /// Get the title of the Component page as a string
    /******  63060f41-bb6a-4136-af7d-e1cad0276996  *******/
    fn title(&self) -> &str {
        self.title.as_str()
    }
    fn run_event(&mut self, app_event: AppEvent) {
        let AppEvent::Component(event) = app_event else {
            return;
        };
        match event {
            ComponentPageEvent::PressButton => self.run_pressbutton(),
        };
    }
    fn show(&self) -> Element<AppEvent> {
        column![
            row![
                Button::new("Switch to Project Page").on_press(AppEvent::GoTo("Project".into())),
                Button::new("Switch to Workspace Page")
                    .on_press(AppEvent::GoTo("Workspace".into())),
            ],
            Text::new(self.checked),
            Button::new("Press me").on_press(ComponentPageEvent::PressButton.into()),
        ]
        .into()
    }
}
