use iced::{
    advanced::widget,
    padding,
    widget::{button, column, container::Style, row},
    Border, Color, Element,
    Length::Fill,
};

use crate::{AppEvent, ColorExt};

use super::Page;

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

impl Page for ComponentPage {
    fn title(&self) -> &str {
        self.title.as_str()
    }
    fn show(&self) -> Element<AppEvent> {
        let column_style = Style::default()
            .background(Color::TRANSPARENT)
            .border(Border::default().color(Color::BLACK).width(1.0));

        let padding = 4.0;

        // Make grid of components
        iced::widget::container(column![
            // Row 1
            row![
                iced::widget::container(button("Button"))
                    .padding(padding)
                    .style(move |_| column_style),
                //container(content),
                //container(content),
                //container(content),
                //container(content),
            ],
            // Row 2
            row![
                //container(content),
                //container(content),
                //container(content),
                //container(content),
                //container(content),
            ],
        ])
        .style(|t| Style::default().background(Color::YELLOW))
        .width(Fill)
        .height(Fill)
        .into()
    }
}
