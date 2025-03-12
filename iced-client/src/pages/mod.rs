mod component;
mod project;
mod workspace;

pub use component::*;
use iced::Element;
pub use project::*;
pub use workspace::*;

use crate::AppEvent;

pub trait Page {
    fn title(&self) -> &str;
    fn show(&self) -> Element<AppEvent>;
}

impl PartialEq for Box<dyn Page> {
    fn eq(&self, other: &Self) -> bool {
        // TODO: Make more robust with unique identifiers
        self.title() == other.title()
    }
}
