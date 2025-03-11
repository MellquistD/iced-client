use iced::advanced::graphics::core::time;
use iced::advanced::graphics::futures::subscription;
use iced::advanced::layout::Node;
use iced::advanced::Layout;
use iced::widget::container::{Catalog, Style};
use iced::widget::{button, checkbox, text};
use iced::widget::{column, slider};
use iced::{widget, Color};
use iced::{Element, Size, Theme};
use iced::{Fill, Subscription};
use widget::*;
#[derive(Default)]
struct State {
    num: u32,
    is_checked: bool,
    window_size: Size,
    active_page: Page,
}

// Pages to show
#[derive(Default, Debug, Clone)]
enum Page {
    #[default]
    MainPage,
    SecondaryPage,
}

#[derive(Debug, Clone)]
enum Message {
    GoTo(Page),
}

fn main() -> iced::Result {
    iced::application("App title", update, view).run()
}

fn update(state: &mut State, message: Message) {
    match message {
        Message::GoTo(v) => state.active_page = v,
    }
}

/// Must return a Element. Element is just a generic signature for widgets
/// The element returned by `view()` must be of same type as update. Im guessing this is the `Message` type
fn view(state: &State) -> Element<Message> {
    match state.active_page {
        Page::MainPage => page_1(state),
        Page::SecondaryPage => page_2(state),
    }
}

fn page_1(state: &State) -> Element<Message> {
    column![
        // Values
        text("Page 1"),
        row![button("Go to page 2").on_press(Message::GoTo(Page::SecondaryPage))]
    ]
    .into()
}

fn page_2(state: &State) -> Element<Message> {
    column![
        // Values
        text("Page 2"),
        row![button("Go to Main Page").on_press(Message::GoTo(Page::MainPage))]
    ]
    .into()
}
