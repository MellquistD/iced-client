mod app;
mod helpers;
mod pages;
mod widgets;

pub use app::*;
pub use helpers::*;
pub use pages::*;
pub use widgets::*;

fn main() -> iced::Result {
    iced::application(App::title, App::update, App::view)
        .antialiasing(false)
        .run()
}
